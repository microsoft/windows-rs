windows_core::imp::define_interface!(IFindReferenceTargetsCallback, IFindReferenceTargetsCallback_Vtbl, 0x04b3486c_4687_4229_8d14_505ab584dd88);
windows_core::imp::interface_hierarchy!(IFindReferenceTargetsCallback, windows_core::IUnknown);
impl IFindReferenceTargetsCallback {
    pub unsafe fn FoundTrackerTarget<P0>(&self, target: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IReferenceTrackerTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).FoundTrackerTarget)(windows_core::Interface::as_raw(self), target.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FoundTrackerTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFindReferenceTargetsCallback_Impl: windows_core::IUnknownImpl {
    fn FoundTrackerTarget(&self, target: windows_core::Ref<IReferenceTrackerTarget>) -> windows_core::Result<()>;
}
impl IFindReferenceTargetsCallback_Vtbl {
    pub const fn new<Identity: IFindReferenceTargetsCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FoundTrackerTarget<Identity: IFindReferenceTargetsCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFindReferenceTargetsCallback_Impl::FoundTrackerTarget(this, core::mem::transmute_copy(&target)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FoundTrackerTarget: FoundTrackerTarget::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFindReferenceTargetsCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFindReferenceTargetsCallback {}
windows_core::imp::define_interface!(IReferenceTracker, IReferenceTracker_Vtbl, 0x11d3b13a_180e_4789_a8be_7712882893e6);
windows_core::imp::interface_hierarchy!(IReferenceTracker, windows_core::IUnknown);
impl IReferenceTracker {
    pub unsafe fn ConnectFromTrackerSource(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConnectFromTrackerSource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DisconnectFromTrackerSource(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisconnectFromTrackerSource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FindTrackerTargets<P0>(&self, callback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFindReferenceTargetsCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindTrackerTargets)(windows_core::Interface::as_raw(self), callback.param().abi()) }
    }
    pub unsafe fn GetReferenceTrackerManager(&self) -> windows_core::Result<IReferenceTrackerManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReferenceTrackerManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddRefFromTrackerSource(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRefFromTrackerSource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseFromTrackerSource(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFromTrackerSource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PegFromTrackerSource(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PegFromTrackerSource)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectFromTrackerSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectFromTrackerSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindTrackerTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReferenceTrackerManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRefFromTrackerSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseFromTrackerSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PegFromTrackerSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IReferenceTracker_Impl: windows_core::IUnknownImpl {
    fn ConnectFromTrackerSource(&self) -> windows_core::Result<()>;
    fn DisconnectFromTrackerSource(&self) -> windows_core::Result<()>;
    fn FindTrackerTargets(&self, callback: windows_core::Ref<IFindReferenceTargetsCallback>) -> windows_core::Result<()>;
    fn GetReferenceTrackerManager(&self) -> windows_core::Result<IReferenceTrackerManager>;
    fn AddRefFromTrackerSource(&self) -> windows_core::Result<()>;
    fn ReleaseFromTrackerSource(&self) -> windows_core::Result<()>;
    fn PegFromTrackerSource(&self) -> windows_core::Result<()>;
}
impl IReferenceTracker_Vtbl {
    pub const fn new<Identity: IReferenceTracker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectFromTrackerSource<Identity: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTracker_Impl::ConnectFromTrackerSource(this).into()
            }
        }
        unsafe extern "system" fn DisconnectFromTrackerSource<Identity: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTracker_Impl::DisconnectFromTrackerSource(this).into()
            }
        }
        unsafe extern "system" fn FindTrackerTargets<Identity: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTracker_Impl::FindTrackerTargets(this, core::mem::transmute_copy(&callback)).into()
            }
        }
        unsafe extern "system" fn GetReferenceTrackerManager<Identity: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceTracker_Impl::GetReferenceTrackerManager(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddRefFromTrackerSource<Identity: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTracker_Impl::AddRefFromTrackerSource(this).into()
            }
        }
        unsafe extern "system" fn ReleaseFromTrackerSource<Identity: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTracker_Impl::ReleaseFromTrackerSource(this).into()
            }
        }
        unsafe extern "system" fn PegFromTrackerSource<Identity: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTracker_Impl::PegFromTrackerSource(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectFromTrackerSource: ConnectFromTrackerSource::<Identity, OFFSET>,
            DisconnectFromTrackerSource: DisconnectFromTrackerSource::<Identity, OFFSET>,
            FindTrackerTargets: FindTrackerTargets::<Identity, OFFSET>,
            GetReferenceTrackerManager: GetReferenceTrackerManager::<Identity, OFFSET>,
            AddRefFromTrackerSource: AddRefFromTrackerSource::<Identity, OFFSET>,
            ReleaseFromTrackerSource: ReleaseFromTrackerSource::<Identity, OFFSET>,
            PegFromTrackerSource: PegFromTrackerSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTracker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IReferenceTracker {}
windows_core::imp::define_interface!(IReferenceTrackerExtension, IReferenceTrackerExtension_Vtbl, 0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0);
windows_core::imp::interface_hierarchy!(IReferenceTrackerExtension, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IReferenceTrackerExtension_Impl: windows_core::IUnknownImpl {}
impl IReferenceTrackerExtension_Vtbl {
    pub const fn new<Identity: IReferenceTrackerExtension_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTrackerExtension as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IReferenceTrackerExtension {}
windows_core::imp::define_interface!(IReferenceTrackerHost, IReferenceTrackerHost_Vtbl, 0x29a71c6a_3c42_4416_a39d_e2825a07a773);
windows_core::imp::interface_hierarchy!(IReferenceTrackerHost, windows_core::IUnknown);
impl IReferenceTrackerHost {
    pub unsafe fn DisconnectUnusedReferenceSources(&self, options: XAML_REFERENCETRACKER_DISCONNECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisconnectUnusedReferenceSources)(windows_core::Interface::as_raw(self), options) }
    }
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDisconnectedReferenceSources)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NotifyEndOfReferenceTrackingOnThread)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTrackerTarget<P0>(&self, unknown: P0) -> windows_core::Result<IReferenceTrackerTarget>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTrackerTarget)(windows_core::Interface::as_raw(self), unknown.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddMemoryPressure)(windows_core::Interface::as_raw(self), bytesallocated) }
    }
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveMemoryPressure)(windows_core::Interface::as_raw(self), bytesallocated) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DisconnectUnusedReferenceSources: unsafe extern "system" fn(*mut core::ffi::c_void, XAML_REFERENCETRACKER_DISCONNECT) -> windows_core::HRESULT,
    pub ReleaseDisconnectedReferenceSources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyEndOfReferenceTrackingOnThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTrackerTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddMemoryPressure: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub RemoveMemoryPressure: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
pub trait IReferenceTrackerHost_Impl: windows_core::IUnknownImpl {
    fn DisconnectUnusedReferenceSources(&self, options: XAML_REFERENCETRACKER_DISCONNECT) -> windows_core::Result<()>;
    fn ReleaseDisconnectedReferenceSources(&self) -> windows_core::Result<()>;
    fn NotifyEndOfReferenceTrackingOnThread(&self) -> windows_core::Result<()>;
    fn GetTrackerTarget(&self, unknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IReferenceTrackerTarget>;
    fn AddMemoryPressure(&self, bytesallocated: u64) -> windows_core::Result<()>;
    fn RemoveMemoryPressure(&self, bytesallocated: u64) -> windows_core::Result<()>;
}
impl IReferenceTrackerHost_Vtbl {
    pub const fn new<Identity: IReferenceTrackerHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisconnectUnusedReferenceSources<Identity: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: XAML_REFERENCETRACKER_DISCONNECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerHost_Impl::DisconnectUnusedReferenceSources(this, core::mem::transmute_copy(&options)).into()
            }
        }
        unsafe extern "system" fn ReleaseDisconnectedReferenceSources<Identity: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerHost_Impl::ReleaseDisconnectedReferenceSources(this).into()
            }
        }
        unsafe extern "system" fn NotifyEndOfReferenceTrackingOnThread<Identity: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerHost_Impl::NotifyEndOfReferenceTrackingOnThread(this).into()
            }
        }
        unsafe extern "system" fn GetTrackerTarget<Identity: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unknown: *mut core::ffi::c_void, newreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceTrackerHost_Impl::GetTrackerTarget(this, core::mem::transmute_copy(&unknown)) {
                    Ok(ok__) => {
                        newreference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddMemoryPressure<Identity: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bytesallocated: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerHost_Impl::AddMemoryPressure(this, core::mem::transmute_copy(&bytesallocated)).into()
            }
        }
        unsafe extern "system" fn RemoveMemoryPressure<Identity: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bytesallocated: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerHost_Impl::RemoveMemoryPressure(this, core::mem::transmute_copy(&bytesallocated)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DisconnectUnusedReferenceSources: DisconnectUnusedReferenceSources::<Identity, OFFSET>,
            ReleaseDisconnectedReferenceSources: ReleaseDisconnectedReferenceSources::<Identity, OFFSET>,
            NotifyEndOfReferenceTrackingOnThread: NotifyEndOfReferenceTrackingOnThread::<Identity, OFFSET>,
            GetTrackerTarget: GetTrackerTarget::<Identity, OFFSET>,
            AddMemoryPressure: AddMemoryPressure::<Identity, OFFSET>,
            RemoveMemoryPressure: RemoveMemoryPressure::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTrackerHost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IReferenceTrackerHost {}
windows_core::imp::define_interface!(IReferenceTrackerManager, IReferenceTrackerManager_Vtbl, 0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298);
windows_core::imp::interface_hierarchy!(IReferenceTrackerManager, windows_core::IUnknown);
impl IReferenceTrackerManager {
    pub unsafe fn ReferenceTrackingStarted(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReferenceTrackingStarted)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "rpc")]
    pub unsafe fn FindTrackerTargetsCompleted(&self, findfailed: super::rpc::boolean) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindTrackerTargetsCompleted)(windows_core::Interface::as_raw(self), findfailed) }
    }
    pub unsafe fn ReferenceTrackingCompleted(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReferenceTrackingCompleted)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetReferenceTrackerHost<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IReferenceTrackerHost>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReferenceTrackerHost)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReferenceTrackingStarted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "rpc")]
    pub FindTrackerTargetsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::rpc::boolean) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    FindTrackerTargetsCompleted: usize,
    pub ReferenceTrackingCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReferenceTrackerHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "rpc")]
pub trait IReferenceTrackerManager_Impl: windows_core::IUnknownImpl {
    fn ReferenceTrackingStarted(&self) -> windows_core::Result<()>;
    fn FindTrackerTargetsCompleted(&self, findfailed: super::rpc::boolean) -> windows_core::Result<()>;
    fn ReferenceTrackingCompleted(&self) -> windows_core::Result<()>;
    fn SetReferenceTrackerHost(&self, value: windows_core::Ref<IReferenceTrackerHost>) -> windows_core::Result<()>;
}
#[cfg(feature = "rpc")]
impl IReferenceTrackerManager_Vtbl {
    pub const fn new<Identity: IReferenceTrackerManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReferenceTrackingStarted<Identity: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerManager_Impl::ReferenceTrackingStarted(this).into()
            }
        }
        unsafe extern "system" fn FindTrackerTargetsCompleted<Identity: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, findfailed: super::rpc::boolean) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerManager_Impl::FindTrackerTargetsCompleted(this, core::mem::transmute_copy(&findfailed)).into()
            }
        }
        unsafe extern "system" fn ReferenceTrackingCompleted<Identity: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerManager_Impl::ReferenceTrackingCompleted(this).into()
            }
        }
        unsafe extern "system" fn SetReferenceTrackerHost<Identity: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerManager_Impl::SetReferenceTrackerHost(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReferenceTrackingStarted: ReferenceTrackingStarted::<Identity, OFFSET>,
            FindTrackerTargetsCompleted: FindTrackerTargetsCompleted::<Identity, OFFSET>,
            ReferenceTrackingCompleted: ReferenceTrackingCompleted::<Identity, OFFSET>,
            SetReferenceTrackerHost: SetReferenceTrackerHost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTrackerManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "rpc")]
impl windows_core::RuntimeName for IReferenceTrackerManager {}
windows_core::imp::define_interface!(IReferenceTrackerTarget, IReferenceTrackerTarget_Vtbl, 0x64bd43f8_bfee_4ec4_b7eb_2935158dae21);
windows_core::imp::interface_hierarchy!(IReferenceTrackerTarget, windows_core::IUnknown);
impl IReferenceTrackerTarget {
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRefFromReferenceTracker)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFromReferenceTracker)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Peg(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Peg)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Unpeg(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unpeg)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefFromReferenceTracker: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub ReleaseFromReferenceTracker: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Peg: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unpeg: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IReferenceTrackerTarget_Impl: windows_core::IUnknownImpl {
    fn AddRefFromReferenceTracker(&self) -> u32;
    fn ReleaseFromReferenceTracker(&self) -> u32;
    fn Peg(&self) -> windows_core::Result<()>;
    fn Unpeg(&self) -> windows_core::Result<()>;
}
impl IReferenceTrackerTarget_Vtbl {
    pub const fn new<Identity: IReferenceTrackerTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRefFromReferenceTracker<Identity: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerTarget_Impl::AddRefFromReferenceTracker(this)
            }
        }
        unsafe extern "system" fn ReleaseFromReferenceTracker<Identity: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerTarget_Impl::ReleaseFromReferenceTracker(this)
            }
        }
        unsafe extern "system" fn Peg<Identity: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerTarget_Impl::Peg(this).into()
            }
        }
        unsafe extern "system" fn Unpeg<Identity: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceTrackerTarget_Impl::Unpeg(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefFromReferenceTracker: AddRefFromReferenceTracker::<Identity, OFFSET>,
            ReleaseFromReferenceTracker: ReleaseFromReferenceTracker::<Identity, OFFSET>,
            Peg: Peg::<Identity, OFFSET>,
            Unpeg: Unpeg::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceTrackerTarget as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IReferenceTrackerTarget {}
windows_core::imp::define_interface!(ITrackerOwner, ITrackerOwner_Vtbl, 0xeb24c20b_9816_4ac7_8cff_36f67a118f4e);
windows_core::imp::interface_hierarchy!(ITrackerOwner, windows_core::IUnknown);
impl ITrackerOwner {
    pub unsafe fn CreateTrackerHandle(&self) -> windows_core::Result<TrackerHandle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTrackerHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeleteTrackerHandle(&self, handle: TrackerHandle) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteTrackerHandle)(windows_core::Interface::as_raw(self), handle) }
    }
    pub unsafe fn SetTrackerValue<P1>(&self, handle: TrackerHandle, value: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTrackerValue)(windows_core::Interface::as_raw(self), handle, value.param().abi()) }
    }
    #[cfg(feature = "rpc")]
    pub unsafe fn TryGetSafeTrackerValue(&self, handle: TrackerHandle, returnvalue: *mut Option<windows_core::IUnknown>) -> super::rpc::boolean {
        unsafe { (windows_core::Interface::vtable(self).TryGetSafeTrackerValue)(windows_core::Interface::as_raw(self), handle, core::mem::transmute(returnvalue)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTrackerHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TrackerHandle) -> windows_core::HRESULT,
    pub DeleteTrackerHandle: unsafe extern "system" fn(*mut core::ffi::c_void, TrackerHandle) -> windows_core::HRESULT,
    pub SetTrackerValue: unsafe extern "system" fn(*mut core::ffi::c_void, TrackerHandle, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "rpc")]
    pub TryGetSafeTrackerValue: unsafe extern "system" fn(*mut core::ffi::c_void, TrackerHandle, *mut *mut core::ffi::c_void) -> super::rpc::boolean,
    #[cfg(not(feature = "rpc"))]
    TryGetSafeTrackerValue: usize,
}
#[cfg(feature = "rpc")]
pub trait ITrackerOwner_Impl: windows_core::IUnknownImpl {
    fn CreateTrackerHandle(&self) -> windows_core::Result<TrackerHandle>;
    fn DeleteTrackerHandle(&self, handle: TrackerHandle) -> windows_core::Result<()>;
    fn SetTrackerValue(&self, handle: TrackerHandle, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn TryGetSafeTrackerValue(&self, handle: TrackerHandle, returnvalue: windows_core::OutRef<windows_core::IUnknown>) -> super::rpc::boolean;
}
#[cfg(feature = "rpc")]
impl ITrackerOwner_Vtbl {
    pub const fn new<Identity: ITrackerOwner_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTrackerHandle<Identity: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, returnvalue: *mut TrackerHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrackerOwner_Impl::CreateTrackerHandle(this) {
                    Ok(ok__) => {
                        returnvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteTrackerHandle<Identity: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: TrackerHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrackerOwner_Impl::DeleteTrackerHandle(this, core::mem::transmute_copy(&handle)).into()
            }
        }
        unsafe extern "system" fn SetTrackerValue<Identity: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: TrackerHandle, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrackerOwner_Impl::SetTrackerValue(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn TryGetSafeTrackerValue<Identity: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: TrackerHandle, returnvalue: *mut *mut core::ffi::c_void) -> super::rpc::boolean {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrackerOwner_Impl::TryGetSafeTrackerValue(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&returnvalue))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTrackerHandle: CreateTrackerHandle::<Identity, OFFSET>,
            DeleteTrackerHandle: DeleteTrackerHandle::<Identity, OFFSET>,
            SetTrackerValue: SetTrackerValue::<Identity, OFFSET>,
            TryGetSafeTrackerValue: TryGetSafeTrackerValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrackerOwner as windows_core::Interface>::IID
    }
}
#[cfg(feature = "rpc")]
impl windows_core::RuntimeName for ITrackerOwner {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrackerHandle(pub *mut core::ffi::c_void);
impl Default for TrackerHandle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type XAML_REFERENCETRACKER_DISCONNECT = i32;
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: XAML_REFERENCETRACKER_DISCONNECT = 0;
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: XAML_REFERENCETRACKER_DISCONNECT = 1;
