#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BOID {
    pub rgb: [super::rpc::byte; 16],
}
#[cfg(feature = "Win32_rpc")]
impl Default for BOID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IKernelTransaction, IKernelTransaction_Vtbl, 0x79427a2b_f895_40e0_be79_b57dc82ed231);
windows_core::imp::interface_hierarchy!(IKernelTransaction, windows_core::IUnknown);
impl IKernelTransaction {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetHandle(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKernelTransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetHandle: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait IKernelTransaction_Impl: windows_core::IUnknownImpl {
    fn GetHandle(&self) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(feature = "Win32_winnt")]
impl IKernelTransaction_Vtbl {
    pub const fn new<Identity: IKernelTransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHandle<Identity: IKernelTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKernelTransaction_Impl::GetHandle(this) {
                    Ok(ok__) => {
                        phandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHandle: GetHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKernelTransaction as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IKernelTransaction {}
pub type ISOFLAG = i32;
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = 16;
pub const ISOFLAG_READONLY: ISOFLAG = 32;
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = 8;
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = 4;
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = 12;
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = 10;
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = 2;
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = 1;
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = 3;
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = 5;
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = 15;
pub type ISOLATIONLEVEL = i32;
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = 256;
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = 16;
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = 4096;
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = 1048576;
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = 4096;
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = 256;
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = 65536;
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = 1048576;
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = -1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ISOLEVEL(pub i32);
windows_core::imp::define_interface!(ITmNodeName, ITmNodeName_Vtbl, 0x30274f88_6ee4_474e_9b95_7807bc9ef8cf);
windows_core::imp::interface_hierarchy!(ITmNodeName, windows_core::IUnknown);
impl ITmNodeName {
    pub unsafe fn GetNodeNameSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNodeNameSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNodeName(&self, cbnodenamebuffersize: u32, pnodenamebuffer: windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNodeName)(windows_core::Interface::as_raw(self), cbnodenamebuffersize, core::mem::transmute(pnodenamebuffer)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITmNodeName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNodeNameSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNodeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ITmNodeName_Impl: windows_core::IUnknownImpl {
    fn GetNodeNameSize(&self) -> windows_core::Result<u32>;
    fn GetNodeName(&self, cbnodenamebuffersize: u32, pnodenamebuffer: windows_core::PWSTR) -> windows_core::Result<()>;
}
impl ITmNodeName_Vtbl {
    pub const fn new<Identity: ITmNodeName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNodeNameSize<Identity: ITmNodeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbnodenamesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITmNodeName_Impl::GetNodeNameSize(this) {
                    Ok(ok__) => {
                        pcbnodenamesize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNodeName<Identity: ITmNodeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITmNodeName_Impl::GetNodeName(this, core::mem::transmute_copy(&cbnodenamebuffersize), core::mem::transmute_copy(&pnodenamebuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNodeNameSize: GetNodeNameSize::<Identity, OFFSET>,
            GetNodeName: GetNodeName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITmNodeName as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITmNodeName {}
windows_core::imp::define_interface!(ITransaction, ITransaction_Vtbl, 0x0fb15084_af41_11ce_bd2b_204c4f4f5020);
windows_core::imp::interface_hierarchy!(ITransaction, windows_core::IUnknown);
impl ITransaction {
    pub unsafe fn Commit(&self, fretaining: bool, grftc: u32, grfrm: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), fretaining.into(), grftc, grfrm) }
    }
    #[cfg(feature = "Win32_rpc")]
    pub unsafe fn Abort(&self, pboidreason: *const BOID, fretaining: bool, fasync: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), pboidreason, fretaining.into(), fasync.into()) }
    }
    #[cfg(feature = "Win32_rpc")]
    pub unsafe fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionInfo)(windows_core::Interface::as_raw(self), pinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_rpc")]
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpc"))]
    Abort: usize,
    #[cfg(feature = "Win32_rpc")]
    pub GetTransactionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTTRANSINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpc"))]
    GetTransactionInfo: usize,
}
#[cfg(feature = "Win32_rpc")]
pub trait ITransaction_Impl: windows_core::IUnknownImpl {
    fn Commit(&self, fretaining: windows_core::BOOL, grftc: u32, grfrm: u32) -> windows_core::Result<()>;
    fn Abort(&self, pboidreason: *const BOID, fretaining: windows_core::BOOL, fasync: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_rpc")]
impl ITransaction_Vtbl {
    pub const fn new<Identity: ITransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: ITransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: windows_core::BOOL, grftc: u32, grfrm: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransaction_Impl::Commit(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grftc), core::mem::transmute_copy(&grfrm)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: ITransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: windows_core::BOOL, fasync: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransaction_Impl::Abort(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&fasync)).into()
            }
        }
        unsafe extern "system" fn GetTransactionInfo<Identity: ITransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransaction_Impl::GetTransactionInfo(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            GetTransactionInfo: GetTransactionInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransaction as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_rpc")]
impl windows_core::RuntimeName for ITransaction {}
windows_core::imp::define_interface!(ITransaction2, ITransaction2_Vtbl, 0x34021548_0065_11d3_bac1_00c04f797be2);
impl core::ops::Deref for ITransaction2 {
    type Target = ITransactionCloner;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransaction2, windows_core::IUnknown, ITransaction, ITransactionCloner);
impl ITransaction2 {
    #[cfg(feature = "Win32_rpc")]
    pub unsafe fn GetTransactionInfo2(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionInfo2)(windows_core::Interface::as_raw(self), pinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction2_Vtbl {
    pub base__: ITransactionCloner_Vtbl,
    #[cfg(feature = "Win32_rpc")]
    pub GetTransactionInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTTRANSINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpc"))]
    GetTransactionInfo2: usize,
}
#[cfg(feature = "Win32_rpc")]
pub trait ITransaction2_Impl: ITransactionCloner_Impl {
    fn GetTransactionInfo2(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_rpc")]
impl ITransaction2_Vtbl {
    pub const fn new<Identity: ITransaction2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransactionInfo2<Identity: ITransaction2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransaction2_Impl::GetTransactionInfo2(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        Self { base__: ITransactionCloner_Vtbl::new::<Identity, OFFSET>(), GetTransactionInfo2: GetTransactionInfo2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransaction2 as windows_core::Interface>::IID || iid == &<ITransaction as windows_core::Interface>::IID || iid == &<ITransactionCloner as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_rpc")]
impl windows_core::RuntimeName for ITransaction2 {}
windows_core::imp::define_interface!(ITransactionCloner, ITransactionCloner_Vtbl, 0x02656950_2152_11d0_944c_00a0c905416e);
impl core::ops::Deref for ITransactionCloner {
    type Target = ITransaction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransactionCloner, windows_core::IUnknown, ITransaction);
impl ITransactionCloner {
    pub unsafe fn CloneWithCommitDisabled(&self) -> windows_core::Result<ITransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloneWithCommitDisabled)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionCloner_Vtbl {
    pub base__: ITransaction_Vtbl,
    pub CloneWithCommitDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_rpc")]
pub trait ITransactionCloner_Impl: ITransaction_Impl {
    fn CloneWithCommitDisabled(&self) -> windows_core::Result<ITransaction>;
}
#[cfg(feature = "Win32_rpc")]
impl ITransactionCloner_Vtbl {
    pub const fn new<Identity: ITransactionCloner_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CloneWithCommitDisabled<Identity: ITransactionCloner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionCloner_Impl::CloneWithCommitDisabled(this) {
                    Ok(ok__) => {
                        ppitransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ITransaction_Vtbl::new::<Identity, OFFSET>(), CloneWithCommitDisabled: CloneWithCommitDisabled::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionCloner as windows_core::Interface>::IID || iid == &<ITransaction as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_rpc")]
impl windows_core::RuntimeName for ITransactionCloner {}
windows_core::imp::define_interface!(ITransactionDispenser, ITransactionDispenser_Vtbl, 0x3a6ad9e1_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionDispenser, windows_core::IUnknown);
impl ITransactionDispenser {
    pub unsafe fn GetOptionsObject(&self) -> windows_core::Result<ITransactionOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionsObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BeginTransaction<P0, P3>(&self, punkouter: P0, isolevel: ISOLEVEL, isoflags: u32, poptions: P3) -> windows_core::Result<ITransaction>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<ITransactionOptions>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), punkouter.param().abi(), isolevel, isoflags, poptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionDispenser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOptionsObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, ISOLEVEL, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionDispenser_Impl: windows_core::IUnknownImpl {
    fn GetOptionsObject(&self) -> windows_core::Result<ITransactionOptions>;
    fn BeginTransaction(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, isolevel: ISOLEVEL, isoflags: u32, poptions: windows_core::Ref<ITransactionOptions>) -> windows_core::Result<ITransaction>;
}
impl ITransactionDispenser_Vtbl {
    pub const fn new<Identity: ITransactionDispenser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionsObject<Identity: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionDispenser_Impl::GetOptionsObject(this) {
                    Ok(ok__) => {
                        ppoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginTransaction<Identity: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, isolevel: ISOLEVEL, isoflags: u32, poptions: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionDispenser_Impl::BeginTransaction(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), core::mem::transmute_copy(&poptions)) {
                    Ok(ok__) => {
                        pptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, OFFSET>,
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionDispenser as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionDispenser {}
windows_core::imp::define_interface!(ITransactionOptions, ITransactionOptions_Vtbl, 0x3a6ad9e0_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionOptions, windows_core::IUnknown);
impl ITransactionOptions {
    pub unsafe fn SetOptions(&self, poptions: *const XACTOPT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), poptions) }
    }
    pub unsafe fn GetOptions(&self, poptions: *mut XACTOPT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), poptions as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const XACTOPT) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTOPT) -> windows_core::HRESULT,
}
pub trait ITransactionOptions_Impl: windows_core::IUnknownImpl {
    fn SetOptions(&self, poptions: *const XACTOPT) -> windows_core::Result<()>;
    fn GetOptions(&self, poptions: *mut XACTOPT) -> windows_core::Result<()>;
}
impl ITransactionOptions_Vtbl {
    pub const fn new<Identity: ITransactionOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOptions<Identity: ITransactionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *const XACTOPT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOptions_Impl::SetOptions(this, core::mem::transmute_copy(&poptions)).into()
            }
        }
        unsafe extern "system" fn GetOptions<Identity: ITransactionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut XACTOPT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOptions_Impl::GetOptions(this, core::mem::transmute_copy(&poptions)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOptions: SetOptions::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionOptions {}
windows_core::imp::define_interface!(ITransactionOutcomeEvents, ITransactionOutcomeEvents_Vtbl, 0x3a6ad9e2_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionOutcomeEvents, windows_core::IUnknown);
impl ITransactionOutcomeEvents {
    #[cfg(feature = "Win32_rpc")]
    pub unsafe fn Committed(&self, fretaining: bool, pnewuow: *const XACTUOW, hr: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self), fretaining.into(), pnewuow, hr) }
    }
    #[cfg(feature = "Win32_rpc")]
    pub unsafe fn Aborted(&self, pboidreason: *const BOID, fretaining: bool, pnewuow: *const XACTUOW, hr: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Aborted)(windows_core::Interface::as_raw(self), pboidreason, fretaining.into(), pnewuow, hr) }
    }
    #[cfg(feature = "Win32_rpc")]
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HeuristicDecision)(windows_core::Interface::as_raw(self), dwdecision, pboidreason, hr) }
    }
    pub unsafe fn Indoubt(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Indoubt)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOutcomeEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_rpc")]
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const XACTUOW, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpc"))]
    Committed: usize,
    #[cfg(feature = "Win32_rpc")]
    pub Aborted: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, windows_core::BOOL, *const XACTUOW, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpc"))]
    Aborted: usize,
    #[cfg(feature = "Win32_rpc")]
    pub HeuristicDecision: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const BOID, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpc"))]
    HeuristicDecision: usize,
    pub Indoubt: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_rpc")]
pub trait ITransactionOutcomeEvents_Impl: windows_core::IUnknownImpl {
    fn Committed(&self, fretaining: windows_core::BOOL, pnewuow: *const XACTUOW, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Aborted(&self, pboidreason: *const BOID, fretaining: windows_core::BOOL, pnewuow: *const XACTUOW, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Indoubt(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_rpc")]
impl ITransactionOutcomeEvents_Vtbl {
    pub const fn new<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Committed<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: windows_core::BOOL, pnewuow: *const XACTUOW, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOutcomeEvents_Impl::Committed(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn Aborted<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: windows_core::BOOL, pnewuow: *const XACTUOW, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOutcomeEvents_Impl::Aborted(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn HeuristicDecision<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOutcomeEvents_Impl::HeuristicDecision(this, core::mem::transmute_copy(&dwdecision), core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn Indoubt<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionOutcomeEvents_Impl::Indoubt(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Committed: Committed::<Identity, OFFSET>,
            Aborted: Aborted::<Identity, OFFSET>,
            HeuristicDecision: HeuristicDecision::<Identity, OFFSET>,
            Indoubt: Indoubt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionOutcomeEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_rpc")]
impl windows_core::RuntimeName for ITransactionOutcomeEvents {}
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = 40;
pub type TX_MISC_CONSTANTS = i32;
pub type XACTCONST = i32;
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = 0;
pub type XACTHEURISTIC = i32;
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = 1;
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = 2;
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = 3;
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [i8; 40],
}
impl Default for XACTOPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type XACTRM = i32;
pub const XACTRM_NOREADONLYPREPARES: XACTRM = 2;
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = 1;
pub type XACTSTAT = i32;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XACTSTATS {
    pub cOpen: u32,
    pub cCommitting: u32,
    pub cCommitted: u32,
    pub cAborting: u32,
    pub cAborted: u32,
    pub cInDoubt: u32,
    pub cHeuristicDecision: u32,
    pub timeTransactionsUp: super::minwindef::FILETIME,
}
pub const XACTSTAT_ABORTED: XACTSTAT = 512;
pub const XACTSTAT_ABORTING: XACTSTAT = 256;
pub const XACTSTAT_ALL: XACTSTAT = 524287;
pub const XACTSTAT_CLOSED: XACTSTAT = 262144;
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = 128;
pub const XACTSTAT_COMMITTED: XACTSTAT = 1024;
pub const XACTSTAT_COMMITTING: XACTSTAT = 64;
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = 32768;
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = 65536;
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = 2048;
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = 4096;
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = 8192;
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = 16384;
pub const XACTSTAT_INDOUBT: XACTSTAT = 131072;
pub const XACTSTAT_NONE: XACTSTAT = 0;
pub const XACTSTAT_NOTPREPARED: XACTSTAT = 524227;
pub const XACTSTAT_OPEN: XACTSTAT = 3;
pub const XACTSTAT_OPENNORMAL: XACTSTAT = 1;
pub const XACTSTAT_OPENREFUSED: XACTSTAT = 2;
pub const XACTSTAT_PREPARED: XACTSTAT = 8;
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = 32;
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = 16;
pub const XACTSTAT_PREPARING: XACTSTAT = 4;
pub type XACTTC = i32;
pub const XACTTC_ASYNC: XACTTC = 4;
pub const XACTTC_ASYNC_PHASEONE: XACTTC = 4;
pub const XACTTC_NONE: XACTTC = 0;
pub const XACTTC_SYNC: XACTTC = 2;
pub const XACTTC_SYNC_PHASEONE: XACTTC = 1;
pub const XACTTC_SYNC_PHASETWO: XACTTC = 2;
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XACTTRANSINFO {
    pub uow: XACTUOW,
    pub isoLevel: ISOLEVEL,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
#[cfg(feature = "Win32_rpc")]
pub type XACTUOW = BOID;
