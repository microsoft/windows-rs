#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalog_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Connect(&self, bstrconnectstring: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn MajorVersion(&self, retval: *mut i32) -> ::windows_core::Result<()>;
    fn MinorVersion(&self, retval: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICatalog {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICatalog_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>() -> ICatalog_Vtbl {
        unsafe extern "system" fn GetCollection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollection(::core::mem::transmute(&bstrcollname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Connect(::core::mem::transmute(&bstrconnectstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MajorVersion(::core::mem::transmute_copy(&retval)).into()
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MinorVersion(::core::mem::transmute_copy(&retval)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCollection: GetCollection::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICatalog as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComponentUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallComponent(&self, bstrdllfile: &::windows_core::BSTR, bstrtypelibfile: &::windows_core::BSTR, bstrproxystubdllfile: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImportComponent(&self, bstrclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImportComponentByName(&self, bstrprogid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetCLSIDs(&self, bstrdllfile: &::windows_core::BSTR, bstrtypelibfile: &::windows_core::BSTR, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IComponentUtil {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IComponentUtil_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>() -> IComponentUtil_Vtbl {
        unsafe extern "system" fn InstallComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtypelibfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrproxystubdllfile: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallComponent(::core::mem::transmute(&bstrdllfile), ::core::mem::transmute(&bstrtypelibfile), ::core::mem::transmute(&bstrproxystubdllfile)).into()
        }
        unsafe extern "system" fn ImportComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportComponent(::core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn ImportComponentByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportComponentByName(::core::mem::transmute(&bstrprogid)).into()
        }
        unsafe extern "system" fn GetCLSIDs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtypelibfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCLSIDs(::core::mem::transmute(&bstrdllfile), ::core::mem::transmute(&bstrtypelibfile), ::core::mem::transmute_copy(&aclsids)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            InstallComponent: InstallComponent::<Identity, Impl, OFFSET>,
            ImportComponent: ImportComponent::<Identity, Impl, OFFSET>,
            ImportComponentByName: ImportComponentByName::<Identity, Impl, OFFSET>,
            GetCLSIDs: GetCLSIDs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComponentUtil as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPackageUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallPackage(&self, bstrpackagefile: &::windows_core::BSTR, bstrinstallpath: &::windows_core::BSTR, loptions: i32) -> ::windows_core::Result<()>;
    fn ExportPackage(&self, bstrpackageid: &::windows_core::BSTR, bstrpackagefile: &::windows_core::BSTR, loptions: i32) -> ::windows_core::Result<()>;
    fn ShutdownPackage(&self, bstrpackageid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPackageUtil {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPackageUtil_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: isize>() -> IPackageUtil_Vtbl {
        unsafe extern "system" fn InstallPackage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackagefile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinstallpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallPackage(::core::mem::transmute(&bstrpackagefile), ::core::mem::transmute(&bstrinstallpath), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn ExportPackage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpackagefile: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportPackage(::core::mem::transmute(&bstrpackageid), ::core::mem::transmute(&bstrpackagefile), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn ShutdownPackage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownPackage(::core::mem::transmute(&bstrpackageid)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            InstallPackage: InstallPackage::<Identity, Impl, OFFSET>,
            ExportPackage: ExportPackage::<Identity, Impl, OFFSET>,
            ShutdownPackage: ShutdownPackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPackageUtil as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteComponentUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallRemoteComponent(&self, bstrserver: &::windows_core::BSTR, bstrpackageid: &::windows_core::BSTR, bstrclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InstallRemoteComponentByName(&self, bstrserver: &::windows_core::BSTR, bstrpackagename: &::windows_core::BSTR, bstrprogid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IRemoteComponentUtil {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRemoteComponentUtil_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteComponentUtil_Impl, const OFFSET: isize>() -> IRemoteComponentUtil_Vtbl {
        unsafe extern "system" fn InstallRemoteComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserver: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpackageid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallRemoteComponent(::core::mem::transmute(&bstrserver), ::core::mem::transmute(&bstrpackageid), ::core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn InstallRemoteComponentByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserver: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpackagename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallRemoteComponentByName(::core::mem::transmute(&bstrserver), ::core::mem::transmute(&bstrpackagename), ::core::mem::transmute(&bstrprogid)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            InstallRemoteComponent: InstallRemoteComponent::<Identity, Impl, OFFSET>,
            InstallRemoteComponentByName: InstallRemoteComponentByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRemoteComponentUtil as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRoleAssociationUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn AssociateRole(&self, bstrroleid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AssociateRoleByName(&self, bstrrolename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IRoleAssociationUtil {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRoleAssociationUtil_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRoleAssociationUtil_Impl, const OFFSET: isize>() -> IRoleAssociationUtil_Vtbl {
        unsafe extern "system" fn AssociateRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssociateRole(::core::mem::transmute(&bstrroleid)).into()
        }
        unsafe extern "system" fn AssociateRoleByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssociateRoleByName(::core::mem::transmute(&bstrrolename)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AssociateRole: AssociateRole::<Identity, Impl, OFFSET>,
            AssociateRoleByName: AssociateRoleByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRoleAssociationUtil as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
