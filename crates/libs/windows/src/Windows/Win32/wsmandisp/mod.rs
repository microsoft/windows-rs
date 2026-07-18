#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSMan, IWSMan_Vtbl, 0x190d8637_5cd3_496d_ad24_69636bb5a3b5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSMan {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSMan, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWSMan {
    pub unsafe fn CreateSession<P2>(&self, connection: &windows_core::BSTR, flags: i32, connectionoptions: P2) -> windows_core::Result<super::IDispatch>
    where
        P2: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSession)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(connection), flags, connectionoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateConnectionOptions(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateConnectionOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommandLine(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommandLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSMan_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateConnectionOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSMan_Impl: super::IDispatch_Impl {
    fn CreateSession(&self, connection: &windows_core::BSTR, flags: i32, connectionoptions: windows_core::Ref<super::IDispatch>) -> windows_core::Result<super::IDispatch>;
    fn CreateConnectionOptions(&self) -> windows_core::Result<super::IDispatch>;
    fn CommandLine(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Error(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSMan_Vtbl {
    pub const fn new<Identity: IWSMan_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSession<Identity: IWSMan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connection: *mut core::ffi::c_void, flags: i32, connectionoptions: *mut core::ffi::c_void, session: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSMan_Impl::CreateSession(this, core::mem::transmute(&connection), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&connectionoptions)) {
                    Ok(ok__) => {
                        session.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateConnectionOptions<Identity: IWSMan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSMan_Impl::CreateConnectionOptions(this) {
                    Ok(ok__) => {
                        connectionoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommandLine<Identity: IWSMan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSMan_Impl::CommandLine(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Error<Identity: IWSMan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSMan_Impl::Error(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateSession: CreateSession::<Identity, OFFSET>,
            CreateConnectionOptions: CreateConnectionOptions::<Identity, OFFSET>,
            CommandLine: CommandLine::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSMan as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSMan {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManConnectionOptions, IWSManConnectionOptions_Vtbl, 0xf704e861_9e52_464f_b786_da5eb2320fdd);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManConnectionOptions {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManConnectionOptions, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWSManConnectionOptions {
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetUserName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUserName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn SetPassword(&self, password: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(password)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManConnectionOptions_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManConnectionOptions_Impl: super::IDispatch_Impl {
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUserName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPassword(&self, password: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManConnectionOptions_Vtbl {
    pub const fn new<Identity: IWSManConnectionOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UserName<Identity: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptions_Impl::UserName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUserName<Identity: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManConnectionOptions_Impl::SetUserName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn SetPassword<Identity: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, password: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManConnectionOptions_Impl::SetPassword(this, core::mem::transmute(&password)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            UserName: UserName::<Identity, OFFSET>,
            SetUserName: SetUserName::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManConnectionOptions as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManConnectionOptions {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManConnectionOptionsEx, IWSManConnectionOptionsEx_Vtbl, 0xef43edf7_2a48_4d93_9526_8bd6ab6d4a6b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManConnectionOptionsEx {
    type Target = IWSManConnectionOptions;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManConnectionOptionsEx, windows_core::IUnknown, super::IDispatch, IWSManConnectionOptions);
#[cfg(feature = "oaidl")]
impl IWSManConnectionOptionsEx {
    pub unsafe fn CertificateThumbprint(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CertificateThumbprint)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCertificateThumbprint(&self, thumbprint: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCertificateThumbprint)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(thumbprint)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManConnectionOptionsEx_Vtbl {
    pub base__: IWSManConnectionOptions_Vtbl,
    pub CertificateThumbprint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCertificateThumbprint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManConnectionOptionsEx_Impl: IWSManConnectionOptions_Impl {
    fn CertificateThumbprint(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCertificateThumbprint(&self, thumbprint: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManConnectionOptionsEx_Vtbl {
    pub const fn new<Identity: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CertificateThumbprint<Identity: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thumbprint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx_Impl::CertificateThumbprint(this) {
                    Ok(ok__) => {
                        thumbprint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCertificateThumbprint<Identity: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thumbprint: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManConnectionOptionsEx_Impl::SetCertificateThumbprint(this, core::mem::transmute(&thumbprint)).into()
            }
        }
        Self {
            base__: IWSManConnectionOptions_Vtbl::new::<Identity, OFFSET>(),
            CertificateThumbprint: CertificateThumbprint::<Identity, OFFSET>,
            SetCertificateThumbprint: SetCertificateThumbprint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManConnectionOptionsEx as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWSManConnectionOptions as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManConnectionOptionsEx {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManConnectionOptionsEx2, IWSManConnectionOptionsEx2_Vtbl, 0xf500c9ec_24ee_48ab_b38d_fc9a164c658e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManConnectionOptionsEx2 {
    type Target = IWSManConnectionOptionsEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManConnectionOptionsEx2, windows_core::IUnknown, super::IDispatch, IWSManConnectionOptions, IWSManConnectionOptionsEx);
#[cfg(feature = "oaidl")]
impl IWSManConnectionOptionsEx2 {
    pub unsafe fn SetProxy(&self, accesstype: i32, authenticationmechanism: i32, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProxy)(windows_core::Interface::as_raw(self), accesstype, authenticationmechanism, core::mem::transmute_copy(username), core::mem::transmute_copy(password)) }
    }
    pub unsafe fn ProxyIEConfig(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyIEConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyWinHttpConfig(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyWinHttpConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyAutoDetect(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyAutoDetect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyNoProxyServer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyNoProxyServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyAuthenticationUseNegotiate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyAuthenticationUseNegotiate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyAuthenticationUseBasic(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyAuthenticationUseBasic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyAuthenticationUseDigest(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyAuthenticationUseDigest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManConnectionOptionsEx2_Vtbl {
    pub base__: IWSManConnectionOptionsEx_Vtbl,
    pub SetProxy: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProxyIEConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyWinHttpConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyAutoDetect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyNoProxyServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyAuthenticationUseNegotiate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyAuthenticationUseBasic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyAuthenticationUseDigest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManConnectionOptionsEx2_Impl: IWSManConnectionOptionsEx_Impl {
    fn SetProxy(&self, accesstype: i32, authenticationmechanism: i32, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProxyIEConfig(&self) -> windows_core::Result<i32>;
    fn ProxyWinHttpConfig(&self) -> windows_core::Result<i32>;
    fn ProxyAutoDetect(&self) -> windows_core::Result<i32>;
    fn ProxyNoProxyServer(&self) -> windows_core::Result<i32>;
    fn ProxyAuthenticationUseNegotiate(&self) -> windows_core::Result<i32>;
    fn ProxyAuthenticationUseBasic(&self) -> windows_core::Result<i32>;
    fn ProxyAuthenticationUseDigest(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManConnectionOptionsEx2_Vtbl {
    pub const fn new<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProxy<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accesstype: i32, authenticationmechanism: i32, username: *mut core::ffi::c_void, password: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManConnectionOptionsEx2_Impl::SetProxy(this, core::mem::transmute_copy(&accesstype), core::mem::transmute_copy(&authenticationmechanism), core::mem::transmute(&username), core::mem::transmute(&password)).into()
            }
        }
        unsafe extern "system" fn ProxyIEConfig<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyIEConfig(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyWinHttpConfig<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyWinHttpConfig(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyAutoDetect<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyAutoDetect(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyNoProxyServer<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyNoProxyServer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseNegotiate<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyAuthenticationUseNegotiate(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseBasic<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyAuthenticationUseBasic(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseDigest<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyAuthenticationUseDigest(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSManConnectionOptionsEx_Vtbl::new::<Identity, OFFSET>(),
            SetProxy: SetProxy::<Identity, OFFSET>,
            ProxyIEConfig: ProxyIEConfig::<Identity, OFFSET>,
            ProxyWinHttpConfig: ProxyWinHttpConfig::<Identity, OFFSET>,
            ProxyAutoDetect: ProxyAutoDetect::<Identity, OFFSET>,
            ProxyNoProxyServer: ProxyNoProxyServer::<Identity, OFFSET>,
            ProxyAuthenticationUseNegotiate: ProxyAuthenticationUseNegotiate::<Identity, OFFSET>,
            ProxyAuthenticationUseBasic: ProxyAuthenticationUseBasic::<Identity, OFFSET>,
            ProxyAuthenticationUseDigest: ProxyAuthenticationUseDigest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManConnectionOptionsEx2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWSManConnectionOptions as windows_core::Interface>::IID || iid == &<IWSManConnectionOptionsEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManConnectionOptionsEx2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManEnumerator, IWSManEnumerator_Vtbl, 0xf3457ca9_abb9_4fa5_b850_90e8ca300e7f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManEnumerator {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManEnumerator, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWSManEnumerator {
    pub unsafe fn ReadItem(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadItem)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AtEndOfStream(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AtEndOfStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEnumerator_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub ReadItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub AtEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AtEndOfStream: usize,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManEnumerator_Impl: super::IDispatch_Impl {
    fn ReadItem(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AtEndOfStream(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Error(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManEnumerator_Vtbl {
    pub const fn new<Identity: IWSManEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadItem<Identity: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEnumerator_Impl::ReadItem(this) {
                    Ok(ok__) => {
                        resource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AtEndOfStream<Identity: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eos: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEnumerator_Impl::AtEndOfStream(this) {
                    Ok(ok__) => {
                        eos.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Error<Identity: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEnumerator_Impl::Error(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ReadItem: ReadItem::<Identity, OFFSET>,
            AtEndOfStream: AtEndOfStream::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManEnumerator as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManEnumerator {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManEx, IWSManEx_Vtbl, 0x2d53bdaa_798e_49e6_a1aa_74d01256f411);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManEx {
    type Target = IWSMan;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManEx, windows_core::IUnknown, super::IDispatch, IWSMan);
#[cfg(feature = "oaidl")]
impl IWSManEx {
    pub unsafe fn CreateResourceLocator(&self, strresourcelocator: &windows_core::BSTR) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateResourceLocator)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strresourcelocator), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SessionFlagUTF8(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUTF8)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagCredUsernamePassword(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagCredUsernamePassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagSkipCACheck(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagSkipCACheck)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagSkipCNCheck(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagSkipCNCheck)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseDigest(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseDigest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseNegotiate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseNegotiate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseBasic(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseBasic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseKerberos(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseKerberos)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagNoEncryption(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagNoEncryption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagEnableSPNServerPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagEnableSPNServerPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseNoAuthentication(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseNoAuthentication)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagNonXmlText(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagNonXmlText)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagReturnEPR(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagReturnEPR)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagReturnObjectAndEPR(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagReturnObjectAndEPR)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetErrorMessage(&self, errornumber: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorMessage)(windows_core::Interface::as_raw(self), errornumber, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EnumerationFlagHierarchyDeep(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagHierarchyDeep)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagHierarchyShallow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagHierarchyShallow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagHierarchyDeepBasePropsOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagReturnObject(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagReturnObject)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEx_Vtbl {
    pub base__: IWSMan_Vtbl,
    pub CreateResourceLocator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionFlagUTF8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagCredUsernamePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagSkipCACheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagSkipCNCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseDigest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseNegotiate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseBasic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseKerberos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagNoEncryption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagEnableSPNServerPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseNoAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagNonXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagReturnEPR: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagReturnObjectAndEPR: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetErrorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerationFlagHierarchyDeep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagHierarchyShallow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagHierarchyDeepBasePropsOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagReturnObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManEx_Impl: IWSMan_Impl {
    fn CreateResourceLocator(&self, strresourcelocator: &windows_core::BSTR) -> windows_core::Result<super::IDispatch>;
    fn SessionFlagUTF8(&self) -> windows_core::Result<i32>;
    fn SessionFlagCredUsernamePassword(&self) -> windows_core::Result<i32>;
    fn SessionFlagSkipCACheck(&self) -> windows_core::Result<i32>;
    fn SessionFlagSkipCNCheck(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseDigest(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseNegotiate(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseBasic(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseKerberos(&self) -> windows_core::Result<i32>;
    fn SessionFlagNoEncryption(&self) -> windows_core::Result<i32>;
    fn SessionFlagEnableSPNServerPort(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseNoAuthentication(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagNonXmlText(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagReturnEPR(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagReturnObjectAndEPR(&self) -> windows_core::Result<i32>;
    fn GetErrorMessage(&self, errornumber: u32) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerationFlagHierarchyDeep(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagHierarchyShallow(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagReturnObject(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManEx_Vtbl {
    pub const fn new<Identity: IWSManEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateResourceLocator<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresourcelocator: *mut core::ffi::c_void, newresourcelocator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::CreateResourceLocator(this, core::mem::transmute(&strresourcelocator)) {
                    Ok(ok__) => {
                        newresourcelocator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUTF8<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUTF8(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagCredUsernamePassword<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagCredUsernamePassword(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagSkipCACheck<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagSkipCACheck(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagSkipCNCheck<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagSkipCNCheck(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseDigest<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseDigest(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseNegotiate<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseNegotiate(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseBasic<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseBasic(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseKerberos<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseKerberos(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagNoEncryption<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagNoEncryption(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagEnableSPNServerPort<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagEnableSPNServerPort(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseNoAuthentication<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseNoAuthentication(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagNonXmlText<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagNonXmlText(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnEPR<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagReturnEPR(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObjectAndEPR<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagReturnObjectAndEPR(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorMessage<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errornumber: u32, errormessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::GetErrorMessage(this, core::mem::transmute_copy(&errornumber)) {
                    Ok(ok__) => {
                        errormessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeep<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagHierarchyDeep(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyShallow<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagHierarchyShallow(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeepBasePropsOnly<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagHierarchyDeepBasePropsOnly(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObject<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagReturnObject(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSMan_Vtbl::new::<Identity, OFFSET>(),
            CreateResourceLocator: CreateResourceLocator::<Identity, OFFSET>,
            SessionFlagUTF8: SessionFlagUTF8::<Identity, OFFSET>,
            SessionFlagCredUsernamePassword: SessionFlagCredUsernamePassword::<Identity, OFFSET>,
            SessionFlagSkipCACheck: SessionFlagSkipCACheck::<Identity, OFFSET>,
            SessionFlagSkipCNCheck: SessionFlagSkipCNCheck::<Identity, OFFSET>,
            SessionFlagUseDigest: SessionFlagUseDigest::<Identity, OFFSET>,
            SessionFlagUseNegotiate: SessionFlagUseNegotiate::<Identity, OFFSET>,
            SessionFlagUseBasic: SessionFlagUseBasic::<Identity, OFFSET>,
            SessionFlagUseKerberos: SessionFlagUseKerberos::<Identity, OFFSET>,
            SessionFlagNoEncryption: SessionFlagNoEncryption::<Identity, OFFSET>,
            SessionFlagEnableSPNServerPort: SessionFlagEnableSPNServerPort::<Identity, OFFSET>,
            SessionFlagUseNoAuthentication: SessionFlagUseNoAuthentication::<Identity, OFFSET>,
            EnumerationFlagNonXmlText: EnumerationFlagNonXmlText::<Identity, OFFSET>,
            EnumerationFlagReturnEPR: EnumerationFlagReturnEPR::<Identity, OFFSET>,
            EnumerationFlagReturnObjectAndEPR: EnumerationFlagReturnObjectAndEPR::<Identity, OFFSET>,
            GetErrorMessage: GetErrorMessage::<Identity, OFFSET>,
            EnumerationFlagHierarchyDeep: EnumerationFlagHierarchyDeep::<Identity, OFFSET>,
            EnumerationFlagHierarchyShallow: EnumerationFlagHierarchyShallow::<Identity, OFFSET>,
            EnumerationFlagHierarchyDeepBasePropsOnly: EnumerationFlagHierarchyDeepBasePropsOnly::<Identity, OFFSET>,
            EnumerationFlagReturnObject: EnumerationFlagReturnObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManEx as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWSMan as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManEx {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManEx2, IWSManEx2_Vtbl, 0x1d1b5ae0_42d9_4021_8261_3987619512e9);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManEx2 {
    type Target = IWSManEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManEx2, windows_core::IUnknown, super::IDispatch, IWSMan, IWSManEx);
#[cfg(feature = "oaidl")]
impl IWSManEx2 {
    pub unsafe fn SessionFlagUseClientCertificate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseClientCertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEx2_Vtbl {
    pub base__: IWSManEx_Vtbl,
    pub SessionFlagUseClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManEx2_Impl: IWSManEx_Impl {
    fn SessionFlagUseClientCertificate(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManEx2_Vtbl {
    pub const fn new<Identity: IWSManEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SessionFlagUseClientCertificate<Identity: IWSManEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx2_Impl::SessionFlagUseClientCertificate(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWSManEx_Vtbl::new::<Identity, OFFSET>(), SessionFlagUseClientCertificate: SessionFlagUseClientCertificate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManEx2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWSMan as windows_core::Interface>::IID || iid == &<IWSManEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManEx2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManEx3, IWSManEx3_Vtbl, 0x6400e966_011d_4eac_8474_049e0848afad);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManEx3 {
    type Target = IWSManEx2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManEx3, windows_core::IUnknown, super::IDispatch, IWSMan, IWSManEx, IWSManEx2);
#[cfg(feature = "oaidl")]
impl IWSManEx3 {
    pub unsafe fn SessionFlagUTF16(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUTF16)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseCredSsp(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseCredSsp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagAssociationInstance(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagAssociationInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagAssociatedInstance(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagAssociatedInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagSkipRevocationCheck(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagSkipRevocationCheck)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagAllowNegotiateImplicitCredentials(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagAllowNegotiateImplicitCredentials)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseSsl(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseSsl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEx3_Vtbl {
    pub base__: IWSManEx2_Vtbl,
    pub SessionFlagUTF16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseCredSsp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagAssociationInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagAssociatedInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagSkipRevocationCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagAllowNegotiateImplicitCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseSsl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManEx3_Impl: IWSManEx2_Impl {
    fn SessionFlagUTF16(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseCredSsp(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagAssociationInstance(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagAssociatedInstance(&self) -> windows_core::Result<i32>;
    fn SessionFlagSkipRevocationCheck(&self) -> windows_core::Result<i32>;
    fn SessionFlagAllowNegotiateImplicitCredentials(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseSsl(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManEx3_Vtbl {
    pub const fn new<Identity: IWSManEx3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SessionFlagUTF16<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagUTF16(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseCredSsp<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagUseCredSsp(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociationInstance<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::EnumerationFlagAssociationInstance(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociatedInstance<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::EnumerationFlagAssociatedInstance(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagSkipRevocationCheck<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagSkipRevocationCheck(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagAllowNegotiateImplicitCredentials<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagAllowNegotiateImplicitCredentials(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseSsl<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagUseSsl(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSManEx2_Vtbl::new::<Identity, OFFSET>(),
            SessionFlagUTF16: SessionFlagUTF16::<Identity, OFFSET>,
            SessionFlagUseCredSsp: SessionFlagUseCredSsp::<Identity, OFFSET>,
            EnumerationFlagAssociationInstance: EnumerationFlagAssociationInstance::<Identity, OFFSET>,
            EnumerationFlagAssociatedInstance: EnumerationFlagAssociatedInstance::<Identity, OFFSET>,
            SessionFlagSkipRevocationCheck: SessionFlagSkipRevocationCheck::<Identity, OFFSET>,
            SessionFlagAllowNegotiateImplicitCredentials: SessionFlagAllowNegotiateImplicitCredentials::<Identity, OFFSET>,
            SessionFlagUseSsl: SessionFlagUseSsl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManEx3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWSMan as windows_core::Interface>::IID || iid == &<IWSManEx as windows_core::Interface>::IID || iid == &<IWSManEx2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManEx3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManInternal, IWSManInternal_Vtbl, 0x04ae2b1d_9954_4d99_94a9_a961e72c3a13);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManInternal {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManInternal, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWSManInternal {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ConfigSDDL<P0>(&self, session: P0, resourceuri: &super::VARIANT, flags: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConfigSDDL)(windows_core::Interface::as_raw(self), session.param().abi(), core::mem::transmute_copy(resourceuri), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManInternal_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ConfigSDDL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ConfigSDDL: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManInternal_Impl: super::IDispatch_Impl {
    fn ConfigSDDL(&self, session: windows_core::Ref<super::IDispatch>, resourceuri: &super::VARIANT, flags: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManInternal_Vtbl {
    pub const fn new<Identity: IWSManInternal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConfigSDDL<Identity: IWSManInternal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, session: *mut core::ffi::c_void, resourceuri: super::VARIANT, flags: i32, resource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManInternal_Impl::ConfigSDDL(this, core::mem::transmute_copy(&session), core::mem::transmute(&resourceuri), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        resource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), ConfigSDDL: ConfigSDDL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManInternal as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManInternal {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManResourceLocator, IWSManResourceLocator_Vtbl, 0xa7a1ba28_de41_466a_ad0a_c4059ead7428);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManResourceLocator {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManResourceLocator, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWSManResourceLocator {
    pub unsafe fn SetResourceURI(&self, uri: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResourceURI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(uri)) }
    }
    pub unsafe fn ResourceURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddSelector(&self, resourceselname: &windows_core::BSTR, selvalue: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddSelector)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceselname), core::mem::transmute_copy(selvalue)) }
    }
    pub unsafe fn ClearSelectors(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearSelectors)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FragmentPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FragmentPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFragmentPath(&self, text: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFragmentPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)) }
    }
    pub unsafe fn FragmentDialect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FragmentDialect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFragmentDialect(&self, text: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFragmentDialect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddOption(&self, optionname: &windows_core::BSTR, optionvalue: &super::VARIANT, mustcomply: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddOption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(optionname), core::mem::transmute_copy(optionvalue), mustcomply.into()) }
    }
    pub unsafe fn SetMustUnderstandOptions(&self, mustunderstand: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMustUnderstandOptions)(windows_core::Interface::as_raw(self), mustunderstand.into()) }
    }
    pub unsafe fn MustUnderstandOptions(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MustUnderstandOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ClearOptions(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearOptions)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManResourceLocator_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetResourceURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AddSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AddSelector: usize,
    pub ClearSelectors: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FragmentPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFragmentPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FragmentDialect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFragmentDialect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AddOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AddOption: usize,
    pub SetMustUnderstandOptions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub MustUnderstandOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ClearOptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManResourceLocator_Impl: super::IDispatch_Impl {
    fn SetResourceURI(&self, uri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ResourceURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AddSelector(&self, resourceselname: &windows_core::BSTR, selvalue: &super::VARIANT) -> windows_core::Result<()>;
    fn ClearSelectors(&self) -> windows_core::Result<()>;
    fn FragmentPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFragmentPath(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FragmentDialect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFragmentDialect(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddOption(&self, optionname: &windows_core::BSTR, optionvalue: &super::VARIANT, mustcomply: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetMustUnderstandOptions(&self, mustunderstand: windows_core::BOOL) -> windows_core::Result<()>;
    fn MustUnderstandOptions(&self) -> windows_core::Result<windows_core::BOOL>;
    fn ClearOptions(&self) -> windows_core::Result<()>;
    fn Error(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManResourceLocator_Vtbl {
    pub const fn new<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetResourceURI<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::SetResourceURI(this, core::mem::transmute(&uri)).into()
            }
        }
        unsafe extern "system" fn ResourceURI<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::ResourceURI(this) {
                    Ok(ok__) => {
                        uri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddSelector<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceselname: *mut core::ffi::c_void, selvalue: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::AddSelector(this, core::mem::transmute(&resourceselname), core::mem::transmute(&selvalue)).into()
            }
        }
        unsafe extern "system" fn ClearSelectors<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::ClearSelectors(this).into()
            }
        }
        unsafe extern "system" fn FragmentPath<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::FragmentPath(this) {
                    Ok(ok__) => {
                        text.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFragmentPath<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::SetFragmentPath(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn FragmentDialect<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::FragmentDialect(this) {
                    Ok(ok__) => {
                        text.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFragmentDialect<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::SetFragmentDialect(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn AddOption<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionname: *mut core::ffi::c_void, optionvalue: super::VARIANT, mustcomply: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::AddOption(this, core::mem::transmute(&optionname), core::mem::transmute(&optionvalue), core::mem::transmute_copy(&mustcomply)).into()
            }
        }
        unsafe extern "system" fn SetMustUnderstandOptions<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mustunderstand: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::SetMustUnderstandOptions(this, core::mem::transmute_copy(&mustunderstand)).into()
            }
        }
        unsafe extern "system" fn MustUnderstandOptions<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mustunderstand: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::MustUnderstandOptions(this) {
                    Ok(ok__) => {
                        mustunderstand.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClearOptions<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::ClearOptions(this).into()
            }
        }
        unsafe extern "system" fn Error<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::Error(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetResourceURI: SetResourceURI::<Identity, OFFSET>,
            ResourceURI: ResourceURI::<Identity, OFFSET>,
            AddSelector: AddSelector::<Identity, OFFSET>,
            ClearSelectors: ClearSelectors::<Identity, OFFSET>,
            FragmentPath: FragmentPath::<Identity, OFFSET>,
            SetFragmentPath: SetFragmentPath::<Identity, OFFSET>,
            FragmentDialect: FragmentDialect::<Identity, OFFSET>,
            SetFragmentDialect: SetFragmentDialect::<Identity, OFFSET>,
            AddOption: AddOption::<Identity, OFFSET>,
            SetMustUnderstandOptions: SetMustUnderstandOptions::<Identity, OFFSET>,
            MustUnderstandOptions: MustUnderstandOptions::<Identity, OFFSET>,
            ClearOptions: ClearOptions::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManResourceLocator as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManResourceLocator {}
windows_core::imp::define_interface!(IWSManResourceLocatorInternal, IWSManResourceLocatorInternal_Vtbl, 0xeffaead7_7ec8_4716_b9be_f2e7e9fb4adb);
windows_core::imp::interface_hierarchy!(IWSManResourceLocatorInternal, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IWSManResourceLocatorInternal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IWSManResourceLocatorInternal_Impl: windows_core::IUnknownImpl {}
impl IWSManResourceLocatorInternal_Vtbl {
    pub const fn new<Identity: IWSManResourceLocatorInternal_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManResourceLocatorInternal as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSManResourceLocatorInternal {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWSManSession, IWSManSession_Vtbl, 0xfc84fc58_1286_40c4_9da0_c8ef6ec241e0);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWSManSession {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWSManSession, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWSManSession {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Get(&self, resourceuri: &super::VARIANT, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Put(&self, resourceuri: &super::VARIANT, resource: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), core::mem::transmute_copy(resource), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Create(&self, resourceuri: &super::VARIANT, resource: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), core::mem::transmute_copy(resource), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Delete(&self, resourceuri: &super::VARIANT, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), flags) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Invoke(&self, actionuri: &windows_core::BSTR, resourceuri: &super::VARIANT, parameters: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(actionuri), core::mem::transmute_copy(resourceuri), core::mem::transmute_copy(parameters), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Enumerate(&self, resourceuri: &super::VARIANT, filter: &windows_core::BSTR, dialect: &windows_core::BSTR, flags: i32) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enumerate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), core::mem::transmute_copy(filter), core::mem::transmute_copy(dialect), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Identify(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Identify)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BatchItems(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BatchItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBatchItems(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBatchItems)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn Timeout(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTimeout(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTimeout)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManSession_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Get: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Put: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Create: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Delete: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Invoke: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Enumerate: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Enumerate: usize,
    pub Identify: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BatchItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBatchItems: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Timeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWSManSession_Impl: super::IDispatch_Impl {
    fn Get(&self, resourceuri: &super::VARIANT, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Put(&self, resourceuri: &super::VARIANT, resource: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Create(&self, resourceuri: &super::VARIANT, resource: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Delete(&self, resourceuri: &super::VARIANT, flags: i32) -> windows_core::Result<()>;
    fn Invoke(&self, actionuri: &windows_core::BSTR, resourceuri: &super::VARIANT, parameters: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Enumerate(&self, resourceuri: &super::VARIANT, filter: &windows_core::BSTR, dialect: &windows_core::BSTR, flags: i32) -> windows_core::Result<super::IDispatch>;
    fn Identify(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Error(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BatchItems(&self) -> windows_core::Result<i32>;
    fn SetBatchItems(&self, value: i32) -> windows_core::Result<()>;
    fn Timeout(&self) -> windows_core::Result<i32>;
    fn SetTimeout(&self, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWSManSession_Vtbl {
    pub const fn new<Identity: IWSManSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::VARIANT, flags: i32, resource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Get(this, core::mem::transmute(&resourceuri), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        resource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Put<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::VARIANT, resource: *mut core::ffi::c_void, flags: i32, resultresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Put(this, core::mem::transmute(&resourceuri), core::mem::transmute(&resource), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        resultresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::VARIANT, resource: *mut core::ffi::c_void, flags: i32, newuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Create(this, core::mem::transmute(&resourceuri), core::mem::transmute(&resource), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        newuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::VARIANT, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManSession_Impl::Delete(this, core::mem::transmute(&resourceuri), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Invoke<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actionuri: *mut core::ffi::c_void, resourceuri: super::VARIANT, parameters: *mut core::ffi::c_void, flags: i32, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Invoke(this, core::mem::transmute(&actionuri), core::mem::transmute(&resourceuri), core::mem::transmute(&parameters), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enumerate<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::VARIANT, filter: *mut core::ffi::c_void, dialect: *mut core::ffi::c_void, flags: i32, resultset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Enumerate(this, core::mem::transmute(&resourceuri), core::mem::transmute(&filter), core::mem::transmute(&dialect), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        resultset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Identify<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Identify(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Error<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Error(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BatchItems<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::BatchItems(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBatchItems<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManSession_Impl::SetBatchItems(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Timeout<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Timeout(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTimeout<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManSession_Impl::SetTimeout(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            Put: Put::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
            Enumerate: Enumerate::<Identity, OFFSET>,
            Identify: Identify::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
            BatchItems: BatchItems::<Identity, OFFSET>,
            SetBatchItems: SetBatchItems::<Identity, OFFSET>,
            Timeout: Timeout::<Identity, OFFSET>,
            SetTimeout: SetTimeout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManSession as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWSManSession {}
pub const WSMan: windows_core::GUID = windows_core::GUID::from_u128(0xbced617b_ec03_420b_8508_977dc7a686bd);
pub type WSManEnumFlags = i32;
pub const WSManFlagAllowNegotiateImplicitCredentials: WSManSessionFlags = 67108864;
pub const WSManFlagAssociatedInstance: WSManEnumFlags = 0;
pub const WSManFlagAssociationInstance: WSManEnumFlags = 128;
pub const WSManFlagCredUsernamePassword: WSManSessionFlags = 4096;
pub const WSManFlagEnableSPNServerPort: WSManSessionFlags = 4194304;
pub const WSManFlagHierarchyDeep: WSManEnumFlags = 0;
pub const WSManFlagHierarchyDeepBasePropsOnly: WSManEnumFlags = 64;
pub const WSManFlagHierarchyShallow: WSManEnumFlags = 32;
pub const WSManFlagNoEncryption: WSManSessionFlags = 1048576;
pub const WSManFlagNonXmlText: WSManEnumFlags = 1;
pub const WSManFlagProxyAuthenticationUseBasic: WSManProxyAuthenticationFlags = 2;
pub const WSManFlagProxyAuthenticationUseDigest: WSManProxyAuthenticationFlags = 4;
pub const WSManFlagProxyAuthenticationUseNegotiate: WSManProxyAuthenticationFlags = 1;
pub const WSManFlagReturnEPR: WSManEnumFlags = 2;
pub const WSManFlagReturnObject: WSManEnumFlags = 0;
pub const WSManFlagReturnObjectAndEPR: WSManEnumFlags = 4;
pub const WSManFlagSkipCACheck: WSManSessionFlags = 8192;
pub const WSManFlagSkipCNCheck: WSManSessionFlags = 16384;
pub const WSManFlagSkipRevocationCheck: WSManSessionFlags = 33554432;
pub const WSManFlagUTF16: WSManSessionFlags = 8388608;
pub const WSManFlagUTF8: WSManSessionFlags = 1;
pub const WSManFlagUseBasic: WSManSessionFlags = 262144;
pub const WSManFlagUseClientCertificate: WSManSessionFlags = 2097152;
pub const WSManFlagUseCredSsp: WSManSessionFlags = 16777216;
pub const WSManFlagUseDigest: WSManSessionFlags = 65536;
pub const WSManFlagUseKerberos: WSManSessionFlags = 524288;
pub const WSManFlagUseNegotiate: WSManSessionFlags = 131072;
pub const WSManFlagUseNoAuthentication: WSManSessionFlags = 32768;
pub const WSManFlagUseSsl: WSManSessionFlags = 134217728;
pub const WSManInternal: windows_core::GUID = windows_core::GUID::from_u128(0x7de087a5_5dcb_4df7_bb12_0924ad8fbd9a);
pub type WSManProxyAccessTypeFlags = i32;
pub type WSManProxyAuthenticationFlags = i32;
pub const WSManProxyAutoDetect: WSManProxyAccessTypeFlags = 4;
pub const WSManProxyIEConfig: WSManProxyAccessTypeFlags = 1;
pub const WSManProxyNoProxyServer: WSManProxyAccessTypeFlags = 8;
pub const WSManProxyWinHttpConfig: WSManProxyAccessTypeFlags = 2;
pub type WSManSessionFlags = i32;
