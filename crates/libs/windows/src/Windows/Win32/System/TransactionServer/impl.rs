#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICatalog_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Connect(&self, bstrconnectstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn MajorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()>;
    fn MinorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICatalog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalog_Impl, const OFFSET: isize>() -> ICatalog_Vtbl {
        unsafe extern "system" fn GetCollection<Identity: ::windows::core::IUnknownImpl, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCollection(::core::mem::transmute(&bstrcollname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Connect(::core::mem::transmute(&bstrconnectstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows::core::IUnknownImpl, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MajorVersion(::core::mem::transmute_copy(&retval)).into()
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows::core::IUnknownImpl, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MinorVersion(::core::mem::transmute_copy(&retval)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCollection: GetCollection::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalog as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComponentUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallComponent(&self, bstrdllfile: &super::super::Foundation::BSTR, bstrtypelibfile: &super::super::Foundation::BSTR, bstrproxystubdllfile: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImportComponent(&self, bstrclsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImportComponentByName(&self, bstrprogid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetCLSIDs(&self, bstrdllfile: &super::super::Foundation::BSTR, bstrtypelibfile: &super::super::Foundation::BSTR, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComponentUtil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentUtil_Impl, const OFFSET: isize>() -> IComponentUtil_Vtbl {
        unsafe extern "system" fn InstallComponent<Identity: ::windows::core::IUnknownImpl, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrproxystubdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstallComponent(::core::mem::transmute(&bstrdllfile), ::core::mem::transmute(&bstrtypelibfile), ::core::mem::transmute(&bstrproxystubdllfile)).into()
        }
        unsafe extern "system" fn ImportComponent<Identity: ::windows::core::IUnknownImpl, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ImportComponent(::core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn ImportComponentByName<Identity: ::windows::core::IUnknownImpl, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ImportComponentByName(::core::mem::transmute(&bstrprogid)).into()
        }
        unsafe extern "system" fn GetCLSIDs<Identity: ::windows::core::IUnknownImpl, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtypelibfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCLSIDs(::core::mem::transmute(&bstrdllfile), ::core::mem::transmute(&bstrtypelibfile), ::core::mem::transmute_copy(&aclsids)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            InstallComponent: InstallComponent::<Identity, Impl, OFFSET>,
            ImportComponent: ImportComponent::<Identity, Impl, OFFSET>,
            ImportComponentByName: ImportComponentByName::<Identity, Impl, OFFSET>,
            GetCLSIDs: GetCLSIDs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentUtil as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPackageUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallPackage(&self, bstrpackagefile: &super::super::Foundation::BSTR, bstrinstallpath: &super::super::Foundation::BSTR, loptions: i32) -> ::windows::core::Result<()>;
    fn ExportPackage(&self, bstrpackageid: &super::super::Foundation::BSTR, bstrpackagefile: &super::super::Foundation::BSTR, loptions: i32) -> ::windows::core::Result<()>;
    fn ShutdownPackage(&self, bstrpackageid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPackageUtil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUtil_Impl, const OFFSET: isize>() -> IPackageUtil_Vtbl {
        unsafe extern "system" fn InstallPackage<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinstallpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstallPackage(::core::mem::transmute(&bstrpackagefile), ::core::mem::transmute(&bstrinstallpath), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn ExportPackage<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagefile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExportPackage(::core::mem::transmute(&bstrpackageid), ::core::mem::transmute(&bstrpackagefile), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn ShutdownPackage<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShutdownPackage(::core::mem::transmute(&bstrpackageid)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            InstallPackage: InstallPackage::<Identity, Impl, OFFSET>,
            ExportPackage: ExportPackage::<Identity, Impl, OFFSET>,
            ShutdownPackage: ShutdownPackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageUtil as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRemoteComponentUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallRemoteComponent(&self, bstrserver: &super::super::Foundation::BSTR, bstrpackageid: &super::super::Foundation::BSTR, bstrclsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InstallRemoteComponentByName(&self, bstrserver: &super::super::Foundation::BSTR, bstrpackagename: &super::super::Foundation::BSTR, bstrprogid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteComponentUtil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteComponentUtil_Impl, const OFFSET: isize>() -> IRemoteComponentUtil_Vtbl {
        unsafe extern "system" fn InstallRemoteComponent<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstallRemoteComponent(::core::mem::transmute(&bstrserver), ::core::mem::transmute(&bstrpackageid), ::core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn InstallRemoteComponentByName<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpackagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstallRemoteComponentByName(::core::mem::transmute(&bstrserver), ::core::mem::transmute(&bstrpackagename), ::core::mem::transmute(&bstrprogid)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            InstallRemoteComponent: InstallRemoteComponent::<Identity, Impl, OFFSET>,
            InstallRemoteComponentByName: InstallRemoteComponentByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteComponentUtil as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRoleAssociationUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn AssociateRole(&self, bstrroleid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AssociateRoleByName(&self, bstrrolename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRoleAssociationUtil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoleAssociationUtil_Impl, const OFFSET: isize>() -> IRoleAssociationUtil_Vtbl {
        unsafe extern "system" fn AssociateRole<Identity: ::windows::core::IUnknownImpl, Impl: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssociateRole(::core::mem::transmute(&bstrroleid)).into()
        }
        unsafe extern "system" fn AssociateRoleByName<Identity: ::windows::core::IUnknownImpl, Impl: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AssociateRoleByName(::core::mem::transmute(&bstrrolename)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AssociateRole: AssociateRole::<Identity, Impl, OFFSET>,
            AssociateRoleByName: AssociateRoleByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRoleAssociationUtil as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
