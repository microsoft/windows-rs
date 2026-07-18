#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertPolicy, ICertPolicy_Vtbl, 0x38bb5a00_7636_11d0_b413_00a0c91bbf8c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertPolicy {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertPolicy, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ICertPolicy {
    pub unsafe fn Initialize(&self, strconfig: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig)) }
    }
    pub unsafe fn VerifyRequest(&self, strconfig: &windows_core::BSTR, context: i32, bnewrequest: i32, flags: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerifyRequest)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), context, bnewrequest, flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ShutDown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShutDown)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertPolicy_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VerifyRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertPolicy_Impl: super::IDispatch_Impl {
    fn Initialize(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<()>;
    fn VerifyRequest(&self, strconfig: &windows_core::BSTR, context: i32, bnewrequest: i32, flags: i32) -> windows_core::Result<i32>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ShutDown(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertPolicy_Vtbl {
    pub const fn new<Identity: ICertPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: ICertPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertPolicy_Impl::Initialize(this, core::mem::transmute(&strconfig)).into()
            }
        }
        unsafe extern "system" fn VerifyRequest<Identity: ICertPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, context: i32, bnewrequest: i32, flags: i32, pdisposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertPolicy_Impl::VerifyRequest(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&context), core::mem::transmute_copy(&bnewrequest), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pdisposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ICertPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertPolicy_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShutDown<Identity: ICertPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertPolicy_Impl::ShutDown(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            VerifyRequest: VerifyRequest::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            ShutDown: ShutDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPolicy as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertPolicy {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICertPolicy2, ICertPolicy2_Vtbl, 0x3db4910e_8001_4bf1_aa1b_f43a808317a0);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICertPolicy2 {
    type Target = ICertPolicy;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICertPolicy2, windows_core::IUnknown, super::IDispatch, ICertPolicy);
#[cfg(feature = "oaidl")]
impl ICertPolicy2 {
    #[cfg(feature = "certmod")]
    pub unsafe fn GetManageModule(&self) -> windows_core::Result<super::ICertManageModule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetManageModule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertPolicy2_Vtbl {
    pub base__: ICertPolicy_Vtbl,
    #[cfg(feature = "certmod")]
    pub GetManageModule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "certmod"))]
    GetManageModule: usize,
}
#[cfg(all(feature = "certmod", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICertPolicy2_Impl: ICertPolicy_Impl {
    fn GetManageModule(&self) -> windows_core::Result<super::ICertManageModule>;
}
#[cfg(all(feature = "certmod", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICertPolicy2_Vtbl {
    pub const fn new<Identity: ICertPolicy2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetManageModule<Identity: ICertPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanagemodule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertPolicy2_Impl::GetManageModule(this) {
                    Ok(ok__) => {
                        ppmanagemodule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ICertPolicy_Vtbl::new::<Identity, OFFSET>(), GetManageModule: GetManageModule::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPolicy2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ICertPolicy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "certmod", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICertPolicy2 {}
windows_core::imp::define_interface!(INDESPolicy, INDESPolicy_Vtbl, 0x13ca515d_431d_46cc_8c2e_1da269bbd625);
windows_core::imp::interface_hierarchy!(INDESPolicy, windows_core::IUnknown);
impl INDESPolicy {
    pub unsafe fn Initialize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Uninitialize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Uninitialize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GenerateChallenge<P0, P1>(&self, pwsztemplate: P0, pwszparams: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateChallenge)(windows_core::Interface::as_raw(self), pwsztemplate.param().abi(), pwszparams.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "certbase")]
    pub unsafe fn VerifyRequest<P2, P3>(&self, pctbrequest: *const super::CERTTRANSBLOB, pctbsigningcertencoded: *const super::CERTTRANSBLOB, pwsztemplate: P2, pwsztransactionid: P3) -> windows_core::Result<windows_core::BOOL>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerifyRequest)(windows_core::Interface::as_raw(self), pctbrequest, pctbsigningcertencoded, pwsztemplate.param().abi(), pwsztransactionid.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "certbase")]
    pub unsafe fn Notify<P0, P1>(&self, pwszchallenge: P0, pwsztransactionid: P1, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *const super::CERTTRANSBLOB) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), pwszchallenge.param().abi(), pwsztransactionid.param().abi(), disposition, lasthresult, pctbissuedcertencoded) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDESPolicy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GenerateChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "certbase")]
    pub VerifyRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::CERTTRANSBLOB, *const super::CERTTRANSBLOB, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "certbase"))]
    VerifyRequest: usize,
    #[cfg(feature = "certbase")]
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, X509SCEPDisposition, i32, *const super::CERTTRANSBLOB) -> windows_core::HRESULT,
    #[cfg(not(feature = "certbase"))]
    Notify: usize,
}
#[cfg(feature = "certbase")]
pub trait INDESPolicy_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
    fn GenerateChallenge(&self, pwsztemplate: &windows_core::PCWSTR, pwszparams: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn VerifyRequest(&self, pctbrequest: *const super::CERTTRANSBLOB, pctbsigningcertencoded: *const super::CERTTRANSBLOB, pwsztemplate: &windows_core::PCWSTR, pwsztransactionid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn Notify(&self, pwszchallenge: &windows_core::PCWSTR, pwsztransactionid: &windows_core::PCWSTR, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *const super::CERTTRANSBLOB) -> windows_core::Result<()>;
}
#[cfg(feature = "certbase")]
impl INDESPolicy_Vtbl {
    pub const fn new<Identity: INDESPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDESPolicy_Impl::Initialize(this).into()
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDESPolicy_Impl::Uninitialize(this).into()
            }
        }
        unsafe extern "system" fn GenerateChallenge<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztemplate: windows_core::PCWSTR, pwszparams: windows_core::PCWSTR, ppwszresponse: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDESPolicy_Impl::GenerateChallenge(this, core::mem::transmute(&pwsztemplate), core::mem::transmute(&pwszparams)) {
                    Ok(ok__) => {
                        ppwszresponse.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VerifyRequest<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctbrequest: *const super::CERTTRANSBLOB, pctbsigningcertencoded: *const super::CERTTRANSBLOB, pwsztemplate: windows_core::PCWSTR, pwsztransactionid: windows_core::PCWSTR, pfverified: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDESPolicy_Impl::VerifyRequest(this, core::mem::transmute_copy(&pctbrequest), core::mem::transmute_copy(&pctbsigningcertencoded), core::mem::transmute(&pwsztemplate), core::mem::transmute(&pwsztransactionid)) {
                    Ok(ok__) => {
                        pfverified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Notify<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszchallenge: windows_core::PCWSTR, pwsztransactionid: windows_core::PCWSTR, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *const super::CERTTRANSBLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDESPolicy_Impl::Notify(this, core::mem::transmute(&pwszchallenge), core::mem::transmute(&pwsztransactionid), core::mem::transmute_copy(&disposition), core::mem::transmute_copy(&lasthresult), core::mem::transmute_copy(&pctbissuedcertencoded)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
            GenerateChallenge: GenerateChallenge::<Identity, OFFSET>,
            VerifyRequest: VerifyRequest::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDESPolicy as windows_core::Interface>::IID
    }
}
#[cfg(feature = "certbase")]
impl windows_core::RuntimeName for INDESPolicy {}
pub const SCEPDispositionFailure: X509SCEPDisposition = 2;
pub const SCEPDispositionPending: X509SCEPDisposition = 3;
pub const SCEPDispositionPendingChallenge: X509SCEPDisposition = 11;
pub const SCEPDispositionSuccess: X509SCEPDisposition = 0;
pub const SCEPDispositionUnknown: X509SCEPDisposition = -1;
pub const SCEPFailBadAlgorithm: X509SCEPFailInfo = 0;
pub const SCEPFailBadCertId: X509SCEPFailInfo = 4;
pub const SCEPFailBadMessageCheck: X509SCEPFailInfo = 1;
pub const SCEPFailBadRequest: X509SCEPFailInfo = 2;
pub const SCEPFailBadTime: X509SCEPFailInfo = 3;
pub const SCEPFailUnknown: X509SCEPFailInfo = -1;
pub const SCEPMessageCertResponse: X509SCEPMessageType = 3;
pub const SCEPMessageClaimChallengeAnswer: X509SCEPMessageType = 41;
pub const SCEPMessageGetCRL: X509SCEPMessageType = 22;
pub const SCEPMessageGetCert: X509SCEPMessageType = 21;
pub const SCEPMessageGetCertInitial: X509SCEPMessageType = 20;
pub const SCEPMessagePKCSRequest: X509SCEPMessageType = 19;
pub const SCEPMessageUnknown: X509SCEPMessageType = -1;
pub type X509SCEPDisposition = i32;
pub type X509SCEPFailInfo = i32;
pub type X509SCEPMessageType = i32;
