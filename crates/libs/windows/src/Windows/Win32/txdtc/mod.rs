pub type DTCINITIATEDRECOVERYWORK = i32;
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: DTCINITIATEDRECOVERYWORK = 1;
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: DTCINITIATEDRECOVERYWORK = 3;
pub const DTCINITIATEDRECOVERYWORK_TRANS: DTCINITIATEDRECOVERYWORK = 2;
pub type DTCLUCOMPARESTATE = i32;
pub type DTCLUCOMPARESTATESCONFIRMATION = i32;
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: DTCLUCOMPARESTATESCONFIRMATION = 1;
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: DTCLUCOMPARESTATESCONFIRMATION = 2;
pub type DTCLUCOMPARESTATESERROR = i32;
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: DTCLUCOMPARESTATESERROR = 1;
pub type DTCLUCOMPARESTATESRESPONSE = i32;
pub const DTCLUCOMPARESTATESRESPONSE_OK: DTCLUCOMPARESTATESRESPONSE = 1;
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: DTCLUCOMPARESTATESRESPONSE = 2;
pub const DTCLUCOMPARESTATE_COMMITTED: DTCLUCOMPARESTATE = 1;
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: DTCLUCOMPARESTATE = 2;
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: DTCLUCOMPARESTATE = 3;
pub const DTCLUCOMPARESTATE_HEURISTICRESET: DTCLUCOMPARESTATE = 4;
pub const DTCLUCOMPARESTATE_INDOUBT: DTCLUCOMPARESTATE = 5;
pub const DTCLUCOMPARESTATE_RESET: DTCLUCOMPARESTATE = 6;
pub type DTCLUXLN = i32;
pub type DTCLUXLNCONFIRMATION = i32;
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: DTCLUXLNCONFIRMATION = 3;
pub const DTCLUXLNCONFIRMATION_CONFIRM: DTCLUXLNCONFIRMATION = 1;
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: DTCLUXLNCONFIRMATION = 2;
pub const DTCLUXLNCONFIRMATION_OBSOLETE: DTCLUXLNCONFIRMATION = 4;
pub type DTCLUXLNERROR = i32;
pub const DTCLUXLNERROR_COLDWARMMISMATCH: DTCLUXLNERROR = 3;
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: DTCLUXLNERROR = 2;
pub const DTCLUXLNERROR_PROTOCOL: DTCLUXLNERROR = 1;
pub type DTCLUXLNRESPONSE = i32;
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: DTCLUXLNRESPONSE = 4;
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: DTCLUXLNRESPONSE = 3;
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: DTCLUXLNRESPONSE = 2;
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: DTCLUXLNRESPONSE = 1;
pub const DTCLUXLN_COLD: DTCLUXLN = 1;
pub const DTCLUXLN_WARM: DTCLUXLN = 2;
windows_core::imp::define_interface!(IDtcLuConfigure, IDtcLuConfigure_Vtbl, 0x4131e760_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuConfigure, windows_core::IUnknown);
impl IDtcLuConfigure {
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn Add(&self, puclupair: &[super::byte]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), puclupair.as_ptr(), puclupair.len().try_into().unwrap()) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn Delete(&self, puclupair: &[super::byte]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), puclupair.as_ptr(), puclupair.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuConfigure_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::byte, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    Add: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::byte, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    Delete: usize,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait IDtcLuConfigure_Impl: windows_core::IUnknownImpl {
    fn Add(&self, puclupair: *const super::byte, cblupair: super::DWORD) -> windows_core::Result<()>;
    fn Delete(&self, puclupair: *const super::byte, cblupair: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl IDtcLuConfigure_Vtbl {
    pub const fn new<Identity: IDtcLuConfigure_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IDtcLuConfigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const super::byte, cblupair: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuConfigure_Impl::Add(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IDtcLuConfigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const super::byte, cblupair: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuConfigure_Impl::Delete(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, OFFSET>, Delete: Delete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuConfigure as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for IDtcLuConfigure {}
windows_core::imp::define_interface!(IDtcLuRecovery, IDtcLuRecovery_Vtbl, 0xac2b8ad2_d6f0_11d0_b386_00a0c9083365);
windows_core::imp::interface_hierarchy!(IDtcLuRecovery, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecovery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDtcLuRecovery_Impl: windows_core::IUnknownImpl {}
impl IDtcLuRecovery_Vtbl {
    pub const fn new<Identity: IDtcLuRecovery_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecovery as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecovery {}
windows_core::imp::define_interface!(IDtcLuRecoveryFactory, IDtcLuRecoveryFactory_Vtbl, 0x4131e762_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryFactory, windows_core::IUnknown);
impl IDtcLuRecoveryFactory {
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn Create(&self, puclupair: &[super::byte], pprecovery: *mut Option<IDtcLuRecovery>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), puclupair.as_ptr(), puclupair.len().try_into().unwrap(), core::mem::transmute(pprecovery)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::byte, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    Create: usize,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait IDtcLuRecoveryFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, puclupair: *const super::byte, cblupair: super::DWORD, pprecovery: windows_core::OutRef<IDtcLuRecovery>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl IDtcLuRecoveryFactory_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IDtcLuRecoveryFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const super::byte, cblupair: super::DWORD, pprecovery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryFactory_Impl::Create(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair), core::mem::transmute_copy(&pprecovery)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for IDtcLuRecoveryFactory {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByDtc, IDtcLuRecoveryInitiatedByDtc_Vtbl, 0x4131e764_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByDtc, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtc {
    pub unsafe fn GetWork(&self, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWork)(windows_core::Interface::as_raw(self), pwork as _, ppv as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DTCINITIATEDRECOVERYWORK, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuRecoveryInitiatedByDtc_Impl: windows_core::IUnknownImpl {
    fn GetWork(&self, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByDtc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWork<Identity: IDtcLuRecoveryInitiatedByDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtc_Impl::GetWork(this, core::mem::transmute_copy(&pwork), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWork: GetWork::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByDtc {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByDtcStatusWork, IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl, 0x4131e766_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByDtcStatusWork, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcStatusWork {
    #[cfg(feature = "shared")]
    pub unsafe fn HandleCheckLuStatus(&self, lrecoveryseqnum: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleCheckLuStatus)(windows_core::Interface::as_raw(self), lrecoveryseqnum) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub HandleCheckLuStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    HandleCheckLuStatus: usize,
}
#[cfg(feature = "shared")]
pub trait IDtcLuRecoveryInitiatedByDtcStatusWork_Impl: windows_core::IUnknownImpl {
    fn HandleCheckLuStatus(&self, lrecoveryseqnum: super::LONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleCheckLuStatus<Identity: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrecoveryseqnum: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcStatusWork_Impl::HandleCheckLuStatus(this, core::mem::transmute_copy(&lrecoveryseqnum)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleCheckLuStatus: HandleCheckLuStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcStatusWork as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByDtcStatusWork {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByDtcTransWork, IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl, 0x4131e765_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByDtcTransWork, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByDtcTransWork {
    #[cfg(feature = "shared")]
    pub unsafe fn GetLogNameSizes(&self, pcbourlogname: *mut super::DWORD, pcbremotelogname: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLogNameSizes)(windows_core::Interface::as_raw(self), pcbourlogname as _, pcbremotelogname as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOurXln)(windows_core::Interface::as_raw(self), pxln as _, pourlogname as _, premotelogname as _, pdwprotocol as _) }
    }
    pub unsafe fn HandleConfirmationFromOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleConfirmationFromOurXln)(windows_core::Interface::as_raw(self), confirmation) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn HandleTheirXlnResponse(&self, xln: DTCLUXLN, premotelogname: *const u8, cbremotelogname: super::DWORD, dwprotocol: super::DWORD, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleTheirXlnResponse)(windows_core::Interface::as_raw(self), xln, premotelogname, cbremotelogname, dwprotocol, pconfirmation as _) }
    }
    pub unsafe fn HandleErrorFromOurXln(&self, error: DTCLUXLNERROR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleErrorFromOurXln)(windows_core::Interface::as_raw(self), error) }
    }
    pub unsafe fn CheckForCompareStates(&self, fcomparestates: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckForCompareStates)(windows_core::Interface::as_raw(self), fcomparestates as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOurTransIdSize(&self, pcbourtransid: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOurTransIdSize)(windows_core::Interface::as_raw(self), pcbourtransid as _) }
    }
    pub unsafe fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOurCompareStates)(windows_core::Interface::as_raw(self), pourtransid as _, pcomparestate as _) }
    }
    pub unsafe fn HandleTheirCompareStatesResponse(&self, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleTheirCompareStatesResponse)(windows_core::Interface::as_raw(self), comparestate, pconfirmation as _) }
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleErrorFromOurCompareStates)(windows_core::Interface::as_raw(self), error) }
    }
    pub unsafe fn ConversationLost(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConversationLost)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRecoverySeqNum)(windows_core::Interface::as_raw(self), plrecoveryseqnum as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ObsoleteRecoverySeqNum)(windows_core::Interface::as_raw(self), lnewrecoveryseqnum) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetLogNameSizes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLogNameSizes: usize,
    #[cfg(feature = "shared")]
    pub GetOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DTCLUXLN, *mut u8, *mut u8, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOurXln: usize,
    pub HandleConfirmationFromOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLNCONFIRMATION) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub HandleTheirXlnResponse: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLN, *const u8, super::DWORD, super::DWORD, *mut DTCLUXLNCONFIRMATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    HandleTheirXlnResponse: usize,
    pub HandleErrorFromOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLNERROR) -> windows_core::HRESULT,
    pub CheckForCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetOurTransIdSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOurTransIdSize: usize,
    pub GetOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT,
    pub HandleTheirCompareStatesResponse: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATE, *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetRecoverySeqNum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRecoverySeqNum: usize,
    #[cfg(feature = "shared")]
    pub ObsoleteRecoverySeqNum: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ObsoleteRecoverySeqNum: usize,
}
#[cfg(feature = "shared")]
pub trait IDtcLuRecoveryInitiatedByDtcTransWork_Impl: windows_core::IUnknownImpl {
    fn GetLogNameSizes(&self, pcbourlogname: *mut super::DWORD, pcbremotelogname: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut super::DWORD) -> windows_core::Result<()>;
    fn HandleConfirmationFromOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()>;
    fn HandleTheirXlnResponse(&self, xln: DTCLUXLN, premotelogname: *const u8, cbremotelogname: super::DWORD, dwprotocol: super::DWORD, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> windows_core::Result<()>;
    fn HandleErrorFromOurXln(&self, error: DTCLUXLNERROR) -> windows_core::Result<()>;
    fn CheckForCompareStates(&self, fcomparestates: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetOurTransIdSize(&self, pcbourtransid: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()>;
    fn HandleTheirCompareStatesResponse(&self, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()>;
    fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()>;
    fn ConversationLost(&self) -> windows_core::Result<()>;
    fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut super::LONG) -> windows_core::Result<()>;
    fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: super::LONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLogNameSizes<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbourlogname: *mut super::DWORD, pcbremotelogname: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetLogNameSizes(this, core::mem::transmute_copy(&pcbourlogname), core::mem::transmute_copy(&pcbremotelogname)).into()
            }
        }
        unsafe extern "system" fn GetOurXln<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetOurXln(this, core::mem::transmute_copy(&pxln), core::mem::transmute_copy(&pourlogname), core::mem::transmute_copy(&premotelogname), core::mem::transmute_copy(&pdwprotocol)).into()
            }
        }
        unsafe extern "system" fn HandleConfirmationFromOurXln<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleConfirmationFromOurXln(this, core::mem::transmute_copy(&confirmation)).into()
            }
        }
        unsafe extern "system" fn HandleTheirXlnResponse<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xln: DTCLUXLN, premotelogname: *const u8, cbremotelogname: super::DWORD, dwprotocol: super::DWORD, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleTheirXlnResponse(this, core::mem::transmute_copy(&xln), core::mem::transmute_copy(&premotelogname), core::mem::transmute_copy(&cbremotelogname), core::mem::transmute_copy(&dwprotocol), core::mem::transmute_copy(&pconfirmation)).into()
            }
        }
        unsafe extern "system" fn HandleErrorFromOurXln<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: DTCLUXLNERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleErrorFromOurXln(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn CheckForCompareStates<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcomparestates: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::CheckForCompareStates(this, core::mem::transmute_copy(&fcomparestates)).into()
            }
        }
        unsafe extern "system" fn GetOurTransIdSize<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbourtransid: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetOurTransIdSize(this, core::mem::transmute_copy(&pcbourtransid)).into()
            }
        }
        unsafe extern "system" fn GetOurCompareStates<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetOurCompareStates(this, core::mem::transmute_copy(&pourtransid), core::mem::transmute_copy(&pcomparestate)).into()
            }
        }
        unsafe extern "system" fn HandleTheirCompareStatesResponse<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleTheirCompareStatesResponse(this, core::mem::transmute_copy(&comparestate), core::mem::transmute_copy(&pconfirmation)).into()
            }
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleErrorFromOurCompareStates(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn ConversationLost<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::ConversationLost(this).into()
            }
        }
        unsafe extern "system" fn GetRecoverySeqNum<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plrecoveryseqnum: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetRecoverySeqNum(this, core::mem::transmute_copy(&plrecoveryseqnum)).into()
            }
        }
        unsafe extern "system" fn ObsoleteRecoverySeqNum<Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnewrecoveryseqnum: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByDtcTransWork_Impl::ObsoleteRecoverySeqNum(this, core::mem::transmute_copy(&lnewrecoveryseqnum)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogNameSizes: GetLogNameSizes::<Identity, OFFSET>,
            GetOurXln: GetOurXln::<Identity, OFFSET>,
            HandleConfirmationFromOurXln: HandleConfirmationFromOurXln::<Identity, OFFSET>,
            HandleTheirXlnResponse: HandleTheirXlnResponse::<Identity, OFFSET>,
            HandleErrorFromOurXln: HandleErrorFromOurXln::<Identity, OFFSET>,
            CheckForCompareStates: CheckForCompareStates::<Identity, OFFSET>,
            GetOurTransIdSize: GetOurTransIdSize::<Identity, OFFSET>,
            GetOurCompareStates: GetOurCompareStates::<Identity, OFFSET>,
            HandleTheirCompareStatesResponse: HandleTheirCompareStatesResponse::<Identity, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, OFFSET>,
            ConversationLost: ConversationLost::<Identity, OFFSET>,
            GetRecoverySeqNum: GetRecoverySeqNum::<Identity, OFFSET>,
            ObsoleteRecoverySeqNum: ObsoleteRecoverySeqNum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcTransWork as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByDtcTransWork {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByLu, IDtcLuRecoveryInitiatedByLu_Vtbl, 0x4131e768_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByLu, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByLu {
    pub unsafe fn GetObjectToHandleWorkFromLu(&self, ppwork: *mut Option<IDtcLuRecoveryInitiatedByLuWork>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectToHandleWorkFromLu)(windows_core::Interface::as_raw(self), core::mem::transmute(ppwork)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectToHandleWorkFromLu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuRecoveryInitiatedByLu_Impl: windows_core::IUnknownImpl {
    fn GetObjectToHandleWorkFromLu(&self, ppwork: windows_core::OutRef<IDtcLuRecoveryInitiatedByLuWork>) -> windows_core::Result<()>;
}
impl IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByLu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectToHandleWorkFromLu<Identity: IDtcLuRecoveryInitiatedByLu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLu_Impl::GetObjectToHandleWorkFromLu(this, core::mem::transmute_copy(&ppwork)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetObjectToHandleWorkFromLu: GetObjectToHandleWorkFromLu::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLu as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByLu {}
windows_core::imp::define_interface!(IDtcLuRecoveryInitiatedByLuWork, IDtcLuRecoveryInitiatedByLuWork_Vtbl, 0xac2b8ad1_d6f0_11d0_b386_00a0c9083365);
windows_core::imp::interface_hierarchy!(IDtcLuRecoveryInitiatedByLuWork, windows_core::IUnknown);
impl IDtcLuRecoveryInitiatedByLuWork {
    #[cfg(feature = "shared")]
    pub unsafe fn HandleTheirXln(&self, lrecoveryseqnum: super::LONG, xln: DTCLUXLN, premotelogname: *const u8, cbremotelogname: super::DWORD, pourlogname: *const u8, cbourlogname: super::DWORD, dwprotocol: super::DWORD, presponse: *mut DTCLUXLNRESPONSE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleTheirXln)(windows_core::Interface::as_raw(self), lrecoveryseqnum, xln, premotelogname, cbremotelogname, pourlogname, cbourlogname, dwprotocol, presponse as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOurLogNameSize(&self, pcbourlogname: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOurLogNameSize)(windows_core::Interface::as_raw(self), pcbourlogname as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOurXln)(windows_core::Interface::as_raw(self), pxln as _, pourlogname as _, pdwprotocol as _) }
    }
    pub unsafe fn HandleConfirmationOfOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleConfirmationOfOurXln)(windows_core::Interface::as_raw(self), confirmation) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: super::DWORD, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleTheirCompareStates)(windows_core::Interface::as_raw(self), premotetransid as _, cbremotetransid, comparestate, presponse as _, pcomparestate as _) }
    }
    pub unsafe fn HandleConfirmationOfOurCompareStates(&self, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleConfirmationOfOurCompareStates)(windows_core::Interface::as_raw(self), confirmation) }
    }
    pub unsafe fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleErrorFromOurCompareStates)(windows_core::Interface::as_raw(self), error) }
    }
    pub unsafe fn ConversationLost(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConversationLost)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRecoveryInitiatedByLuWork_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub HandleTheirXln: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, DTCLUXLN, *const u8, super::DWORD, *const u8, super::DWORD, super::DWORD, *mut DTCLUXLNRESPONSE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    HandleTheirXln: usize,
    #[cfg(feature = "shared")]
    pub GetOurLogNameSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOurLogNameSize: usize,
    #[cfg(feature = "shared")]
    pub GetOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DTCLUXLN, *mut u8, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOurXln: usize,
    pub HandleConfirmationOfOurXln: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUXLNCONFIRMATION) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub HandleTheirCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, super::DWORD, DTCLUCOMPARESTATE, *mut DTCLUCOMPARESTATESRESPONSE, *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    HandleTheirCompareStates: usize,
    pub HandleConfirmationOfOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT,
    pub HandleErrorFromOurCompareStates: unsafe extern "system" fn(*mut core::ffi::c_void, DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT,
    pub ConversationLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IDtcLuRecoveryInitiatedByLuWork_Impl: windows_core::IUnknownImpl {
    fn HandleTheirXln(&self, lrecoveryseqnum: super::LONG, xln: DTCLUXLN, premotelogname: *const u8, cbremotelogname: super::DWORD, pourlogname: *const u8, cbourlogname: super::DWORD, dwprotocol: super::DWORD, presponse: *mut DTCLUXLNRESPONSE) -> windows_core::Result<()>;
    fn GetOurLogNameSize(&self, pcbourlogname: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut super::DWORD) -> windows_core::Result<()>;
    fn HandleConfirmationOfOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()>;
    fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: super::DWORD, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()>;
    fn HandleConfirmationOfOurCompareStates(&self, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()>;
    fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()>;
    fn ConversationLost(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDtcLuRecoveryInitiatedByLuWork_Vtbl {
    pub const fn new<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleTheirXln<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrecoveryseqnum: super::LONG, xln: DTCLUXLN, premotelogname: *const u8, cbremotelogname: super::DWORD, pourlogname: *const u8, cbourlogname: super::DWORD, dwprotocol: super::DWORD, presponse: *mut DTCLUXLNRESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleTheirXln(this, core::mem::transmute_copy(&lrecoveryseqnum), core::mem::transmute_copy(&xln), core::mem::transmute_copy(&premotelogname), core::mem::transmute_copy(&cbremotelogname), core::mem::transmute_copy(&pourlogname), core::mem::transmute_copy(&cbourlogname), core::mem::transmute_copy(&dwprotocol), core::mem::transmute_copy(&presponse)).into()
            }
        }
        unsafe extern "system" fn GetOurLogNameSize<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbourlogname: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::GetOurLogNameSize(this, core::mem::transmute_copy(&pcbourlogname)).into()
            }
        }
        unsafe extern "system" fn GetOurXln<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::GetOurXln(this, core::mem::transmute_copy(&pxln), core::mem::transmute_copy(&pourlogname), core::mem::transmute_copy(&pdwprotocol)).into()
            }
        }
        unsafe extern "system" fn HandleConfirmationOfOurXln<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleConfirmationOfOurXln(this, core::mem::transmute_copy(&confirmation)).into()
            }
        }
        unsafe extern "system" fn HandleTheirCompareStates<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: super::DWORD, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleTheirCompareStates(this, core::mem::transmute_copy(&premotetransid), core::mem::transmute_copy(&cbremotetransid), core::mem::transmute_copy(&comparestate), core::mem::transmute_copy(&presponse), core::mem::transmute_copy(&pcomparestate)).into()
            }
        }
        unsafe extern "system" fn HandleConfirmationOfOurCompareStates<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleConfirmationOfOurCompareStates(this, core::mem::transmute_copy(&confirmation)).into()
            }
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::HandleErrorFromOurCompareStates(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn ConversationLost<Identity: IDtcLuRecoveryInitiatedByLuWork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRecoveryInitiatedByLuWork_Impl::ConversationLost(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleTheirXln: HandleTheirXln::<Identity, OFFSET>,
            GetOurLogNameSize: GetOurLogNameSize::<Identity, OFFSET>,
            GetOurXln: GetOurXln::<Identity, OFFSET>,
            HandleConfirmationOfOurXln: HandleConfirmationOfOurXln::<Identity, OFFSET>,
            HandleTheirCompareStates: HandleTheirCompareStates::<Identity, OFFSET>,
            HandleConfirmationOfOurCompareStates: HandleConfirmationOfOurCompareStates::<Identity, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, OFFSET>,
            ConversationLost: ConversationLost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLuWork as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByLuWork {}
windows_core::imp::define_interface!(IDtcLuRmEnlistment, IDtcLuRmEnlistment_Vtbl, 0x4131e769_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRmEnlistment, windows_core::IUnknown);
impl IDtcLuRmEnlistment {
    pub unsafe fn Unplug(&self, fconversationlost: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unplug)(windows_core::Interface::as_raw(self), fconversationlost.into()) }
    }
    pub unsafe fn BackedOut(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BackOut(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Committed(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Forget(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unplug: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuRmEnlistment_Impl: windows_core::IUnknownImpl {
    fn Unplug(&self, fconversationlost: windows_core::BOOL) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl IDtcLuRmEnlistment_Vtbl {
    pub const fn new<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Unplug<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fconversationlost: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::Unplug(this, core::mem::transmute_copy(&fconversationlost)).into()
            }
        }
        unsafe extern "system" fn BackedOut<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::BackedOut(this).into()
            }
        }
        unsafe extern "system" fn BackOut<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::BackOut(this).into()
            }
        }
        unsafe extern "system" fn Committed<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::Committed(this).into()
            }
        }
        unsafe extern "system" fn Forget<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::Forget(this).into()
            }
        }
        unsafe extern "system" fn RequestCommit<Identity: IDtcLuRmEnlistment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistment_Impl::RequestCommit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Unplug: Unplug::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRmEnlistment {}
windows_core::imp::define_interface!(IDtcLuRmEnlistmentFactory, IDtcLuRmEnlistmentFactory_Vtbl, 0x4131e771_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRmEnlistmentFactory, windows_core::IUnknown);
impl IDtcLuRmEnlistmentFactory {
    #[cfg(all(feature = "shared", feature = "transact"))]
    pub unsafe fn Create<P2, P5>(&self, puclupair: *const u8, cblupair: super::DWORD, pitransaction: P2, ptransid: *const u8, cbtransid: super::DWORD, prmenlistmentsink: P5, pprmenlistment: *mut Option<IDtcLuRmEnlistment>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::ITransaction>,
        P5: windows_core::Param<IDtcLuRmEnlistmentSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), puclupair, cblupair, pitransaction.param().abi(), ptransid, cbtransid, prmenlistmentsink.param().abi(), core::mem::transmute(pprmenlistment)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "transact"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, super::DWORD, *mut core::ffi::c_void, *const u8, super::DWORD, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "transact")))]
    Create: usize,
}
#[cfg(all(feature = "shared", feature = "transact"))]
pub trait IDtcLuRmEnlistmentFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, puclupair: *const u8, cblupair: super::DWORD, pitransaction: windows_core::Ref<super::ITransaction>, ptransid: *const u8, cbtransid: super::DWORD, prmenlistmentsink: windows_core::Ref<IDtcLuRmEnlistmentSink>, pprmenlistment: windows_core::OutRef<IDtcLuRmEnlistment>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "transact"))]
impl IDtcLuRmEnlistmentFactory_Vtbl {
    pub const fn new<Identity: IDtcLuRmEnlistmentFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IDtcLuRmEnlistmentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const u8, cblupair: super::DWORD, pitransaction: *mut core::ffi::c_void, ptransid: *const u8, cbtransid: super::DWORD, prmenlistmentsink: *mut core::ffi::c_void, pprmenlistment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentFactory_Impl::Create(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair), core::mem::transmute_copy(&pitransaction), core::mem::transmute_copy(&ptransid), core::mem::transmute_copy(&cbtransid), core::mem::transmute_copy(&prmenlistmentsink), core::mem::transmute_copy(&pprmenlistment)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "transact"))]
impl windows_core::RuntimeName for IDtcLuRmEnlistmentFactory {}
windows_core::imp::define_interface!(IDtcLuRmEnlistmentSink, IDtcLuRmEnlistmentSink_Vtbl, 0x4131e770_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuRmEnlistmentSink, windows_core::IUnknown);
impl IDtcLuRmEnlistmentSink {
    pub unsafe fn AckUnplug(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AckUnplug)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TmDown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TmDown)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SessionLost(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SessionLost)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BackedOut(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BackOut(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Committed(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Forget(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Prepare(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuRmEnlistmentSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AckUnplug: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TmDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuRmEnlistmentSink_Impl: windows_core::IUnknownImpl {
    fn AckUnplug(&self) -> windows_core::Result<()>;
    fn TmDown(&self) -> windows_core::Result<()>;
    fn SessionLost(&self) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn Prepare(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl IDtcLuRmEnlistmentSink_Vtbl {
    pub const fn new<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AckUnplug<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::AckUnplug(this).into()
            }
        }
        unsafe extern "system" fn TmDown<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::TmDown(this).into()
            }
        }
        unsafe extern "system" fn SessionLost<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::SessionLost(this).into()
            }
        }
        unsafe extern "system" fn BackedOut<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::BackedOut(this).into()
            }
        }
        unsafe extern "system" fn BackOut<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::BackOut(this).into()
            }
        }
        unsafe extern "system" fn Committed<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::Committed(this).into()
            }
        }
        unsafe extern "system" fn Forget<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::Forget(this).into()
            }
        }
        unsafe extern "system" fn Prepare<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::Prepare(this).into()
            }
        }
        unsafe extern "system" fn RequestCommit<Identity: IDtcLuRmEnlistmentSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuRmEnlistmentSink_Impl::RequestCommit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AckUnplug: AckUnplug::<Identity, OFFSET>,
            TmDown: TmDown::<Identity, OFFSET>,
            SessionLost: SessionLost::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            Prepare: Prepare::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuRmEnlistmentSink {}
windows_core::imp::define_interface!(IDtcLuSubordinateDtc, IDtcLuSubordinateDtc_Vtbl, 0x4131e773_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuSubordinateDtc, windows_core::IUnknown);
impl IDtcLuSubordinateDtc {
    pub unsafe fn Unplug(&self, fconversationlost: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unplug)(windows_core::Interface::as_raw(self), fconversationlost.into()) }
    }
    pub unsafe fn BackedOut(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BackOut(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Committed(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Forget(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Prepare(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unplug: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuSubordinateDtc_Impl: windows_core::IUnknownImpl {
    fn Unplug(&self, fconversationlost: windows_core::BOOL) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn Prepare(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl IDtcLuSubordinateDtc_Vtbl {
    pub const fn new<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Unplug<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fconversationlost: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::Unplug(this, core::mem::transmute_copy(&fconversationlost)).into()
            }
        }
        unsafe extern "system" fn BackedOut<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::BackedOut(this).into()
            }
        }
        unsafe extern "system" fn BackOut<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::BackOut(this).into()
            }
        }
        unsafe extern "system" fn Committed<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::Committed(this).into()
            }
        }
        unsafe extern "system" fn Forget<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::Forget(this).into()
            }
        }
        unsafe extern "system" fn Prepare<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::Prepare(this).into()
            }
        }
        unsafe extern "system" fn RequestCommit<Identity: IDtcLuSubordinateDtc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtc_Impl::RequestCommit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Unplug: Unplug::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            Prepare: Prepare::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuSubordinateDtc {}
windows_core::imp::define_interface!(IDtcLuSubordinateDtcFactory, IDtcLuSubordinateDtcFactory_Vtbl, 0x4131e775_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuSubordinateDtcFactory, windows_core::IUnknown);
impl IDtcLuSubordinateDtcFactory {
    #[cfg(all(feature = "shared", feature = "transact"))]
    pub unsafe fn Create<P2, P5, P9>(&self, puclupair: *const u8, cblupair: super::DWORD, punktransactionouter: P2, isolevel: super::ISOLEVEL, isoflags: super::ULONG, poptions: P5, pptransaction: *mut Option<super::ITransaction>, ptransid: *const u8, cbtransid: super::DWORD, psubordinatedtcsink: P9, ppsubordinatedtc: *mut Option<IDtcLuSubordinateDtc>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<super::ITransactionOptions>,
        P9: windows_core::Param<IDtcLuSubordinateDtcSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), puclupair, cblupair, punktransactionouter.param().abi(), isolevel, isoflags, poptions.param().abi(), core::mem::transmute(pptransaction), ptransid, cbtransid, psubordinatedtcsink.param().abi(), core::mem::transmute(ppsubordinatedtc)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "transact"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, super::DWORD, *mut core::ffi::c_void, super::ISOLEVEL, super::ULONG, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *const u8, super::DWORD, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "transact")))]
    Create: usize,
}
#[cfg(all(feature = "shared", feature = "transact"))]
pub trait IDtcLuSubordinateDtcFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, puclupair: *const u8, cblupair: super::DWORD, punktransactionouter: windows_core::Ref<windows_core::IUnknown>, isolevel: super::ISOLEVEL, isoflags: super::ULONG, poptions: windows_core::Ref<super::ITransactionOptions>, pptransaction: windows_core::OutRef<super::ITransaction>, ptransid: *const u8, cbtransid: super::DWORD, psubordinatedtcsink: windows_core::Ref<IDtcLuSubordinateDtcSink>, ppsubordinatedtc: windows_core::OutRef<IDtcLuSubordinateDtc>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "transact"))]
impl IDtcLuSubordinateDtcFactory_Vtbl {
    pub const fn new<Identity: IDtcLuSubordinateDtcFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IDtcLuSubordinateDtcFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const u8, cblupair: super::DWORD, punktransactionouter: *mut core::ffi::c_void, isolevel: super::ISOLEVEL, isoflags: super::ULONG, poptions: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void, ptransid: *const u8, cbtransid: super::DWORD, psubordinatedtcsink: *mut core::ffi::c_void, ppsubordinatedtc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcFactory_Impl::Create(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair), core::mem::transmute_copy(&punktransactionouter), core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), core::mem::transmute_copy(&poptions), core::mem::transmute_copy(&pptransaction), core::mem::transmute_copy(&ptransid), core::mem::transmute_copy(&cbtransid), core::mem::transmute_copy(&psubordinatedtcsink), core::mem::transmute_copy(&ppsubordinatedtc)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "transact"))]
impl windows_core::RuntimeName for IDtcLuSubordinateDtcFactory {}
windows_core::imp::define_interface!(IDtcLuSubordinateDtcSink, IDtcLuSubordinateDtcSink_Vtbl, 0x4131e774_1aea_11d0_944b_00a0c905416e);
windows_core::imp::interface_hierarchy!(IDtcLuSubordinateDtcSink, windows_core::IUnknown);
impl IDtcLuSubordinateDtcSink {
    pub unsafe fn AckUnplug(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AckUnplug)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TmDown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TmDown)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SessionLost(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SessionLost)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BackedOut(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackedOut)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BackOut(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackOut)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Committed(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Committed)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Forget(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Forget)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RequestCommit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestCommit)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDtcLuSubordinateDtcSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AckUnplug: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TmDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackedOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Committed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDtcLuSubordinateDtcSink_Impl: windows_core::IUnknownImpl {
    fn AckUnplug(&self) -> windows_core::Result<()>;
    fn TmDown(&self) -> windows_core::Result<()>;
    fn SessionLost(&self) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl IDtcLuSubordinateDtcSink_Vtbl {
    pub const fn new<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AckUnplug<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::AckUnplug(this).into()
            }
        }
        unsafe extern "system" fn TmDown<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::TmDown(this).into()
            }
        }
        unsafe extern "system" fn SessionLost<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::SessionLost(this).into()
            }
        }
        unsafe extern "system" fn BackedOut<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::BackedOut(this).into()
            }
        }
        unsafe extern "system" fn BackOut<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::BackOut(this).into()
            }
        }
        unsafe extern "system" fn Committed<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::Committed(this).into()
            }
        }
        unsafe extern "system" fn Forget<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::Forget(this).into()
            }
        }
        unsafe extern "system" fn RequestCommit<Identity: IDtcLuSubordinateDtcSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDtcLuSubordinateDtcSink_Impl::RequestCommit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AckUnplug: AckUnplug::<Identity, OFFSET>,
            TmDown: TmDown::<Identity, OFFSET>,
            SessionLost: SessionLost::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDtcLuSubordinateDtcSink {}
windows_core::imp::define_interface!(IGetDispenser, IGetDispenser_Vtbl, 0xc23cc370_87ef_11ce_8081_0080c758527e);
windows_core::imp::interface_hierarchy!(IGetDispenser, windows_core::IUnknown);
impl IGetDispenser {
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetDispenser<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDispenser)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetDispenser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub GetDispenser: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetDispenser: usize,
}
#[cfg(feature = "guiddef")]
pub trait IGetDispenser_Impl: windows_core::IUnknownImpl {
    fn GetDispenser(&self, iid: *const super::IID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl IGetDispenser_Vtbl {
    pub const fn new<Identity: IGetDispenser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDispenser<Identity: IGetDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const super::IID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetDispenser_Impl::GetDispenser(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDispenser: GetDispenser::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetDispenser as windows_core::Interface>::IID
    }
}
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IGetDispenser {}
pub const IID_IDtcLuConfigure: windows_core::GUID = windows_core::GUID::from_u128(0x4131e760_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecovery: windows_core::GUID = windows_core::GUID::from_u128(0xac2b8ad2_d6f0_11d0_b386_00a0c9083365);
pub const IID_IDtcLuRecoveryFactory: windows_core::GUID = windows_core::GUID::from_u128(0x4131e762_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByDtc: windows_core::GUID = windows_core::GUID::from_u128(0x4131e764_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByDtcStatusWork: windows_core::GUID = windows_core::GUID::from_u128(0x4131e766_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByDtcTransWork: windows_core::GUID = windows_core::GUID::from_u128(0x4131e765_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByLu: windows_core::GUID = windows_core::GUID::from_u128(0x4131e768_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByLuWork: windows_core::GUID = windows_core::GUID::from_u128(0xac2b8ad1_d6f0_11d0_b386_00a0c9083365);
pub const IID_IDtcLuRmEnlistment: windows_core::GUID = windows_core::GUID::from_u128(0x4131e769_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRmEnlistmentFactory: windows_core::GUID = windows_core::GUID::from_u128(0x4131e771_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRmEnlistmentSink: windows_core::GUID = windows_core::GUID::from_u128(0x4131e770_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuSubordinateDtc: windows_core::GUID = windows_core::GUID::from_u128(0x4131e773_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuSubordinateDtcFactory: windows_core::GUID = windows_core::GUID::from_u128(0x4131e775_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuSubordinateDtcSink: windows_core::GUID = windows_core::GUID::from_u128(0x4131e774_1aea_11d0_944b_00a0c905416e);
pub const IID_IGetDispenser: windows_core::GUID = windows_core::GUID::from_u128(0xc23cc370_87ef_11ce_8081_0080c758527e);
pub const IID_ILastResourceManager: windows_core::GUID = windows_core::GUID::from_u128(0x4d964ad4_5b33_11d3_8a91_00c04f79eb6d);
pub const IID_IPrepareInfo: windows_core::GUID = windows_core::GUID::from_u128(0x80c7bfd0_87ee_11ce_8081_0080c758527e);
pub const IID_IPrepareInfo2: windows_core::GUID = windows_core::GUID::from_u128(0x5fab2547_9779_11d1_b886_00c04fb9618a);
pub const IID_IRMHelper: windows_core::GUID = windows_core::GUID::from_u128(0xe793f6d1_f53d_11cf_a60d_00a0c905416e);
pub const IID_IResourceManager: windows_core::GUID = windows_core::GUID::from_u128(0x56a868ac_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IResourceManager2: windows_core::GUID = windows_core::GUID::from_u128(0xd136c69a_f749_11d1_8f47_00c04f8ee57d);
pub const IID_IResourceManagerFactory: windows_core::GUID = windows_core::GUID::from_u128(0x13741d20_87eb_11ce_8081_0080c758527e);
pub const IID_IResourceManagerFactory2: windows_core::GUID = windows_core::GUID::from_u128(0x6b369c21_fbd2_11d1_8f47_00c04f8ee57d);
pub const IID_IResourceManagerRejoinable: windows_core::GUID = windows_core::GUID::from_u128(0x6f6de620_b5df_4f3e_9cfa_c8aebd05172b);
pub const IID_IResourceManagerSink: windows_core::GUID = windows_core::GUID::from_u128(0x0d563181_defb_11ce_aed1_00aa0051e2c4);
pub const IID_ITransactionPhase0EnlistmentAsync: windows_core::GUID = windows_core::GUID::from_u128(0x82dc88e1_a954_11d1_8f88_00600895e7d5);
pub const IID_ITransactionPhase0Factory: windows_core::GUID = windows_core::GUID::from_u128(0x82dc88e0_a954_11d1_8f88_00600895e7d5);
pub const IID_ITransactionPhase0NotifyAsync: windows_core::GUID = windows_core::GUID::from_u128(0xef081809_0c76_11d2_87a6_00c04f990f34);
pub const IID_ITransactionReceiver: windows_core::GUID = windows_core::GUID::from_u128(0x59313e03_b36c_11cf_a539_00aa006887c3);
pub const IID_ITransactionReceiverFactory: windows_core::GUID = windows_core::GUID::from_u128(0x59313e02_b36c_11cf_a539_00aa006887c3);
pub const IID_ITransactionTransmitter: windows_core::GUID = windows_core::GUID::from_u128(0x59313e01_b36c_11cf_a539_00aa006887c3);
pub const IID_ITransactionTransmitterFactory: windows_core::GUID = windows_core::GUID::from_u128(0x59313e00_b36c_11cf_a539_00aa006887c3);
pub const IID_ITransactionVoterBallotAsync2: windows_core::GUID = windows_core::GUID::from_u128(0x5433376c_414d_11d3_b206_00c04fc2f3ef);
pub const IID_ITransactionVoterFactory2: windows_core::GUID = windows_core::GUID::from_u128(0x5433376a_414d_11d3_b206_00c04fc2f3ef);
pub const IID_ITransactionVoterNotifyAsync2: windows_core::GUID = windows_core::GUID::from_u128(0x5433376b_414d_11d3_b206_00c04fc2f3ef);
pub const IID_IXAConfig: windows_core::GUID = windows_core::GUID::from_u128(0xc8a6e3a1_9a8c_11cf_a308_00a0c905416e);
pub const IID_IXAObtainRMInfo: windows_core::GUID = windows_core::GUID::from_u128(0xe793f6d2_f53d_11cf_a60d_00a0c905416e);
pub const IID_IXATransLookup: windows_core::GUID = windows_core::GUID::from_u128(0xf3b1f131_eeda_11ce_aed4_00aa0051e2c4);
pub const IID_IXATransLookup2: windows_core::GUID = windows_core::GUID::from_u128(0xbf193c85_0d1a_4290_b88f_d2cb8873d1e7);
windows_core::imp::define_interface!(ILastResourceManager, ILastResourceManager_Vtbl, 0x4d964ad4_5b33_11d3_8a91_00c04f79eb6d);
windows_core::imp::interface_hierarchy!(ILastResourceManager, windows_core::IUnknown);
impl ILastResourceManager {
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn TransactionCommitted(&self, pprepinfo: &[super::byte]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TransactionCommitted)(windows_core::Interface::as_raw(self), pprepinfo.as_ptr(), pprepinfo.len().try_into().unwrap()) }
    }
    pub unsafe fn RecoveryDone(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RecoveryDone)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILastResourceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub TransactionCommitted: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::byte, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    TransactionCommitted: usize,
    pub RecoveryDone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait ILastResourceManager_Impl: windows_core::IUnknownImpl {
    fn TransactionCommitted(&self, pprepinfo: *const super::byte, cbprepinfo: super::ULONG) -> windows_core::Result<()>;
    fn RecoveryDone(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl ILastResourceManager_Vtbl {
    pub const fn new<Identity: ILastResourceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransactionCommitted<Identity: ILastResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *const super::byte, cbprepinfo: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILastResourceManager_Impl::TransactionCommitted(this, core::mem::transmute_copy(&pprepinfo), core::mem::transmute_copy(&cbprepinfo)).into()
            }
        }
        unsafe extern "system" fn RecoveryDone<Identity: ILastResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILastResourceManager_Impl::RecoveryDone(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransactionCommitted: TransactionCommitted::<Identity, OFFSET>,
            RecoveryDone: RecoveryDone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILastResourceManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for ILastResourceManager {}
windows_core::imp::define_interface!(IPrepareInfo, IPrepareInfo_Vtbl, 0x80c7bfd0_87ee_11ce_8081_0080c758527e);
windows_core::imp::interface_hierarchy!(IPrepareInfo, windows_core::IUnknown);
impl IPrepareInfo {
    #[cfg(feature = "shared")]
    pub unsafe fn GetPrepareInfoSize(&self, pcbprepinfo: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrepareInfoSize)(windows_core::Interface::as_raw(self), pcbprepinfo as _) }
    }
    #[cfg(feature = "rpc")]
    pub unsafe fn GetPrepareInfo(&self, pprepinfo: *mut super::byte) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrepareInfo)(windows_core::Interface::as_raw(self), pprepinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetPrepareInfoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPrepareInfoSize: usize,
    #[cfg(feature = "rpc")]
    pub GetPrepareInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::byte) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    GetPrepareInfo: usize,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait IPrepareInfo_Impl: windows_core::IUnknownImpl {
    fn GetPrepareInfoSize(&self, pcbprepinfo: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetPrepareInfo(&self, pprepinfo: *mut super::byte) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl IPrepareInfo_Vtbl {
    pub const fn new<Identity: IPrepareInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: IPrepareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbprepinfo: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrepareInfo_Impl::GetPrepareInfoSize(this, core::mem::transmute_copy(&pcbprepinfo)).into()
            }
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: IPrepareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *mut super::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrepareInfo_Impl::GetPrepareInfo(this, core::mem::transmute_copy(&pprepinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrepareInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for IPrepareInfo {}
windows_core::imp::define_interface!(IPrepareInfo2, IPrepareInfo2_Vtbl, 0x5fab2547_9779_11d1_b886_00c04fb9618a);
windows_core::imp::interface_hierarchy!(IPrepareInfo2, windows_core::IUnknown);
impl IPrepareInfo2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetPrepareInfoSize(&self, pcbprepinfo: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrepareInfoSize)(windows_core::Interface::as_raw(self), pcbprepinfo as _) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn GetPrepareInfo(&self, cbprepareinfo: super::ULONG, pprepinfo: *mut super::byte) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrepareInfo)(windows_core::Interface::as_raw(self), cbprepareinfo, pprepinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareInfo2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetPrepareInfoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPrepareInfoSize: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub GetPrepareInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut super::byte) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    GetPrepareInfo: usize,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait IPrepareInfo2_Impl: windows_core::IUnknownImpl {
    fn GetPrepareInfoSize(&self, pcbprepinfo: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetPrepareInfo(&self, cbprepareinfo: super::ULONG, pprepinfo: *mut super::byte) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl IPrepareInfo2_Vtbl {
    pub const fn new<Identity: IPrepareInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: IPrepareInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbprepinfo: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrepareInfo2_Impl::GetPrepareInfoSize(this, core::mem::transmute_copy(&pcbprepinfo)).into()
            }
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: IPrepareInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbprepareinfo: super::ULONG, pprepinfo: *mut super::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrepareInfo2_Impl::GetPrepareInfo(this, core::mem::transmute_copy(&cbprepareinfo), core::mem::transmute_copy(&pprepinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrepareInfo2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for IPrepareInfo2 {}
windows_core::imp::define_interface!(IRMHelper, IRMHelper_Vtbl, 0xe793f6d1_f53d_11cf_a60d_00a0c905416e);
windows_core::imp::interface_hierarchy!(IRMHelper, windows_core::IUnknown);
impl IRMHelper {
    #[cfg(feature = "shared")]
    pub unsafe fn RMCount(&self, dwctotalnumberofrms: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RMCount)(windows_core::Interface::as_raw(self), dwctotalnumberofrms) }
    }
    pub unsafe fn RMInfo(&self, pxa_switch: *const xa_switch_t, fcdeclcallingconv: bool, pszopenstring: *const i8, pszclosestring: *const i8, guidrmrecovery: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RMInfo)(windows_core::Interface::as_raw(self), pxa_switch, fcdeclcallingconv.into(), pszopenstring, pszclosestring, guidrmrecovery) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRMHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub RMCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RMCount: usize,
    pub RMInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const xa_switch_t, windows_core::BOOL, *const i8, *const i8, windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IRMHelper_Impl: windows_core::IUnknownImpl {
    fn RMCount(&self, dwctotalnumberofrms: super::DWORD) -> windows_core::Result<()>;
    fn RMInfo(&self, pxa_switch: *const xa_switch_t, fcdeclcallingconv: windows_core::BOOL, pszopenstring: *const i8, pszclosestring: *const i8, guidrmrecovery: &windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IRMHelper_Vtbl {
    pub const fn new<Identity: IRMHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RMCount<Identity: IRMHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwctotalnumberofrms: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRMHelper_Impl::RMCount(this, core::mem::transmute_copy(&dwctotalnumberofrms)).into()
            }
        }
        unsafe extern "system" fn RMInfo<Identity: IRMHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxa_switch: *const xa_switch_t, fcdeclcallingconv: windows_core::BOOL, pszopenstring: *const i8, pszclosestring: *const i8, guidrmrecovery: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRMHelper_Impl::RMInfo(this, core::mem::transmute_copy(&pxa_switch), core::mem::transmute_copy(&fcdeclcallingconv), core::mem::transmute_copy(&pszopenstring), core::mem::transmute_copy(&pszclosestring), core::mem::transmute(&guidrmrecovery)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RMCount: RMCount::<Identity, OFFSET>, RMInfo: RMInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRMHelper as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IRMHelper {}
#[cfg(feature = "strmif")]
windows_core::imp::define_interface!(IResourceManager2, IResourceManager2_Vtbl, 0xd136c69a_f749_11d1_8f47_00c04f8ee57d);
#[cfg(feature = "strmif")]
impl core::ops::Deref for IResourceManager2 {
    type Target = super::IResourceManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "strmif")]
windows_core::imp::interface_hierarchy!(IResourceManager2, windows_core::IUnknown, super::IResourceManager);
#[cfg(feature = "strmif")]
impl IResourceManager2 {
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact", feature = "txcoord"))]
    pub unsafe fn Enlist2<P0, P1>(&self, ptransaction: P0, presasync: P1, puow: *mut super::XACTUOW, pisolevel: *mut super::LONG, pxid: *mut XID, ppenlist: *mut Option<super::ITransactionEnlistmentAsync>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ITransaction>,
        P1: windows_core::Param<super::ITransactionResourceAsync>,
    {
        unsafe { (windows_core::Interface::vtable(self).Enlist2)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), presasync.param().abi(), puow as _, pisolevel as _, pxid as _, core::mem::transmute(ppenlist)) }
    }
    #[cfg(all(feature = "shared", feature = "transact"))]
    pub unsafe fn Reenlist2(&self, pxid: *const XID, dwtimeout: super::DWORD, pxactstat: *mut super::XACTSTAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reenlist2)(windows_core::Interface::as_raw(self), pxid, dwtimeout, pxactstat as _) }
    }
}
#[cfg(feature = "strmif")]
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_Vtbl {
    pub base__: super::IResourceManager_Vtbl,
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact", feature = "txcoord"))]
    pub Enlist2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::XACTUOW, *mut super::LONG, *mut XID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared", feature = "transact", feature = "txcoord")))]
    Enlist2: usize,
    #[cfg(all(feature = "shared", feature = "transact"))]
    pub Reenlist2: unsafe extern "system" fn(*mut core::ffi::c_void, *const XID, super::DWORD, *mut super::XACTSTAT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "transact")))]
    Reenlist2: usize,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "strmif", feature = "transact", feature = "txcoord"))]
pub trait IResourceManager2_Impl: super::IResourceManager_Impl {
    fn Enlist2(&self, ptransaction: windows_core::Ref<super::ITransaction>, presasync: windows_core::Ref<super::ITransactionResourceAsync>, puow: *mut super::XACTUOW, pisolevel: *mut super::LONG, pxid: *mut XID, ppenlist: windows_core::OutRef<super::ITransactionEnlistmentAsync>) -> windows_core::Result<()>;
    fn Reenlist2(&self, pxid: *const XID, dwtimeout: super::DWORD, pxactstat: *mut super::XACTSTAT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "strmif", feature = "transact", feature = "txcoord"))]
impl IResourceManager2_Vtbl {
    pub const fn new<Identity: IResourceManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enlist2<Identity: IResourceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, presasync: *mut core::ffi::c_void, puow: *mut super::XACTUOW, pisolevel: *mut super::LONG, pxid: *mut XID, ppenlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManager2_Impl::Enlist2(this, core::mem::transmute_copy(&ptransaction), core::mem::transmute_copy(&presasync), core::mem::transmute_copy(&puow), core::mem::transmute_copy(&pisolevel), core::mem::transmute_copy(&pxid), core::mem::transmute_copy(&ppenlist)).into()
            }
        }
        unsafe extern "system" fn Reenlist2<Identity: IResourceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxid: *const XID, dwtimeout: super::DWORD, pxactstat: *mut super::XACTSTAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManager2_Impl::Reenlist2(this, core::mem::transmute_copy(&pxid), core::mem::transmute_copy(&dwtimeout), core::mem::transmute_copy(&pxactstat)).into()
            }
        }
        Self { base__: super::IResourceManager_Vtbl::new::<Identity, OFFSET>(), Enlist2: Enlist2::<Identity, OFFSET>, Reenlist2: Reenlist2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManager2 as windows_core::Interface>::IID || iid == &<super::IResourceManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "strmif", feature = "transact", feature = "txcoord"))]
impl windows_core::RuntimeName for IResourceManager2 {}
windows_core::imp::define_interface!(IResourceManagerFactory, IResourceManagerFactory_Vtbl, 0x13741d20_87eb_11ce_8081_0080c758527e);
windows_core::imp::interface_hierarchy!(IResourceManagerFactory, windows_core::IUnknown);
impl IResourceManagerFactory {
    #[cfg(all(feature = "shared", feature = "strmif"))]
    pub unsafe fn Create<P2>(&self, pguidrm: *const windows_core::GUID, pszrmname: *const super::CHAR, piresmgrsink: P2, ppresmgr: *mut Option<super::IResourceManager>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IResourceManagerSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pguidrm, pszrmname, piresmgrsink.param().abi(), core::mem::transmute(ppresmgr)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "strmif"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::CHAR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "strmif")))]
    Create: usize,
}
#[cfg(all(feature = "shared", feature = "strmif"))]
pub trait IResourceManagerFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, pguidrm: *const windows_core::GUID, pszrmname: *const super::CHAR, piresmgrsink: windows_core::Ref<IResourceManagerSink>, ppresmgr: windows_core::OutRef<super::IResourceManager>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "strmif"))]
impl IResourceManagerFactory_Vtbl {
    pub const fn new<Identity: IResourceManagerFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IResourceManagerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidrm: *const windows_core::GUID, pszrmname: *const super::CHAR, piresmgrsink: *mut core::ffi::c_void, ppresmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManagerFactory_Impl::Create(this, core::mem::transmute_copy(&pguidrm), core::mem::transmute_copy(&pszrmname), core::mem::transmute_copy(&piresmgrsink), core::mem::transmute_copy(&ppresmgr)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "strmif"))]
impl windows_core::RuntimeName for IResourceManagerFactory {}
windows_core::imp::define_interface!(IResourceManagerFactory2, IResourceManagerFactory2_Vtbl, 0x6b369c21_fbd2_11d1_8f47_00c04f8ee57d);
impl core::ops::Deref for IResourceManagerFactory2 {
    type Target = IResourceManagerFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IResourceManagerFactory2, windows_core::IUnknown, IResourceManagerFactory);
impl IResourceManagerFactory2 {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn CreateEx<P2, T>(&self, pguidrm: *const windows_core::GUID, pszrmname: *const super::CHAR, piresmgrsink: P2) -> windows_core::Result<T>
    where
        P2: windows_core::Param<IResourceManagerSink>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateEx)(windows_core::Interface::as_raw(self), pguidrm, pszrmname, piresmgrsink.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory2_Vtbl {
    pub base__: IResourceManagerFactory_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub CreateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::CHAR, *mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    CreateEx: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "strmif"))]
pub trait IResourceManagerFactory2_Impl: IResourceManagerFactory_Impl {
    fn CreateEx(&self, pguidrm: *const windows_core::GUID, pszrmname: *const super::CHAR, piresmgrsink: windows_core::Ref<IResourceManagerSink>, riidrequested: *const super::IID, ppvresmgr: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "strmif"))]
impl IResourceManagerFactory2_Vtbl {
    pub const fn new<Identity: IResourceManagerFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEx<Identity: IResourceManagerFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidrm: *const windows_core::GUID, pszrmname: *const super::CHAR, piresmgrsink: *mut core::ffi::c_void, riidrequested: *const super::IID, ppvresmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManagerFactory2_Impl::CreateEx(this, core::mem::transmute_copy(&pguidrm), core::mem::transmute_copy(&pszrmname), core::mem::transmute_copy(&piresmgrsink), core::mem::transmute_copy(&riidrequested), core::mem::transmute_copy(&ppvresmgr)).into()
            }
        }
        Self { base__: IResourceManagerFactory_Vtbl::new::<Identity, OFFSET>(), CreateEx: CreateEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerFactory2 as windows_core::Interface>::IID || iid == &<IResourceManagerFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "strmif"))]
impl windows_core::RuntimeName for IResourceManagerFactory2 {}
#[cfg(feature = "strmif")]
windows_core::imp::define_interface!(IResourceManagerRejoinable, IResourceManagerRejoinable_Vtbl, 0x6f6de620_b5df_4f3e_9cfa_c8aebd05172b);
#[cfg(feature = "strmif")]
impl core::ops::Deref for IResourceManagerRejoinable {
    type Target = IResourceManager2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "strmif")]
windows_core::imp::interface_hierarchy!(IResourceManagerRejoinable, windows_core::IUnknown, super::IResourceManager, IResourceManager2);
#[cfg(feature = "strmif")]
impl IResourceManagerRejoinable {
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
    pub unsafe fn Rejoin(&self, pprepinfo: &[super::byte], ltimeout: super::DWORD, pxactstat: *mut super::XACTSTAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Rejoin)(windows_core::Interface::as_raw(self), pprepinfo.as_ptr(), pprepinfo.len().try_into().unwrap(), ltimeout, pxactstat as _) }
    }
}
#[cfg(feature = "strmif")]
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerRejoinable_Vtbl {
    pub base__: IResourceManager2_Vtbl,
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
    pub Rejoin: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::byte, super::ULONG, super::DWORD, *mut super::XACTSTAT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared", feature = "transact")))]
    Rejoin: usize,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "strmif", feature = "transact", feature = "txcoord"))]
pub trait IResourceManagerRejoinable_Impl: IResourceManager2_Impl {
    fn Rejoin(&self, pprepinfo: *const super::byte, cbprepinfo: super::ULONG, ltimeout: super::DWORD, pxactstat: *mut super::XACTSTAT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "strmif", feature = "transact", feature = "txcoord"))]
impl IResourceManagerRejoinable_Vtbl {
    pub const fn new<Identity: IResourceManagerRejoinable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Rejoin<Identity: IResourceManagerRejoinable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *const super::byte, cbprepinfo: super::ULONG, ltimeout: super::DWORD, pxactstat: *mut super::XACTSTAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManagerRejoinable_Impl::Rejoin(this, core::mem::transmute_copy(&pprepinfo), core::mem::transmute_copy(&cbprepinfo), core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&pxactstat)).into()
            }
        }
        Self { base__: IResourceManager2_Vtbl::new::<Identity, OFFSET>(), Rejoin: Rejoin::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerRejoinable as windows_core::Interface>::IID || iid == &<super::IResourceManager as windows_core::Interface>::IID || iid == &<IResourceManager2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "strmif", feature = "transact", feature = "txcoord"))]
impl windows_core::RuntimeName for IResourceManagerRejoinable {}
windows_core::imp::define_interface!(IResourceManagerSink, IResourceManagerSink_Vtbl, 0x0d563181_defb_11ce_aed1_00aa0051e2c4);
windows_core::imp::interface_hierarchy!(IResourceManagerSink, windows_core::IUnknown);
impl IResourceManagerSink {
    pub unsafe fn TMDown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TMDown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TMDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IResourceManagerSink_Impl: windows_core::IUnknownImpl {
    fn TMDown(&self) -> windows_core::Result<()>;
}
impl IResourceManagerSink_Vtbl {
    pub const fn new<Identity: IResourceManagerSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TMDown<Identity: IResourceManagerSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceManagerSink_Impl::TMDown(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TMDown: TMDown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IResourceManagerSink {}
windows_core::imp::define_interface!(ITransactionPhase0EnlistmentAsync, ITransactionPhase0EnlistmentAsync_Vtbl, 0x82dc88e1_a954_11d1_8f88_00600895e7d5);
windows_core::imp::interface_hierarchy!(ITransactionPhase0EnlistmentAsync, windows_core::IUnknown);
impl ITransactionPhase0EnlistmentAsync {
    pub unsafe fn Enable(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn WaitForEnlistment(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForEnlistment)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Phase0Done(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Phase0Done)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Unenlist(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unenlist)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "transact")]
    pub unsafe fn GetTransaction(&self, ppitransaction: *mut Option<super::ITransaction>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransaction)(windows_core::Interface::as_raw(self), core::mem::transmute(ppitransaction)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0EnlistmentAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForEnlistment: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Phase0Done: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unenlist: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "transact")]
    pub GetTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    GetTransaction: usize,
}
#[cfg(feature = "transact")]
pub trait ITransactionPhase0EnlistmentAsync_Impl: windows_core::IUnknownImpl {
    fn Enable(&self) -> windows_core::Result<()>;
    fn WaitForEnlistment(&self) -> windows_core::Result<()>;
    fn Phase0Done(&self) -> windows_core::Result<()>;
    fn Unenlist(&self) -> windows_core::Result<()>;
    fn GetTransaction(&self, ppitransaction: windows_core::OutRef<super::ITransaction>) -> windows_core::Result<()>;
}
#[cfg(feature = "transact")]
impl ITransactionPhase0EnlistmentAsync_Vtbl {
    pub const fn new<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enable<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0EnlistmentAsync_Impl::Enable(this).into()
            }
        }
        unsafe extern "system" fn WaitForEnlistment<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0EnlistmentAsync_Impl::WaitForEnlistment(this).into()
            }
        }
        unsafe extern "system" fn Phase0Done<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0EnlistmentAsync_Impl::Phase0Done(this).into()
            }
        }
        unsafe extern "system" fn Unenlist<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0EnlistmentAsync_Impl::Unenlist(this).into()
            }
        }
        unsafe extern "system" fn GetTransaction<Identity: ITransactionPhase0EnlistmentAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0EnlistmentAsync_Impl::GetTransaction(this, core::mem::transmute_copy(&ppitransaction)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enable: Enable::<Identity, OFFSET>,
            WaitForEnlistment: WaitForEnlistment::<Identity, OFFSET>,
            Phase0Done: Phase0Done::<Identity, OFFSET>,
            Unenlist: Unenlist::<Identity, OFFSET>,
            GetTransaction: GetTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionPhase0EnlistmentAsync as windows_core::Interface>::IID
    }
}
#[cfg(feature = "transact")]
impl windows_core::RuntimeName for ITransactionPhase0EnlistmentAsync {}
windows_core::imp::define_interface!(ITransactionPhase0Factory, ITransactionPhase0Factory_Vtbl, 0x82dc88e0_a954_11d1_8f88_00600895e7d5);
windows_core::imp::interface_hierarchy!(ITransactionPhase0Factory, windows_core::IUnknown);
impl ITransactionPhase0Factory {
    pub unsafe fn Create<P0>(&self, pphase0notify: P0, ppphase0enlistment: *mut Option<ITransactionPhase0EnlistmentAsync>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITransactionPhase0NotifyAsync>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pphase0notify.param().abi(), core::mem::transmute(ppphase0enlistment)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0Factory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionPhase0Factory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, pphase0notify: windows_core::Ref<ITransactionPhase0NotifyAsync>, ppphase0enlistment: windows_core::OutRef<ITransactionPhase0EnlistmentAsync>) -> windows_core::Result<()>;
}
impl ITransactionPhase0Factory_Vtbl {
    pub const fn new<Identity: ITransactionPhase0Factory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: ITransactionPhase0Factory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphase0notify: *mut core::ffi::c_void, ppphase0enlistment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0Factory_Impl::Create(this, core::mem::transmute_copy(&pphase0notify), core::mem::transmute_copy(&ppphase0enlistment)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionPhase0Factory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionPhase0Factory {}
windows_core::imp::define_interface!(ITransactionPhase0NotifyAsync, ITransactionPhase0NotifyAsync_Vtbl, 0xef081809_0c76_11d2_87a6_00c04f990f34);
windows_core::imp::interface_hierarchy!(ITransactionPhase0NotifyAsync, windows_core::IUnknown);
impl ITransactionPhase0NotifyAsync {
    pub unsafe fn Phase0Request(&self, fabortinghint: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Phase0Request)(windows_core::Interface::as_raw(self), fabortinghint.into()) }
    }
    pub unsafe fn EnlistCompleted(&self, status: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnlistCompleted)(windows_core::Interface::as_raw(self), status) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionPhase0NotifyAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Phase0Request: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub EnlistCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ITransactionPhase0NotifyAsync_Impl: windows_core::IUnknownImpl {
    fn Phase0Request(&self, fabortinghint: windows_core::BOOL) -> windows_core::Result<()>;
    fn EnlistCompleted(&self, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ITransactionPhase0NotifyAsync_Vtbl {
    pub const fn new<Identity: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Phase0Request<Identity: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fabortinghint: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0NotifyAsync_Impl::Phase0Request(this, core::mem::transmute_copy(&fabortinghint)).into()
            }
        }
        unsafe extern "system" fn EnlistCompleted<Identity: ITransactionPhase0NotifyAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionPhase0NotifyAsync_Impl::EnlistCompleted(this, core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Phase0Request: Phase0Request::<Identity, OFFSET>,
            EnlistCompleted: EnlistCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionPhase0NotifyAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionPhase0NotifyAsync {}
windows_core::imp::define_interface!(ITransactionReceiver, ITransactionReceiver_Vtbl, 0x59313e03_b36c_11cf_a539_00aa006887c3);
windows_core::imp::interface_hierarchy!(ITransactionReceiver, windows_core::IUnknown);
impl ITransactionReceiver {
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
    pub unsafe fn UnmarshalPropagationToken(&self, rgbtoken: &[super::byte], pptransaction: *mut Option<super::ITransaction>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnmarshalPropagationToken)(windows_core::Interface::as_raw(self), rgbtoken.len().try_into().unwrap(), rgbtoken.as_ptr(), core::mem::transmute(pptransaction)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetReturnTokenSize(&self, pcbreturntoken: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReturnTokenSize)(windows_core::Interface::as_raw(self), pcbreturntoken as _) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn MarshalReturnToken(&self, cbreturntoken: super::ULONG, rgbreturntoken: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MarshalReturnToken)(windows_core::Interface::as_raw(self), cbreturntoken, rgbreturntoken as _, pcbused as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
    pub UnmarshalPropagationToken: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::byte, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared", feature = "transact")))]
    UnmarshalPropagationToken: usize,
    #[cfg(feature = "shared")]
    pub GetReturnTokenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetReturnTokenSize: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub MarshalReturnToken: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut super::byte, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    MarshalReturnToken: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
pub trait ITransactionReceiver_Impl: windows_core::IUnknownImpl {
    fn UnmarshalPropagationToken(&self, cbtoken: super::ULONG, rgbtoken: *const super::byte, pptransaction: windows_core::OutRef<super::ITransaction>) -> windows_core::Result<()>;
    fn GetReturnTokenSize(&self, pcbreturntoken: *mut super::ULONG) -> windows_core::Result<()>;
    fn MarshalReturnToken(&self, cbreturntoken: super::ULONG, rgbreturntoken: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl ITransactionReceiver_Vtbl {
    pub const fn new<Identity: ITransactionReceiver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UnmarshalPropagationToken<Identity: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtoken: super::ULONG, rgbtoken: *const super::byte, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionReceiver_Impl::UnmarshalPropagationToken(this, core::mem::transmute_copy(&cbtoken), core::mem::transmute_copy(&rgbtoken), core::mem::transmute_copy(&pptransaction)).into()
            }
        }
        unsafe extern "system" fn GetReturnTokenSize<Identity: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbreturntoken: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionReceiver_Impl::GetReturnTokenSize(this, core::mem::transmute_copy(&pcbreturntoken)).into()
            }
        }
        unsafe extern "system" fn MarshalReturnToken<Identity: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbreturntoken: super::ULONG, rgbreturntoken: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionReceiver_Impl::MarshalReturnToken(this, core::mem::transmute_copy(&cbreturntoken), core::mem::transmute_copy(&rgbreturntoken), core::mem::transmute_copy(&pcbused)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ITransactionReceiver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionReceiver_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UnmarshalPropagationToken: UnmarshalPropagationToken::<Identity, OFFSET>,
            GetReturnTokenSize: GetReturnTokenSize::<Identity, OFFSET>,
            MarshalReturnToken: MarshalReturnToken::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionReceiver as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionReceiver {}
windows_core::imp::define_interface!(ITransactionReceiverFactory, ITransactionReceiverFactory_Vtbl, 0x59313e02_b36c_11cf_a539_00aa006887c3);
windows_core::imp::interface_hierarchy!(ITransactionReceiverFactory, windows_core::IUnknown);
impl ITransactionReceiverFactory {
    pub unsafe fn Create(&self, ppreceiver: *mut Option<ITransactionReceiver>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(ppreceiver)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionReceiverFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionReceiverFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, ppreceiver: windows_core::OutRef<ITransactionReceiver>) -> windows_core::Result<()>;
}
impl ITransactionReceiverFactory_Vtbl {
    pub const fn new<Identity: ITransactionReceiverFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: ITransactionReceiverFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreceiver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionReceiverFactory_Impl::Create(this, core::mem::transmute_copy(&ppreceiver)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionReceiverFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionReceiverFactory {}
windows_core::imp::define_interface!(ITransactionTransmitter, ITransactionTransmitter_Vtbl, 0x59313e01_b36c_11cf_a539_00aa006887c3);
windows_core::imp::interface_hierarchy!(ITransactionTransmitter, windows_core::IUnknown);
impl ITransactionTransmitter {
    #[cfg(feature = "transact")]
    pub unsafe fn Set<P0>(&self, ptransaction: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ITransaction>,
    {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), ptransaction.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPropagationTokenSize(&self, pcbtoken: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropagationTokenSize)(windows_core::Interface::as_raw(self), pcbtoken as _) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn MarshalPropagationToken(&self, cbtoken: super::ULONG, rgbtoken: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MarshalPropagationToken)(windows_core::Interface::as_raw(self), cbtoken, rgbtoken as _, pcbused as _) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn UnmarshalReturnToken(&self, rgbreturntoken: &[super::byte]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnmarshalReturnToken)(windows_core::Interface::as_raw(self), rgbreturntoken.len().try_into().unwrap(), rgbreturntoken.as_ptr()) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "transact")]
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    Set: usize,
    #[cfg(feature = "shared")]
    pub GetPropagationTokenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPropagationTokenSize: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub MarshalPropagationToken: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut super::byte, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    MarshalPropagationToken: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub UnmarshalReturnToken: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::byte) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    UnmarshalReturnToken: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
pub trait ITransactionTransmitter_Impl: windows_core::IUnknownImpl {
    fn Set(&self, ptransaction: windows_core::Ref<super::ITransaction>) -> windows_core::Result<()>;
    fn GetPropagationTokenSize(&self, pcbtoken: *mut super::ULONG) -> windows_core::Result<()>;
    fn MarshalPropagationToken(&self, cbtoken: super::ULONG, rgbtoken: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::Result<()>;
    fn UnmarshalReturnToken(&self, cbreturntoken: super::ULONG, rgbreturntoken: *const super::byte) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl ITransactionTransmitter_Vtbl {
    pub const fn new<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitter_Impl::Set(this, core::mem::transmute_copy(&ptransaction)).into()
            }
        }
        unsafe extern "system" fn GetPropagationTokenSize<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbtoken: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitter_Impl::GetPropagationTokenSize(this, core::mem::transmute_copy(&pcbtoken)).into()
            }
        }
        unsafe extern "system" fn MarshalPropagationToken<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtoken: super::ULONG, rgbtoken: *mut super::byte, pcbused: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitter_Impl::MarshalPropagationToken(this, core::mem::transmute_copy(&cbtoken), core::mem::transmute_copy(&rgbtoken), core::mem::transmute_copy(&pcbused)).into()
            }
        }
        unsafe extern "system" fn UnmarshalReturnToken<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbreturntoken: super::ULONG, rgbreturntoken: *const super::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitter_Impl::UnmarshalReturnToken(this, core::mem::transmute_copy(&cbreturntoken), core::mem::transmute_copy(&rgbreturntoken)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ITransactionTransmitter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitter_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, OFFSET>,
            GetPropagationTokenSize: GetPropagationTokenSize::<Identity, OFFSET>,
            MarshalPropagationToken: MarshalPropagationToken::<Identity, OFFSET>,
            UnmarshalReturnToken: UnmarshalReturnToken::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionTransmitter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionTransmitter {}
windows_core::imp::define_interface!(ITransactionTransmitterFactory, ITransactionTransmitterFactory_Vtbl, 0x59313e00_b36c_11cf_a539_00aa006887c3);
windows_core::imp::interface_hierarchy!(ITransactionTransmitterFactory, windows_core::IUnknown);
impl ITransactionTransmitterFactory {
    pub unsafe fn Create(&self, pptransmitter: *mut Option<ITransactionTransmitter>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(pptransmitter)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionTransmitterFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionTransmitterFactory_Impl: windows_core::IUnknownImpl {
    fn Create(&self, pptransmitter: windows_core::OutRef<ITransactionTransmitter>) -> windows_core::Result<()>;
}
impl ITransactionTransmitterFactory_Vtbl {
    pub const fn new<Identity: ITransactionTransmitterFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: ITransactionTransmitterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptransmitter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionTransmitterFactory_Impl::Create(this, core::mem::transmute_copy(&pptransmitter)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionTransmitterFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionTransmitterFactory {}
windows_core::imp::define_interface!(ITransactionVoterBallotAsync2, ITransactionVoterBallotAsync2_Vtbl, 0x5433376c_414d_11d3_b206_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITransactionVoterBallotAsync2, windows_core::IUnknown);
impl ITransactionVoterBallotAsync2 {
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub unsafe fn VoteRequestDone(&self, hr: windows_core::HRESULT, pboidreason: Option<*const super::BOID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).VoteRequestDone)(windows_core::Interface::as_raw(self), hr, pboidreason.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterBallotAsync2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "rpc", feature = "transact"))]
    pub VoteRequestDone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *const super::BOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "transact")))]
    VoteRequestDone: usize,
}
#[cfg(all(feature = "rpc", feature = "transact"))]
pub trait ITransactionVoterBallotAsync2_Impl: windows_core::IUnknownImpl {
    fn VoteRequestDone(&self, hr: windows_core::HRESULT, pboidreason: *const super::BOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "transact"))]
impl ITransactionVoterBallotAsync2_Vtbl {
    pub const fn new<Identity: ITransactionVoterBallotAsync2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VoteRequestDone<Identity: ITransactionVoterBallotAsync2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, pboidreason: *const super::BOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionVoterBallotAsync2_Impl::VoteRequestDone(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&pboidreason)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), VoteRequestDone: VoteRequestDone::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionVoterBallotAsync2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionVoterBallotAsync2 {}
windows_core::imp::define_interface!(ITransactionVoterFactory2, ITransactionVoterFactory2_Vtbl, 0x5433376a_414d_11d3_b206_00c04fc2f3ef);
windows_core::imp::interface_hierarchy!(ITransactionVoterFactory2, windows_core::IUnknown);
impl ITransactionVoterFactory2 {
    #[cfg(feature = "transact")]
    pub unsafe fn Create<P0, P1>(&self, ptransaction: P0, pvoternotify: P1, ppvoterballot: *mut Option<ITransactionVoterBallotAsync2>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ITransaction>,
        P1: windows_core::Param<ITransactionVoterNotifyAsync2>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), pvoternotify.param().abi(), core::mem::transmute(ppvoterballot)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "transact")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    Create: usize,
}
#[cfg(feature = "transact")]
pub trait ITransactionVoterFactory2_Impl: windows_core::IUnknownImpl {
    fn Create(&self, ptransaction: windows_core::Ref<super::ITransaction>, pvoternotify: windows_core::Ref<ITransactionVoterNotifyAsync2>, ppvoterballot: windows_core::OutRef<ITransactionVoterBallotAsync2>) -> windows_core::Result<()>;
}
#[cfg(feature = "transact")]
impl ITransactionVoterFactory2_Vtbl {
    pub const fn new<Identity: ITransactionVoterFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: ITransactionVoterFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, pvoternotify: *mut core::ffi::c_void, ppvoterballot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionVoterFactory2_Impl::Create(this, core::mem::transmute_copy(&ptransaction), core::mem::transmute_copy(&pvoternotify), core::mem::transmute_copy(&ppvoterballot)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionVoterFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "transact")]
impl windows_core::RuntimeName for ITransactionVoterFactory2 {}
#[cfg(feature = "transact")]
windows_core::imp::define_interface!(ITransactionVoterNotifyAsync2, ITransactionVoterNotifyAsync2_Vtbl, 0x5433376b_414d_11d3_b206_00c04fc2f3ef);
#[cfg(feature = "transact")]
impl core::ops::Deref for ITransactionVoterNotifyAsync2 {
    type Target = super::ITransactionOutcomeEvents;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "transact")]
windows_core::imp::interface_hierarchy!(ITransactionVoterNotifyAsync2, windows_core::IUnknown, super::ITransactionOutcomeEvents);
#[cfg(feature = "transact")]
impl ITransactionVoterNotifyAsync2 {
    pub unsafe fn VoteRequest(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).VoteRequest)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "transact")]
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionVoterNotifyAsync2_Vtbl {
    pub base__: super::ITransactionOutcomeEvents_Vtbl,
    pub VoteRequest: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
pub trait ITransactionVoterNotifyAsync2_Impl: super::ITransactionOutcomeEvents_Impl {
    fn VoteRequest(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl ITransactionVoterNotifyAsync2_Vtbl {
    pub const fn new<Identity: ITransactionVoterNotifyAsync2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VoteRequest<Identity: ITransactionVoterNotifyAsync2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionVoterNotifyAsync2_Impl::VoteRequest(this).into()
            }
        }
        Self { base__: super::ITransactionOutcomeEvents_Vtbl::new::<Identity, OFFSET>(), VoteRequest: VoteRequest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionVoterNotifyAsync2 as windows_core::Interface>::IID || iid == &<super::ITransactionOutcomeEvents as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared", feature = "transact"))]
impl windows_core::RuntimeName for ITransactionVoterNotifyAsync2 {}
windows_core::imp::define_interface!(IXAConfig, IXAConfig_Vtbl, 0xc8a6e3a1_9a8c_11cf_a308_00a0c905416e);
windows_core::imp::interface_hierarchy!(IXAConfig, windows_core::IUnknown);
impl IXAConfig {
    pub unsafe fn Initialize(&self, clsidhelperdll: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), clsidhelperdll) }
    }
    pub unsafe fn Terminate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXAConfig_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, clsidhelperdll: &windows_core::GUID) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
}
impl IXAConfig_Vtbl {
    pub const fn new<Identity: IXAConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IXAConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidhelperdll: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAConfig_Impl::Initialize(this, core::mem::transmute(&clsidhelperdll)).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IXAConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAConfig_Impl::Terminate(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXAConfig {}
windows_core::imp::define_interface!(IXAObtainRMInfo, IXAObtainRMInfo_Vtbl, 0xe793f6d2_f53d_11cf_a60d_00a0c905416e);
windows_core::imp::interface_hierarchy!(IXAObtainRMInfo, windows_core::IUnknown);
impl IXAObtainRMInfo {
    pub unsafe fn ObtainRMInfo<P0>(&self, pirmhelper: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRMHelper>,
    {
        unsafe { (windows_core::Interface::vtable(self).ObtainRMInfo)(windows_core::Interface::as_raw(self), pirmhelper.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAObtainRMInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ObtainRMInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXAObtainRMInfo_Impl: windows_core::IUnknownImpl {
    fn ObtainRMInfo(&self, pirmhelper: windows_core::Ref<IRMHelper>) -> windows_core::Result<()>;
}
impl IXAObtainRMInfo_Vtbl {
    pub const fn new<Identity: IXAObtainRMInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ObtainRMInfo<Identity: IXAObtainRMInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirmhelper: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAObtainRMInfo_Impl::ObtainRMInfo(this, core::mem::transmute_copy(&pirmhelper)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ObtainRMInfo: ObtainRMInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAObtainRMInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXAObtainRMInfo {}
windows_core::imp::define_interface!(IXATransLookup, IXATransLookup_Vtbl, 0xf3b1f131_eeda_11ce_aed4_00aa0051e2c4);
windows_core::imp::interface_hierarchy!(IXATransLookup, windows_core::IUnknown);
impl IXATransLookup {
    #[cfg(feature = "transact")]
    pub unsafe fn Lookup(&self, pptransaction: *mut Option<super::ITransaction>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lookup)(windows_core::Interface::as_raw(self), core::mem::transmute(pptransaction)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "transact")]
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    Lookup: usize,
}
#[cfg(feature = "transact")]
pub trait IXATransLookup_Impl: windows_core::IUnknownImpl {
    fn Lookup(&self, pptransaction: windows_core::OutRef<super::ITransaction>) -> windows_core::Result<()>;
}
#[cfg(feature = "transact")]
impl IXATransLookup_Vtbl {
    pub const fn new<Identity: IXATransLookup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lookup<Identity: IXATransLookup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXATransLookup_Impl::Lookup(this, core::mem::transmute_copy(&pptransaction)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Lookup: Lookup::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXATransLookup as windows_core::Interface>::IID
    }
}
#[cfg(feature = "transact")]
impl windows_core::RuntimeName for IXATransLookup {}
windows_core::imp::define_interface!(IXATransLookup2, IXATransLookup2_Vtbl, 0xbf193c85_0d1a_4290_b88f_d2cb8873d1e7);
windows_core::imp::interface_hierarchy!(IXATransLookup2, windows_core::IUnknown);
impl IXATransLookup2 {
    #[cfg(feature = "transact")]
    pub unsafe fn Lookup(&self, pxid: *const XID, pptransaction: *mut Option<super::ITransaction>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lookup)(windows_core::Interface::as_raw(self), pxid, core::mem::transmute(pptransaction)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXATransLookup2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "transact")]
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, *const XID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "transact"))]
    Lookup: usize,
}
#[cfg(feature = "transact")]
pub trait IXATransLookup2_Impl: windows_core::IUnknownImpl {
    fn Lookup(&self, pxid: *const XID, pptransaction: windows_core::OutRef<super::ITransaction>) -> windows_core::Result<()>;
}
#[cfg(feature = "transact")]
impl IXATransLookup2_Vtbl {
    pub const fn new<Identity: IXATransLookup2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lookup<Identity: IXATransLookup2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxid: *const XID, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXATransLookup2_Impl::Lookup(this, core::mem::transmute_copy(&pxid), core::mem::transmute_copy(&pptransaction)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Lookup: Lookup::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXATransLookup2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "transact")]
impl windows_core::RuntimeName for IXATransLookup2 {}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROXY_CONFIG_PARAMS {
    pub wcThreadsMax: super::WORD,
}
pub type XACT_DTC_CONSTANTS = i32;
pub const XACT_E_CONNECTION_REQUEST_DENIED: XACT_DTC_CONSTANTS = -2147168000;
pub const XACT_E_DUPLICATE_GUID: XACT_DTC_CONSTANTS = -2147167998;
pub const XACT_E_DUPLICATE_LU: XACT_DTC_CONSTANTS = -2147167991;
pub const XACT_E_DUPLICATE_TRANSID: XACT_DTC_CONSTANTS = -2147167989;
pub const XACT_E_LRMRECOVERYALREADYDONE: XACT_DTC_CONSTANTS = -2147167982;
pub const XACT_E_LU_BUSY: XACT_DTC_CONSTANTS = -2147167988;
pub const XACT_E_LU_DOWN: XACT_DTC_CONSTANTS = -2147167986;
pub const XACT_E_LU_NOT_CONNECTED: XACT_DTC_CONSTANTS = -2147167990;
pub const XACT_E_LU_NOT_FOUND: XACT_DTC_CONSTANTS = -2147167992;
pub const XACT_E_LU_NO_RECOVERY_PROCESS: XACT_DTC_CONSTANTS = -2147167987;
pub const XACT_E_LU_RECOVERING: XACT_DTC_CONSTANTS = -2147167985;
pub const XACT_E_LU_RECOVERY_MISMATCH: XACT_DTC_CONSTANTS = -2147167984;
pub const XACT_E_NOLASTRESOURCEINTERFACE: XACT_DTC_CONSTANTS = -2147167981;
pub const XACT_E_NOTSINGLEPHASE: XACT_DTC_CONSTANTS = -2147167997;
pub const XACT_E_PROTOCOL: XACT_DTC_CONSTANTS = -2147167995;
pub const XACT_E_RECOVERYALREADYDONE: XACT_DTC_CONSTANTS = -2147167996;
pub const XACT_E_RECOVERY_FAILED: XACT_DTC_CONSTANTS = -2147167993;
pub const XACT_E_RM_FAILURE: XACT_DTC_CONSTANTS = -2147167994;
pub const XACT_E_RM_UNAVAILABLE: XACT_DTC_CONSTANTS = -2147167983;
pub const XACT_E_TOOMANY_ENLISTMENTS: XACT_DTC_CONSTANTS = -2147167999;
pub const XACT_OK_NONOTIFY: XACT_DTC_CONSTANTS = 315649;
pub const XACT_S_NONOTIFY: XACT_DTC_CONSTANTS = 315648;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XID {
    pub formatID: i32,
    pub gtrid_length: i32,
    pub bqual_length: i32,
    pub data: [i8; 128],
}
impl Default for XID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = 65535;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct xa_switch_t {
    pub name: [i8; 32],
    pub flags: i32,
    pub version: i32,
    pub xa_open_entry: Option<unsafe extern "C" fn(*mut i8, i32, i32) -> i32>,
    pub xa_close_entry: Option<unsafe extern "C" fn(*mut i8, i32, i32) -> i32>,
    pub xa_start_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_end_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_rollback_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_prepare_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_commit_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_recover_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32, i32) -> i32>,
    pub xa_forget_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_complete_entry: Option<unsafe extern "C" fn(*mut i32, *mut i32, i32, i32) -> i32>,
}
impl Default for xa_switch_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
