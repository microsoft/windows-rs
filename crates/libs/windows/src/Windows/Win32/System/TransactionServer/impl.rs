#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICatalog_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &::windows::core::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Connect(&self, bstrconnectstring: &::windows::core::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn MajorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()>;
    fn MinorVersion(&self, retval: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICatalog {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICatalog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>() -> ICatalog_Vtbl {
        unsafe extern "system" fn GetCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollection(::core::mem::transmute(&bstrcollname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectstring: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Connect(::core::mem::transmute(&bstrconnectstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MajorVersion(::core::mem::transmute_copy(&retval)).into()
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalog as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComponentUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallComponent(&self, bstrdllfile: &::windows::core::BSTR, bstrtypelibfile: &::windows::core::BSTR, bstrproxystubdllfile: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ImportComponent(&self, bstrclsid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ImportComponentByName(&self, bstrprogid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetCLSIDs(&self, bstrdllfile: &::windows::core::BSTR, bstrtypelibfile: &::windows::core::BSTR, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IComponentUtil {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComponentUtil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>() -> IComponentUtil_Vtbl {
        unsafe extern "system" fn InstallComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrtypelibfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrproxystubdllfile: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallComponent(::core::mem::transmute(&bstrdllfile), ::core::mem::transmute(&bstrtypelibfile), ::core::mem::transmute(&bstrproxystubdllfile)).into()
        }
        unsafe extern "system" fn ImportComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportComponent(::core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn ImportComponentByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprogid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportComponentByName(::core::mem::transmute(&bstrprogid)).into()
        }
        unsafe extern "system" fn GetCLSIDs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrtypelibfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, aclsids: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentUtil as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPackageUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallPackage(&self, bstrpackagefile: &::windows::core::BSTR, bstrinstallpath: &::windows::core::BSTR, loptions: i32) -> ::windows::core::Result<()>;
    fn ExportPackage(&self, bstrpackageid: &::windows::core::BSTR, bstrpackagefile: &::windows::core::BSTR, loptions: i32) -> ::windows::core::Result<()>;
    fn ShutdownPackage(&self, bstrpackageid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IPackageUtil {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPackageUtil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: isize>() -> IPackageUtil_Vtbl {
        unsafe extern "system" fn InstallPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackagefile: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrinstallpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallPackage(::core::mem::transmute(&bstrpackagefile), ::core::mem::transmute(&bstrinstallpath), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn ExportPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpackagefile: ::std::mem::MaybeUninit<::windows::core::BSTR>, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportPackage(::core::mem::transmute(&bstrpackageid), ::core::mem::transmute(&bstrpackagefile), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn ShutdownPackage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPackageUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpackageid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageUtil as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRemoteComponentUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn InstallRemoteComponent(&self, bstrserver: &::windows::core::BSTR, bstrpackageid: &::windows::core::BSTR, bstrclsid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn InstallRemoteComponentByName(&self, bstrserver: &::windows::core::BSTR, bstrpackagename: &::windows::core::BSTR, bstrprogid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRemoteComponentUtil {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteComponentUtil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRemoteComponentUtil_Impl, const OFFSET: isize>() -> IRemoteComponentUtil_Vtbl {
        unsafe extern "system" fn InstallRemoteComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserver: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpackageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallRemoteComponent(::core::mem::transmute(&bstrserver), ::core::mem::transmute(&bstrpackageid), ::core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn InstallRemoteComponentByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrserver: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpackagename: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrprogid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteComponentUtil as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_TransactionServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRoleAssociationUtil_Impl: Sized + super::Com::IDispatch_Impl {
    fn AssociateRole(&self, bstrroleid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AssociateRoleByName(&self, bstrrolename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRoleAssociationUtil {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRoleAssociationUtil_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRoleAssociationUtil_Impl, const OFFSET: isize>() -> IRoleAssociationUtil_Vtbl {
        unsafe extern "system" fn AssociateRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssociateRole(::core::mem::transmute(&bstrroleid)).into()
        }
        unsafe extern "system" fn AssociateRoleByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRoleAssociationUtil as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
