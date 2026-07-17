pub const CpAicLaunchAdminProcess: CreateProcessMethod = 2;
pub const CpCreateProcess: CreateProcessMethod = 0;
pub const CpCreateProcessAsUser: CreateProcessMethod = 1;
pub type CreateProcessMethod = i32;
pub type HANDLE_ACCESS_OPTIONS = u32;
pub type HANDLE_CREATION_OPTIONS = i32;
pub type HANDLE_OPTIONS = u32;
pub type HANDLE_SHARING_OPTIONS = u32;
pub const HAO_DELETE: HANDLE_ACCESS_OPTIONS = 65536;
pub const HAO_NONE: HANDLE_ACCESS_OPTIONS = 0;
pub const HAO_READ: HANDLE_ACCESS_OPTIONS = 1179785;
pub const HAO_READ_ATTRIBUTES: HANDLE_ACCESS_OPTIONS = 128;
pub const HAO_WRITE: HANDLE_ACCESS_OPTIONS = 1179926;
pub const HCO_CREATE_ALWAYS: HANDLE_CREATION_OPTIONS = 2;
pub const HCO_CREATE_NEW: HANDLE_CREATION_OPTIONS = 1;
pub const HCO_OPEN_ALWAYS: HANDLE_CREATION_OPTIONS = 4;
pub const HCO_OPEN_EXISTING: HANDLE_CREATION_OPTIONS = 3;
pub const HCO_TRUNCATE_EXISTING: HANDLE_CREATION_OPTIONS = 5;
pub const HO_DELETE_ON_CLOSE: HANDLE_OPTIONS = 67108864;
pub const HO_NONE: HANDLE_OPTIONS = 0;
pub const HO_NO_BUFFERING: HANDLE_OPTIONS = 536870912;
pub const HO_OPEN_REQUIRING_OPLOCK: HANDLE_OPTIONS = 262144;
pub const HO_OVERLAPPED: HANDLE_OPTIONS = 1073741824;
pub const HO_RANDOM_ACCESS: HANDLE_OPTIONS = 268435456;
pub const HO_SEQUENTIAL_SCAN: HANDLE_OPTIONS = 134217728;
pub const HO_WRITE_THROUGH: HANDLE_OPTIONS = 2147483648;
pub const HSO_SHARE_DELETE: HANDLE_SHARING_OPTIONS = 4;
pub const HSO_SHARE_NONE: HANDLE_SHARING_OPTIONS = 0;
pub const HSO_SHARE_READ: HANDLE_SHARING_OPTIONS = 1;
pub const HSO_SHARE_WRITE: HANDLE_SHARING_OPTIONS = 2;
windows_core::imp::define_interface!(IDDEInitializer, IDDEInitializer_Vtbl, 0x30dc931f_33fc_4ffd_a168_942258cf3ca4);
windows_core::imp::interface_hierarchy!(IDDEInitializer, windows_core::IUnknown);
impl IDDEInitializer {
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn Initialize<P0, P2, P3, P4, P5, P6, P7, P8>(&self, fileextensionorprotocol: P0, method: CreateProcessMethod, currentdirectory: P2, exectarget: P3, site: P4, application: P5, targetfile: P6, arguments: P7, verb: P8) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::shobjidl_core::IShellItem>,
        P4: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
        P8: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), fileextensionorprotocol.param().abi(), method, currentdirectory.param().abi(), exectarget.param().abi(), site.param().abi(), application.param().abi(), targetfile.param().abi(), arguments.param().abi(), verb.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDEInitializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shobjidl_core")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, CreateProcessMethod, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    Initialize: usize,
}
#[cfg(feature = "shobjidl_core")]
pub trait IDDEInitializer_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, fileextensionorprotocol: &windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: &windows_core::PCWSTR, exectarget: windows_core::Ref<super::shobjidl_core::IShellItem>, site: windows_core::Ref<windows_core::IUnknown>, application: &windows_core::PCWSTR, targetfile: &windows_core::PCWSTR, arguments: &windows_core::PCWSTR, verb: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shobjidl_core")]
impl IDDEInitializer_Vtbl {
    pub const fn new<Identity: IDDEInitializer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDDEInitializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileextensionorprotocol: windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: windows_core::PCWSTR, exectarget: *mut core::ffi::c_void, site: *mut core::ffi::c_void, application: windows_core::PCWSTR, targetfile: windows_core::PCWSTR, arguments: windows_core::PCWSTR, verb: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDDEInitializer_Impl::Initialize(this, core::mem::transmute(&fileextensionorprotocol), core::mem::transmute_copy(&method), core::mem::transmute(&currentdirectory), core::mem::transmute_copy(&exectarget), core::mem::transmute_copy(&site), core::mem::transmute(&application), core::mem::transmute(&targetfile), core::mem::transmute(&arguments), core::mem::transmute(&verb)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDDEInitializer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shobjidl_core")]
impl windows_core::RuntimeName for IDDEInitializer {}
windows_core::imp::define_interface!(IOplockBreakingHandler, IOplockBreakingHandler_Vtbl, 0x826abe3d_3acd_47d3_84f2_88aaedcf6304);
windows_core::imp::interface_hierarchy!(IOplockBreakingHandler, windows_core::IUnknown);
impl IOplockBreakingHandler {
    pub unsafe fn OplockBreaking(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OplockBreaking)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockBreakingHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OplockBreaking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOplockBreakingHandler_Impl: windows_core::IUnknownImpl {
    fn OplockBreaking(&self) -> windows_core::Result<()>;
}
impl IOplockBreakingHandler_Vtbl {
    pub const fn new<Identity: IOplockBreakingHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OplockBreaking<Identity: IOplockBreakingHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOplockBreakingHandler_Impl::OplockBreaking(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OplockBreaking: OplockBreaking::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOplockBreakingHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOplockBreakingHandler {}
windows_core::imp::define_interface!(IRandomAccessStreamFileAccessMode, IRandomAccessStreamFileAccessMode_Vtbl, 0x332e5848_2e15_458e_85c4_c911c0c3d6f4);
windows_core::imp::interface_hierarchy!(IRandomAccessStreamFileAccessMode, windows_core::IUnknown);
impl IRandomAccessStreamFileAccessMode {
    pub unsafe fn GetMode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamFileAccessMode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IRandomAccessStreamFileAccessMode_Impl: windows_core::IUnknownImpl {
    fn GetMode(&self) -> windows_core::Result<u32>;
}
impl IRandomAccessStreamFileAccessMode_Vtbl {
    pub const fn new<Identity: IRandomAccessStreamFileAccessMode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMode<Identity: IRandomAccessStreamFileAccessMode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileaccessmode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRandomAccessStreamFileAccessMode_Impl::GetMode(this) {
                    Ok(ok__) => {
                        fileaccessmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMode: GetMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRandomAccessStreamFileAccessMode as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRandomAccessStreamFileAccessMode {}
windows_core::imp::define_interface!(IStorageFolderHandleAccess, IStorageFolderHandleAccess_Vtbl, 0xdf19938f_5462_48a0_be65_d2a3271a08d6);
windows_core::imp::interface_hierarchy!(IStorageFolderHandleAccess, windows_core::IUnknown);
impl IStorageFolderHandleAccess {
    #[cfg(feature = "winnt")]
    pub unsafe fn Create<P0, P5>(&self, filename: P0, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: P5) -> windows_core::Result<super::winnt::HANDLE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<IOplockBreakingHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), filename.param().abi(), creationoptions, accessoptions, sharingoptions, options, oplockbreakinghandler.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderHandleAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, HANDLE_CREATION_OPTIONS, HANDLE_ACCESS_OPTIONS, HANDLE_SHARING_OPTIONS, HANDLE_OPTIONS, *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Create: usize,
}
#[cfg(feature = "winnt")]
pub trait IStorageFolderHandleAccess_Impl: windows_core::IUnknownImpl {
    fn Create(&self, filename: &windows_core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: windows_core::Ref<IOplockBreakingHandler>) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(feature = "winnt")]
impl IStorageFolderHandleAccess_Vtbl {
    pub const fn new<Identity: IStorageFolderHandleAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IStorageFolderHandleAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut core::ffi::c_void, interophandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFolderHandleAccess_Impl::Create(this, core::mem::transmute(&filename), core::mem::transmute_copy(&creationoptions), core::mem::transmute_copy(&accessoptions), core::mem::transmute_copy(&sharingoptions), core::mem::transmute_copy(&options), core::mem::transmute_copy(&oplockbreakinghandler)) {
                    Ok(ok__) => {
                        interophandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageFolderHandleAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IStorageFolderHandleAccess {}
windows_core::imp::define_interface!(IStorageItemHandleAccess, IStorageItemHandleAccess_Vtbl, 0x5ca296b2_2c25_4d22_b785_b885c8201e6a);
windows_core::imp::interface_hierarchy!(IStorageItemHandleAccess, windows_core::IUnknown);
impl IStorageItemHandleAccess {
    #[cfg(feature = "winnt")]
    pub unsafe fn Create<P3>(&self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: P3) -> windows_core::Result<super::winnt::HANDLE>
    where
        P3: windows_core::Param<IOplockBreakingHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), accessoptions, sharingoptions, options, oplockbreakinghandler.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemHandleAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, HANDLE_ACCESS_OPTIONS, HANDLE_SHARING_OPTIONS, HANDLE_OPTIONS, *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Create: usize,
}
#[cfg(feature = "winnt")]
pub trait IStorageItemHandleAccess_Impl: windows_core::IUnknownImpl {
    fn Create(&self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: windows_core::Ref<IOplockBreakingHandler>) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(feature = "winnt")]
impl IStorageItemHandleAccess_Vtbl {
    pub const fn new<Identity: IStorageItemHandleAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IStorageItemHandleAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut core::ffi::c_void, interophandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemHandleAccess_Impl::Create(this, core::mem::transmute_copy(&accessoptions), core::mem::transmute_copy(&sharingoptions), core::mem::transmute_copy(&options), core::mem::transmute_copy(&oplockbreakinghandler)) {
                    Ok(ok__) => {
                        interophandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageItemHandleAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IStorageItemHandleAccess {}
windows_core::imp::define_interface!(IUnbufferedFileHandleOplockCallback, IUnbufferedFileHandleOplockCallback_Vtbl, 0xd1019a0e_6243_4329_8497_2e75894d7710);
windows_core::imp::interface_hierarchy!(IUnbufferedFileHandleOplockCallback, windows_core::IUnknown);
impl IUnbufferedFileHandleOplockCallback {
    pub unsafe fn OnBrokenCallback(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnBrokenCallback)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleOplockCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnBrokenCallback: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUnbufferedFileHandleOplockCallback_Impl: windows_core::IUnknownImpl {
    fn OnBrokenCallback(&self) -> windows_core::Result<()>;
}
impl IUnbufferedFileHandleOplockCallback_Vtbl {
    pub const fn new<Identity: IUnbufferedFileHandleOplockCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnBrokenCallback<Identity: IUnbufferedFileHandleOplockCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUnbufferedFileHandleOplockCallback_Impl::OnBrokenCallback(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnBrokenCallback: OnBrokenCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleOplockCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUnbufferedFileHandleOplockCallback {}
windows_core::imp::define_interface!(IUnbufferedFileHandleProvider, IUnbufferedFileHandleProvider_Vtbl, 0xa65c9109_42ab_4b94_a7b1_dd2e4e68515e);
windows_core::imp::interface_hierarchy!(IUnbufferedFileHandleProvider, windows_core::IUnknown);
impl IUnbufferedFileHandleProvider {
    pub unsafe fn OpenUnbufferedFileHandle<P0>(&self, oplockbreakcallback: P0) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<IUnbufferedFileHandleOplockCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenUnbufferedFileHandle)(windows_core::Interface::as_raw(self), oplockbreakcallback.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CloseUnbufferedFileHandle(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CloseUnbufferedFileHandle)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenUnbufferedFileHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub CloseUnbufferedFileHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUnbufferedFileHandleProvider_Impl: windows_core::IUnknownImpl {
    fn OpenUnbufferedFileHandle(&self, oplockbreakcallback: windows_core::Ref<IUnbufferedFileHandleOplockCallback>) -> windows_core::Result<usize>;
    fn CloseUnbufferedFileHandle(&self) -> windows_core::Result<()>;
}
impl IUnbufferedFileHandleProvider_Vtbl {
    pub const fn new<Identity: IUnbufferedFileHandleProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenUnbufferedFileHandle<Identity: IUnbufferedFileHandleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oplockbreakcallback: *mut core::ffi::c_void, filehandle: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUnbufferedFileHandleProvider_Impl::OpenUnbufferedFileHandle(this, core::mem::transmute_copy(&oplockbreakcallback)) {
                    Ok(ok__) => {
                        filehandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseUnbufferedFileHandle<Identity: IUnbufferedFileHandleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUnbufferedFileHandleProvider_Impl::CloseUnbufferedFileHandle(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenUnbufferedFileHandle: OpenUnbufferedFileHandle::<Identity, OFFSET>,
            CloseUnbufferedFileHandle: CloseUnbufferedFileHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnbufferedFileHandleProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUnbufferedFileHandleProvider {}
