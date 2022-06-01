#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IGPEInformation_Impl: Sized {
    fn GetName(&self, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetDisplayName(&self, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::Result<()>;
    fn GetDSPath(&self, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::Result<()>;
    fn GetOptions(&self, dwoptions: *mut u32) -> ::windows::core::Result<()>;
    fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::Result<()>;
    fn GetHint(&self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::core::Result<()>;
    fn PolicyChanged(&self, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguidsnapin: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows::core::RuntimeName for IGPEInformation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IGPEInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>() -> IGPEInformation_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetRegistryKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRegistryKey(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&hkey)).into()
        }
        unsafe extern "system" fn GetDSPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDSPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetFileSysPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFileSysPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOptions(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetType(::core::mem::transmute_copy(&gpotype)).into()
        }
        unsafe extern "system" fn GetHint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHint(::core::mem::transmute_copy(&gphint)).into()
        }
        unsafe extern "system" fn PolicyChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguidsnapin: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PolicyChanged(::core::mem::transmute_copy(&bmachine), ::core::mem::transmute_copy(&badd), ::core::mem::transmute_copy(&pguidextension), ::core::mem::transmute_copy(&pguidsnapin)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn GetDomain(&self, bstrdomain: &super::super::Foundation::BSTR, bstrdomaincontroller: &super::super::Foundation::BSTR, ldcflags: i32) -> ::windows::core::Result<IGPMDomain>;
    fn GetBackupDir(&self, bstrbackupdir: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMBackupDir>;
    fn GetSitesContainer(&self, bstrforest: &super::super::Foundation::BSTR, bstrdomain: &super::super::Foundation::BSTR, bstrdomaincontroller: &super::super::Foundation::BSTR, ldcflags: i32) -> ::windows::core::Result<IGPMSitesContainer>;
    fn GetRSOP(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: &super::super::Foundation::BSTR, lflags: i32) -> ::windows::core::Result<IGPMRSOP>;
    fn CreatePermission(&self, bstrtrustee: &super::super::Foundation::BSTR, perm: GPMPermissionType, binheritable: i16) -> ::windows::core::Result<IGPMPermission>;
    fn CreateSearchCriteria(&self) -> ::windows::core::Result<IGPMSearchCriteria>;
    fn CreateTrustee(&self, bstrtrustee: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMTrustee>;
    fn GetClientSideExtensions(&self) -> ::windows::core::Result<IGPMCSECollection>;
    fn GetConstants(&self) -> ::windows::core::Result<IGPMConstants>;
    fn GetMigrationTable(&self, bstrmigrationtablepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMMigrationTable>;
    fn CreateMigrationTable(&self) -> ::windows::core::Result<IGPMMigrationTable>;
    fn InitializeReporting(&self, bstradmpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPM_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>() -> IGPM_Vtbl {
        unsafe extern "system" fn GetDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, pigpmdomain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDomain(::core::mem::transmute(&bstrdomain), ::core::mem::transmute(&bstrdomaincontroller), ::core::mem::transmute_copy(&ldcflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pigpmdomain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupDir<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pigpmbackupdir: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackupDir(::core::mem::transmute(&bstrbackupdir)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pigpmbackupdir, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSitesContainer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrforest: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSitesContainer(::core::mem::transmute(&bstrforest), ::core::mem::transmute(&bstrdomain), ::core::mem::transmute(&bstrdomaincontroller), ::core::mem::transmute_copy(&ldcflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsitescontainer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRSOP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, ppigpmrsop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRSOP(::core::mem::transmute_copy(&gpmrsopmode), ::core::mem::transmute(&bstrnamespace), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmrsop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePermission<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, perm: GPMPermissionType, binheritable: i16, ppperm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePermission(::core::mem::transmute(&bstrtrustee), ::core::mem::transmute_copy(&perm), ::core::mem::transmute_copy(&binheritable)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppperm, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSearchCriteria<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsearchcriteria: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSearchCriteria() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsearchcriteria, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrustee<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmtrustee: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTrustee(::core::mem::transmute(&bstrtrustee)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtrustee, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientSideExtensions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmcsecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClientSideExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmcsecollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmconstants: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConstants() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmconstants, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMigrationTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppmigrationtable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMigrationTable(::core::mem::transmute(&bstrmigrationtablepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmigrationtable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMigrationTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmigrationtable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMigrationTable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmigrationtable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeReporting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeReporting(::core::mem::transmute(&bstradmpath)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn GetBackupDirEx(&self, bstrbackupdir: &super::super::Foundation::BSTR, backupdirtype: GPMBackupType) -> ::windows::core::Result<IGPMBackupDirEx>;
    fn InitializeReportingEx(&self, bstradmpath: &super::super::Foundation::BSTR, reportingoptions: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPM2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPM2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM2_Impl, const OFFSET: isize>() -> IGPM2_Vtbl {
        unsafe extern "system" fn GetBackupDirEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackupDirEx(::core::mem::transmute(&bstrbackupdir), ::core::mem::transmute_copy(&backupdirtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmbackupdirex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeReportingEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPM2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportingoptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeReportingEx(::core::mem::transmute(&bstradmpath), ::core::mem::transmute_copy(&reportingoptions)).into()
        }
        Self {
            base__: IGPM_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMAsyncCancel {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMAsyncCancel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMAsyncCancel_Impl, const OFFSET: isize>() -> IGPMAsyncCancel_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMAsyncCancel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Cancel: Cancel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMAsyncCancel as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMAsyncProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn Status(&self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: &::core::option::Option<IGPMStatusMsgCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMAsyncProgress {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMAsyncProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMAsyncProgress_Impl, const OFFSET: isize>() -> IGPMAsyncProgress_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMAsyncProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows::core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Status(::core::mem::transmute_copy(&lprogressnumerator), ::core::mem::transmute_copy(&lprogressdenominator), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&presult), ::core::mem::transmute(&ppigpmstatusmsgcollection)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Status: Status::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMAsyncProgress as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackup_Impl: Sized + super::Com::IDispatch_Impl {
    fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPOID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPODomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPODisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Timestamp(&self) -> ::windows::core::Result<f64>;
    fn Comment(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BackupDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMBackup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>() -> IGPMBackup_Vtbl {
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPOID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GPOID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GPODisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Comment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupDir<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackupDir() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMBackupCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>() -> IGPMBackupCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmbackup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmbackup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMBackupCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupDir_Impl: Sized + super::Com::IDispatch_Impl {
    fn BackupDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetBackup(&self, bstrid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMBackup>;
    fn SearchBackups(&self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMBackupCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMBackupDir {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupDir_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: isize>() -> IGPMBackupDir_Vtbl {
        unsafe extern "system" fn BackupDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackupDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbackup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackup(::core::mem::transmute(&bstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbackup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchBackups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmbackupcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchBackups(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmbackupcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn BackupDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BackupType(&self) -> ::windows::core::Result<GPMBackupType>;
    fn GetBackup(&self, bstrid: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SearchBackups(&self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMBackupDirEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupDirEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>() -> IGPMBackupDirEx_Vtbl {
        unsafe extern "system" fn BackupDir<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackupDir() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbackupdir, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackupType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgpmbackuptype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbackup: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackup(::core::mem::transmute(&bstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbackup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchBackups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, pvarbackupcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchBackups(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbackupcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMCSECollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMCSECollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: isize>() -> IGPMCSECollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmcses: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmcses, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMCSECollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMClientSideExtension_Impl: Sized + super::Com::IDispatch_Impl {
    fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsUserEnabled(&self) -> ::windows::core::Result<i16>;
    fn IsComputerEnabled(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMClientSideExtension {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMClientSideExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>() -> IGPMClientSideExtension_Vtbl {
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUserEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsComputerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn PermGPOApply(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermGPORead(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermGPOEdit(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermGPOEditSecurityAndDelete(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermGPOCustom(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermWMIFilterEdit(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermWMIFilterFullControl(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermWMIFilterCustom(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMLink(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMLogging(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMPlanning(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMGPOCreate(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMWMICreate(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermSOMWMIFullControl(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn SearchPropertyGPOPermissions(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOEffectivePermissions(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODisplayName(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOWMIFilter(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOID(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOComputerExtensions(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOUserExtensions(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertySOMLinks(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODomain(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyBackupMostRecent(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchOpEquals(&self) -> ::windows::core::Result<GPMSearchOperation>;
    fn SearchOpContains(&self) -> ::windows::core::Result<GPMSearchOperation>;
    fn SearchOpNotContains(&self) -> ::windows::core::Result<GPMSearchOperation>;
    fn SearchOpNotEquals(&self) -> ::windows::core::Result<GPMSearchOperation>;
    fn UsePDC(&self) -> ::windows::core::Result<i32>;
    fn UseAnyDC(&self) -> ::windows::core::Result<i32>;
    fn DoNotUseW2KDC(&self) -> ::windows::core::Result<i32>;
    fn SOMSite(&self) -> ::windows::core::Result<GPMSOMType>;
    fn SOMDomain(&self) -> ::windows::core::Result<GPMSOMType>;
    fn SOMOU(&self) -> ::windows::core::Result<GPMSOMType>;
    fn get_SecurityFlags(&self, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16) -> ::windows::core::Result<i32>;
    fn DoNotValidateDC(&self) -> ::windows::core::Result<i32>;
    fn ReportHTML(&self) -> ::windows::core::Result<GPMReportType>;
    fn ReportXML(&self) -> ::windows::core::Result<GPMReportType>;
    fn RSOPModeUnknown(&self) -> ::windows::core::Result<GPMRSOPMode>;
    fn RSOPModePlanning(&self) -> ::windows::core::Result<GPMRSOPMode>;
    fn RSOPModeLogging(&self) -> ::windows::core::Result<GPMRSOPMode>;
    fn EntryTypeUser(&self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeComputer(&self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeLocalGroup(&self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeGlobalGroup(&self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeUniversalGroup(&self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeUNCPath(&self) -> ::windows::core::Result<GPMEntryType>;
    fn EntryTypeUnknown(&self) -> ::windows::core::Result<GPMEntryType>;
    fn DestinationOptionSameAsSource(&self) -> ::windows::core::Result<GPMDestinationOption>;
    fn DestinationOptionNone(&self) -> ::windows::core::Result<GPMDestinationOption>;
    fn DestinationOptionByRelativeName(&self) -> ::windows::core::Result<GPMDestinationOption>;
    fn DestinationOptionSet(&self) -> ::windows::core::Result<GPMDestinationOption>;
    fn MigrationTableOnly(&self) -> ::windows::core::Result<i32>;
    fn ProcessSecurity(&self) -> ::windows::core::Result<i32>;
    fn RsopLoggingNoComputer(&self) -> ::windows::core::Result<i32>;
    fn RsopLoggingNoUser(&self) -> ::windows::core::Result<i32>;
    fn RsopPlanningAssumeSlowLink(&self) -> ::windows::core::Result<i32>;
    fn get_RsopPlanningLoopbackOption(&self, vbmerge: i16) -> ::windows::core::Result<i32>;
    fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> ::windows::core::Result<i32>;
    fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMConstants {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMConstants_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>() -> IGPMConstants_Vtbl {
        unsafe extern "system" fn PermGPOApply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermGPOApply() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPORead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermGPORead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOEdit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermGPOEdit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOEditSecurityAndDelete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermGPOEditSecurityAndDelete() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOCustom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermGPOCustom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterEdit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermWMIFilterEdit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterFullControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermWMIFilterFullControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterCustom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermWMIFilterCustom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermSOMLink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMLogging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermSOMLogging() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMPlanning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermSOMPlanning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMGPOCreate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermSOMGPOCreate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMWMICreate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermSOMWMICreate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMWMIFullControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermSOMWMIFullControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOPermissions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyGPOPermissions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOEffectivePermissions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyGPOEffectivePermissions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPODisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyGPODisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOWMIFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyGPOWMIFilter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyGPOID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOComputerExtensions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyGPOComputerExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOUserExtensions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyGPOUserExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertySOMLinks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertySOMLinks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPODomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyGPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyBackupMostRecent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyBackupMostRecent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpEquals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchOpEquals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpContains<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchOpContains() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpNotContains<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchOpNotContains() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpNotEquals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchOpNotEquals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsePDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UsePDC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseAnyDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseAnyDC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotUseW2KDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoNotUseW2KDC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMSite<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SOMSite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SOMDomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMOU<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SOMOU() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_SecurityFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_SecurityFlags(::core::mem::transmute_copy(&vbowner), ::core::mem::transmute_copy(&vbgroup), ::core::mem::transmute_copy(&vbdacl), ::core::mem::transmute_copy(&vbsacl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotValidateDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoNotValidateDC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportHTML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReportHTML() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportXML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReportXML() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModeUnknown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RSOPModeUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModePlanning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RSOPModePlanning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModeLogging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RSOPModeLogging() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EntryTypeUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeComputer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EntryTypeComputer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeLocalGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EntryTypeLocalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeGlobalGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EntryTypeGlobalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUniversalGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EntryTypeUniversalGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUNCPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EntryTypeUNCPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUnknown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EntryTypeUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionSameAsSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationOptionSameAsSource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionNone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationOptionNone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionByRelativeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationOptionByRelativeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationOptionSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MigrationTableOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MigrationTableOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessSecurity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProcessSecurity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopLoggingNoComputer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RsopLoggingNoComputer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopLoggingNoUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RsopLoggingNoUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeSlowLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RsopPlanningAssumeSlowLink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RsopPlanningLoopbackOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbmerge: i16, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_RsopPlanningLoopbackOption(::core::mem::transmute_copy(&vbmerge)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeUserWQLFilterTrue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RsopPlanningAssumeUserWQLFilterTrue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeCompWQLFilterTrue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RsopPlanningAssumeCompWQLFilterTrue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
            get_SecurityFlags: get_SecurityFlags::<Identity, Impl, OFFSET>,
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
            get_RsopPlanningLoopbackOption: get_RsopPlanningLoopbackOption::<Identity, Impl, OFFSET>,
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
    fn BackupTypeGPO(&self) -> ::windows::core::Result<GPMBackupType>;
    fn BackupTypeStarterGPO(&self) -> ::windows::core::Result<GPMBackupType>;
    fn StarterGPOTypeSystem(&self) -> ::windows::core::Result<GPMStarterGPOType>;
    fn StarterGPOTypeCustom(&self) -> ::windows::core::Result<GPMStarterGPOType>;
    fn SearchPropertyStarterGPOPermissions(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOEffectivePermissions(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODisplayName(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOID(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODomain(&self) -> ::windows::core::Result<GPMSearchProperty>;
    fn PermStarterGPORead(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermStarterGPOEdit(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermStarterGPOFullControl(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn PermStarterGPOCustom(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn ReportLegacy(&self) -> ::windows::core::Result<GPMReportingOptions>;
    fn ReportComments(&self) -> ::windows::core::Result<GPMReportingOptions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMConstants2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMConstants2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>() -> IGPMConstants2_Vtbl {
        unsafe extern "system" fn BackupTypeGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackupTypeGPO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupTypeStarterGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackupTypeStarterGPO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOTypeSystem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StarterGPOTypeSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOTypeCustom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StarterGPOTypeCustom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOPermissions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyStarterGPOPermissions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOEffectivePermissions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyStarterGPOEffectivePermissions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyStarterGPODisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyStarterGPOID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchPropertyStarterGPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPORead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermStarterGPORead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOEdit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermStarterGPOEdit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOFullControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermStarterGPOFullControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOCustom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermStarterGPOCustom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportLegacy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReportLegacy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportComments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReportComments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IGPMConstants_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn DomainController(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateGPO(&self) -> ::windows::core::Result<IGPMGPO>;
    fn GetGPO(&self, bstrguid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMGPO>;
    fn SearchGPOs(&self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMGPOCollection>;
    fn RestoreGPO(&self, pigpmbackup: &::core::option::Option<IGPMBackup>, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GetSOM(&self, bstrpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMSOM>;
    fn SearchSOMs(&self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMSOMCollection>;
    fn GetWMIFilter(&self, bstrpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMWMIFilter>;
    fn SearchWMIFilters(&self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMWMIFilterCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMDomain {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>() -> IGPMDomain_Vtbl {
        unsafe extern "system" fn DomainController<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainController() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Domain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewgpo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGPO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewgpo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgpo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGPO(::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgpo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchGPOs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmgpocollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchGPOs(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmgpocollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmbackup: *mut ::core::ffi::c_void, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreGPO(::core::mem::transmute(&pigpmbackup), ::core::mem::transmute_copy(&ldcflags), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GetSOM<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSOM(::core::mem::transmute(&bstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsom, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchSOMs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmsomcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchSOMs(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsomcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwmifilter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWMIFilter(::core::mem::transmute(&bstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwmifilter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchWMIFilters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmwmifiltercollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchWMIFilters(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmwmifiltercollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn CreateStarterGPO(&self) -> ::windows::core::Result<IGPMStarterGPO>;
    fn CreateGPOFromStarterGPO(&self, pgpotemplate: &::core::option::Option<IGPMStarterGPO>) -> ::windows::core::Result<IGPMGPO>;
    fn GetStarterGPO(&self, bstrguid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMStarterGPO>;
    fn SearchStarterGPOs(&self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMStarterGPOCollection>;
    fn LoadStarterGPO(&self, bstrloadfile: &super::super::Foundation::BSTR, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn RestoreStarterGPO(&self, pigpmtmplbackup: &::core::option::Option<IGPMStarterGPOBackup>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMDomain2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>() -> IGPMDomain2_Vtbl {
        unsafe extern "system" fn CreateStarterGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewtemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStarterGPO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewtemplate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPOFromStarterGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpotemplate: *mut ::core::ffi::c_void, ppnewgpo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGPOFromStarterGPO(::core::mem::transmute(&pgpotemplate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewgpo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStarterGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStarterGPO(::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptemplate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchStarterGPOs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmtemplatecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchStarterGPOs(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtemplatecollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStarterGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrloadfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadStarterGPO(::core::mem::transmute(&bstrloadfile), ::core::mem::transmute_copy(&boverwrite), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn RestoreStarterGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmtmplbackup: *mut ::core::ffi::c_void, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreStarterGPO(::core::mem::transmute(&pigpmtmplbackup), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        Self {
            base__: IGPMDomain_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn InfrastructureDC(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInfrastructureDC(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMDomain3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMDomain3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>() -> IGPMDomain3_Vtbl {
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn InfrastructureDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InfrastructureDC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInfrastructureDC(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInfrastructureFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: IGPMDomain2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DomainName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreationTime(&self) -> ::windows::core::Result<f64>;
    fn ModificationTime(&self) -> ::windows::core::Result<f64>;
    fn UserDSVersionNumber(&self) -> ::windows::core::Result<i32>;
    fn ComputerDSVersionNumber(&self) -> ::windows::core::Result<i32>;
    fn UserSysvolVersionNumber(&self) -> ::windows::core::Result<i32>;
    fn ComputerSysvolVersionNumber(&self) -> ::windows::core::Result<i32>;
    fn GetWMIFilter(&self) -> ::windows::core::Result<IGPMWMIFilter>;
    fn SetWMIFilter(&self, pigpmwmifilter: &::core::option::Option<IGPMWMIFilter>) -> ::windows::core::Result<()>;
    fn SetUserEnabled(&self, vbenabled: i16) -> ::windows::core::Result<()>;
    fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows::core::Result<()>;
    fn IsUserEnabled(&self) -> ::windows::core::Result<i16>;
    fn IsComputerEnabled(&self) -> ::windows::core::Result<i16>;
    fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: &::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Backup(&self, bstrbackupdir: &super::super::Foundation::BSTR, bstrcomment: &super::super::Foundation::BSTR, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn Import(&self, lflags: i32, pigpmbackup: &::core::option::Option<IGPMBackup>, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
    fn CopyTo(&self, lflags: i32, pigpmdomain: &::core::option::Option<IGPMDomain>, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn SetSecurityDescriptor(&self, lflags: i32, psd: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn IsACLConsistent(&self) -> ::windows::core::Result<i16>;
    fn MakeACLConsistent(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMGPO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>() -> IGPMGPO_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModificationTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModificationTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDSVersionNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserDSVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerDSVersionNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputerDSVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSysvolVersionNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserSysvolVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerSysvolVersionNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputerSysvolVersionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmwmifilter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWMIFilter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmwmifilter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWMIFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmwmifilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWMIFilter(::core::mem::transmute(&pigpmwmifilter)).into()
        }
        unsafe extern "system" fn SetUserEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserEnabled(::core::mem::transmute_copy(&vbenabled)).into()
        }
        unsafe extern "system" fn SetComputerEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetComputerEnabled(::core::mem::transmute_copy(&vbenabled)).into()
        }
        unsafe extern "system" fn IsUserEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUserEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsComputerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Backup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Backup(::core::mem::transmute(&bstrbackupdir), ::core::mem::transmute(&bstrcomment), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn Import<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmbackup: *mut ::core::ffi::c_void, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Import(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pigpmbackup), ::core::mem::transmute_copy(&pvarmigrationtable), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmdomain: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyTo(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pigpmdomain), ::core::mem::transmute_copy(&pvarnewdisplayname), ::core::mem::transmute_copy(&pvarmigrationtable), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, psd: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityDescriptor(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&psd)).into()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppsd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityDescriptor(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsACLConsistent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbconsistent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsACLConsistent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbconsistent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeACLConsistent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeACLConsistent().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMGPO2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO2_Impl, const OFFSET: isize>() -> IGPMGPO2_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&newval)).into()
        }
        Self {
            base__: IGPMGPO_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn InfrastructureDC(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInfrastructureDC(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMGPO3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPO3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: isize>() -> IGPMGPO3_Vtbl {
        unsafe extern "system" fn InfrastructureDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InfrastructureDC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInfrastructureDC(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInfrastructureFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: IGPMGPO2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMGPOCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>() -> IGPMGPOCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmgpos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmgpos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOLink_Impl: Sized + super::Com::IDispatch_Impl {
    fn GPOID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GPODomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, newval: i16) -> ::windows::core::Result<()>;
    fn Enforced(&self) -> ::windows::core::Result<i16>;
    fn SetEnforced(&self, newval: i16) -> ::windows::core::Result<()>;
    fn SOMLinkOrder(&self) -> ::windows::core::Result<i32>;
    fn SOM(&self) -> ::windows::core::Result<IGPMSOM>;
    fn Delete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMGPOLink {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOLink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>() -> IGPMGPOLink_Vtbl {
        unsafe extern "system" fn GPOID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GPOID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GPODomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Enforced<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enforced() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnforced<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnforced(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SOMLinkOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SOMLinkOrder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOM<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SOM() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsom, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMGPOLinksCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOLinksCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>() -> IGPMGPOLinksCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmlinks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmlinks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMGPOLinksCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMapEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn Source(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Destination(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DestinationOption(&self) -> ::windows::core::Result<GPMDestinationOption>;
    fn EntryType(&self) -> ::windows::core::Result<GPMEntryType>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMMapEntry {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMapEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>() -> IGPMMapEntry_Vtbl {
        unsafe extern "system" fn Source<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Source() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Destination() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdestination, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationOption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgpmdestoption, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EntryType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgpmentrytype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMMapEntryCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMapEntryCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>() -> IGPMMapEntryCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMMapEntryCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMigrationTable_Impl: Sized + super::Com::IDispatch_Impl {
    fn Save(&self, bstrmigrationtablepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Add(&self, lflags: i32, var: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddEntry(&self, bstrsource: &super::super::Foundation::BSTR, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMMapEntry>;
    fn GetEntry(&self, bstrsource: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMMapEntry>;
    fn DeleteEntry(&self, bstrsource: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UpdateDestination(&self, bstrsource: &super::super::Foundation::BSTR, pvardestination: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMMapEntry>;
    fn Validate(&self) -> ::windows::core::Result<IGPMResult>;
    fn GetEntries(&self) -> ::windows::core::Result<IGPMMapEntryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMMigrationTable {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMigrationTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>() -> IGPMMigrationTable_Vtbl {
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute(&bstrmigrationtablepath)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn AddEntry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddEntry(::core::mem::transmute(&bstrsource), ::core::mem::transmute_copy(&gpmentrytype), ::core::mem::transmute_copy(&pvardestination)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppentry, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEntry(::core::mem::transmute(&bstrsource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppentry, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteEntry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteEntry(::core::mem::transmute(&bstrsource)).into()
        }
        unsafe extern "system" fn UpdateDestination<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardestination: *const super::Com::VARIANT, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UpdateDestination(::core::mem::transmute(&bstrsource), ::core::mem::transmute_copy(&pvardestination)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppentry, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Validate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppentries: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEntries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppentries, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Inherited(&self) -> ::windows::core::Result<i16>;
    fn Inheritable(&self) -> ::windows::core::Result<i16>;
    fn Denied(&self) -> ::windows::core::Result<i16>;
    fn Permission(&self) -> ::windows::core::Result<GPMPermissionType>;
    fn Trustee(&self) -> ::windows::core::Result<IGPMTrustee>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMPermission {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMPermission_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>() -> IGPMPermission_Vtbl {
        unsafe extern "system" fn Inherited<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Inherited() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inheritable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Inheritable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Denied<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Denied() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Permission<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Permission() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trustee<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtrustee: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Trustee() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtrustee, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Mode(&self) -> ::windows::core::Result<GPMRSOPMode>;
    fn Namespace(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoggingComputer(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoggingComputer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoggingUser(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoggingUser(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLoggingFlags(&self, lval: i32) -> ::windows::core::Result<()>;
    fn LoggingFlags(&self) -> ::windows::core::Result<i32>;
    fn SetPlanningFlags(&self, lval: i32) -> ::windows::core::Result<()>;
    fn PlanningFlags(&self) -> ::windows::core::Result<i32>;
    fn SetPlanningDomainController(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningDomainController(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningSiteName(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningSiteName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningUser(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningUser(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningUserSOM(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningUserSOM(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningUserWMIFilters(&self, varval: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningUserWMIFilters(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetPlanningUserSecurityGroups(&self, varval: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningUserSecurityGroups(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetPlanningComputer(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningComputer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningComputerSOM(&self, bstrval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlanningComputerSOM(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPlanningComputerWMIFilters(&self, varval: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningComputerWMIFilters(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetPlanningComputerSecurityGroups(&self, varval: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlanningComputerSecurityGroups(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn LoggingEnumerateUsers(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CreateQueryResults(&self) -> ::windows::core::Result<()>;
    fn ReleaseQueryResults(&self) -> ::windows::core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMRSOP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMRSOP_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>() -> IGPMRSOP_Vtbl {
        unsafe extern "system" fn Mode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Mode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Namespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingComputer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLoggingComputer(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn LoggingComputer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoggingComputer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLoggingUser(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn LoggingUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoggingUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLoggingFlags(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn LoggingFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoggingFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningFlags(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn PlanningFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningDomainController<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningDomainController(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningDomainController<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningDomainController() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningSiteName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningSiteName(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningSiteName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningSiteName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningUser(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserSOM<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningUserSOM(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningUserSOM<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningUserSOM() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserWMIFilters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningUserWMIFilters(::core::mem::transmute(&varval)).into()
        }
        unsafe extern "system" fn PlanningUserWMIFilters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningUserWMIFilters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserSecurityGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningUserSecurityGroups(::core::mem::transmute(&varval)).into()
        }
        unsafe extern "system" fn PlanningUserSecurityGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningUserSecurityGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningComputer(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningComputer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningComputer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerSOM<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningComputerSOM(::core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningComputerSOM<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningComputerSOM() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerWMIFilters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningComputerWMIFilters(::core::mem::transmute(&varval)).into()
        }
        unsafe extern "system" fn PlanningComputerWMIFilters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningComputerWMIFilters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerSecurityGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlanningComputerSecurityGroups(::core::mem::transmute(&varval)).into()
        }
        unsafe extern "system" fn PlanningComputerSecurityGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlanningComputerSecurityGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoggingEnumerateUsers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoggingEnumerateUsers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateQueryResults().into()
        }
        unsafe extern "system" fn ReleaseQueryResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseQueryResults().into()
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Status(&self) -> ::windows::core::Result<IGPMStatusMsgCollection>;
    fn Result(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn OverallStatus(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: isize>() -> IGPMResult_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmstatusmsgcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmstatusmsgcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarresult: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Result() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OverallStatus().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn GPOInheritanceBlocked(&self) -> ::windows::core::Result<i16>;
    fn SetGPOInheritanceBlocked(&self, newval: i16) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateGPOLink(&self, llinkpos: i32, pgpo: &::core::option::Option<IGPMGPO>) -> ::windows::core::Result<IGPMGPOLink>;
    fn Type(&self) -> ::windows::core::Result<GPMSOMType>;
    fn GetGPOLinks(&self) -> ::windows::core::Result<IGPMGPOLinksCollection>;
    fn GetInheritedGPOLinks(&self) -> ::windows::core::Result<IGPMGPOLinksCollection>;
    fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: &::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMSOM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSOM_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>() -> IGPMSOM_Vtbl {
        unsafe extern "system" fn GPOInheritanceBlocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GPOInheritanceBlocked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGPOInheritanceBlocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGPOInheritanceBlocked(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPOLink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llinkpos: i32, pgpo: *mut ::core::ffi::c_void, ppnewgpolink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGPOLink(::core::mem::transmute_copy(&llinkpos), ::core::mem::transmute(&pgpo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewgpolink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPOLinks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGPOLinks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgpolinks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInheritedGPOLinks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInheritedGPOLinks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgpolinks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMSOMCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSOMCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>() -> IGPMSOMCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsom, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSOMCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSearchCriteria_Impl: Sized + super::Com::IDispatch_Impl {
    fn Add(&self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMSearchCriteria {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSearchCriteria_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSearchCriteria_Impl, const OFFSET: isize>() -> IGPMSearchCriteria_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSearchCriteria_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute_copy(&searchproperty), ::core::mem::transmute_copy(&searchoperation), ::core::mem::transmute(&varvalue)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Add: Add::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMSearchCriteria as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSecurityInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
    fn Add(&self, pperm: &::core::option::Option<IGPMPermission>) -> ::windows::core::Result<()>;
    fn Remove(&self, pperm: &::core::option::Option<IGPMPermission>) -> ::windows::core::Result<()>;
    fn RemoveTrustee(&self, bstrtrustee: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMSecurityInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSecurityInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>() -> IGPMSecurityInfo_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&pperm)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&pperm)).into()
        }
        unsafe extern "system" fn RemoveTrustee<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveTrustee(::core::mem::transmute(&bstrtrustee)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn DomainController(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Forest(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSite(&self, bstrsitename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMSOM>;
    fn SearchSites(&self, pigpmsearchcriteria: &::core::option::Option<IGPMSearchCriteria>) -> ::windows::core::Result<IGPMSOMCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMSitesContainer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSitesContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>() -> IGPMSitesContainer_Vtbl {
        unsafe extern "system" fn DomainController<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainController() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Domain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Forest() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSite<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsitename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsom: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSite(::core::mem::transmute(&bstrsitename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsom, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchSites<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmsomcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchSites(::core::mem::transmute(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsomcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Author(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Product(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreationTime(&self) -> ::windows::core::Result<f64>;
    fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ModifiedTime(&self) -> ::windows::core::Result<f64>;
    fn Type(&self) -> ::windows::core::Result<GPMStarterGPOType>;
    fn ComputerVersion(&self) -> ::windows::core::Result<u16>;
    fn UserVersion(&self) -> ::windows::core::Result<u16>;
    fn StarterGPOVersion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Save(&self, bstrsavefile: &super::super::Foundation::BSTR, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn Backup(&self, bstrbackupdir: &super::super::Foundation::BSTR, bstrcomment: &super::super::Foundation::BSTR, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn CopyTo(&self, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMResult>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows::core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
    fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: &::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMStarterGPO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>() -> IGPMStarterGPO_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Author() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Product<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Product() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModifiedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputerVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StarterGPOVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsavefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute(&bstrsavefile), ::core::mem::transmute_copy(&boverwrite), ::core::mem::transmute_copy(&bsaveassystem), ::core::mem::transmute_copy(&bstrlanguage), ::core::mem::transmute_copy(&bstrauthor), ::core::mem::transmute_copy(&bstrproduct), ::core::mem::transmute_copy(&bstruniqueid), ::core::mem::transmute_copy(&bstrversion), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult))
                .into()
        }
        unsafe extern "system" fn Backup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Backup(::core::mem::transmute(&bstrbackupdir), ::core::mem::transmute(&bstrcomment), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyTo(::core::mem::transmute_copy(&pvarnewdisplayname), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn BackupDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Comment(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StarterGPOID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Timestamp(&self) -> ::windows::core::Result<f64>;
    fn Type(&self) -> ::windows::core::Result<GPMStarterGPOType>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows::core::Result<()>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMStarterGPOBackup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOBackup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>() -> IGPMStarterGPOBackup_Vtbl {
        unsafe extern "system" fn BackupDir<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackupDir() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbackupdir, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Comment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcomment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisplayname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplatedomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Domain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtemplatedomain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtemplateid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StarterGPOID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtemplateid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptimestamp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateReport(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into()
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateReportToFile(::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMStarterGPOBackupCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOBackupCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>() -> IGPMStarterGPOBackupCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtmplbackup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtmplbackup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackupCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMStarterGPOCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>() -> IGPMStarterGPOCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppigpmtemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtemplates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStarterGPOCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStatusMessage_Impl: Sized + super::Com::IDispatch_Impl {
    fn ObjectPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ErrorCode(&self) -> ::windows::core::Result<()>;
    fn ExtensionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SettingsName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn OperationCode(&self) -> ::windows::core::Result<()>;
    fn Message(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMStatusMessage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStatusMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>() -> IGPMStatusMessage_Vtbl {
        unsafe extern "system" fn ObjectPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ObjectPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ErrorCode().into()
        }
        unsafe extern "system" fn ExtensionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtensionName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettingsName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SettingsName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OperationCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OperationCode().into()
        }
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Message() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMStatusMsgCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStatusMsgCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>() -> IGPMStatusMsgCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMStatusMsgCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMTrustee_Impl: Sized + super::Com::IDispatch_Impl {
    fn TrusteeSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeDomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeDSPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TrusteeType(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMTrustee {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMTrustee_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>() -> IGPMTrustee_Vtbl {
        unsafe extern "system" fn TrusteeSid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TrusteeSid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TrusteeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TrusteeDomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeDSPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TrusteeDSPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TrusteeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetQueryList(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetSecurityInfo(&self) -> ::windows::core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: &::core::option::Option<IGPMSecurityInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMWMIFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMWMIFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>() -> IGPMWMIFilter_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqrylist: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQueryList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pqrylist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityInfo(::core::mem::transmute(&psecurityinfo)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGPMWMIFilterCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMWMIFilterCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>() -> IGPMWMIFilterCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGPMWMIFilterCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
pub trait IGroupPolicyObject_Impl: Sized {
    fn New(&self, pszdomainname: &::windows::core::PCWSTR, pszdisplayname: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn OpenDSGPO(&self, pszpath: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn OpenLocalMachineGPO(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn OpenRemoteMachineGPO(&self, pszcomputername: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn Save(&self, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn GetName(&self, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetDisplayName(&self, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn SetDisplayName(&self, pszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetPath(&self, pszpath: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetDSPath(&self, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::Result<()>;
    fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::Result<()>;
    fn GetOptions(&self, dwoptions: *mut u32) -> ::windows::core::Result<()>;
    fn SetOptions(&self, dwoptions: u32, dwmask: u32) -> ::windows::core::Result<()>;
    fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::Result<()>;
    fn GetMachineName(&self, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetPropertySheetPages(&self, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl ::windows::core::RuntimeName for IGroupPolicyObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl IGroupPolicyObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>() -> IGroupPolicyObject_Vtbl {
        unsafe extern "system" fn New<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdomainname: ::windows::core::PCWSTR, pszdisplayname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.New(::core::mem::transmute(&pszdomainname), ::core::mem::transmute(&pszdisplayname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenDSGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenDSGPO(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenLocalMachineGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenLocalMachineGPO(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenRemoteMachineGPO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcomputername: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenRemoteMachineGPO(::core::mem::transmute(&pszcomputername), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows::core::GUID, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute_copy(&bmachine), ::core::mem::transmute_copy(&badd), ::core::mem::transmute_copy(&pguidextension), ::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPath(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDSPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDSPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetFileSysPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows::core::PWSTR, cchmaxpath: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFileSysPath(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetRegistryKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRegistryKey(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&hkey)).into()
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOptions(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOptions(::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetType(::core::mem::transmute_copy(&gpotype)).into()
        }
        unsafe extern "system" fn GetMachineName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMachineName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetPropertySheetPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertySheetPages(::core::mem::transmute_copy(&hpages), ::core::mem::transmute_copy(&upagecount)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
pub trait IRSOPInformation_Impl: Sized {
    fn GetNamespace(&self, dwsection: u32, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::Result<()>;
    fn GetFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetEventLogEntryText(&self, pszeventsource: &::windows::core::PCWSTR, pszeventlogname: &::windows::core::PCWSTR, pszeventtime: &::windows::core::PCWSTR, dweventid: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IRSOPInformation {}
impl IRSOPInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: isize>() -> IRSOPInformation_Vtbl {
        unsafe extern "system" fn GetNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszname: ::windows::core::PWSTR, cchmaxlength: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNamespace(::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetEventLogEntryText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszeventsource: ::windows::core::PCWSTR, pszeventlogname: ::windows::core::PCWSTR, pszeventtime: ::windows::core::PCWSTR, dweventid: u32, ppsztext: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventLogEntryText(::core::mem::transmute(&pszeventsource), ::core::mem::transmute(&pszeventlogname), ::core::mem::transmute(&pszeventtime), ::core::mem::transmute_copy(&dweventid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNamespace: GetNamespace::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetEventLogEntryText: GetEventLogEntryText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRSOPInformation as ::windows::core::Interface>::IID
    }
}
