windows_core::imp::define_interface!(IChannelCredentials, IChannelCredentials_Vtbl, 0x181b448c_c17c_4b17_ac6d_06699b93198f);
impl core::ops::Deref for IChannelCredentials {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IChannelCredentials, windows_core::IUnknown, super::IDispatch);
impl IChannelCredentials {
    pub unsafe fn SetWindowsCredential<P0, P1, P2, P3>(&self, domain: P0, username: P1, password: P2, impersonationlevel: i32, allowntlm: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWindowsCredential)(windows_core::Interface::as_raw(self), domain.param().abi(), username.param().abi(), password.param().abi(), impersonationlevel, allowntlm.param().abi()).ok()
    }
    pub unsafe fn SetUserNameCredential<P0, P1>(&self, username: P0, password: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUserNameCredential)(windows_core::Interface::as_raw(self), username.param().abi(), password.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetClientCertificateFromStore<P0, P1, P2, P3>(&self, storelocation: P0, storename: P1, findyype: P2, findvalue: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Variant::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetClientCertificateFromStore)(windows_core::Interface::as_raw(self), storelocation.param().abi(), storename.param().abi(), findyype.param().abi(), findvalue.param().abi()).ok()
    }
    pub unsafe fn SetClientCertificateFromStoreByName<P0, P1, P2>(&self, subjectname: P0, storelocation: P1, storename: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientCertificateFromStoreByName)(windows_core::Interface::as_raw(self), subjectname.param().abi(), storelocation.param().abi(), storename.param().abi()).ok()
    }
    pub unsafe fn SetClientCertificateFromFile<P0, P1, P2>(&self, filename: P0, password: P1, keystorageflags: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientCertificateFromFile)(windows_core::Interface::as_raw(self), filename.param().abi(), password.param().abi(), keystorageflags.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDefaultServiceCertificateFromStore<P0, P1, P2, P3>(&self, storelocation: P0, storename: P1, findtype: P2, findvalue: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Variant::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromStore)(windows_core::Interface::as_raw(self), storelocation.param().abi(), storename.param().abi(), findtype.param().abi(), findvalue.param().abi()).ok()
    }
    pub unsafe fn SetDefaultServiceCertificateFromStoreByName<P0, P1, P2>(&self, subjectname: P0, storelocation: P1, storename: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromStoreByName)(windows_core::Interface::as_raw(self), subjectname.param().abi(), storelocation.param().abi(), storename.param().abi()).ok()
    }
    pub unsafe fn SetDefaultServiceCertificateFromFile<P0, P1, P2>(&self, filename: P0, password: P1, keystorageflags: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromFile)(windows_core::Interface::as_raw(self), filename.param().abi(), password.param().abi(), keystorageflags.param().abi()).ok()
    }
    pub unsafe fn SetServiceCertificateAuthentication<P0, P1, P2>(&self, storelocation: P0, revocationmode: P1, certificatevalidationmode: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceCertificateAuthentication)(windows_core::Interface::as_raw(self), storelocation.param().abi(), revocationmode.param().abi(), certificatevalidationmode.param().abi()).ok()
    }
    pub unsafe fn SetIssuedToken<P0, P1, P2>(&self, localissueraddres: P0, localissuerbindingtype: P1, localissuerbinding: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetIssuedToken)(windows_core::Interface::as_raw(self), localissueraddres.param().abi(), localissuerbindingtype.param().abi(), localissuerbinding.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IChannelCredentials_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetWindowsCredential: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetUserNameCredential: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetClientCertificateFromStore: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetClientCertificateFromStore: usize,
    pub SetClientCertificateFromStoreByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClientCertificateFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDefaultServiceCertificateFromStore: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDefaultServiceCertificateFromStore: usize,
    pub SetDefaultServiceCertificateFromStoreByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDefaultServiceCertificateFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceCertificateAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetIssuedToken: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IChannelCredentials_Impl: Sized + super::IDispatch_Impl {
    fn SetWindowsCredential(&self, domain: &windows_core::BSTR, username: &windows_core::BSTR, password: &windows_core::BSTR, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetUserNameCredential(&self, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findyype: &windows_core::BSTR, findvalue: &super::super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetClientCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findtype: &windows_core::BSTR, findvalue: &super::super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetServiceCertificateAuthentication(&self, storelocation: &windows_core::BSTR, revocationmode: &windows_core::BSTR, certificatevalidationmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetIssuedToken(&self, localissueraddres: &windows_core::BSTR, localissuerbindingtype: &windows_core::BSTR, localissuerbinding: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IChannelCredentials {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IChannelCredentials_Vtbl {
    pub const fn new<Identity: IChannelCredentials_Impl, const OFFSET: isize>() -> IChannelCredentials_Vtbl {
        unsafe extern "system" fn SetWindowsCredential<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domain: core::mem::MaybeUninit<windows_core::BSTR>, username: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetWindowsCredential(this, core::mem::transmute(&domain), core::mem::transmute(&username), core::mem::transmute(&password), core::mem::transmute_copy(&impersonationlevel), core::mem::transmute_copy(&allowntlm)).into()
        }
        unsafe extern "system" fn SetUserNameCredential<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetUserNameCredential(this, core::mem::transmute(&username), core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, storename: core::mem::MaybeUninit<windows_core::BSTR>, findyype: core::mem::MaybeUninit<windows_core::BSTR>, findvalue: core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetClientCertificateFromStore(this, core::mem::transmute(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&findyype), core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subjectname: core::mem::MaybeUninit<windows_core::BSTR>, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, storename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetClientCertificateFromStoreByName(this, core::mem::transmute(&subjectname), core::mem::transmute(&storelocation), core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>, keystorageflags: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetClientCertificateFromFile(this, core::mem::transmute(&filename), core::mem::transmute(&password), core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, storename: core::mem::MaybeUninit<windows_core::BSTR>, findtype: core::mem::MaybeUninit<windows_core::BSTR>, findvalue: core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetDefaultServiceCertificateFromStore(this, core::mem::transmute(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&findtype), core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subjectname: core::mem::MaybeUninit<windows_core::BSTR>, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, storename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetDefaultServiceCertificateFromStoreByName(this, core::mem::transmute(&subjectname), core::mem::transmute(&storelocation), core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>, keystorageflags: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetDefaultServiceCertificateFromFile(this, core::mem::transmute(&filename), core::mem::transmute(&password), core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: core::mem::MaybeUninit<windows_core::BSTR>, revocationmode: core::mem::MaybeUninit<windows_core::BSTR>, certificatevalidationmode: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetServiceCertificateAuthentication(this, core::mem::transmute(&storelocation), core::mem::transmute(&revocationmode), core::mem::transmute(&certificatevalidationmode)).into()
        }
        unsafe extern "system" fn SetIssuedToken<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localissueraddres: core::mem::MaybeUninit<windows_core::BSTR>, localissuerbindingtype: core::mem::MaybeUninit<windows_core::BSTR>, localissuerbinding: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelCredentials_Impl::SetIssuedToken(this, core::mem::transmute(&localissueraddres), core::mem::transmute(&localissuerbindingtype), core::mem::transmute(&localissuerbinding)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetWindowsCredential: SetWindowsCredential::<Identity, OFFSET>,
            SetUserNameCredential: SetUserNameCredential::<Identity, OFFSET>,
            SetClientCertificateFromStore: SetClientCertificateFromStore::<Identity, OFFSET>,
            SetClientCertificateFromStoreByName: SetClientCertificateFromStoreByName::<Identity, OFFSET>,
            SetClientCertificateFromFile: SetClientCertificateFromFile::<Identity, OFFSET>,
            SetDefaultServiceCertificateFromStore: SetDefaultServiceCertificateFromStore::<Identity, OFFSET>,
            SetDefaultServiceCertificateFromStoreByName: SetDefaultServiceCertificateFromStoreByName::<Identity, OFFSET>,
            SetDefaultServiceCertificateFromFile: SetDefaultServiceCertificateFromFile::<Identity, OFFSET>,
            SetServiceCertificateAuthentication: SetServiceCertificateAuthentication::<Identity, OFFSET>,
            SetIssuedToken: SetIssuedToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChannelCredentials as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
