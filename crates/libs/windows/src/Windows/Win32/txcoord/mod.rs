pub type AUTHENTICATION_LEVEL = i32;
windows_core::imp::define_interface!(IDtcNetworkAccessConfig, IDtcNetworkAccessConfig_Vtbl, 0x9797c15d_a428_4291_87b6_0995031a678d);
windows_core::imp::interface_hierarchy!(IDtcNetworkAccessConfig, windows_core::IUnknown);
impl IDtcNetworkAccessConfig {
    pub unsafe fn GetAnyNetworkAccess(&self, pbanynetworkaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAnyNetworkAccess)(windows_core::Interface::as_raw(self), pbanynetworkaccess as _) }
    }
    pub unsafe fn SetAnyNetworkAccess(&self, banynetworkaccess: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAnyNetworkAccess)(windows_core::Interface::as_raw(self), banynetworkaccess.into()) }
    }
    pub unsafe fn GetNetworkAdministrationAccess(&self, pbnetworkadministrationaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkAdministrationAccess)(windows_core::Interface::as_raw(self), pbnetworkadministrationaccess as _) }
    }
    pub unsafe fn SetNetworkAdministrationAccess(&self, bnetworkadministrationaccess: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkAdministrationAccess)(windows_core::Interface::as_raw(self), bnetworkadministrationaccess.into()) }
    }
    pub unsafe fn GetNetworkTransactionAccess(&self, pbnetworktransactionaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkTransactionAccess)(windows_core::Interface::as_raw(self), pbnetworktransactionaccess as _) }
    }
    pub unsafe fn SetNetworkTransactionAccess(&self, bnetworktransactionaccess: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkTransactionAccess)(windows_core::Interface::as_raw(self), bnetworktransactionaccess.into()) }
    }
    pub unsafe fn GetNetworkClientAccess(&self, pbnetworkclientaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkClientAccess)(windows_core::Interface::as_raw(self), pbnetworkclientaccess as _) }
    }
    pub unsafe fn SetNetworkClientAccess(&self, bnetworkclientaccess: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkClientAccess)(windows_core::Interface::as_raw(self), bnetworkclientaccess.into()) }
    }
    pub unsafe fn GetNetworkTIPAccess(&self, pbnetworktipaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkTIPAccess)(windows_core::Interface::as_raw(self), pbnetworktipaccess as _) }
    }
    pub unsafe fn SetNetworkTIPAccess(&self, bnetworktipaccess: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkTIPAccess)(windows_core::Interface::as_raw(self), bnetworktipaccess.into()) }
    }
    pub unsafe fn GetXAAccess(&self, pbxaaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetXAAccess)(windows_core::Interface::as_raw(self), pbxaaccess as _) }
    }
    pub unsafe fn SetXAAccess(&self, bxaaccess: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetXAAccess)(windows_core::Interface::as_raw(self), bxaaccess.into()) }
    }
    pub unsafe fn RestartDtcService(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestartDtcService)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAnyNetworkAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetAnyNetworkAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkAdministrationAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkAdministrationAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkTransactionAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkTransactionAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkClientAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkClientAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkTIPAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkTIPAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetXAAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetXAAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub RestartDtcService: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcNetworkAccessConfig_Impl: windows_core::IUnknownImpl {
    fn GetAnyNetworkAccess(&self, pbanynetworkaccess: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetAnyNetworkAccess(&self, banynetworkaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNetworkAdministrationAccess(&self, pbnetworkadministrationaccess: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNetworkAdministrationAccess(&self, bnetworkadministrationaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNetworkTransactionAccess(&self, pbnetworktransactionaccess: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNetworkTransactionAccess(&self, bnetworktransactionaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNetworkClientAccess(&self, pbnetworkclientaccess: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNetworkClientAccess(&self, bnetworkclientaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNetworkTIPAccess(&self, pbnetworktipaccess: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNetworkTIPAccess(&self, bnetworktipaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetXAAccess(&self, pbxaaccess: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetXAAccess(&self, bxaaccess: windows_core::BOOL) -> windows_core::Result<()>;
    fn RestartDtcService(&self) -> windows_core::Result<()>;
}
impl IDtcNetworkAccessConfig_Vtbl {
    pub const fn new<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAnyNetworkAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbanynetworkaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::GetAnyNetworkAccess(this, core::mem::transmute_copy(&pbanynetworkaccess)).into()
            }
        }
        unsafe extern "system" fn SetAnyNetworkAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, banynetworkaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetAnyNetworkAccess(this, core::mem::transmute_copy(&banynetworkaccess)).into()
            }
        }
        unsafe extern "system" fn GetNetworkAdministrationAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworkadministrationaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::GetNetworkAdministrationAccess(this, core::mem::transmute_copy(&pbnetworkadministrationaccess)).into()
            }
        }
        unsafe extern "system" fn SetNetworkAdministrationAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworkadministrationaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetNetworkAdministrationAccess(this, core::mem::transmute_copy(&bnetworkadministrationaccess)).into()
            }
        }
        unsafe extern "system" fn GetNetworkTransactionAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworktransactionaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::GetNetworkTransactionAccess(this, core::mem::transmute_copy(&pbnetworktransactionaccess)).into()
            }
        }
        unsafe extern "system" fn SetNetworkTransactionAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworktransactionaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetNetworkTransactionAccess(this, core::mem::transmute_copy(&bnetworktransactionaccess)).into()
            }
        }
        unsafe extern "system" fn GetNetworkClientAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworkclientaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::GetNetworkClientAccess(this, core::mem::transmute_copy(&pbnetworkclientaccess)).into()
            }
        }
        unsafe extern "system" fn SetNetworkClientAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworkclientaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetNetworkClientAccess(this, core::mem::transmute_copy(&bnetworkclientaccess)).into()
            }
        }
        unsafe extern "system" fn GetNetworkTIPAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworktipaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::GetNetworkTIPAccess(this, core::mem::transmute_copy(&pbnetworktipaccess)).into()
            }
        }
        unsafe extern "system" fn SetNetworkTIPAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworktipaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetNetworkTIPAccess(this, core::mem::transmute_copy(&bnetworktipaccess)).into()
            }
        }
        unsafe extern "system" fn GetXAAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbxaaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::GetXAAccess(this, core::mem::transmute_copy(&pbxaaccess)).into()
            }
        }
        unsafe extern "system" fn SetXAAccess<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bxaaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::SetXAAccess(this, core::mem::transmute_copy(&bxaaccess)).into()
            }
        }
        unsafe extern "system" fn RestartDtcService<Identity: IDtcNetworkAccessConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig_Impl::RestartDtcService(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAnyNetworkAccess: GetAnyNetworkAccess::<Identity, OFFSET>,
            SetAnyNetworkAccess: SetAnyNetworkAccess::<Identity, OFFSET>,
            GetNetworkAdministrationAccess: GetNetworkAdministrationAccess::<Identity, OFFSET>,
            SetNetworkAdministrationAccess: SetNetworkAdministrationAccess::<Identity, OFFSET>,
            GetNetworkTransactionAccess: GetNetworkTransactionAccess::<Identity, OFFSET>,
            SetNetworkTransactionAccess: SetNetworkTransactionAccess::<Identity, OFFSET>,
            GetNetworkClientAccess: GetNetworkClientAccess::<Identity, OFFSET>,
            SetNetworkClientAccess: SetNetworkClientAccess::<Identity, OFFSET>,
            GetNetworkTIPAccess: GetNetworkTIPAccess::<Identity, OFFSET>,
            SetNetworkTIPAccess: SetNetworkTIPAccess::<Identity, OFFSET>,
            GetXAAccess: GetXAAccess::<Identity, OFFSET>,
            SetXAAccess: SetXAAccess::<Identity, OFFSET>,
            RestartDtcService: RestartDtcService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcNetworkAccessConfig {}
windows_core::imp::define_interface!(IDtcNetworkAccessConfig2, IDtcNetworkAccessConfig2_Vtbl, 0xa7aa013b_eb7d_4f42_b41c_b2dec09ae034);
impl core::ops::Deref for IDtcNetworkAccessConfig2 {
    type Target = IDtcNetworkAccessConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcNetworkAccessConfig2, windows_core::IUnknown, IDtcNetworkAccessConfig);
impl IDtcNetworkAccessConfig2 {
    pub unsafe fn GetNetworkInboundAccess(&self, pbinbound: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkInboundAccess)(windows_core::Interface::as_raw(self), pbinbound as _) }
    }
    pub unsafe fn GetNetworkOutboundAccess(&self, pboutbound: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkOutboundAccess)(windows_core::Interface::as_raw(self), pboutbound as _) }
    }
    pub unsafe fn SetNetworkInboundAccess(&self, binbound: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkInboundAccess)(windows_core::Interface::as_raw(self), binbound.into()) }
    }
    pub unsafe fn SetNetworkOutboundAccess(&self, boutbound: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkOutboundAccess)(windows_core::Interface::as_raw(self), boutbound.into()) }
    }
    pub unsafe fn GetAuthenticationLevel(&self, pauthlevel: *mut AUTHENTICATION_LEVEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAuthenticationLevel)(windows_core::Interface::as_raw(self), pauthlevel as _) }
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationLevel)(windows_core::Interface::as_raw(self), authlevel) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig2_Vtbl {
    pub base__: IDtcNetworkAccessConfig_Vtbl,
    pub GetNetworkInboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetNetworkOutboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkInboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNetworkOutboundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AUTHENTICATION_LEVEL) -> windows_core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, AUTHENTICATION_LEVEL) -> windows_core::HRESULT,
}
pub trait IDtcNetworkAccessConfig2_Impl: IDtcNetworkAccessConfig_Impl {
    fn GetNetworkInboundAccess(&self, pbinbound: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNetworkOutboundAccess(&self, pboutbound: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNetworkInboundAccess(&self, binbound: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNetworkOutboundAccess(&self, boutbound: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAuthenticationLevel(&self, pauthlevel: *mut AUTHENTICATION_LEVEL) -> windows_core::Result<()>;
    fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> windows_core::Result<()>;
}
impl IDtcNetworkAccessConfig2_Vtbl {
    pub const fn new<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNetworkInboundAccess<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbinbound: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig2_Impl::GetNetworkInboundAccess(this, core::mem::transmute_copy(&pbinbound)).into()
            }
        }
        unsafe extern "system" fn GetNetworkOutboundAccess<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboutbound: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig2_Impl::GetNetworkOutboundAccess(this, core::mem::transmute_copy(&pboutbound)).into()
            }
        }
        unsafe extern "system" fn SetNetworkInboundAccess<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binbound: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig2_Impl::SetNetworkInboundAccess(this, core::mem::transmute_copy(&binbound)).into()
            }
        }
        unsafe extern "system" fn SetNetworkOutboundAccess<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boutbound: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig2_Impl::SetNetworkOutboundAccess(this, core::mem::transmute_copy(&boutbound)).into()
            }
        }
        unsafe extern "system" fn GetAuthenticationLevel<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig2_Impl::GetAuthenticationLevel(this, core::mem::transmute_copy(&pauthlevel)).into()
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: IDtcNetworkAccessConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig2_Impl::SetAuthenticationLevel(this, core::mem::transmute_copy(&authlevel)).into()
            }
        }
        Self {
            base__: IDtcNetworkAccessConfig_Vtbl::new::<Identity, OFFSET>(),
            GetNetworkInboundAccess: GetNetworkInboundAccess::<Identity, OFFSET>,
            GetNetworkOutboundAccess: GetNetworkOutboundAccess::<Identity, OFFSET>,
            SetNetworkInboundAccess: SetNetworkInboundAccess::<Identity, OFFSET>,
            SetNetworkOutboundAccess: SetNetworkOutboundAccess::<Identity, OFFSET>,
            GetAuthenticationLevel: GetAuthenticationLevel::<Identity, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig2 as windows_core::Interface>::IID || iid == &<IDtcNetworkAccessConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcNetworkAccessConfig2 {}
windows_core::imp::define_interface!(IDtcNetworkAccessConfig3, IDtcNetworkAccessConfig3_Vtbl, 0x76e4b4f3_2ca5_466b_89d5_fd218ee75b49);
impl core::ops::Deref for IDtcNetworkAccessConfig3 {
    type Target = IDtcNetworkAccessConfig2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDtcNetworkAccessConfig3, windows_core::IUnknown, IDtcNetworkAccessConfig, IDtcNetworkAccessConfig2);
impl IDtcNetworkAccessConfig3 {
    pub unsafe fn GetLUAccess(&self, pbluaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLUAccess)(windows_core::Interface::as_raw(self), pbluaccess as _) }
    }
    pub unsafe fn SetLUAccess(&self, bluaccess: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLUAccess)(windows_core::Interface::as_raw(self), bluaccess.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcNetworkAccessConfig3_Vtbl {
    pub base__: IDtcNetworkAccessConfig2_Vtbl,
    pub GetLUAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetLUAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IDtcNetworkAccessConfig3_Impl: IDtcNetworkAccessConfig2_Impl {
    fn GetLUAccess(&self, pbluaccess: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetLUAccess(&self, bluaccess: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IDtcNetworkAccessConfig3_Vtbl {
    pub const fn new<Identity: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLUAccess<Identity: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbluaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig3_Impl::GetLUAccess(this, core::mem::transmute_copy(&pbluaccess)).into()
            }
        }
        unsafe extern "system" fn SetLUAccess<Identity: IDtcNetworkAccessConfig3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bluaccess: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcNetworkAccessConfig3_Impl::SetLUAccess(this, core::mem::transmute_copy(&bluaccess)).into()
            }
        }
        Self {
            base__: IDtcNetworkAccessConfig2_Vtbl::new::<Identity, OFFSET>(),
            GetLUAccess: GetLUAccess::<Identity, OFFSET>,
            SetLUAccess: SetLUAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig3 as windows_core::Interface>::IID || iid == &<IDtcNetworkAccessConfig as windows_core::Interface>::IID || iid == &<IDtcNetworkAccessConfig2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcNetworkAccessConfig3 {}
pub const IID_IDtcNetworkAccessConfig: windows_core::GUID = windows_core::GUID::from_u128(0x9797c15d_a428_4291_87b6_0995031a678d);
pub const IID_IDtcNetworkAccessConfig2: windows_core::GUID = windows_core::GUID::from_u128(0xa7aa013b_eb7d_4f42_b41c_b2dec09ae034);
pub const IID_IDtcNetworkAccessConfig3: windows_core::GUID = windows_core::GUID::from_u128(0x76e4b4f3_2ca5_466b_89d5_fd218ee75b49);
pub const IID_ITipHelper: windows_core::GUID = windows_core::GUID::from_u128(0x17cf72d1_bac5_11d1_b1bf_00c04fc2f3ef);
pub const IID_ITipPullSink: windows_core::GUID = windows_core::GUID::from_u128(0x17cf72d2_bac5_11d1_b1bf_00c04fc2f3ef);
pub const IID_ITipTransaction: windows_core::GUID = windows_core::GUID::from_u128(0x17cf72d0_bac5_11d1_b1bf_00c04fc2f3ef);
pub const IID_ITransactionEnlistmentAsync: windows_core::GUID = windows_core::GUID::from_u128(0x0fb15081_af41_11ce_bd2b_204c4f4f5020);
pub const IID_ITransactionExport: windows_core::GUID = windows_core::GUID::from_u128(0x0141fda5_8fc0_11ce_bd18_204c4f4f5020);
pub const IID_ITransactionExportFactory: windows_core::GUID = windows_core::GUID::from_u128(0xe1cf9b53_8745_11ce_a9ba_00aa006c3706);
pub const IID_ITransactionImport: windows_core::GUID = windows_core::GUID::from_u128(0xe1cf9b5a_8745_11ce_a9ba_00aa006c3706);
pub const IID_ITransactionImportWhereabouts: windows_core::GUID = windows_core::GUID::from_u128(0x0141fda4_8fc0_11ce_bd18_204c4f4f5020);
pub const IID_ITransactionLastEnlistmentAsync: windows_core::GUID = windows_core::GUID::from_u128(0xc82bd533_5b30_11d3_8a91_00c04f79eb6d);
pub const IID_ITransactionLastResourceAsync: windows_core::GUID = windows_core::GUID::from_u128(0xc82bd532_5b30_11d3_8a91_00c04f79eb6d);
pub const IID_ITransactionResource: windows_core::GUID = windows_core::GUID::from_u128(0xee5ff7b3_4572_11d0_9452_00a0c905416e);
pub const IID_ITransactionResourceAsync: windows_core::GUID = windows_core::GUID::from_u128(0x69e971f0_23ce_11cf_ad60_00aa00a74ccd);
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 1;
windows_core::imp::define_interface!(ITipHelper, ITipHelper_Vtbl, 0x17cf72d1_bac5_11d1_b1bf_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITipHelper, windows_core::IUnknown);
impl ITipHelper {
    #[cfg(feature = "transact")]
    pub unsafe fn Pull(&self, i_psztxurl: *const i8, o_ppitransaction: *mut Option<super::ITransaction>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), i_psztxurl, core::mem::transmute(o_ppitransaction)) }
    }
    #[cfg(feature = "transact")]
    pub unsafe fn PullAsync<P1>(&self, i_psztxurl: *const i8, i_ptippullsink: P1, o_ppitransaction: *mut Option<super::ITransaction>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITipPullSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).PullAsync)(windows_core::Interface::as_raw(self), i_psztxurl, i_ptippullsink.param().abi(), core::mem::transmute(o_ppitransaction)) }
    }
    pub unsafe fn GetLocalTmUrl(&self, o_ppszlocaltmurl: *mut *mut i8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocalTmUrl)(windows_core::Interface::as_raw(self), o_ppszlocaltmurl as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "transact")]
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    Pull: usize,
    #[cfg(feature = "transact")]
    pub PullAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    PullAsync: usize,
    pub GetLocalTmUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut i8) -> windows_core::HRESULT,
}
#[cfg(feature = "transact")]
pub trait ITipHelper_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, i_psztxurl: *const i8, o_ppitransaction: windows_core::OutRef<super::ITransaction>) -> windows_core::Result<()>;
    fn PullAsync(&self, i_psztxurl: *const i8, i_ptippullsink: windows_core::Ref<ITipPullSink>, o_ppitransaction: windows_core::OutRef<super::ITransaction>) -> windows_core::Result<()>;
    fn GetLocalTmUrl(&self, o_ppszlocaltmurl: *mut *mut i8) -> windows_core::Result<()>;
}
#[cfg(feature = "transact")]
impl ITipHelper_Vtbl {
    pub const fn new<Identity: ITipHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: ITipHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_psztxurl: *const i8, o_ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITipHelper_Impl::Pull(this, core::mem::transmute_copy(&i_psztxurl), core::mem::transmute_copy(&o_ppitransaction)).into()
            }
        }
        unsafe extern "system" fn PullAsync<Identity: ITipHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_psztxurl: *const i8, i_ptippullsink: *mut core::ffi::c_void, o_ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITipHelper_Impl::PullAsync(this, core::mem::transmute_copy(&i_psztxurl), core::mem::transmute_copy(&i_ptippullsink), core::mem::transmute_copy(&o_ppitransaction)).into()
            }
        }
        unsafe extern "system" fn GetLocalTmUrl<Identity: ITipHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, o_ppszlocaltmurl: *mut *mut i8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITipHelper_Impl::GetLocalTmUrl(this, core::mem::transmute_copy(&o_ppszlocaltmurl)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Pull: Pull::<Identity, OFFSET>,
            PullAsync: PullAsync::<Identity, OFFSET>,
            GetLocalTmUrl: GetLocalTmUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipHelper as windows_core::Interface>::IID
    }
}
#[cfg(feature = "transact")]
impl windows_core::RuntimeName for ITipHelper {}
windows_core::imp::define_interface!(ITipPullSink, ITipPullSink_Vtbl, 0x17cf72d2_bac5_11d1_b1bf_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITipPullSink, windows_core::IUnknown);
impl ITipPullSink {
    pub unsafe fn PullComplete(&self, i_hrpull: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PullComplete)(windows_core::Interface::as_raw(self), i_hrpull) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipPullSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PullComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ITipPullSink_Impl: windows_core::IUnknownImpl {
    fn PullComplete(&self, i_hrpull: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ITipPullSink_Vtbl {
    pub const fn new<Identity: ITipPullSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PullComplete<Identity: ITipPullSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_hrpull: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITipPullSink_Impl::PullComplete(this, core::mem::transmute_copy(&i_hrpull)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PullComplete: PullComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipPullSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITipPullSink {}
windows_core::imp::define_interface!(ITipTransaction, ITipTransaction_Vtbl, 0x17cf72d0_bac5_11d1_b1bf_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITipTransaction, windows_core::IUnknown);
impl ITipTransaction {
    pub unsafe fn Push(&self, i_pszremotetmurl: *const i8, o_ppszremotetxurl: *mut windows_core::PSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), i_pszremotetmurl, o_ppszremotetxurl as _) }
    }
    pub unsafe fn GetTransactionUrl(&self, o_ppszlocaltxurl: *mut windows_core::PSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionUrl)(windows_core::Interface::as_raw(self), o_ppszlocaltxurl as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipTransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const i8, *mut windows_core::PSTR) -> windows_core::HRESULT,
    pub GetTransactionUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PSTR) -> windows_core::HRESULT,
}
pub trait ITipTransaction_Impl: windows_core::IUnknownImpl {
    fn Push(&self, i_pszremotetmurl: *const i8, o_ppszremotetxurl: *mut windows_core::PSTR) -> windows_core::Result<()>;
    fn GetTransactionUrl(&self, o_ppszlocaltxurl: *mut windows_core::PSTR) -> windows_core::Result<()>;
}
impl ITipTransaction_Vtbl {
    pub const fn new<Identity: ITipTransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Push<Identity: ITipTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_pszremotetmurl: *const i8, o_ppszremotetxurl: *mut windows_core::PSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITipTransaction_Impl::Push(this, core::mem::transmute_copy(&i_pszremotetmurl), core::mem::transmute_copy(&o_ppszremotetxurl)).into()
            }
        }
        unsafe extern "system" fn GetTransactionUrl<Identity: ITipTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, o_ppszlocaltxurl: *mut windows_core::PSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITipTransaction_Impl::GetTransactionUrl(this, core::mem::transmute_copy(&o_ppszlocaltxurl)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Push: Push::<Identity, OFFSET>,
            GetTransactionUrl: GetTransactionUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipTransaction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITipTransaction {}
windows_core::imp::define_interface!(ITransactionEnlistmentAsync, ITransactionEnlistmentAsync_Vtbl, 0x0fb15081_af41_11ce_bd2b_204c4f4f5020);
windows_core::imp::interface_hierarchy!(ITransactionEnlistmentAsync, windows_core::IUnknown);
impl ITransactionEnlistmentAsync {
    #[cfg(all(feature = "objidl", feature = "rpc", feature = "transact"))]
    pub unsafe fn PrepareRequestDone<P1>(&self, hr: windows_core::HRESULT, pmk: P1, pboidreason: Option<*const super::BOID>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).PrepareRequestDone)(windows_core::Interface::as_raw(self), hr, pmk.param().abi(), pboidreason.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CommitRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitRequestDone)(windows_core::Interface::as_raw(self), hr) }
    }
    pub unsafe fn AbortRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AbortRequestDone)(windows_core::Interface::as_raw(self), hr) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionEnlistmentAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "rpc", feature = "transact"))]
    pub PrepareRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void, *const super::BOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "rpc", feature = "transact")))]
    PrepareRequestDone: usize,
    pub CommitRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub AbortRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidl", feature = "rpc", feature = "transact"))]
pub trait ITransactionEnlistmentAsync_Impl: windows_core::IUnknownImpl {
    fn PrepareRequestDone(&self, hr: windows_core::HRESULT, pmk: windows_core::Ref<super::IMoniker>, pboidreason: *const super::BOID) -> windows_core::Result<()>;
    fn CommitRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn AbortRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "rpc", feature = "transact"))]
impl ITransactionEnlistmentAsync_Vtbl {
    pub const fn new<Identity: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareRequestDone<Identity: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, pmk: *mut core::ffi::c_void, pboidreason: *const super::BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionEnlistmentAsync_Impl::PrepareRequestDone(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&pmk), core::mem::transmute_copy(&pboidreason)).into()
            }
        }
        unsafe extern "system" fn CommitRequestDone<Identity: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionEnlistmentAsync_Impl::CommitRequestDone(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn AbortRequestDone<Identity: ITransactionEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionEnlistmentAsync_Impl::AbortRequestDone(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrepareRequestDone: PrepareRequestDone::<Identity, OFFSET>,
            CommitRequestDone: CommitRequestDone::<Identity, OFFSET>,
            AbortRequestDone: AbortRequestDone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionEnlistmentAsync as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "rpc", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionEnlistmentAsync {}
windows_core::imp::define_interface!(ITransactionExport, ITransactionExport_Vtbl, 0x0141fda5_8fc0_11ce_bd18_204c4f4f5020);
windows_core::imp::interface_hierarchy!(ITransactionExport, windows_core::IUnknown);
impl ITransactionExport {
    #[cfg(feature = "shared")]
    pub unsafe fn Export<P0>(&self, punktransaction: P0, pcbtransactioncookie: *mut super::ULONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Export)(windows_core::Interface::as_raw(self), punktransaction.param().abi(), pcbtransactioncookie as _) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn GetTransactionCookie<P0>(&self, punktransaction: P0, cbtransactioncookie: super::ULONG, rgbtransactioncookie: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionCookie)(windows_core::Interface::as_raw(self), punktransaction.param().abi(), cbtransactioncookie, rgbtransactioncookie as _, pcbused as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Export: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Export: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub GetTransactionCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG, *mut super::byte, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    GetTransactionCookie: usize,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait ITransactionExport_Impl: windows_core::IUnknownImpl {
    fn Export(&self, punktransaction: windows_core::Ref<windows_core::IUnknown>, pcbtransactioncookie: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetTransactionCookie(&self, punktransaction: windows_core::Ref<windows_core::IUnknown>, cbtransactioncookie: super::ULONG, rgbtransactioncookie: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl ITransactionExport_Vtbl {
    pub const fn new<Identity: ITransactionExport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Export<Identity: ITransactionExport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punktransaction: *mut core::ffi::c_void, pcbtransactioncookie: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionExport_Impl::Export(this, core::mem::transmute_copy(&punktransaction), core::mem::transmute_copy(&pcbtransactioncookie)).into()
            }
        }
        unsafe extern "system" fn GetTransactionCookie<Identity: ITransactionExport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punktransaction: *mut core::ffi::c_void, cbtransactioncookie: super::ULONG, rgbtransactioncookie: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionExport_Impl::GetTransactionCookie(this, core::mem::transmute_copy(&punktransaction), core::mem::transmute_copy(&cbtransactioncookie), core::mem::transmute_copy(&rgbtransactioncookie), core::mem::transmute_copy(&pcbused)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Export: Export::<Identity, OFFSET>,
            GetTransactionCookie: GetTransactionCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionExport as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for ITransactionExport {}
windows_core::imp::define_interface!(ITransactionExportFactory, ITransactionExportFactory_Vtbl, 0xe1cf9b53_8745_11ce_a9ba_00aa006c3706);
windows_core::imp::interface_hierarchy!(ITransactionExportFactory, windows_core::IUnknown);
impl ITransactionExportFactory {
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetRemoteClassId(&self, pclsid: *mut super::CLSID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRemoteClassId)(windows_core::Interface::as_raw(self), pclsid as _) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn Create(&self, rgbwhereabouts: &[super::byte], ppexport: *mut Option<ITransactionExport>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), rgbwhereabouts.len().try_into().unwrap(), rgbwhereabouts.as_ptr(), core::mem::transmute(ppexport)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionExportFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub GetRemoteClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CLSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetRemoteClassId: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::byte, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    Create: usize,
}
#[cfg(all(feature = "guiddef", feature = "rpc", feature = "shared"))]
pub trait ITransactionExportFactory_Impl: windows_core::IUnknownImpl {
    fn GetRemoteClassId(&self, pclsid: *mut super::CLSID) -> windows_core::Result<()>;
    fn Create(&self, cbwhereabouts: super::ULONG, rgbwhereabouts: *const super::byte, ppexport: windows_core::OutRef<ITransactionExport>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "rpc", feature = "shared"))]
impl ITransactionExportFactory_Vtbl {
    pub const fn new<Identity: ITransactionExportFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRemoteClassId<Identity: ITransactionExportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut super::CLSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionExportFactory_Impl::GetRemoteClassId(this, core::mem::transmute_copy(&pclsid)).into()
            }
        }
        unsafe extern "system" fn Create<Identity: ITransactionExportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbwhereabouts: super::ULONG, rgbwhereabouts: *const super::byte, ppexport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionExportFactory_Impl::Create(this, core::mem::transmute_copy(&cbwhereabouts), core::mem::transmute_copy(&rgbwhereabouts), core::mem::transmute_copy(&ppexport)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRemoteClassId: GetRemoteClassId::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionExportFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for ITransactionExportFactory {}
windows_core::imp::define_interface!(ITransactionImport, ITransactionImport_Vtbl, 0xe1cf9b5a_8745_11ce_a9ba_00aa006c3706);
windows_core::imp::interface_hierarchy!(ITransactionImport, windows_core::IUnknown);
impl ITransactionImport {
    #[cfg(all(feature = "guiddef", feature = "rpc", feature = "shared"))]
    pub unsafe fn Import<T>(&self, rgbtransactioncookie: &[super::byte]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), rgbtransactioncookie.len().try_into().unwrap(), rgbtransactioncookie.as_ptr(), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "rpc", feature = "shared"))]
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::byte, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "rpc", feature = "shared")))]
    Import: usize,
}
#[cfg(all(feature = "guiddef", feature = "rpc", feature = "shared"))]
pub trait ITransactionImport_Impl: windows_core::IUnknownImpl {
    fn Import(&self, cbtransactioncookie: super::ULONG, rgbtransactioncookie: *const super::byte, piid: *const super::IID, ppvtransaction: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "rpc", feature = "shared"))]
impl ITransactionImport_Vtbl {
    pub const fn new<Identity: ITransactionImport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Import<Identity: ITransactionImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtransactioncookie: super::ULONG, rgbtransactioncookie: *const super::byte, piid: *const super::IID, ppvtransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionImport_Impl::Import(this, core::mem::transmute_copy(&cbtransactioncookie), core::mem::transmute_copy(&rgbtransactioncookie), core::mem::transmute_copy(&piid), core::mem::transmute_copy(&ppvtransaction)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Import: Import::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionImport as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for ITransactionImport {}
windows_core::imp::define_interface!(ITransactionImportWhereabouts, ITransactionImportWhereabouts_Vtbl, 0x0141fda4_8fc0_11ce_bd18_204c4f4f5020);
windows_core::imp::interface_hierarchy!(ITransactionImportWhereabouts, windows_core::IUnknown);
impl ITransactionImportWhereabouts {
    #[cfg(feature = "shared")]
    pub unsafe fn GetWhereaboutsSize(&self, pcbwhereabouts: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWhereaboutsSize)(windows_core::Interface::as_raw(self), pcbwhereabouts as _) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn GetWhereabouts(&self, cbwhereabouts: super::ULONG, rgbwhereabouts: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWhereabouts)(windows_core::Interface::as_raw(self), cbwhereabouts, rgbwhereabouts as _, pcbused as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionImportWhereabouts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetWhereaboutsSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetWhereaboutsSize: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub GetWhereabouts: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut super::byte, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    GetWhereabouts: usize,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait ITransactionImportWhereabouts_Impl: windows_core::IUnknownImpl {
    fn GetWhereaboutsSize(&self, pcbwhereabouts: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetWhereabouts(&self, cbwhereabouts: super::ULONG, rgbwhereabouts: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl ITransactionImportWhereabouts_Vtbl {
    pub const fn new<Identity: ITransactionImportWhereabouts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWhereaboutsSize<Identity: ITransactionImportWhereabouts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbwhereabouts: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionImportWhereabouts_Impl::GetWhereaboutsSize(this, core::mem::transmute_copy(&pcbwhereabouts)).into()
            }
        }
        unsafe extern "system" fn GetWhereabouts<Identity: ITransactionImportWhereabouts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbwhereabouts: super::ULONG, rgbwhereabouts: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionImportWhereabouts_Impl::GetWhereabouts(this, core::mem::transmute_copy(&cbwhereabouts), core::mem::transmute_copy(&rgbwhereabouts), core::mem::transmute_copy(&pcbused)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWhereaboutsSize: GetWhereaboutsSize::<Identity, OFFSET>,
            GetWhereabouts: GetWhereabouts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionImportWhereabouts as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for ITransactionImportWhereabouts {}
windows_core::imp::define_interface!(ITransactionLastEnlistmentAsync, ITransactionLastEnlistmentAsync_Vtbl, 0xc82bd533_5b30_11d3_8a91_00c04f79eb6d);
windows_core::imp::interface_hierarchy!(ITransactionLastEnlistmentAsync, windows_core::IUnknown);
impl ITransactionLastEnlistmentAsync {
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub unsafe fn TransactionOutcome(&self, xactstat: super::XACTSTAT, pboidreason: Option<*const super::BOID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TransactionOutcome)(windows_core::Interface::as_raw(self), xactstat, pboidreason.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastEnlistmentAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub TransactionOutcome: unsafe extern "system" fn(*mut core::ffi::c_void, super::XACTSTAT, *const super::BOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "transact")))]
    TransactionOutcome: usize,
}
#[cfg(all(feature = "rpc", feature = "transact"))]
pub trait ITransactionLastEnlistmentAsync_Impl: windows_core::IUnknownImpl {
    fn TransactionOutcome(&self, xactstat: super::XACTSTAT, pboidreason: *const super::BOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "transact"))]
impl ITransactionLastEnlistmentAsync_Vtbl {
    pub const fn new<Identity: ITransactionLastEnlistmentAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransactionOutcome<Identity: ITransactionLastEnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xactstat: super::XACTSTAT, pboidreason: *const super::BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionLastEnlistmentAsync_Impl::TransactionOutcome(this, core::mem::transmute_copy(&xactstat), core::mem::transmute_copy(&pboidreason)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TransactionOutcome: TransactionOutcome::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionLastEnlistmentAsync as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionLastEnlistmentAsync {}
windows_core::imp::define_interface!(ITransactionLastResourceAsync, ITransactionLastResourceAsync_Vtbl, 0xc82bd532_5b30_11d3_8a91_00c04f79eb6d);
windows_core::imp::interface_hierarchy!(ITransactionLastResourceAsync, windows_core::IUnknown);
impl ITransactionLastResourceAsync {
    #[cfg(feature = "shared")]
    pub unsafe fn DelegateCommit(&self, grfrm: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DelegateCommit)(windows_core::Interface::as_raw(self), grfrm) }
    }
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub unsafe fn ForgetRequest(&self, pnewuow: *const super::XACTUOW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ForgetRequest)(windows_core::Interface::as_raw(self), pnewuow) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionLastResourceAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub DelegateCommit: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DelegateCommit: usize,
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub ForgetRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::XACTUOW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "transact")))]
    ForgetRequest: usize,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
pub trait ITransactionLastResourceAsync_Impl: windows_core::IUnknownImpl {
    fn DelegateCommit(&self, grfrm: super::DWORD) -> windows_core::Result<()>;
    fn ForgetRequest(&self, pnewuow: *const super::XACTUOW) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl ITransactionLastResourceAsync_Vtbl {
    pub const fn new<Identity: ITransactionLastResourceAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DelegateCommit<Identity: ITransactionLastResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfrm: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionLastResourceAsync_Impl::DelegateCommit(this, core::mem::transmute_copy(&grfrm)).into()
            }
        }
        unsafe extern "system" fn ForgetRequest<Identity: ITransactionLastResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewuow: *const super::XACTUOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionLastResourceAsync_Impl::ForgetRequest(this, core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DelegateCommit: DelegateCommit::<Identity, OFFSET>,
            ForgetRequest: ForgetRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionLastResourceAsync as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionLastResourceAsync {}
windows_core::imp::define_interface!(ITransactionResource, ITransactionResource_Vtbl, 0xee5ff7b3_4572_11d0_9452_00a0c905416e);
windows_core::imp::interface_hierarchy!(ITransactionResource, windows_core::IUnknown);
impl ITransactionResource {
    #[cfg(feature = "shared")]
    pub unsafe fn PrepareRequest(&self, fretaining: bool, grfrm: super::DWORD, fwantmoniker: bool, fsinglephase: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrepareRequest)(windows_core::Interface::as_raw(self), fretaining.into(), grfrm, fwantmoniker.into(), fsinglephase.into()) }
    }
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
    pub unsafe fn CommitRequest(&self, grfrm: super::DWORD, pnewuow: Option<*const super::XACTUOW>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitRequest)(windows_core::Interface::as_raw(self), grfrm, pnewuow.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub unsafe fn AbortRequest(&self, pboidreason: Option<*const super::BOID>, fretaining: bool, pnewuow: Option<*const super::XACTUOW>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AbortRequest)(windows_core::Interface::as_raw(self), pboidreason.unwrap_or(core::mem::zeroed()) as _, fretaining.into(), pnewuow.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn TMDown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TMDown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub PrepareRequest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::DWORD, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    PrepareRequest: usize,
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
    pub CommitRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::XACTUOW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared", feature = "transact")))]
    CommitRequest: usize,
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub AbortRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BOID, windows_core::BOOL, *const super::XACTUOW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "transact")))]
    AbortRequest: usize,
    pub TMDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
pub trait ITransactionResource_Impl: windows_core::IUnknownImpl {
    fn PrepareRequest(&self, fretaining: windows_core::BOOL, grfrm: super::DWORD, fwantmoniker: windows_core::BOOL, fsinglephase: windows_core::BOOL) -> windows_core::Result<()>;
    fn CommitRequest(&self, grfrm: super::DWORD, pnewuow: *const super::XACTUOW) -> windows_core::Result<()>;
    fn AbortRequest(&self, pboidreason: *const super::BOID, fretaining: windows_core::BOOL, pnewuow: *const super::XACTUOW) -> windows_core::Result<()>;
    fn TMDown(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl ITransactionResource_Vtbl {
    pub const fn new<Identity: ITransactionResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareRequest<Identity: ITransactionResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: windows_core::BOOL, grfrm: super::DWORD, fwantmoniker: windows_core::BOOL, fsinglephase: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResource_Impl::PrepareRequest(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&fwantmoniker), core::mem::transmute_copy(&fsinglephase)).into()
            }
        }
        unsafe extern "system" fn CommitRequest<Identity: ITransactionResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfrm: super::DWORD, pnewuow: *const super::XACTUOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResource_Impl::CommitRequest(this, core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        unsafe extern "system" fn AbortRequest<Identity: ITransactionResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const super::BOID, fretaining: windows_core::BOOL, pnewuow: *const super::XACTUOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResource_Impl::AbortRequest(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        unsafe extern "system" fn TMDown<Identity: ITransactionResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResource_Impl::TMDown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrepareRequest: PrepareRequest::<Identity, OFFSET>,
            CommitRequest: CommitRequest::<Identity, OFFSET>,
            AbortRequest: AbortRequest::<Identity, OFFSET>,
            TMDown: TMDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionResource {}
windows_core::imp::define_interface!(ITransactionResourceAsync, ITransactionResourceAsync_Vtbl, 0x69e971f0_23ce_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionResourceAsync, windows_core::IUnknown);
impl ITransactionResourceAsync {
    #[cfg(feature = "shared")]
    pub unsafe fn PrepareRequest(&self, fretaining: bool, grfrm: super::DWORD, fwantmoniker: bool, fsinglephase: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrepareRequest)(windows_core::Interface::as_raw(self), fretaining.into(), grfrm, fwantmoniker.into(), fsinglephase.into()) }
    }
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
    pub unsafe fn CommitRequest(&self, grfrm: super::DWORD, pnewuow: Option<*const super::XACTUOW>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitRequest)(windows_core::Interface::as_raw(self), grfrm, pnewuow.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub unsafe fn AbortRequest(&self, pboidreason: Option<*const super::BOID>, fretaining: bool, pnewuow: Option<*const super::XACTUOW>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AbortRequest)(windows_core::Interface::as_raw(self), pboidreason.unwrap_or(core::mem::zeroed()) as _, fretaining.into(), pnewuow.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn TMDown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TMDown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourceAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub PrepareRequest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::DWORD, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    PrepareRequest: usize,
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
    pub CommitRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::XACTUOW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared", feature = "transact")))]
    CommitRequest: usize,
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub AbortRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BOID, windows_core::BOOL, *const super::XACTUOW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "transact")))]
    AbortRequest: usize,
    pub TMDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
pub trait ITransactionResourceAsync_Impl: windows_core::IUnknownImpl {
    fn PrepareRequest(&self, fretaining: windows_core::BOOL, grfrm: super::DWORD, fwantmoniker: windows_core::BOOL, fsinglephase: windows_core::BOOL) -> windows_core::Result<()>;
    fn CommitRequest(&self, grfrm: super::DWORD, pnewuow: *const super::XACTUOW) -> windows_core::Result<()>;
    fn AbortRequest(&self, pboidreason: *const super::BOID, fretaining: windows_core::BOOL, pnewuow: *const super::XACTUOW) -> windows_core::Result<()>;
    fn TMDown(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl ITransactionResourceAsync_Vtbl {
    pub const fn new<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareRequest<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: windows_core::BOOL, grfrm: super::DWORD, fwantmoniker: windows_core::BOOL, fsinglephase: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResourceAsync_Impl::PrepareRequest(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&fwantmoniker), core::mem::transmute_copy(&fsinglephase)).into()
            }
        }
        unsafe extern "system" fn CommitRequest<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfrm: super::DWORD, pnewuow: *const super::XACTUOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResourceAsync_Impl::CommitRequest(this, core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        unsafe extern "system" fn AbortRequest<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const super::BOID, fretaining: windows_core::BOOL, pnewuow: *const super::XACTUOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResourceAsync_Impl::AbortRequest(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow)).into()
            }
        }
        unsafe extern "system" fn TMDown<Identity: ITransactionResourceAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResourceAsync_Impl::TMDown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrepareRequest: PrepareRequest::<Identity, OFFSET>,
            CommitRequest: CommitRequest::<Identity, OFFSET>,
            AbortRequest: AbortRequest::<Identity, OFFSET>,
            TMDown: TMDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionResourceAsync as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionResourceAsync {}
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 2;
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 0;
