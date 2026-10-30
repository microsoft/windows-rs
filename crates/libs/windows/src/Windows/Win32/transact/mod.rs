#[repr(C)]
#[cfg(feature = "rpc")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BOID {
    pub rgb: [super::byte; 16],
}
#[cfg(feature = "rpc")]
impl Default for BOID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IID_IKernelTransaction: windows_core::GUID = windows_core::GUID::from_u128(0x79427a2b_f895_40e0_be79_b57dc82ed231);
pub const IID_ITmNodeName: windows_core::GUID = windows_core::GUID::from_u128(0x30274f88_6ee4_474e_9b95_7807bc9ef8cf);
pub const IID_ITransaction: windows_core::GUID = windows_core::GUID::from_u128(0x0fb15084_af41_11ce_bd2b_204c4f4f5020);
pub const IID_ITransaction2: windows_core::GUID = windows_core::GUID::from_u128(0x34021548_0065_11d3_bac1_00c04f797be2);
pub const IID_ITransactionCloner: windows_core::GUID = windows_core::GUID::from_u128(0x02656950_2152_11d0_944c_00a0c905416e);
pub const IID_ITransactionDispenser: windows_core::GUID = windows_core::GUID::from_u128(0x3a6ad9e1_23b9_11cf_ad60_00aa00a74ccd);
pub const IID_ITransactionOptions: windows_core::GUID = windows_core::GUID::from_u128(0x3a6ad9e0_23b9_11cf_ad60_00aa00a74ccd);
pub const IID_ITransactionOutcomeEvents: windows_core::GUID = windows_core::GUID::from_u128(0x3a6ad9e2_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::define_interface!(IKernelTransaction, IKernelTransaction_Vtbl, 0x79427a2b_f895_40e0_be79_b57dc82ed231);
windows_core::imp::interface_hierarchy!(IKernelTransaction, windows_core::IUnknown);
impl IKernelTransaction {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetHandle(&self, phandle: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHandle)(windows_core::Interface::as_raw(self), phandle as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKernelTransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetHandle: usize,
}
#[cfg(feature = "winnt")]
pub trait IKernelTransaction_Impl: windows_core::IUnknownImpl {
    fn GetHandle(&self, phandle: *mut super::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IKernelTransaction_Vtbl {
    pub const fn new<Identity: IKernelTransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHandle<Identity: IKernelTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKernelTransaction_Impl::GetHandle(this, core::mem::transmute_copy(&phandle)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHandle: GetHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKernelTransaction as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
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
pub type ISOLEVEL = i32;
windows_core::imp::define_interface!(ITmNodeName, ITmNodeName_Vtbl, 0x30274f88_6ee4_474e_9b95_7807bc9ef8cf);
windows_core::imp::interface_hierarchy!(ITmNodeName, windows_core::IUnknown);
impl ITmNodeName {
    #[cfg(feature = "shared")]
    pub unsafe fn GetNodeNameSize(&self, pcbnodenamesize: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNodeNameSize)(windows_core::Interface::as_raw(self), pcbnodenamesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetNodeName(&self, cbnodenamebuffersize: super::ULONG, pnodenamebuffer: windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNodeName)(windows_core::Interface::as_raw(self), cbnodenamebuffersize, pnodenamebuffer) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITmNodeName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetNodeNameSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetNodeNameSize: usize,
    #[cfg(feature = "shared")]
    pub GetNodeName: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetNodeName: usize,
}
#[cfg(feature = "shared")]
pub trait ITmNodeName_Impl: windows_core::IUnknownImpl {
    fn GetNodeNameSize(&self, pcbnodenamesize: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetNodeName(&self, cbnodenamebuffersize: super::ULONG, pnodenamebuffer: windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITmNodeName_Vtbl {
    pub const fn new<Identity: ITmNodeName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNodeNameSize<Identity: ITmNodeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbnodenamesize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITmNodeName_Impl::GetNodeNameSize(this, core::mem::transmute_copy(&pcbnodenamesize)).into()
            }
        }
        unsafe extern "system" fn GetNodeName<Identity: ITmNodeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbnodenamebuffersize: super::ULONG, pnodenamebuffer: windows_core::PWSTR) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITmNodeName {}
windows_core::imp::define_interface!(ITransaction, ITransaction_Vtbl, 0x0fb15084_af41_11ce_bd2b_204c4f4f5020);
windows_core::imp::interface_hierarchy!(ITransaction, windows_core::IUnknown);
impl ITransaction {
    #[cfg(feature = "shared")]
    pub unsafe fn Commit(&self, fretaining: bool, grftc: super::DWORD, grfrm: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), fretaining.into(), grftc, grfrm) }
    }
    #[cfg(feature = "rpc")]
    pub unsafe fn Abort(&self, pboidreason: Option<*const BOID>, fretaining: bool, fasync: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), pboidreason.unwrap_or(core::mem::zeroed()) as _, fretaining.into(), fasync.into()) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionInfo)(windows_core::Interface::as_raw(self), pinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Commit: usize,
    #[cfg(feature = "rpc")]
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    Abort: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub GetTransactionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTTRANSINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    GetTransactionInfo: usize,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait ITransaction_Impl: windows_core::IUnknownImpl {
    fn Commit(&self, fretaining: windows_core::BOOL, grftc: super::DWORD, grfrm: super::DWORD) -> windows_core::Result<()>;
    fn Abort(&self, pboidreason: *const BOID, fretaining: windows_core::BOOL, fasync: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl ITransaction_Vtbl {
    pub const fn new<Identity: ITransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: ITransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: windows_core::BOOL, grftc: super::DWORD, grfrm: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "rpc", feature = "shared"))]
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
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn GetTransactionInfo2(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionInfo2)(windows_core::Interface::as_raw(self), pinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransaction2_Vtbl {
    pub base__: ITransactionCloner_Vtbl,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub GetTransactionInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTTRANSINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    GetTransactionInfo2: usize,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait ITransaction2_Impl: ITransactionCloner_Impl {
    fn GetTransactionInfo2(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
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
#[cfg(all(feature = "rpc", feature = "shared"))]
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
    pub unsafe fn CloneWithCommitDisabled(&self, ppitransaction: *mut Option<ITransaction>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CloneWithCommitDisabled)(windows_core::Interface::as_raw(self), core::mem::transmute(ppitransaction)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionCloner_Vtbl {
    pub base__: ITransaction_Vtbl,
    pub CloneWithCommitDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait ITransactionCloner_Impl: ITransaction_Impl {
    fn CloneWithCommitDisabled(&self, ppitransaction: windows_core::OutRef<ITransaction>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl ITransactionCloner_Vtbl {
    pub const fn new<Identity: ITransactionCloner_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CloneWithCommitDisabled<Identity: ITransactionCloner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionCloner_Impl::CloneWithCommitDisabled(this, core::mem::transmute_copy(&ppitransaction)).into()
            }
        }
        Self { base__: ITransaction_Vtbl::new::<Identity, OFFSET>(), CloneWithCommitDisabled: CloneWithCommitDisabled::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionCloner as windows_core::Interface>::IID || iid == &<ITransaction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for ITransactionCloner {}
windows_core::imp::define_interface!(ITransactionDispenser, ITransactionDispenser_Vtbl, 0x3a6ad9e1_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionDispenser, windows_core::IUnknown);
impl ITransactionDispenser {
    pub unsafe fn GetOptionsObject(&self, ppoptions: *mut Option<ITransactionOptions>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOptionsObject)(windows_core::Interface::as_raw(self), core::mem::transmute(ppoptions)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn BeginTransaction<P0, P3>(&self, punkouter: P0, isolevel: ISOLEVEL, isoflags: super::ULONG, poptions: P3, pptransaction: *mut Option<ITransaction>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<ITransactionOptions>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), punkouter.param().abi(), isolevel, isoflags, poptions.param().abi(), core::mem::transmute(pptransaction)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionDispenser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOptionsObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, ISOLEVEL, super::ULONG, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    BeginTransaction: usize,
}
#[cfg(feature = "shared")]
pub trait ITransactionDispenser_Impl: windows_core::IUnknownImpl {
    fn GetOptionsObject(&self, ppoptions: windows_core::OutRef<ITransactionOptions>) -> windows_core::Result<()>;
    fn BeginTransaction(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, isolevel: ISOLEVEL, isoflags: super::ULONG, poptions: windows_core::Ref<ITransactionOptions>, pptransaction: windows_core::OutRef<ITransaction>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITransactionDispenser_Vtbl {
    pub const fn new<Identity: ITransactionDispenser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionsObject<Identity: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionDispenser_Impl::GetOptionsObject(this, core::mem::transmute_copy(&ppoptions)).into()
            }
        }
        unsafe extern "system" fn BeginTransaction<Identity: ITransactionDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, isolevel: ISOLEVEL, isoflags: super::ULONG, poptions: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionDispenser_Impl::BeginTransaction(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), core::mem::transmute_copy(&poptions), core::mem::transmute_copy(&pptransaction)).into()
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITransactionDispenser {}
windows_core::imp::define_interface!(ITransactionOptions, ITransactionOptions_Vtbl, 0x3a6ad9e0_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionOptions, windows_core::IUnknown);
impl ITransactionOptions {
    #[cfg(feature = "shared")]
    pub unsafe fn SetOptions(&self, poptions: *const XACTOPT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), poptions) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOptions(&self, poptions: *mut XACTOPT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), poptions as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const XACTOPT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOptions: usize,
    #[cfg(feature = "shared")]
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XACTOPT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOptions: usize,
}
#[cfg(feature = "shared")]
pub trait ITransactionOptions_Impl: windows_core::IUnknownImpl {
    fn SetOptions(&self, poptions: *const XACTOPT) -> windows_core::Result<()>;
    fn GetOptions(&self, poptions: *mut XACTOPT) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITransactionOptions {}
windows_core::imp::define_interface!(ITransactionOutcomeEvents, ITransactionOutcomeEvents_Vtbl, 0x3a6ad9e2_23b9_11cf_ad60_00aa00a74ccd);
windows_core::imp::interface_hierarchy!(ITransactionOutcomeEvents, windows_core::IUnknown);
impl ITransactionOutcomeEvents {
    #[cfg(feature = "rpc")]
    pub unsafe fn Committed(&self, fretaining: bool, pnewuow: Option<*const XACTUOW>, hr: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self), fretaining.into(), pnewuow.unwrap_or(core::mem::zeroed()) as _, hr) }
    }
    #[cfg(feature = "rpc")]
    pub unsafe fn Aborted(&self, pboidreason: Option<*const BOID>, fretaining: bool, pnewuow: Option<*const XACTUOW>, hr: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Aborted)(windows_core::Interface::as_raw(self), pboidreason.unwrap_or(core::mem::zeroed()) as _, fretaining.into(), pnewuow.unwrap_or(core::mem::zeroed()) as _, hr) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn HeuristicDecision(&self, dwdecision: super::DWORD, pboidreason: Option<*const BOID>, hr: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HeuristicDecision)(windows_core::Interface::as_raw(self), dwdecision, pboidreason.unwrap_or(core::mem::zeroed()) as _, hr) }
    }
    pub unsafe fn Indoubt(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Indoubt)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionOutcomeEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "rpc")]
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const XACTUOW, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    Committed: usize,
    #[cfg(feature = "rpc")]
    pub Aborted: unsafe extern "system" fn(*mut core::ffi::c_void, *const BOID, windows_core::BOOL, *const XACTUOW, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    Aborted: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub HeuristicDecision: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const BOID, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    HeuristicDecision: usize,
    pub Indoubt: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait ITransactionOutcomeEvents_Impl: windows_core::IUnknownImpl {
    fn Committed(&self, fretaining: windows_core::BOOL, pnewuow: *const XACTUOW, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Aborted(&self, pboidreason: *const BOID, fretaining: windows_core::BOOL, pnewuow: *const XACTUOW, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn HeuristicDecision(&self, dwdecision: super::DWORD, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Indoubt(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
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
        unsafe extern "system" fn HeuristicDecision<Identity: ITransactionOutcomeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdecision: super::DWORD, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "rpc", feature = "shared"))]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XACTOPT {
    pub ulTimeout: super::ULONG,
    pub szDescription: [i8; 40],
}
#[cfg(feature = "shared")]
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
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XACTSTATS {
    pub cOpen: super::ULONG,
    pub cCommitting: super::ULONG,
    pub cCommitted: super::ULONG,
    pub cAborting: super::ULONG,
    pub cAborted: super::ULONG,
    pub cInDoubt: super::ULONG,
    pub cHeuristicDecision: super::ULONG,
    pub timeTransactionsUp: super::FILETIME,
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
#[cfg(all(feature = "rpc", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XACTTRANSINFO {
    pub uow: XACTUOW,
    pub isoLevel: ISOLEVEL,
    pub isoFlags: super::ULONG,
    pub grfTCSupported: super::DWORD,
    pub grfRMSupported: super::DWORD,
    pub grfTCSupportedRetaining: super::DWORD,
    pub grfRMSupportedRetaining: super::DWORD,
}
#[cfg(feature = "rpc")]
pub type XACTUOW = BOID;
