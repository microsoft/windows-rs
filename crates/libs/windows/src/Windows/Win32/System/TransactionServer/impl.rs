#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalog_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn Connect(&self, bstrconnectstring: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn MajorVersion(&self, retval: *mut i32) -> windows_core::Result<()>;
    fn MinorVersion(&self, retval: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICatalog {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICatalog_Vtbl {
    pub const fn new<Identity: ICatalog_Impl, const OFFSET: isize>() -> ICatalog_Vtbl {
        unsafe extern "system" fn GetCollection<Identity: ICatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: core::mem::MaybeUninit<windows_core::BSTR>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalog_Impl::GetCollection(this, core::mem::transmute(&bstrcollname)) {
                Ok(ok__) => {
                    ppcatalogcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ICatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnectstring: core::mem::MaybeUninit<windows_core::BSTR>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalog_Impl::Connect(this, core::mem::transmute(&bstrconnectstring)) {
                Ok(ok__) => {
                    ppcatalogcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ICatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalog_Impl::MajorVersion(this, core::mem::transmute_copy(&retval)).into()
        }
        unsafe extern "system" fn MinorVersion<Identity: ICatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalog_Impl::MinorVersion(this, core::mem::transmute_copy(&retval)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetCollection: GetCollection::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalog as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComponentUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallComponent(&self, bstrdllfile: &windows_core::BSTR, bstrtypelibfile: &windows_core::BSTR, bstrproxystubdllfile: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportComponent(&self, bstrclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportComponentByName(&self, bstrprogid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCLSIDs(&self, bstrdllfile: &windows_core::BSTR, bstrtypelibfile: &windows_core::BSTR, aclsids: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IComponentUtil {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IComponentUtil_Vtbl {
    pub const fn new<Identity: IComponentUtil_Impl, const OFFSET: isize>() -> IComponentUtil_Vtbl {
        unsafe extern "system" fn InstallComponent<Identity: IComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdllfile: core::mem::MaybeUninit<windows_core::BSTR>, bstrtypelibfile: core::mem::MaybeUninit<windows_core::BSTR>, bstrproxystubdllfile: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComponentUtil_Impl::InstallComponent(this, core::mem::transmute(&bstrdllfile), core::mem::transmute(&bstrtypelibfile), core::mem::transmute(&bstrproxystubdllfile)).into()
        }
        unsafe extern "system" fn ImportComponent<Identity: IComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComponentUtil_Impl::ImportComponent(this, core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn ImportComponentByName<Identity: IComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprogid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComponentUtil_Impl::ImportComponentByName(this, core::mem::transmute(&bstrprogid)).into()
        }
        unsafe extern "system" fn GetCLSIDs<Identity: IComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdllfile: core::mem::MaybeUninit<windows_core::BSTR>, bstrtypelibfile: core::mem::MaybeUninit<windows_core::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComponentUtil_Impl::GetCLSIDs(this, core::mem::transmute(&bstrdllfile), core::mem::transmute(&bstrtypelibfile), core::mem::transmute_copy(&aclsids)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InstallComponent: InstallComponent::<Identity, OFFSET>,
            ImportComponent: ImportComponent::<Identity, OFFSET>,
            ImportComponentByName: ImportComponentByName::<Identity, OFFSET>,
            GetCLSIDs: GetCLSIDs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComponentUtil as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPackageUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallPackage(&self, bstrpackagefile: &windows_core::BSTR, bstrinstallpath: &windows_core::BSTR, loptions: i32) -> windows_core::Result<()>;
    fn ExportPackage(&self, bstrpackageid: &windows_core::BSTR, bstrpackagefile: &windows_core::BSTR, loptions: i32) -> windows_core::Result<()>;
    fn ShutdownPackage(&self, bstrpackageid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPackageUtil {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPackageUtil_Vtbl {
    pub const fn new<Identity: IPackageUtil_Impl, const OFFSET: isize>() -> IPackageUtil_Vtbl {
        unsafe extern "system" fn InstallPackage<Identity: IPackageUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpackagefile: core::mem::MaybeUninit<windows_core::BSTR>, bstrinstallpath: core::mem::MaybeUninit<windows_core::BSTR>, loptions: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPackageUtil_Impl::InstallPackage(this, core::mem::transmute(&bstrpackagefile), core::mem::transmute(&bstrinstallpath), core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn ExportPackage<Identity: IPackageUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpackageid: core::mem::MaybeUninit<windows_core::BSTR>, bstrpackagefile: core::mem::MaybeUninit<windows_core::BSTR>, loptions: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPackageUtil_Impl::ExportPackage(this, core::mem::transmute(&bstrpackageid), core::mem::transmute(&bstrpackagefile), core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn ShutdownPackage<Identity: IPackageUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpackageid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPackageUtil_Impl::ShutdownPackage(this, core::mem::transmute(&bstrpackageid)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InstallPackage: InstallPackage::<Identity, OFFSET>,
            ExportPackage: ExportPackage::<Identity, OFFSET>,
            ShutdownPackage: ShutdownPackage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPackageUtil as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteComponentUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallRemoteComponent(&self, bstrserver: &windows_core::BSTR, bstrpackageid: &windows_core::BSTR, bstrclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InstallRemoteComponentByName(&self, bstrserver: &windows_core::BSTR, bstrpackagename: &windows_core::BSTR, bstrprogid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRemoteComponentUtil {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRemoteComponentUtil_Vtbl {
    pub const fn new<Identity: IRemoteComponentUtil_Impl, const OFFSET: isize>() -> IRemoteComponentUtil_Vtbl {
        unsafe extern "system" fn InstallRemoteComponent<Identity: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserver: core::mem::MaybeUninit<windows_core::BSTR>, bstrpackageid: core::mem::MaybeUninit<windows_core::BSTR>, bstrclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteComponentUtil_Impl::InstallRemoteComponent(this, core::mem::transmute(&bstrserver), core::mem::transmute(&bstrpackageid), core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn InstallRemoteComponentByName<Identity: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserver: core::mem::MaybeUninit<windows_core::BSTR>, bstrpackagename: core::mem::MaybeUninit<windows_core::BSTR>, bstrprogid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRemoteComponentUtil_Impl::InstallRemoteComponentByName(this, core::mem::transmute(&bstrserver), core::mem::transmute(&bstrpackagename), core::mem::transmute(&bstrprogid)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InstallRemoteComponent: InstallRemoteComponent::<Identity, OFFSET>,
            InstallRemoteComponentByName: InstallRemoteComponentByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteComponentUtil as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRoleAssociationUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn AssociateRole(&self, bstrroleid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AssociateRoleByName(&self, bstrrolename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRoleAssociationUtil {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRoleAssociationUtil_Vtbl {
    pub const fn new<Identity: IRoleAssociationUtil_Impl, const OFFSET: isize>() -> IRoleAssociationUtil_Vtbl {
        unsafe extern "system" fn AssociateRole<Identity: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroleid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRoleAssociationUtil_Impl::AssociateRole(this, core::mem::transmute(&bstrroleid)).into()
        }
        unsafe extern "system" fn AssociateRoleByName<Identity: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrolename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRoleAssociationUtil_Impl::AssociateRoleByName(this, core::mem::transmute(&bstrrolename)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AssociateRole: AssociateRole::<Identity, OFFSET>,
            AssociateRoleByName: AssociateRoleByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRoleAssociationUtil as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
