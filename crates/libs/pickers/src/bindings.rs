windows_core::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *mut core::ffi::c_void));
windows_core::link!("shell32.dll" "system" fn SHCreateItemFromParsingName(pszpath : windows_core::PCWSTR, pbc : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
windows_core::link!("shell32.dll" "system" fn SHCreateItemInKnownFolder(kfid : *const KNOWNFOLDERID, dwkfflags : u32, pszitem : windows_core::PCWSTR, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMDLG_FILTERSPEC {
    pub pszName: windows_core::PCWSTR,
    pub pszSpec: windows_core::PCWSTR,
}
pub type FILEOPENDIALOGOPTIONS = u32;
pub const FOLDERID_ComputerFolder: windows_core::GUID =
    windows_core::GUID::from_u128(0x0ac0837c_bbf8_452a_850d_79d08e667ca7);
pub const FOLDERID_Desktop: windows_core::GUID =
    windows_core::GUID::from_u128(0xb4bfcc3a_db2c_424c_b029_7fe99a87c641);
pub const FOLDERID_DocumentsLibrary: windows_core::GUID =
    windows_core::GUID::from_u128(0x7b0db17d_9cd2_4a93_9733_46cc89022e7c);
pub const FOLDERID_Downloads: windows_core::GUID =
    windows_core::GUID::from_u128(0x374de290_123f_4565_9164_39c4925e467b);
pub const FOLDERID_MusicLibrary: windows_core::GUID =
    windows_core::GUID::from_u128(0x2112ab0a_c86a_4ffe_a368_0de96e47012e);
pub const FOLDERID_Objects3D: windows_core::GUID =
    windows_core::GUID::from_u128(0x31c0dd25_9439_4f12_bf41_7ff4eda38722);
pub const FOLDERID_PicturesLibrary: windows_core::GUID =
    windows_core::GUID::from_u128(0xa990ae9f_a03b_4e80_94bc_9912d7504104);
pub const FOLDERID_VideosLibrary: windows_core::GUID =
    windows_core::GUID::from_u128(0x491e922f_5643_4af4_a7eb_4e7a138d8174);
pub const FOS_ALLOWMULTISELECT: FILEOPENDIALOGOPTIONS = 512;
pub const FOS_FORCEFILESYSTEM: FILEOPENDIALOGOPTIONS = 64;
pub const FOS_OVERWRITEPROMPT: FILEOPENDIALOGOPTIONS = 2;
pub const FOS_PICKFOLDERS: FILEOPENDIALOGOPTIONS = 32;
pub const FOS_STRICTFILETYPES: FILEOPENDIALOGOPTIONS = 4;
pub const FileOpenDialog: windows_core::GUID =
    windows_core::GUID::from_u128(0xdc1c5a9c_e88a_4dde_a5a1_60f82a20aef7);
pub const FileSaveDialog: windows_core::GUID =
    windows_core::GUID::from_u128(0xc0b4e2f3_ba21_4773_8dba_335ec946eb8b);
pub type HWND = *mut core::ffi::c_void;
windows_core::imp::define_interface!(
    IBindCtx,
    IBindCtx_Vtbl,
    0x0000000e_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IBindCtx, windows_core::IUnknown);
#[repr(C)]
pub struct IBindCtx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    RegisterObjectBound: usize,
    RevokeObjectBound: usize,
    ReleaseBoundObjects: usize,
    SetBindOptions: usize,
    GetBindOptions: usize,
    GetRunningObjectTable: usize,
    RegisterObjectParam: usize,
    GetObjectParam: usize,
    EnumObjectParam: usize,
    RevokeObjectParam: usize,
}
impl windows_core::RuntimeName for IBindCtx {}
windows_core::imp::define_interface!(
    IFileDialog,
    IFileDialog_Vtbl,
    0x42f85136_db7e_439c_85f1_e4075d135fc8
);
impl core::ops::Deref for IFileDialog {
    type Target = IModalWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFileDialog, windows_core::IUnknown, IModalWindow);
impl IFileDialog {
    pub(crate) unsafe fn SetFileTypes(
        &self,
        cfiletypes: u32,
        rgfilterspec: *const COMDLG_FILTERSPEC,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFileTypes)(
                windows_core::Interface::as_raw(self),
                cfiletypes,
                rgfilterspec,
            )
        }
    }
    pub(crate) unsafe fn SetFileTypeIndex(&self, ifiletype: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFileTypeIndex)(
                windows_core::Interface::as_raw(self),
                ifiletype,
            )
        }
    }
    pub(crate) unsafe fn SetOptions(&self, fos: FILEOPENDIALOGOPTIONS) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetOptions)(
                windows_core::Interface::as_raw(self),
                fos,
            )
        }
    }
    pub(crate) unsafe fn GetOptions(&self) -> windows_core::Result<FILEOPENDIALOGOPTIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetDefaultFolder<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IShellItem>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultFolder)(
                windows_core::Interface::as_raw(self),
                psi.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetFolder<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IShellItem>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFolder)(
                windows_core::Interface::as_raw(self),
                psi.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetFileName<P0>(&self, pszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFileName)(
                windows_core::Interface::as_raw(self),
                pszname.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetTitle<P0>(&self, psztitle: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                psztitle.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetOkButtonLabel<P0>(&self, psztext: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetOkButtonLabel)(
                windows_core::Interface::as_raw(self),
                psztext.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn GetResult(&self) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResult)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn SetDefaultExtension<P0>(
        &self,
        pszdefaultextension: P0,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultExtension)(
                windows_core::Interface::as_raw(self),
                pszdefaultextension.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetClientGuid(
        &self,
        guid: *const windows_core::GUID,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetClientGuid)(
                windows_core::Interface::as_raw(self),
                guid,
            )
        }
    }
}
#[repr(C)]
pub struct IFileDialog_Vtbl {
    pub base__: IModalWindow_Vtbl,
    pub SetFileTypes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const COMDLG_FILTERSPEC,
    ) -> windows_core::HRESULT,
    pub SetFileTypeIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    GetFileTypeIndex: usize,
    Advise: usize,
    Unadvise: usize,
    pub SetOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FILEOPENDIALOGOPTIONS,
    ) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FILEOPENDIALOGOPTIONS,
    ) -> windows_core::HRESULT,
    pub SetDefaultFolder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetFolder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetFolder: usize,
    GetCurrentSelection: usize,
    pub SetFileName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    GetFileName: usize,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SetOkButtonLabel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    SetFileNameLabel: usize,
    pub GetResult: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    AddPlace: usize,
    pub SetDefaultExtension: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    Close: usize,
    pub SetClientGuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
    ) -> windows_core::HRESULT,
    ClearClientData: usize,
    SetFilter: usize,
}
impl windows_core::RuntimeName for IFileDialog {}
windows_core::imp::define_interface!(
    IFileOpenDialog,
    IFileOpenDialog_Vtbl,
    0xd57c7288_d4ad_4768_be02_9d969532d960
);
impl core::ops::Deref for IFileOpenDialog {
    type Target = IFileDialog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IFileOpenDialog,
    windows_core::IUnknown,
    IModalWindow,
    IFileDialog
);
impl IFileOpenDialog {
    pub(crate) unsafe fn GetResults(&self) -> windows_core::Result<IShellItemArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResults)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IFileOpenDialog_Vtbl {
    pub base__: IFileDialog_Vtbl,
    pub GetResults: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetSelectedItems: usize,
}
impl windows_core::RuntimeName for IFileOpenDialog {}
windows_core::imp::define_interface!(
    IFileSaveDialog,
    IFileSaveDialog_Vtbl,
    0x84bccd23_5fde_4cdb_aea4_af64b83d78ab
);
impl core::ops::Deref for IFileSaveDialog {
    type Target = IFileDialog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IFileSaveDialog,
    windows_core::IUnknown,
    IModalWindow,
    IFileDialog
);
#[repr(C)]
pub struct IFileSaveDialog_Vtbl {
    pub base__: IFileDialog_Vtbl,
    SetSaveAsItem: usize,
    SetProperties: usize,
    SetCollectedProperties: usize,
    GetProperties: usize,
    ApplyProperties: usize,
}
impl windows_core::RuntimeName for IFileSaveDialog {}
windows_core::imp::define_interface!(
    IModalWindow,
    IModalWindow_Vtbl,
    0xb4db1657_70d7_485e_8e3e_6fcb5a5c1802
);
windows_core::imp::interface_hierarchy!(IModalWindow, windows_core::IUnknown);
impl IModalWindow {
    pub(crate) unsafe fn Show(&self, hwndowner: Option<HWND>) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Show)(
                windows_core::Interface::as_raw(self),
                hwndowner.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
}
#[repr(C)]
pub struct IModalWindow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, HWND) -> windows_core::HRESULT,
}
pub trait IModalWindow_Impl: windows_core::IUnknownImpl {
    fn Show(&self, hwndowner: HWND) -> windows_core::Result<()>;
}
impl IModalWindow_Vtbl {
    pub const fn new<Identity: IModalWindow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: IModalWindow_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            hwndowner: HWND,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModalWindow_Impl::Show(this, core::mem::transmute_copy(&hwndowner)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Show: Show::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IModalWindow as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IModalWindow {}
windows_core::imp::define_interface!(
    IShellItem,
    IShellItem_Vtbl,
    0x43826d1e_e718_42ee_bc55_a1e261c37bfe
);
windows_core::imp::interface_hierarchy!(IShellItem, windows_core::IUnknown);
impl IShellItem {
    pub(crate) unsafe fn GetDisplayName(
        &self,
        sigdnname: SIGDN,
    ) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(
                windows_core::Interface::as_raw(self),
                sigdnname,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IShellItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    BindToHandler: usize,
    GetParent: usize,
    pub GetDisplayName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SIGDN,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
    GetAttributes: usize,
    Compare: usize,
}
impl windows_core::RuntimeName for IShellItem {}
windows_core::imp::define_interface!(
    IShellItemArray,
    IShellItemArray_Vtbl,
    0xb63ea76d_1f85_456f_a19c_48159efa858b
);
windows_core::imp::interface_hierarchy!(IShellItemArray, windows_core::IUnknown);
impl IShellItemArray {
    pub(crate) unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetItemAt(&self, dwindex: u32) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemAt)(
                windows_core::Interface::as_raw(self),
                dwindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IShellItemArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    BindToHandler: usize,
    GetPropertyStore: usize,
    GetPropertyDescriptionList: usize,
    GetAttributes: usize,
    pub GetCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    EnumItems: usize,
}
impl windows_core::RuntimeName for IShellItemArray {}
pub type KNOWNFOLDERID = windows_core::GUID;
pub type SIGDN = i32;
pub const SIGDN_FILESYSPATH: SIGDN = -2147123200;
