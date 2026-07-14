#[cfg(feature = "propsys")]
#[inline]
pub unsafe fn SHAddDefaultPropertiesByExt<P0, P1>(pszext: P0, ppropstore: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::propsys::IPropertyStore>,
{
    windows_core::link!("shell32.dll" "system" fn SHAddDefaultPropertiesByExt(pszext : windows_core::PCWSTR, ppropstore : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SHAddDefaultPropertiesByExt(pszext.param().abi(), ppropstore.param().abi()) }
}
#[cfg(feature = "shobjidl_core")]
#[inline]
pub unsafe fn SHCreateDefaultPropertiesOp<P0>(psi: P0) -> windows_core::Result<super::shobjidl_core::IFileOperation>
where
    P0: windows_core::Param<super::shobjidl_core::IShellItem>,
{
    windows_core::link!("shell32.dll" "system" fn SHCreateDefaultPropertiesOp(psi : *mut core::ffi::c_void, ppfileop : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCreateDefaultPropertiesOp(psi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
#[inline]
pub unsafe fn SHSetDefaultProperties<P1, P3>(hwnd: Option<super::windef::HWND>, psi: P1, dwfileopflags: u32, pfops: P3) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::shobjidl_core::IShellItem>,
    P3: windows_core::Param<super::shobjidl_core::IFileOperationProgressSink>,
{
    windows_core::link!("shell32.dll" "system" fn SHSetDefaultProperties(hwnd : super::windef::HWND, psi : *mut core::ffi::c_void, dwfileopflags : u32, pfops : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SHSetDefaultProperties(hwnd.unwrap_or(core::mem::zeroed()) as _, psi.param().abi(), dwfileopflags, pfops.param().abi()) }
}
pub const ACDD_VISIBLE: u32 = 1;
pub const AccessibilityDockingService: windows_core::GUID = windows_core::GUID::from_u128(0x29ce1d46_b481_4aa0_a08a_d3ebc8aca402);
pub const AlphabeticalCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0x3c2654c6_7372_4f6b_b310_55d6128f49d2);
pub const ApplicationAssociationRegistrationUI: windows_core::GUID = windows_core::GUID::from_u128(0x1968106d_f3b5_44cf_890e_116fcb9ecef1);
pub const AttachmentServices: windows_core::GUID = windows_core::GUID::from_u128(0x4125dd96_e03a_4103_8f70_e0597d803b9c);
pub type CDBE_ACTIONS = u32;
pub const CDBE_RET_DEFAULT: tagCDBURNINGEXTENSIONRET = 0;
pub const CDBE_RET_DONTRUNOTHEREXTS: tagCDBURNINGEXTENSIONRET = 1;
pub const CDBE_RET_STOPWIZARD: tagCDBURNINGEXTENSIONRET = 2;
pub const CDBE_TYPE_ALL: CDBE_ACTIONS = 4294967295;
pub const CDBE_TYPE_DATA: CDBE_ACTIONS = 2;
pub const CDBE_TYPE_MUSIC: CDBE_ACTIONS = 1;
pub const CDBurn: windows_core::GUID = windows_core::GUID::from_u128(0xfbeb8a05_beee_4442_804e_409d6c4515e9);
pub const DSH_ALLOWDROPDESCRIPTIONTEXT: DSH_FLAGS = 1;
pub type DSH_FLAGS = u32;
pub const DesktopGadget: windows_core::GUID = windows_core::GUID::from_u128(0x924ccc1b_6562_4c85_8657_d177925222b6);
pub const DocPropShellExtension: windows_core::GUID = windows_core::GUID::from_u128(0x883373c3_bf89_11d1_be35_080036b11a03);
pub const ExecuteFolder: windows_core::GUID = windows_core::GUID::from_u128(0x11dbb47c_a525_400b_9e80_a54615a090c0);
pub const ExplorerBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x71f96385_ddd6_48d3_a0c1_ae06e8b055fb);
pub type FOLDERVIEWOPTIONS = u32;
pub const FSCopyHandler: windows_core::GUID = windows_core::GUID::from_u128(0xd197380a_0a79_4dc8_a033_ed882c2fa14b);
pub const FVO_CUSTOMORDERING: FOLDERVIEWOPTIONS = 4;
pub const FVO_CUSTOMPOSITION: FOLDERVIEWOPTIONS = 2;
pub const FVO_DEFAULT: FOLDERVIEWOPTIONS = 0;
pub const FVO_NOANIMATIONS: FOLDERVIEWOPTIONS = 16;
pub const FVO_NOSCROLLTIPS: FOLDERVIEWOPTIONS = 32;
pub const FVO_SUPPORTHYPERLINKS: FOLDERVIEWOPTIONS = 8;
pub const FVO_VISTALAYOUT: FOLDERVIEWOPTIONS = 1;
pub const FolderViewHost: windows_core::GUID = windows_core::GUID::from_u128(0x20b1cb23_6968_4eb9_b7d4_a66d00d07cee);
windows_core::imp::define_interface!(IAccessibilityDockingService, IAccessibilityDockingService_Vtbl, 0x8849dc22_cedf_4c95_998d_051419dd3f76);
windows_core::imp::interface_hierarchy!(IAccessibilityDockingService, windows_core::IUnknown);
impl IAccessibilityDockingService {
    #[cfg(feature = "windef")]
    pub unsafe fn GetAvailableSize(&self, hmonitor: super::windef::HMONITOR, pcxfixed: *mut u32, pcymax: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableSize)(windows_core::Interface::as_raw(self), hmonitor, pcxfixed as _, pcymax as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn DockWindow<P3>(&self, hwnd: super::windef::HWND, hmonitor: super::windef::HMONITOR, cyrequested: u32, pcallback: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IAccessibilityDockingServiceCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).DockWindow)(windows_core::Interface::as_raw(self), hwnd, hmonitor, cyrequested, pcallback.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn UndockWindow(&self, hwnd: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UndockWindow)(windows_core::Interface::as_raw(self), hwnd) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibilityDockingService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetAvailableSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HMONITOR, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetAvailableSize: usize,
    #[cfg(feature = "windef")]
    pub DockWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, super::windef::HMONITOR, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    DockWindow: usize,
    #[cfg(feature = "windef")]
    pub UndockWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    UndockWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IAccessibilityDockingService_Impl: windows_core::IUnknownImpl {
    fn GetAvailableSize(&self, hmonitor: super::windef::HMONITOR, pcxfixed: *mut u32, pcymax: *mut u32) -> windows_core::Result<()>;
    fn DockWindow(&self, hwnd: super::windef::HWND, hmonitor: super::windef::HMONITOR, cyrequested: u32, pcallback: windows_core::Ref<IAccessibilityDockingServiceCallback>) -> windows_core::Result<()>;
    fn UndockWindow(&self, hwnd: super::windef::HWND) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IAccessibilityDockingService_Vtbl {
    pub const fn new<Identity: IAccessibilityDockingService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailableSize<Identity: IAccessibilityDockingService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmonitor: super::windef::HMONITOR, pcxfixed: *mut u32, pcymax: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessibilityDockingService_Impl::GetAvailableSize(this, core::mem::transmute_copy(&hmonitor), core::mem::transmute_copy(&pcxfixed), core::mem::transmute_copy(&pcymax)).into()
            }
        }
        unsafe extern "system" fn DockWindow<Identity: IAccessibilityDockingService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, hmonitor: super::windef::HMONITOR, cyrequested: u32, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessibilityDockingService_Impl::DockWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&hmonitor), core::mem::transmute_copy(&cyrequested), core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn UndockWindow<Identity: IAccessibilityDockingService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessibilityDockingService_Impl::UndockWindow(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableSize: GetAvailableSize::<Identity, OFFSET>,
            DockWindow: DockWindow::<Identity, OFFSET>,
            UndockWindow: UndockWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessibilityDockingService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IAccessibilityDockingService {}
windows_core::imp::define_interface!(IAccessibilityDockingServiceCallback, IAccessibilityDockingServiceCallback_Vtbl, 0x157733fd_a592_42e5_b594_248468c5a81b);
windows_core::imp::interface_hierarchy!(IAccessibilityDockingServiceCallback, windows_core::IUnknown);
impl IAccessibilityDockingServiceCallback {
    pub unsafe fn Undocked(&self, undockreason: UNDOCK_REASON) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Undocked)(windows_core::Interface::as_raw(self), undockreason) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibilityDockingServiceCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Undocked: unsafe extern "system" fn(*mut core::ffi::c_void, UNDOCK_REASON) -> windows_core::HRESULT,
}
pub trait IAccessibilityDockingServiceCallback_Impl: windows_core::IUnknownImpl {
    fn Undocked(&self, undockreason: UNDOCK_REASON) -> windows_core::Result<()>;
}
impl IAccessibilityDockingServiceCallback_Vtbl {
    pub const fn new<Identity: IAccessibilityDockingServiceCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Undocked<Identity: IAccessibilityDockingServiceCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, undockreason: UNDOCK_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessibilityDockingServiceCallback_Impl::Undocked(this, core::mem::transmute_copy(&undockreason)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Undocked: Undocked::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessibilityDockingServiceCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAccessibilityDockingServiceCallback {}
windows_core::imp::define_interface!(IAccessibleObject, IAccessibleObject_Vtbl, 0x95a391c5_9ed4_4c28_8401_ab9e06719e11);
windows_core::imp::interface_hierarchy!(IAccessibleObject, windows_core::IUnknown);
impl IAccessibleObject {
    pub unsafe fn SetAccessibleName<P0>(&self, pszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccessibleName)(windows_core::Interface::as_raw(self), pszname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAccessibleName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IAccessibleObject_Impl: windows_core::IUnknownImpl {
    fn SetAccessibleName(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IAccessibleObject_Vtbl {
    pub const fn new<Identity: IAccessibleObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAccessibleName<Identity: IAccessibleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessibleObject_Impl::SetAccessibleName(this, core::mem::transmute(&pszname)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetAccessibleName: SetAccessibleName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessibleObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAccessibleObject {}
windows_core::imp::define_interface!(IApplicationAssociationRegistrationUI, IApplicationAssociationRegistrationUI_Vtbl, 0x1f76a169_f994_40ac_8fc8_0959e8874710);
windows_core::imp::interface_hierarchy!(IApplicationAssociationRegistrationUI, windows_core::IUnknown);
impl IApplicationAssociationRegistrationUI {
    pub unsafe fn LaunchAdvancedAssociationUI<P0>(&self, pszappregistryname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LaunchAdvancedAssociationUI)(windows_core::Interface::as_raw(self), pszappregistryname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationAssociationRegistrationUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LaunchAdvancedAssociationUI: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IApplicationAssociationRegistrationUI_Impl: windows_core::IUnknownImpl {
    fn LaunchAdvancedAssociationUI(&self, pszappregistryname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IApplicationAssociationRegistrationUI_Vtbl {
    pub const fn new<Identity: IApplicationAssociationRegistrationUI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LaunchAdvancedAssociationUI<Identity: IApplicationAssociationRegistrationUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszappregistryname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationAssociationRegistrationUI_Impl::LaunchAdvancedAssociationUI(this, core::mem::transmute(&pszappregistryname)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LaunchAdvancedAssociationUI: LaunchAdvancedAssociationUI::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationAssociationRegistrationUI as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IApplicationAssociationRegistrationUI {}
windows_core::imp::define_interface!(IAutoCompleteDropDown, IAutoCompleteDropDown_Vtbl, 0x3cd141f4_3c6a_11d2_bcaa_00c04fd929db);
windows_core::imp::interface_hierarchy!(IAutoCompleteDropDown, windows_core::IUnknown);
impl IAutoCompleteDropDown {
    pub unsafe fn GetDropDownStatus(&self, pdwflags: *mut u32, ppwszstring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDropDownStatus)(windows_core::Interface::as_raw(self), pdwflags as _, ppwszstring as _) }
    }
    pub unsafe fn ResetEnumerator(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetEnumerator)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoCompleteDropDown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDropDownStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub ResetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAutoCompleteDropDown_Impl: windows_core::IUnknownImpl {
    fn GetDropDownStatus(&self, pdwflags: *mut u32, ppwszstring: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn ResetEnumerator(&self) -> windows_core::Result<()>;
}
impl IAutoCompleteDropDown_Vtbl {
    pub const fn new<Identity: IAutoCompleteDropDown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDropDownStatus<Identity: IAutoCompleteDropDown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32, ppwszstring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutoCompleteDropDown_Impl::GetDropDownStatus(this, core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&ppwszstring)).into()
            }
        }
        unsafe extern "system" fn ResetEnumerator<Identity: IAutoCompleteDropDown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutoCompleteDropDown_Impl::ResetEnumerator(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDropDownStatus: GetDropDownStatus::<Identity, OFFSET>,
            ResetEnumerator: ResetEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutoCompleteDropDown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAutoCompleteDropDown {}
windows_core::imp::define_interface!(IBandHost, IBandHost_Vtbl, 0xb9075c7c_d48e_403f_ab99_d6c77a1084ac);
windows_core::imp::interface_hierarchy!(IBandHost, windows_core::IUnknown);
impl IBandHost {
    pub unsafe fn CreateBand<T>(&self, rclsidband: *const windows_core::GUID, favailable: bool, fvisible: bool) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateBand)(windows_core::Interface::as_raw(self), rclsidband, favailable.into(), fvisible.into(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn SetBandAvailability(&self, rclsidband: *const windows_core::GUID, favailable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBandAvailability)(windows_core::Interface::as_raw(self), rclsidband, favailable.into()) }
    }
    pub unsafe fn DestroyBand(&self, rclsidband: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DestroyBand)(windows_core::Interface::as_raw(self), rclsidband) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBandHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateBand: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::BOOL, windows_core::BOOL, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBandAvailability: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub DestroyBand: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IBandHost_Impl: windows_core::IUnknownImpl {
    fn CreateBand(&self, rclsidband: *const windows_core::GUID, favailable: windows_core::BOOL, fvisible: windows_core::BOOL, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetBandAvailability(&self, rclsidband: *const windows_core::GUID, favailable: windows_core::BOOL) -> windows_core::Result<()>;
    fn DestroyBand(&self, rclsidband: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IBandHost_Vtbl {
    pub const fn new<Identity: IBandHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBand<Identity: IBandHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsidband: *const windows_core::GUID, favailable: windows_core::BOOL, fvisible: windows_core::BOOL, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBandHost_Impl::CreateBand(this, core::mem::transmute_copy(&rclsidband), core::mem::transmute_copy(&favailable), core::mem::transmute_copy(&fvisible), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn SetBandAvailability<Identity: IBandHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsidband: *const windows_core::GUID, favailable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBandHost_Impl::SetBandAvailability(this, core::mem::transmute_copy(&rclsidband), core::mem::transmute_copy(&favailable)).into()
            }
        }
        unsafe extern "system" fn DestroyBand<Identity: IBandHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsidband: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBandHost_Impl::DestroyBand(this, core::mem::transmute_copy(&rclsidband)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBand: CreateBand::<Identity, OFFSET>,
            SetBandAvailability: SetBandAvailability::<Identity, OFFSET>,
            DestroyBand: DestroyBand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBandHost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBandHost {}
windows_core::imp::define_interface!(ICDBurn, ICDBurn_Vtbl, 0x3d73a659_e5d0_4d42_afc0_5121ba425c8d);
windows_core::imp::interface_hierarchy!(ICDBurn, windows_core::IUnknown);
impl ICDBurn {
    pub unsafe fn GetRecorderDriveLetter(&self, pszdrive: windows_core::PWSTR, cch: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRecorderDriveLetter)(windows_core::Interface::as_raw(self), pszdrive, cch) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Burn(&self, hwnd: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Burn)(windows_core::Interface::as_raw(self), hwnd) }
    }
    pub unsafe fn HasRecordableDrive(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasRecordableDrive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICDBurn_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRecorderDriveLetter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Burn: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Burn: usize,
    pub HasRecordableDrive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait ICDBurn_Impl: windows_core::IUnknownImpl {
    fn GetRecorderDriveLetter(&self, pszdrive: windows_core::PWSTR, cch: u32) -> windows_core::Result<()>;
    fn Burn(&self, hwnd: super::windef::HWND) -> windows_core::Result<()>;
    fn HasRecordableDrive(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "windef")]
impl ICDBurn_Vtbl {
    pub const fn new<Identity: ICDBurn_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRecorderDriveLetter<Identity: ICDBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdrive: windows_core::PWSTR, cch: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICDBurn_Impl::GetRecorderDriveLetter(this, core::mem::transmute_copy(&pszdrive), core::mem::transmute_copy(&cch)).into()
            }
        }
        unsafe extern "system" fn Burn<Identity: ICDBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICDBurn_Impl::Burn(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn HasRecordableDrive<Identity: ICDBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasrecorder: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICDBurn_Impl::HasRecordableDrive(this) {
                    Ok(ok__) => {
                        pfhasrecorder.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRecorderDriveLetter: GetRecorderDriveLetter::<Identity, OFFSET>,
            Burn: Burn::<Identity, OFFSET>,
            HasRecordableDrive: HasRecordableDrive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICDBurn as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ICDBurn {}
windows_core::imp::define_interface!(ICDBurnExt, ICDBurnExt_Vtbl, 0x2271dcca_74fc_4414_8fb7_c56b05ace2d7);
windows_core::imp::interface_hierarchy!(ICDBurnExt, windows_core::IUnknown);
impl ICDBurnExt {
    pub unsafe fn GetSupportedActionTypes(&self) -> windows_core::Result<CDBE_ACTIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedActionTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICDBurnExt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSupportedActionTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CDBE_ACTIONS) -> windows_core::HRESULT,
}
pub trait ICDBurnExt_Impl: windows_core::IUnknownImpl {
    fn GetSupportedActionTypes(&self) -> windows_core::Result<CDBE_ACTIONS>;
}
impl ICDBurnExt_Vtbl {
    pub const fn new<Identity: ICDBurnExt_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedActionTypes<Identity: ICDBurnExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwactions: *mut CDBE_ACTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICDBurnExt_Impl::GetSupportedActionTypes(this) {
                    Ok(ok__) => {
                        pdwactions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSupportedActionTypes: GetSupportedActionTypes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICDBurnExt as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICDBurnExt {}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::define_interface!(ICommDlgBrowser3, ICommDlgBrowser3_Vtbl, 0xc8ad25a1_3294_41ee_8165_71174bd01c57);
#[cfg(feature = "shobjidl_core")]
impl core::ops::Deref for ICommDlgBrowser3 {
    type Target = super::shobjidl_core::ICommDlgBrowser2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::interface_hierarchy!(ICommDlgBrowser3, windows_core::IUnknown, super::shobjidl_core::ICommDlgBrowser, super::shobjidl_core::ICommDlgBrowser2);
#[cfg(feature = "shobjidl_core")]
impl ICommDlgBrowser3 {
    #[cfg(feature = "oleidl")]
    pub unsafe fn OnColumnClicked<P0>(&self, ppshv: P0, icolumn: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellView>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnColumnClicked)(windows_core::Interface::as_raw(self), ppshv.param().abi(), icolumn) }
    }
    pub unsafe fn GetCurrentFilter(&self, pszfilespec: windows_core::PWSTR, cchfilespec: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentFilter)(windows_core::Interface::as_raw(self), pszfilespec, cchfilespec) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn OnPreViewCreated<P0>(&self, ppshv: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellView>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPreViewCreated)(windows_core::Interface::as_raw(self), ppshv.param().abi()) }
    }
}
#[cfg(feature = "shobjidl_core")]
#[repr(C)]
#[doc(hidden)]
pub struct ICommDlgBrowser3_Vtbl {
    pub base__: super::shobjidl_core::ICommDlgBrowser2_Vtbl,
    #[cfg(feature = "oleidl")]
    pub OnColumnClicked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    OnColumnClicked: usize,
    pub GetCurrentFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    #[cfg(feature = "oleidl")]
    pub OnPreViewCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    OnPreViewCreated: usize,
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "shtypes"))]
pub trait ICommDlgBrowser3_Impl: super::shobjidl_core::ICommDlgBrowser2_Impl {
    fn OnColumnClicked(&self, ppshv: windows_core::Ref<super::shobjidl_core::IShellView>, icolumn: i32) -> windows_core::Result<()>;
    fn GetCurrentFilter(&self, pszfilespec: windows_core::PWSTR, cchfilespec: i32) -> windows_core::Result<()>;
    fn OnPreViewCreated(&self, ppshv: windows_core::Ref<super::shobjidl_core::IShellView>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "shtypes"))]
impl ICommDlgBrowser3_Vtbl {
    pub const fn new<Identity: ICommDlgBrowser3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnColumnClicked<Identity: ICommDlgBrowser3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshv: *mut core::ffi::c_void, icolumn: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommDlgBrowser3_Impl::OnColumnClicked(this, core::mem::transmute_copy(&ppshv), core::mem::transmute_copy(&icolumn)).into()
            }
        }
        unsafe extern "system" fn GetCurrentFilter<Identity: ICommDlgBrowser3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilespec: windows_core::PWSTR, cchfilespec: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommDlgBrowser3_Impl::GetCurrentFilter(this, core::mem::transmute_copy(&pszfilespec), core::mem::transmute_copy(&cchfilespec)).into()
            }
        }
        unsafe extern "system" fn OnPreViewCreated<Identity: ICommDlgBrowser3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommDlgBrowser3_Impl::OnPreViewCreated(this, core::mem::transmute_copy(&ppshv)).into()
            }
        }
        Self {
            base__: super::shobjidl_core::ICommDlgBrowser2_Vtbl::new::<Identity, OFFSET>(),
            OnColumnClicked: OnColumnClicked::<Identity, OFFSET>,
            GetCurrentFilter: GetCurrentFilter::<Identity, OFFSET>,
            OnPreViewCreated: OnPreViewCreated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommDlgBrowser3 as windows_core::Interface>::IID || iid == &<super::shobjidl_core::ICommDlgBrowser as windows_core::Interface>::IID || iid == &<super::shobjidl_core::ICommDlgBrowser2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "shtypes"))]
impl windows_core::RuntimeName for ICommDlgBrowser3 {}
windows_core::imp::define_interface!(IComputerInfoChangeNotify, IComputerInfoChangeNotify_Vtbl, 0x0df60d92_6818_46d6_b358_d66170dde466);
windows_core::imp::interface_hierarchy!(IComputerInfoChangeNotify, windows_core::IUnknown);
impl IComputerInfoChangeNotify {
    pub unsafe fn ComputerInfoChanged(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ComputerInfoChanged)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComputerInfoChangeNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ComputerInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IComputerInfoChangeNotify_Impl: windows_core::IUnknownImpl {
    fn ComputerInfoChanged(&self) -> windows_core::Result<()>;
}
impl IComputerInfoChangeNotify_Vtbl {
    pub const fn new<Identity: IComputerInfoChangeNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ComputerInfoChanged<Identity: IComputerInfoChangeNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComputerInfoChangeNotify_Impl::ComputerInfoChanged(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ComputerInfoChanged: ComputerInfoChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComputerInfoChangeNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComputerInfoChangeNotify {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IControlMarkup(pub u8);
pub const IDD_WIZEXTN_FIRST: u32 = 20480;
pub const IDD_WIZEXTN_LAST: u32 = 20736;
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
windows_core::imp::define_interface!(IDeskBand2, IDeskBand2_Vtbl, 0x79d16de4_abee_4021_8d9d_9169b261d657);
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
impl core::ops::Deref for IDeskBand2 {
    type Target = super::shobjidl_core::IDeskBand;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
windows_core::imp::interface_hierarchy!(IDeskBand2, windows_core::IUnknown, super::oleidl::IOleWindow, super::shobjidl_core::IDockingWindow, super::shobjidl_core::IDeskBand);
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
impl IDeskBand2 {
    pub unsafe fn CanRenderComposited(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRenderComposited)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCompositionState(&self, fcompositionenabled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionState)(windows_core::Interface::as_raw(self), fcompositionenabled.into()) }
    }
    pub unsafe fn GetCompositionState(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCompositionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDeskBand2_Vtbl {
    pub base__: super::shobjidl_core::IDeskBand_Vtbl,
    pub CanRenderComposited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCompositionState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCompositionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
pub trait IDeskBand2_Impl: super::shobjidl_core::IDeskBand_Impl {
    fn CanRenderComposited(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetCompositionState(&self, fcompositionenabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetCompositionState(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
impl IDeskBand2_Vtbl {
    pub const fn new<Identity: IDeskBand2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanRenderComposited<Identity: IDeskBand2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcanrendercomposited: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeskBand2_Impl::CanRenderComposited(this) {
                    Ok(ok__) => {
                        pfcanrendercomposited.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompositionState<Identity: IDeskBand2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcompositionenabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeskBand2_Impl::SetCompositionState(this, core::mem::transmute_copy(&fcompositionenabled)).into()
            }
        }
        unsafe extern "system" fn GetCompositionState<Identity: IDeskBand2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcompositionenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeskBand2_Impl::GetCompositionState(this) {
                    Ok(ok__) => {
                        pfcompositionenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::shobjidl_core::IDeskBand_Vtbl::new::<Identity, OFFSET>(),
            CanRenderComposited: CanRenderComposited::<Identity, OFFSET>,
            SetCompositionState: SetCompositionState::<Identity, OFFSET>,
            GetCompositionState: GetCompositionState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeskBand2 as windows_core::Interface>::IID || iid == &<super::oleidl::IOleWindow as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IDockingWindow as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IDeskBand as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
impl windows_core::RuntimeName for IDeskBand2 {}
windows_core::imp::define_interface!(IDesktopGadget, IDesktopGadget_Vtbl, 0xc1646bc4_f298_4f91_a204_eb2dd1709d1a);
windows_core::imp::interface_hierarchy!(IDesktopGadget, windows_core::IUnknown);
impl IDesktopGadget {
    pub unsafe fn RunGadget<P0>(&self, gadgetpath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RunGadget)(windows_core::Interface::as_raw(self), gadgetpath.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopGadget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RunGadget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IDesktopGadget_Impl: windows_core::IUnknownImpl {
    fn RunGadget(&self, gadgetpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IDesktopGadget_Vtbl {
    pub const fn new<Identity: IDesktopGadget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RunGadget<Identity: IDesktopGadget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gadgetpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDesktopGadget_Impl::RunGadget(this, core::mem::transmute(&gadgetpath)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RunGadget: RunGadget::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopGadget as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDesktopGadget {}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::define_interface!(IDragSourceHelper2, IDragSourceHelper2_Vtbl, 0x83e07d0d_0c5f_4163_bf1a_60b274051e40);
#[cfg(feature = "shobjidl_core")]
impl core::ops::Deref for IDragSourceHelper2 {
    type Target = super::shobjidl_core::IDragSourceHelper;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::interface_hierarchy!(IDragSourceHelper2, windows_core::IUnknown, super::shobjidl_core::IDragSourceHelper);
#[cfg(feature = "shobjidl_core")]
impl IDragSourceHelper2 {
    pub unsafe fn SetFlags(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), dwflags) }
    }
}
#[cfg(feature = "shobjidl_core")]
#[repr(C)]
#[doc(hidden)]
pub struct IDragSourceHelper2_Vtbl {
    pub base__: super::shobjidl_core::IDragSourceHelper_Vtbl,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidl", feature = "shobjidl_core", feature = "windef"))]
pub trait IDragSourceHelper2_Impl: super::shobjidl_core::IDragSourceHelper_Impl {
    fn SetFlags(&self, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "shobjidl_core", feature = "windef"))]
impl IDragSourceHelper2_Vtbl {
    pub const fn new<Identity: IDragSourceHelper2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFlags<Identity: IDragSourceHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDragSourceHelper2_Impl::SetFlags(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self { base__: super::shobjidl_core::IDragSourceHelper_Vtbl::new::<Identity, OFFSET>(), SetFlags: SetFlags::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDragSourceHelper2 as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IDragSourceHelper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "shobjidl_core", feature = "windef"))]
impl windows_core::RuntimeName for IDragSourceHelper2 {}
windows_core::imp::define_interface!(IDynamicHWHandler, IDynamicHWHandler_Vtbl, 0xdc2601d7_059e_42fc_a09d_2afd21b6d5f7);
windows_core::imp::interface_hierarchy!(IDynamicHWHandler, windows_core::IUnknown);
impl IDynamicHWHandler {
    pub unsafe fn GetDynamicInfo<P0>(&self, pszdeviceid: P0, dwcontenttype: u32) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDynamicInfo)(windows_core::Interface::as_raw(self), pszdeviceid.param().abi(), dwcontenttype, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicHWHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDynamicInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IDynamicHWHandler_Impl: windows_core::IUnknownImpl {
    fn GetDynamicInfo(&self, pszdeviceid: &windows_core::PCWSTR, dwcontenttype: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl IDynamicHWHandler_Vtbl {
    pub const fn new<Identity: IDynamicHWHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDynamicInfo<Identity: IDynamicHWHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdeviceid: windows_core::PCWSTR, dwcontenttype: u32, ppszaction: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicHWHandler_Impl::GetDynamicInfo(this, core::mem::transmute(&pszdeviceid), core::mem::transmute_copy(&dwcontenttype)) {
                    Ok(ok__) => {
                        ppszaction.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDynamicInfo: GetDynamicInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDynamicHWHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDynamicHWHandler {}
pub const IENamespaceTreeControl: windows_core::GUID = windows_core::GUID::from_u128(0xace52d03_e5cd_4b20_82ff_e71b11beae1d);
windows_core::imp::define_interface!(IEnumReadyCallback, IEnumReadyCallback_Vtbl, 0x61e00d45_8fff_4e60_924e_6537b61612dd);
windows_core::imp::interface_hierarchy!(IEnumReadyCallback, windows_core::IUnknown);
impl IEnumReadyCallback {
    pub unsafe fn EnumReady(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumReady)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumReadyCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumReadyCallback_Impl: windows_core::IUnknownImpl {
    fn EnumReady(&self) -> windows_core::Result<()>;
}
impl IEnumReadyCallback_Vtbl {
    pub const fn new<Identity: IEnumReadyCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumReady<Identity: IEnumReadyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumReadyCallback_Impl::EnumReady(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumReady: EnumReady::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumReadyCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumReadyCallback {}
windows_core::imp::define_interface!(IEnumerableView, IEnumerableView_Vtbl, 0x8c8bf236_1aec_495f_9894_91d57c3c686f);
windows_core::imp::interface_hierarchy!(IEnumerableView, windows_core::IUnknown);
impl IEnumerableView {
    pub unsafe fn SetEnumReadyCallback<P0>(&self, percb: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IEnumReadyCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEnumReadyCallback)(windows_core::Interface::as_raw(self), percb.param().abi()) }
    }
    #[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
    pub unsafe fn CreateEnumIDListFromContents(&self, pidlfolder: *const super::shtypes::ITEMIDLIST, dwenumflags: u32) -> windows_core::Result<super::shobjidl_core::IEnumIDList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEnumIDListFromContents)(windows_core::Interface::as_raw(self), pidlfolder, dwenumflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumerableView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetEnumReadyCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
    pub CreateEnumIDListFromContents: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shobjidl_core", feature = "shtypes")))]
    CreateEnumIDListFromContents: usize,
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
pub trait IEnumerableView_Impl: windows_core::IUnknownImpl {
    fn SetEnumReadyCallback(&self, percb: windows_core::Ref<IEnumReadyCallback>) -> windows_core::Result<()>;
    fn CreateEnumIDListFromContents(&self, pidlfolder: *const super::shtypes::ITEMIDLIST, dwenumflags: u32) -> windows_core::Result<super::shobjidl_core::IEnumIDList>;
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
impl IEnumerableView_Vtbl {
    pub const fn new<Identity: IEnumerableView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEnumReadyCallback<Identity: IEnumerableView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, percb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumerableView_Impl::SetEnumReadyCallback(this, core::mem::transmute_copy(&percb)).into()
            }
        }
        unsafe extern "system" fn CreateEnumIDListFromContents<Identity: IEnumerableView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidlfolder: *const super::shtypes::ITEMIDLIST, dwenumflags: u32, ppenumidlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumerableView_Impl::CreateEnumIDListFromContents(this, core::mem::transmute_copy(&pidlfolder), core::mem::transmute_copy(&dwenumflags)) {
                    Ok(ok__) => {
                        ppenumidlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetEnumReadyCallback: SetEnumReadyCallback::<Identity, OFFSET>,
            CreateEnumIDListFromContents: CreateEnumIDListFromContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumerableView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
impl windows_core::RuntimeName for IEnumerableView {}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::define_interface!(IFileDialog2, IFileDialog2_Vtbl, 0x61744fc7_85b5_4791_a9b0_272276309b13);
#[cfg(feature = "shobjidl_core")]
impl core::ops::Deref for IFileDialog2 {
    type Target = super::shobjidl_core::IFileDialog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::interface_hierarchy!(IFileDialog2, windows_core::IUnknown, super::shobjidl_core::IModalWindow, super::shobjidl_core::IFileDialog);
#[cfg(feature = "shobjidl_core")]
impl IFileDialog2 {
    pub unsafe fn SetCancelButtonLabel<P0>(&self, pszlabel: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCancelButtonLabel)(windows_core::Interface::as_raw(self), pszlabel.param().abi()) }
    }
    pub unsafe fn SetNavigationRoot<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNavigationRoot)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
}
#[cfg(feature = "shobjidl_core")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileDialog2_Vtbl {
    pub base__: super::shobjidl_core::IFileDialog_Vtbl,
    pub SetCancelButtonLabel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetNavigationRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub trait IFileDialog2_Impl: super::shobjidl_core::IFileDialog_Impl {
    fn SetCancelButtonLabel(&self, pszlabel: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetNavigationRoot(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl IFileDialog2_Vtbl {
    pub const fn new<Identity: IFileDialog2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCancelButtonLabel<Identity: IFileDialog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlabel: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog2_Impl::SetCancelButtonLabel(this, core::mem::transmute(&pszlabel)).into()
            }
        }
        unsafe extern "system" fn SetNavigationRoot<Identity: IFileDialog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog2_Impl::SetNavigationRoot(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        Self {
            base__: super::shobjidl_core::IFileDialog_Vtbl::new::<Identity, OFFSET>(),
            SetCancelButtonLabel: SetCancelButtonLabel::<Identity, OFFSET>,
            SetNavigationRoot: SetNavigationRoot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileDialog2 as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IModalWindow as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IFileDialog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl windows_core::RuntimeName for IFileDialog2 {}
windows_core::imp::define_interface!(IFileDialogControlEvents, IFileDialogControlEvents_Vtbl, 0x36116642_d713_4b97_9b83_7484a9d00433);
windows_core::imp::interface_hierarchy!(IFileDialogControlEvents, windows_core::IUnknown);
impl IFileDialogControlEvents {
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnItemSelected<P0>(&self, pfdc: P0, dwidctl: u32, dwiditem: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IFileDialogCustomize>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnItemSelected)(windows_core::Interface::as_raw(self), pfdc.param().abi(), dwidctl, dwiditem) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnButtonClicked<P0>(&self, pfdc: P0, dwidctl: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IFileDialogCustomize>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnButtonClicked)(windows_core::Interface::as_raw(self), pfdc.param().abi(), dwidctl) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnCheckButtonToggled<P0>(&self, pfdc: P0, dwidctl: u32, bchecked: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IFileDialogCustomize>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCheckButtonToggled)(windows_core::Interface::as_raw(self), pfdc.param().abi(), dwidctl, bchecked.into()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnControlActivating<P0>(&self, pfdc: P0, dwidctl: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IFileDialogCustomize>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnControlActivating)(windows_core::Interface::as_raw(self), pfdc.param().abi(), dwidctl) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileDialogControlEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shobjidl_core")]
    pub OnItemSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnItemSelected: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnButtonClicked: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnCheckButtonToggled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnCheckButtonToggled: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnControlActivating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnControlActivating: usize,
}
#[cfg(feature = "shobjidl_core")]
pub trait IFileDialogControlEvents_Impl: windows_core::IUnknownImpl {
    fn OnItemSelected(&self, pfdc: windows_core::Ref<super::shobjidl_core::IFileDialogCustomize>, dwidctl: u32, dwiditem: u32) -> windows_core::Result<()>;
    fn OnButtonClicked(&self, pfdc: windows_core::Ref<super::shobjidl_core::IFileDialogCustomize>, dwidctl: u32) -> windows_core::Result<()>;
    fn OnCheckButtonToggled(&self, pfdc: windows_core::Ref<super::shobjidl_core::IFileDialogCustomize>, dwidctl: u32, bchecked: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnControlActivating(&self, pfdc: windows_core::Ref<super::shobjidl_core::IFileDialogCustomize>, dwidctl: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "shobjidl_core")]
impl IFileDialogControlEvents_Vtbl {
    pub const fn new<Identity: IFileDialogControlEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnItemSelected<Identity: IFileDialogControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdc: *mut core::ffi::c_void, dwidctl: u32, dwiditem: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialogControlEvents_Impl::OnItemSelected(this, core::mem::transmute_copy(&pfdc), core::mem::transmute_copy(&dwidctl), core::mem::transmute_copy(&dwiditem)).into()
            }
        }
        unsafe extern "system" fn OnButtonClicked<Identity: IFileDialogControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdc: *mut core::ffi::c_void, dwidctl: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialogControlEvents_Impl::OnButtonClicked(this, core::mem::transmute_copy(&pfdc), core::mem::transmute_copy(&dwidctl)).into()
            }
        }
        unsafe extern "system" fn OnCheckButtonToggled<Identity: IFileDialogControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdc: *mut core::ffi::c_void, dwidctl: u32, bchecked: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialogControlEvents_Impl::OnCheckButtonToggled(this, core::mem::transmute_copy(&pfdc), core::mem::transmute_copy(&dwidctl), core::mem::transmute_copy(&bchecked)).into()
            }
        }
        unsafe extern "system" fn OnControlActivating<Identity: IFileDialogControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdc: *mut core::ffi::c_void, dwidctl: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialogControlEvents_Impl::OnControlActivating(this, core::mem::transmute_copy(&pfdc), core::mem::transmute_copy(&dwidctl)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnItemSelected: OnItemSelected::<Identity, OFFSET>,
            OnButtonClicked: OnButtonClicked::<Identity, OFFSET>,
            OnCheckButtonToggled: OnCheckButtonToggled::<Identity, OFFSET>,
            OnControlActivating: OnControlActivating::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileDialogControlEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shobjidl_core")]
impl windows_core::RuntimeName for IFileDialogControlEvents {}
windows_core::imp::define_interface!(IFolderBandPriv, IFolderBandPriv_Vtbl, 0x47c01f95_e185_412c_b5c5_4f27df965aea);
windows_core::imp::interface_hierarchy!(IFolderBandPriv, windows_core::IUnknown);
impl IFolderBandPriv {
    pub unsafe fn SetCascade(&self, fcascade: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCascade)(windows_core::Interface::as_raw(self), fcascade.into()) }
    }
    pub unsafe fn SetAccelerators(&self, faccelerators: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAccelerators)(windows_core::Interface::as_raw(self), faccelerators.into()) }
    }
    pub unsafe fn SetNoIcons(&self, fnoicons: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNoIcons)(windows_core::Interface::as_raw(self), fnoicons.into()) }
    }
    pub unsafe fn SetNoText(&self, fnotext: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNoText)(windows_core::Interface::as_raw(self), fnotext.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderBandPriv_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCascade: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetAccelerators: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNoIcons: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNoText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IFolderBandPriv_Impl: windows_core::IUnknownImpl {
    fn SetCascade(&self, fcascade: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetAccelerators(&self, faccelerators: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNoIcons(&self, fnoicons: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNoText(&self, fnotext: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IFolderBandPriv_Vtbl {
    pub const fn new<Identity: IFolderBandPriv_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCascade<Identity: IFolderBandPriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcascade: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderBandPriv_Impl::SetCascade(this, core::mem::transmute_copy(&fcascade)).into()
            }
        }
        unsafe extern "system" fn SetAccelerators<Identity: IFolderBandPriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, faccelerators: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderBandPriv_Impl::SetAccelerators(this, core::mem::transmute_copy(&faccelerators)).into()
            }
        }
        unsafe extern "system" fn SetNoIcons<Identity: IFolderBandPriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnoicons: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderBandPriv_Impl::SetNoIcons(this, core::mem::transmute_copy(&fnoicons)).into()
            }
        }
        unsafe extern "system" fn SetNoText<Identity: IFolderBandPriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnotext: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderBandPriv_Impl::SetNoText(this, core::mem::transmute_copy(&fnotext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCascade: SetCascade::<Identity, OFFSET>,
            SetAccelerators: SetAccelerators::<Identity, OFFSET>,
            SetNoIcons: SetNoIcons::<Identity, OFFSET>,
            SetNoText: SetNoText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderBandPriv as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFolderBandPriv {}
windows_core::imp::define_interface!(IFolderViewHost, IFolderViewHost_Vtbl, 0x1ea58f02_d55a_411d_b09e_9e65ac21605b);
windows_core::imp::interface_hierarchy!(IFolderViewHost, windows_core::IUnknown);
impl IFolderViewHost {
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn Initialize<P1>(&self, hwndparent: super::windef::HWND, pdo: P1, prc: *const super::windef::RECT) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hwndparent, pdo.param().abi(), prc) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderViewHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    Initialize: usize,
}
#[cfg(all(feature = "objidl", feature = "windef"))]
pub trait IFolderViewHost_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, hwndparent: super::windef::HWND, pdo: windows_core::Ref<super::objidl::IDataObject>, prc: *const super::windef::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "windef"))]
impl IFolderViewHost_Vtbl {
    pub const fn new<Identity: IFolderViewHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IFolderViewHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, pdo: *mut core::ffi::c_void, prc: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderViewHost_Impl::Initialize(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&pdo), core::mem::transmute_copy(&prc)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderViewHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "windef"))]
impl windows_core::RuntimeName for IFolderViewHost {}
windows_core::imp::define_interface!(IFolderViewOptions, IFolderViewOptions_Vtbl, 0x3cc974d2_b302_4d36_ad3e_06d93f695d3f);
windows_core::imp::interface_hierarchy!(IFolderViewOptions, windows_core::IUnknown);
impl IFolderViewOptions {
    pub unsafe fn SetFolderViewOptions(&self, fvomask: FOLDERVIEWOPTIONS, fvoflags: FOLDERVIEWOPTIONS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFolderViewOptions)(windows_core::Interface::as_raw(self), fvomask, fvoflags) }
    }
    pub unsafe fn GetFolderViewOptions(&self) -> windows_core::Result<FOLDERVIEWOPTIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolderViewOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderViewOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFolderViewOptions: unsafe extern "system" fn(*mut core::ffi::c_void, FOLDERVIEWOPTIONS, FOLDERVIEWOPTIONS) -> windows_core::HRESULT,
    pub GetFolderViewOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FOLDERVIEWOPTIONS) -> windows_core::HRESULT,
}
pub trait IFolderViewOptions_Impl: windows_core::IUnknownImpl {
    fn SetFolderViewOptions(&self, fvomask: FOLDERVIEWOPTIONS, fvoflags: FOLDERVIEWOPTIONS) -> windows_core::Result<()>;
    fn GetFolderViewOptions(&self) -> windows_core::Result<FOLDERVIEWOPTIONS>;
}
impl IFolderViewOptions_Vtbl {
    pub const fn new<Identity: IFolderViewOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFolderViewOptions<Identity: IFolderViewOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvomask: FOLDERVIEWOPTIONS, fvoflags: FOLDERVIEWOPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderViewOptions_Impl::SetFolderViewOptions(this, core::mem::transmute_copy(&fvomask), core::mem::transmute_copy(&fvoflags)).into()
            }
        }
        unsafe extern "system" fn GetFolderViewOptions<Identity: IFolderViewOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvoflags: *mut FOLDERVIEWOPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderViewOptions_Impl::GetFolderViewOptions(this) {
                    Ok(ok__) => {
                        pfvoflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFolderViewOptions: SetFolderViewOptions::<Identity, OFFSET>,
            GetFolderViewOptions: GetFolderViewOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderViewOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFolderViewOptions {}
windows_core::imp::define_interface!(IHWEventHandler, IHWEventHandler_Vtbl, 0xc1fb73d0_ec3a_4ba2_b512_8cdb9187b6d1);
windows_core::imp::interface_hierarchy!(IHWEventHandler, windows_core::IUnknown);
impl IHWEventHandler {
    pub unsafe fn Initialize<P0>(&self, pszparams: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pszparams.param().abi()) }
    }
    pub unsafe fn HandleEvent<P0, P1, P2>(&self, pszdeviceid: P0, pszaltdeviceid: P1, pszeventtype: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleEvent)(windows_core::Interface::as_raw(self), pszdeviceid.param().abi(), pszaltdeviceid.param().abi(), pszeventtype.param().abi()) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn HandleEventWithContent<P0, P1, P2, P3, P4>(&self, pszdeviceid: P0, pszaltdeviceid: P1, pszeventtype: P2, pszcontenttypehandler: P3, pdataobject: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleEventWithContent)(windows_core::Interface::as_raw(self), pszdeviceid.param().abi(), pszaltdeviceid.param().abi(), pszeventtype.param().abi(), pszcontenttypehandler.param().abi(), pdataobject.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHWEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub HandleEvent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub HandleEventWithContent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    HandleEventWithContent: usize,
}
#[cfg(feature = "objidl")]
pub trait IHWEventHandler_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pszparams: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn HandleEvent(&self, pszdeviceid: &windows_core::PCWSTR, pszaltdeviceid: &windows_core::PCWSTR, pszeventtype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn HandleEventWithContent(&self, pszdeviceid: &windows_core::PCWSTR, pszaltdeviceid: &windows_core::PCWSTR, pszeventtype: &windows_core::PCWSTR, pszcontenttypehandler: &windows_core::PCWSTR, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IHWEventHandler_Vtbl {
    pub const fn new<Identity: IHWEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IHWEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszparams: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHWEventHandler_Impl::Initialize(this, core::mem::transmute(&pszparams)).into()
            }
        }
        unsafe extern "system" fn HandleEvent<Identity: IHWEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdeviceid: windows_core::PCWSTR, pszaltdeviceid: windows_core::PCWSTR, pszeventtype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHWEventHandler_Impl::HandleEvent(this, core::mem::transmute(&pszdeviceid), core::mem::transmute(&pszaltdeviceid), core::mem::transmute(&pszeventtype)).into()
            }
        }
        unsafe extern "system" fn HandleEventWithContent<Identity: IHWEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdeviceid: windows_core::PCWSTR, pszaltdeviceid: windows_core::PCWSTR, pszeventtype: windows_core::PCWSTR, pszcontenttypehandler: windows_core::PCWSTR, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHWEventHandler_Impl::HandleEventWithContent(this, core::mem::transmute(&pszdeviceid), core::mem::transmute(&pszaltdeviceid), core::mem::transmute(&pszeventtype), core::mem::transmute(&pszcontenttypehandler), core::mem::transmute_copy(&pdataobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            HandleEvent: HandleEvent::<Identity, OFFSET>,
            HandleEventWithContent: HandleEventWithContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHWEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IHWEventHandler {}
windows_core::imp::define_interface!(IHWEventHandler2, IHWEventHandler2_Vtbl, 0xcfcc809f_295d_42e8_9ffc_424b33c487e6);
impl core::ops::Deref for IHWEventHandler2 {
    type Target = IHWEventHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHWEventHandler2, windows_core::IUnknown, IHWEventHandler);
impl IHWEventHandler2 {
    #[cfg(feature = "windef")]
    pub unsafe fn HandleEventWithHWND<P0, P1, P2>(&self, pszdeviceid: P0, pszaltdeviceid: P1, pszeventtype: P2, hwndowner: super::windef::HWND) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleEventWithHWND)(windows_core::Interface::as_raw(self), pszdeviceid.param().abi(), pszaltdeviceid.param().abi(), pszeventtype.param().abi(), hwndowner) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHWEventHandler2_Vtbl {
    pub base__: IHWEventHandler_Vtbl,
    #[cfg(feature = "windef")]
    pub HandleEventWithHWND: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    HandleEventWithHWND: usize,
}
#[cfg(all(feature = "objidl", feature = "windef"))]
pub trait IHWEventHandler2_Impl: IHWEventHandler_Impl {
    fn HandleEventWithHWND(&self, pszdeviceid: &windows_core::PCWSTR, pszaltdeviceid: &windows_core::PCWSTR, pszeventtype: &windows_core::PCWSTR, hwndowner: super::windef::HWND) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "windef"))]
impl IHWEventHandler2_Vtbl {
    pub const fn new<Identity: IHWEventHandler2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleEventWithHWND<Identity: IHWEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdeviceid: windows_core::PCWSTR, pszaltdeviceid: windows_core::PCWSTR, pszeventtype: windows_core::PCWSTR, hwndowner: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHWEventHandler2_Impl::HandleEventWithHWND(this, core::mem::transmute(&pszdeviceid), core::mem::transmute(&pszaltdeviceid), core::mem::transmute(&pszeventtype), core::mem::transmute_copy(&hwndowner)).into()
            }
        }
        Self { base__: IHWEventHandler_Vtbl::new::<Identity, OFFSET>(), HandleEventWithHWND: HandleEventWithHWND::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHWEventHandler2 as windows_core::Interface>::IID || iid == &<IHWEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "windef"))]
impl windows_core::RuntimeName for IHWEventHandler2 {}
windows_core::imp::define_interface!(IImageRecompress, IImageRecompress_Vtbl, 0x505f1513_6b3e_4892_a272_59f8889a4d3e);
windows_core::imp::interface_hierarchy!(IImageRecompress, windows_core::IUnknown);
impl IImageRecompress {
    #[cfg(all(feature = "objidl", feature = "objidlbase", feature = "shobjidl_core"))]
    pub unsafe fn RecompressImage<P0, P4>(&self, psi: P0, cx: i32, cy: i32, iquality: i32, pstg: P4) -> windows_core::Result<super::objidlbase::IStream>
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
        P4: windows_core::Param<super::objidl::IStorage>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RecompressImage)(windows_core::Interface::as_raw(self), psi.param().abi(), cx, cy, iquality, pstg.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageRecompress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "objidlbase", feature = "shobjidl_core"))]
    pub RecompressImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "objidlbase", feature = "shobjidl_core")))]
    RecompressImage: usize,
}
#[cfg(all(feature = "objidl", feature = "objidlbase", feature = "shobjidl_core"))]
pub trait IImageRecompress_Impl: windows_core::IUnknownImpl {
    fn RecompressImage(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, cx: i32, cy: i32, iquality: i32, pstg: windows_core::Ref<super::objidl::IStorage>) -> windows_core::Result<super::objidlbase::IStream>;
}
#[cfg(all(feature = "objidl", feature = "objidlbase", feature = "shobjidl_core"))]
impl IImageRecompress_Vtbl {
    pub const fn new<Identity: IImageRecompress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RecompressImage<Identity: IImageRecompress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, cx: i32, cy: i32, iquality: i32, pstg: *mut core::ffi::c_void, ppstrmout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageRecompress_Impl::RecompressImage(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy), core::mem::transmute_copy(&iquality), core::mem::transmute_copy(&pstg)) {
                    Ok(ok__) => {
                        ppstrmout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RecompressImage: RecompressImage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageRecompress as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "objidlbase", feature = "shobjidl_core"))]
impl windows_core::RuntimeName for IImageRecompress {}
windows_core::imp::define_interface!(IInsertItem, IInsertItem_Vtbl, 0xd2b57227_3d23_4b95_93c0_492bd454c356);
windows_core::imp::interface_hierarchy!(IInsertItem, windows_core::IUnknown);
impl IInsertItem {
    #[cfg(feature = "shtypes")]
    pub unsafe fn InsertItem(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertItem)(windows_core::Interface::as_raw(self), pidl) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInsertItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shtypes")]
    pub InsertItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    InsertItem: usize,
}
#[cfg(feature = "shtypes")]
pub trait IInsertItem_Impl: windows_core::IUnknownImpl {
    fn InsertItem(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<()>;
}
#[cfg(feature = "shtypes")]
impl IInsertItem_Vtbl {
    pub const fn new<Identity: IInsertItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertItem<Identity: IInsertItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInsertItem_Impl::InsertItem(this, core::mem::transmute_copy(&pidl)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InsertItem: InsertItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInsertItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shtypes")]
impl windows_core::RuntimeName for IInsertItem {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMarkupCallback(pub u8);
windows_core::imp::define_interface!(INameSpaceTreeAccessible, INameSpaceTreeAccessible_Vtbl, 0x71f312de_43ed_4190_8477_e9536b82350b);
windows_core::imp::interface_hierarchy!(INameSpaceTreeAccessible, windows_core::IUnknown);
impl INameSpaceTreeAccessible {
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnGetDefaultAccessibilityAction<P0>(&self, psi: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnGetDefaultAccessibilityAction)(windows_core::Interface::as_raw(self), psi.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnDoDefaultAccessibilityAction<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDoDefaultAccessibilityAction)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(all(feature = "oaidl", feature = "shobjidl_core", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn OnGetAccessibilityRole<P0>(&self, psi: P0) -> windows_core::Result<super::oaidl::VARIANT>
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnGetAccessibilityRole)(windows_core::Interface::as_raw(self), psi.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INameSpaceTreeAccessible_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shobjidl_core")]
    pub OnGetDefaultAccessibilityAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnGetDefaultAccessibilityAction: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnDoDefaultAccessibilityAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnDoDefaultAccessibilityAction: usize,
    #[cfg(all(feature = "oaidl", feature = "shobjidl_core", feature = "wtypes", feature = "wtypesbase"))]
    pub OnGetAccessibilityRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shobjidl_core", feature = "wtypes", feature = "wtypesbase")))]
    OnGetAccessibilityRole: usize,
}
#[cfg(all(feature = "oaidl", feature = "shobjidl_core", feature = "wtypes", feature = "wtypesbase"))]
pub trait INameSpaceTreeAccessible_Impl: windows_core::IUnknownImpl {
    fn OnGetDefaultAccessibilityAction(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<windows_core::BSTR>;
    fn OnDoDefaultAccessibilityAction(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn OnGetAccessibilityRole(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "shobjidl_core", feature = "wtypes", feature = "wtypesbase"))]
impl INameSpaceTreeAccessible_Vtbl {
    pub const fn new<Identity: INameSpaceTreeAccessible_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnGetDefaultAccessibilityAction<Identity: INameSpaceTreeAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, pbstrdefaultaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INameSpaceTreeAccessible_Impl::OnGetDefaultAccessibilityAction(this, core::mem::transmute_copy(&psi)) {
                    Ok(ok__) => {
                        pbstrdefaultaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnDoDefaultAccessibilityAction<Identity: INameSpaceTreeAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeAccessible_Impl::OnDoDefaultAccessibilityAction(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn OnGetAccessibilityRole<Identity: INameSpaceTreeAccessible_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, pvarrole: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INameSpaceTreeAccessible_Impl::OnGetAccessibilityRole(this, core::mem::transmute_copy(&psi)) {
                    Ok(ok__) => {
                        pvarrole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnGetDefaultAccessibilityAction: OnGetDefaultAccessibilityAction::<Identity, OFFSET>,
            OnDoDefaultAccessibilityAction: OnDoDefaultAccessibilityAction::<Identity, OFFSET>,
            OnGetAccessibilityRole: OnGetAccessibilityRole::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INameSpaceTreeAccessible as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shobjidl_core", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INameSpaceTreeAccessible {}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::define_interface!(INameSpaceTreeControl2, INameSpaceTreeControl2_Vtbl, 0x7cc7aed8_290e_49bc_8945_c1401cc9306c);
#[cfg(feature = "shobjidl_core")]
impl core::ops::Deref for INameSpaceTreeControl2 {
    type Target = super::shobjidl_core::INameSpaceTreeControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::interface_hierarchy!(INameSpaceTreeControl2, windows_core::IUnknown, super::shobjidl_core::INameSpaceTreeControl);
#[cfg(feature = "shobjidl_core")]
impl INameSpaceTreeControl2 {
    pub unsafe fn SetControlStyle(&self, nstcsmask: super::shobjidl_core::NSTCSTYLE, nstcsstyle: super::shobjidl_core::NSTCSTYLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetControlStyle)(windows_core::Interface::as_raw(self), nstcsmask, nstcsstyle) }
    }
    pub unsafe fn GetControlStyle(&self, nstcsmask: super::shobjidl_core::NSTCSTYLE) -> windows_core::Result<super::shobjidl_core::NSTCSTYLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetControlStyle)(windows_core::Interface::as_raw(self), nstcsmask, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetControlStyle2(&self, nstcsmask: NSTCSTYLE2, nstcsstyle: NSTCSTYLE2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetControlStyle2)(windows_core::Interface::as_raw(self), nstcsmask, nstcsstyle) }
    }
    pub unsafe fn GetControlStyle2(&self, nstcsmask: NSTCSTYLE2) -> windows_core::Result<NSTCSTYLE2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetControlStyle2)(windows_core::Interface::as_raw(self), nstcsmask, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "shobjidl_core")]
#[repr(C)]
#[doc(hidden)]
pub struct INameSpaceTreeControl2_Vtbl {
    pub base__: super::shobjidl_core::INameSpaceTreeControl_Vtbl,
    pub SetControlStyle: unsafe extern "system" fn(*mut core::ffi::c_void, super::shobjidl_core::NSTCSTYLE, super::shobjidl_core::NSTCSTYLE) -> windows_core::HRESULT,
    pub GetControlStyle: unsafe extern "system" fn(*mut core::ffi::c_void, super::shobjidl_core::NSTCSTYLE, *mut super::shobjidl_core::NSTCSTYLE) -> windows_core::HRESULT,
    pub SetControlStyle2: unsafe extern "system" fn(*mut core::ffi::c_void, NSTCSTYLE2, NSTCSTYLE2) -> windows_core::HRESULT,
    pub GetControlStyle2: unsafe extern "system" fn(*mut core::ffi::c_void, NSTCSTYLE2, *mut NSTCSTYLE2) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
pub trait INameSpaceTreeControl2_Impl: super::shobjidl_core::INameSpaceTreeControl_Impl {
    fn SetControlStyle(&self, nstcsmask: super::shobjidl_core::NSTCSTYLE, nstcsstyle: super::shobjidl_core::NSTCSTYLE) -> windows_core::Result<()>;
    fn GetControlStyle(&self, nstcsmask: super::shobjidl_core::NSTCSTYLE) -> windows_core::Result<super::shobjidl_core::NSTCSTYLE>;
    fn SetControlStyle2(&self, nstcsmask: NSTCSTYLE2, nstcsstyle: NSTCSTYLE2) -> windows_core::Result<()>;
    fn GetControlStyle2(&self, nstcsmask: NSTCSTYLE2) -> windows_core::Result<NSTCSTYLE2>;
}
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
impl INameSpaceTreeControl2_Vtbl {
    pub const fn new<Identity: INameSpaceTreeControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetControlStyle<Identity: INameSpaceTreeControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nstcsmask: super::shobjidl_core::NSTCSTYLE, nstcsstyle: super::shobjidl_core::NSTCSTYLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControl2_Impl::SetControlStyle(this, core::mem::transmute_copy(&nstcsmask), core::mem::transmute_copy(&nstcsstyle)).into()
            }
        }
        unsafe extern "system" fn GetControlStyle<Identity: INameSpaceTreeControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nstcsmask: super::shobjidl_core::NSTCSTYLE, pnstcsstyle: *mut super::shobjidl_core::NSTCSTYLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INameSpaceTreeControl2_Impl::GetControlStyle(this, core::mem::transmute_copy(&nstcsmask)) {
                    Ok(ok__) => {
                        pnstcsstyle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetControlStyle2<Identity: INameSpaceTreeControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nstcsmask: NSTCSTYLE2, nstcsstyle: NSTCSTYLE2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControl2_Impl::SetControlStyle2(this, core::mem::transmute_copy(&nstcsmask), core::mem::transmute_copy(&nstcsstyle)).into()
            }
        }
        unsafe extern "system" fn GetControlStyle2<Identity: INameSpaceTreeControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nstcsmask: NSTCSTYLE2, pnstcsstyle: *mut NSTCSTYLE2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INameSpaceTreeControl2_Impl::GetControlStyle2(this, core::mem::transmute_copy(&nstcsmask)) {
                    Ok(ok__) => {
                        pnstcsstyle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::shobjidl_core::INameSpaceTreeControl_Vtbl::new::<Identity, OFFSET>(),
            SetControlStyle: SetControlStyle::<Identity, OFFSET>,
            GetControlStyle: GetControlStyle::<Identity, OFFSET>,
            SetControlStyle2: SetControlStyle2::<Identity, OFFSET>,
            GetControlStyle2: GetControlStyle2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INameSpaceTreeControl2 as windows_core::Interface>::IID || iid == &<super::shobjidl_core::INameSpaceTreeControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
impl windows_core::RuntimeName for INameSpaceTreeControl2 {}
windows_core::imp::define_interface!(INameSpaceTreeControlCustomDraw, INameSpaceTreeControlCustomDraw_Vtbl, 0x2d3ba758_33ee_42d5_bb7b_5f3431d86c78);
windows_core::imp::interface_hierarchy!(INameSpaceTreeControlCustomDraw, windows_core::IUnknown);
impl INameSpaceTreeControlCustomDraw {
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub unsafe fn PrePaint(&self, hdc: super::windef::HDC, prc: *const super::windef::RECT) -> windows_core::Result<super::minwindef::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrePaint)(windows_core::Interface::as_raw(self), hdc, prc, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn PostPaint(&self, hdc: super::windef::HDC, prc: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PostPaint)(windows_core::Interface::as_raw(self), hdc, prc) }
    }
    #[cfg(all(feature = "commctrl", feature = "minwindef", feature = "shobjidl_core", feature = "windef"))]
    pub unsafe fn ItemPrePaint(&self, hdc: super::windef::HDC, prc: *const super::windef::RECT, pnstccditem: *const NSTCCUSTOMDRAW, pclrtext: *mut super::windef::COLORREF, pclrtextbk: *mut super::windef::COLORREF, plres: *mut super::minwindef::LRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ItemPrePaint)(windows_core::Interface::as_raw(self), hdc, prc, pnstccditem, pclrtext as _, pclrtextbk as _, plres as _) }
    }
    #[cfg(all(feature = "commctrl", feature = "shobjidl_core", feature = "windef"))]
    pub unsafe fn ItemPostPaint(&self, hdc: super::windef::HDC, prc: *const super::windef::RECT, pnstccditem: *const NSTCCUSTOMDRAW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ItemPostPaint)(windows_core::Interface::as_raw(self), hdc, prc, pnstccditem) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INameSpaceTreeControlCustomDraw_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub PrePaint: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, *const super::windef::RECT, *mut super::minwindef::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef")))]
    PrePaint: usize,
    #[cfg(feature = "windef")]
    pub PostPaint: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    PostPaint: usize,
    #[cfg(all(feature = "commctrl", feature = "minwindef", feature = "shobjidl_core", feature = "windef"))]
    pub ItemPrePaint: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, *const super::windef::RECT, *const NSTCCUSTOMDRAW, *mut super::windef::COLORREF, *mut super::windef::COLORREF, *mut super::minwindef::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "commctrl", feature = "minwindef", feature = "shobjidl_core", feature = "windef")))]
    ItemPrePaint: usize,
    #[cfg(all(feature = "commctrl", feature = "shobjidl_core", feature = "windef"))]
    pub ItemPostPaint: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, *const super::windef::RECT, *const NSTCCUSTOMDRAW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "commctrl", feature = "shobjidl_core", feature = "windef")))]
    ItemPostPaint: usize,
}
#[cfg(all(feature = "commctrl", feature = "minwindef", feature = "shobjidl_core", feature = "windef"))]
pub trait INameSpaceTreeControlCustomDraw_Impl: windows_core::IUnknownImpl {
    fn PrePaint(&self, hdc: super::windef::HDC, prc: *const super::windef::RECT) -> windows_core::Result<super::minwindef::LRESULT>;
    fn PostPaint(&self, hdc: super::windef::HDC, prc: *const super::windef::RECT) -> windows_core::Result<()>;
    fn ItemPrePaint(&self, hdc: super::windef::HDC, prc: *const super::windef::RECT, pnstccditem: *const NSTCCUSTOMDRAW, pclrtext: *mut super::windef::COLORREF, pclrtextbk: *mut super::windef::COLORREF, plres: *mut super::minwindef::LRESULT) -> windows_core::Result<()>;
    fn ItemPostPaint(&self, hdc: super::windef::HDC, prc: *const super::windef::RECT, pnstccditem: *const NSTCCUSTOMDRAW) -> windows_core::Result<()>;
}
#[cfg(all(feature = "commctrl", feature = "minwindef", feature = "shobjidl_core", feature = "windef"))]
impl INameSpaceTreeControlCustomDraw_Vtbl {
    pub const fn new<Identity: INameSpaceTreeControlCustomDraw_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrePaint<Identity: INameSpaceTreeControlCustomDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, prc: *const super::windef::RECT, plres: *mut super::minwindef::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INameSpaceTreeControlCustomDraw_Impl::PrePaint(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&prc)) {
                    Ok(ok__) => {
                        plres.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PostPaint<Identity: INameSpaceTreeControlCustomDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, prc: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlCustomDraw_Impl::PostPaint(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn ItemPrePaint<Identity: INameSpaceTreeControlCustomDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, prc: *const super::windef::RECT, pnstccditem: *const NSTCCUSTOMDRAW, pclrtext: *mut super::windef::COLORREF, pclrtextbk: *mut super::windef::COLORREF, plres: *mut super::minwindef::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlCustomDraw_Impl::ItemPrePaint(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pnstccditem), core::mem::transmute_copy(&pclrtext), core::mem::transmute_copy(&pclrtextbk), core::mem::transmute_copy(&plres)).into()
            }
        }
        unsafe extern "system" fn ItemPostPaint<Identity: INameSpaceTreeControlCustomDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, prc: *const super::windef::RECT, pnstccditem: *const NSTCCUSTOMDRAW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlCustomDraw_Impl::ItemPostPaint(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pnstccditem)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrePaint: PrePaint::<Identity, OFFSET>,
            PostPaint: PostPaint::<Identity, OFFSET>,
            ItemPrePaint: ItemPrePaint::<Identity, OFFSET>,
            ItemPostPaint: ItemPostPaint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INameSpaceTreeControlCustomDraw as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "commctrl", feature = "minwindef", feature = "shobjidl_core", feature = "windef"))]
impl windows_core::RuntimeName for INameSpaceTreeControlCustomDraw {}
windows_core::imp::define_interface!(INameSpaceTreeControlDropHandler, INameSpaceTreeControlDropHandler_Vtbl, 0xf9c665d6_c2f2_4c19_bf33_8322d7352f51);
windows_core::imp::interface_hierarchy!(INameSpaceTreeControlDropHandler, windows_core::IUnknown);
impl INameSpaceTreeControlDropHandler {
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnDragEnter<P0, P1>(&self, psiover: P0, psiadata: P1, foutsidesource: bool, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
        P1: windows_core::Param<super::shobjidl_core::IShellItemArray>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDragEnter)(windows_core::Interface::as_raw(self), psiover.param().abi(), psiadata.param().abi(), foutsidesource.into(), grfkeystate, pdweffect as _) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnDragOver<P0, P1>(&self, psiover: P0, psiadata: P1, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
        P1: windows_core::Param<super::shobjidl_core::IShellItemArray>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDragOver)(windows_core::Interface::as_raw(self), psiover.param().abi(), psiadata.param().abi(), grfkeystate, pdweffect as _) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnDragPosition<P0, P1>(&self, psiover: P0, psiadata: P1, inewposition: i32, ioldposition: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
        P1: windows_core::Param<super::shobjidl_core::IShellItemArray>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDragPosition)(windows_core::Interface::as_raw(self), psiover.param().abi(), psiadata.param().abi(), inewposition, ioldposition) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnDrop<P0, P1>(&self, psiover: P0, psiadata: P1, iposition: i32, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
        P1: windows_core::Param<super::shobjidl_core::IShellItemArray>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDrop)(windows_core::Interface::as_raw(self), psiover.param().abi(), psiadata.param().abi(), iposition, grfkeystate, pdweffect as _) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnDropPosition<P0, P1>(&self, psiover: P0, psiadata: P1, inewposition: i32, ioldposition: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
        P1: windows_core::Param<super::shobjidl_core::IShellItemArray>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDropPosition)(windows_core::Interface::as_raw(self), psiover.param().abi(), psiadata.param().abi(), inewposition, ioldposition) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnDragLeave<P0>(&self, psiover: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDragLeave)(windows_core::Interface::as_raw(self), psiover.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INameSpaceTreeControlDropHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shobjidl_core")]
    pub OnDragEnter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnDragEnter: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnDragOver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnDragOver: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnDragPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnDragPosition: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnDrop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnDrop: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnDropPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnDropPosition: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnDragLeave: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnDragLeave: usize,
}
#[cfg(feature = "shobjidl_core")]
pub trait INameSpaceTreeControlDropHandler_Impl: windows_core::IUnknownImpl {
    fn OnDragEnter(&self, psiover: windows_core::Ref<super::shobjidl_core::IShellItem>, psiadata: windows_core::Ref<super::shobjidl_core::IShellItemArray>, foutsidesource: windows_core::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::Result<()>;
    fn OnDragOver(&self, psiover: windows_core::Ref<super::shobjidl_core::IShellItem>, psiadata: windows_core::Ref<super::shobjidl_core::IShellItemArray>, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::Result<()>;
    fn OnDragPosition(&self, psiover: windows_core::Ref<super::shobjidl_core::IShellItem>, psiadata: windows_core::Ref<super::shobjidl_core::IShellItemArray>, inewposition: i32, ioldposition: i32) -> windows_core::Result<()>;
    fn OnDrop(&self, psiover: windows_core::Ref<super::shobjidl_core::IShellItem>, psiadata: windows_core::Ref<super::shobjidl_core::IShellItemArray>, iposition: i32, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::Result<()>;
    fn OnDropPosition(&self, psiover: windows_core::Ref<super::shobjidl_core::IShellItem>, psiadata: windows_core::Ref<super::shobjidl_core::IShellItemArray>, inewposition: i32, ioldposition: i32) -> windows_core::Result<()>;
    fn OnDragLeave(&self, psiover: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
}
#[cfg(feature = "shobjidl_core")]
impl INameSpaceTreeControlDropHandler_Vtbl {
    pub const fn new<Identity: INameSpaceTreeControlDropHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDragEnter<Identity: INameSpaceTreeControlDropHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psiover: *mut core::ffi::c_void, psiadata: *mut core::ffi::c_void, foutsidesource: windows_core::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlDropHandler_Impl::OnDragEnter(this, core::mem::transmute_copy(&psiover), core::mem::transmute_copy(&psiadata), core::mem::transmute_copy(&foutsidesource), core::mem::transmute_copy(&grfkeystate), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        unsafe extern "system" fn OnDragOver<Identity: INameSpaceTreeControlDropHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psiover: *mut core::ffi::c_void, psiadata: *mut core::ffi::c_void, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlDropHandler_Impl::OnDragOver(this, core::mem::transmute_copy(&psiover), core::mem::transmute_copy(&psiadata), core::mem::transmute_copy(&grfkeystate), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        unsafe extern "system" fn OnDragPosition<Identity: INameSpaceTreeControlDropHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psiover: *mut core::ffi::c_void, psiadata: *mut core::ffi::c_void, inewposition: i32, ioldposition: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlDropHandler_Impl::OnDragPosition(this, core::mem::transmute_copy(&psiover), core::mem::transmute_copy(&psiadata), core::mem::transmute_copy(&inewposition), core::mem::transmute_copy(&ioldposition)).into()
            }
        }
        unsafe extern "system" fn OnDrop<Identity: INameSpaceTreeControlDropHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psiover: *mut core::ffi::c_void, psiadata: *mut core::ffi::c_void, iposition: i32, grfkeystate: u32, pdweffect: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlDropHandler_Impl::OnDrop(this, core::mem::transmute_copy(&psiover), core::mem::transmute_copy(&psiadata), core::mem::transmute_copy(&iposition), core::mem::transmute_copy(&grfkeystate), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        unsafe extern "system" fn OnDropPosition<Identity: INameSpaceTreeControlDropHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psiover: *mut core::ffi::c_void, psiadata: *mut core::ffi::c_void, inewposition: i32, ioldposition: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlDropHandler_Impl::OnDropPosition(this, core::mem::transmute_copy(&psiover), core::mem::transmute_copy(&psiadata), core::mem::transmute_copy(&inewposition), core::mem::transmute_copy(&ioldposition)).into()
            }
        }
        unsafe extern "system" fn OnDragLeave<Identity: INameSpaceTreeControlDropHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psiover: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlDropHandler_Impl::OnDragLeave(this, core::mem::transmute_copy(&psiover)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDragEnter: OnDragEnter::<Identity, OFFSET>,
            OnDragOver: OnDragOver::<Identity, OFFSET>,
            OnDragPosition: OnDragPosition::<Identity, OFFSET>,
            OnDrop: OnDrop::<Identity, OFFSET>,
            OnDropPosition: OnDropPosition::<Identity, OFFSET>,
            OnDragLeave: OnDragLeave::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INameSpaceTreeControlDropHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shobjidl_core")]
impl windows_core::RuntimeName for INameSpaceTreeControlDropHandler {}
windows_core::imp::define_interface!(INameSpaceTreeControlEvents, INameSpaceTreeControlEvents_Vtbl, 0x93d77985_b3d8_4484_8318_672cdda002ce);
windows_core::imp::interface_hierarchy!(INameSpaceTreeControlEvents, windows_core::IUnknown);
impl INameSpaceTreeControlEvents {
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnItemClick<P0>(&self, psi: P0, nstcehittest: NSTCEHITTEST, nstceclicktype: NSTCECLICKTYPE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnItemClick)(windows_core::Interface::as_raw(self), psi.param().abi(), nstcehittest, nstceclicktype) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnPropertyItemCommit<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPropertyItemCommit)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnItemStateChanging<P0>(&self, psi: P0, nstcismask: super::shobjidl_core::NSTCITEMSTATE, nstcisstate: super::shobjidl_core::NSTCITEMSTATE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnItemStateChanging)(windows_core::Interface::as_raw(self), psi.param().abi(), nstcismask, nstcisstate) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnItemStateChanged<P0>(&self, psi: P0, nstcismask: super::shobjidl_core::NSTCITEMSTATE, nstcisstate: super::shobjidl_core::NSTCITEMSTATE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnItemStateChanged)(windows_core::Interface::as_raw(self), psi.param().abi(), nstcismask, nstcisstate) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnSelectionChanged<P0>(&self, psiaselection: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItemArray>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSelectionChanged)(windows_core::Interface::as_raw(self), psiaselection.param().abi()) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnKeyboardInput(&self, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnKeyboardInput)(windows_core::Interface::as_raw(self), umsg, wparam, lparam) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnBeforeExpand<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBeforeExpand)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnAfterExpand<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnAfterExpand)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnBeginLabelEdit<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBeginLabelEdit)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnEndLabelEdit<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEndLabelEdit)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnGetToolTip<P0>(&self, psi: P0, psztip: &mut [u16]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnGetToolTip)(windows_core::Interface::as_raw(self), psi.param().abi(), core::mem::transmute(psztip.as_mut_ptr()), psztip.len().try_into().unwrap()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnBeforeItemDelete<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBeforeItemDelete)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnItemAdded<P0>(&self, psi: P0, fisroot: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnItemAdded)(windows_core::Interface::as_raw(self), psi.param().abi(), fisroot.into()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnItemDeleted<P0>(&self, psi: P0, fisroot: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnItemDeleted)(windows_core::Interface::as_raw(self), psi.param().abi(), fisroot.into()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnBeforeContextMenu<P0, T>(&self, psi: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OnBeforeContextMenu)(windows_core::Interface::as_raw(self), psi.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnAfterContextMenu<P0, P1, T>(&self, psi: P0, pcmin: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
        P1: windows_core::Param<super::shobjidl_core::IContextMenu>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OnAfterContextMenu)(windows_core::Interface::as_raw(self), psi.param().abi(), pcmin.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnBeforeStateImageChange<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBeforeStateImageChange)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn OnGetDefaultIconIndex<P0>(&self, psi: P0, pidefaulticon: *mut i32, piopenicon: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnGetDefaultIconIndex)(windows_core::Interface::as_raw(self), psi.param().abi(), pidefaulticon as _, piopenicon as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INameSpaceTreeControlEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shobjidl_core")]
    pub OnItemClick: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, NSTCEHITTEST, NSTCECLICKTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnItemClick: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnPropertyItemCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnPropertyItemCommit: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnItemStateChanging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::shobjidl_core::NSTCITEMSTATE, super::shobjidl_core::NSTCITEMSTATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnItemStateChanging: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnItemStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::shobjidl_core::NSTCITEMSTATE, super::shobjidl_core::NSTCITEMSTATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnItemStateChanged: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnSelectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnSelectionChanged: usize,
    #[cfg(feature = "minwindef")]
    pub OnKeyboardInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::minwindef::WPARAM, super::minwindef::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnKeyboardInput: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnBeforeExpand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnBeforeExpand: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnAfterExpand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnAfterExpand: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnBeginLabelEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnBeginLabelEdit: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnEndLabelEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnEndLabelEdit: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnGetToolTip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnGetToolTip: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnBeforeItemDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnBeforeItemDelete: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnItemAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnItemAdded: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnItemDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnItemDeleted: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnBeforeContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnBeforeContextMenu: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnAfterContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnAfterContextMenu: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnBeforeStateImageChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnBeforeStateImageChange: usize,
    #[cfg(feature = "shobjidl_core")]
    pub OnGetDefaultIconIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    OnGetDefaultIconIndex: usize,
}
#[cfg(all(feature = "minwindef", feature = "shobjidl_core"))]
pub trait INameSpaceTreeControlEvents_Impl: windows_core::IUnknownImpl {
    fn OnItemClick(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, nstcehittest: NSTCEHITTEST, nstceclicktype: NSTCECLICKTYPE) -> windows_core::Result<()>;
    fn OnPropertyItemCommit(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn OnItemStateChanging(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, nstcismask: super::shobjidl_core::NSTCITEMSTATE, nstcisstate: super::shobjidl_core::NSTCITEMSTATE) -> windows_core::Result<()>;
    fn OnItemStateChanged(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, nstcismask: super::shobjidl_core::NSTCITEMSTATE, nstcisstate: super::shobjidl_core::NSTCITEMSTATE) -> windows_core::Result<()>;
    fn OnSelectionChanged(&self, psiaselection: windows_core::Ref<super::shobjidl_core::IShellItemArray>) -> windows_core::Result<()>;
    fn OnKeyboardInput(&self, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<()>;
    fn OnBeforeExpand(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn OnAfterExpand(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn OnBeginLabelEdit(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn OnEndLabelEdit(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn OnGetToolTip(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, psztip: windows_core::PWSTR, cchtip: i32) -> windows_core::Result<()>;
    fn OnBeforeItemDelete(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn OnItemAdded(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, fisroot: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnItemDeleted(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, fisroot: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnBeforeContextMenu(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OnAfterContextMenu(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, pcmin: windows_core::Ref<super::shobjidl_core::IContextMenu>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OnBeforeStateImageChange(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn OnGetDefaultIconIndex(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>, pidefaulticon: *mut i32, piopenicon: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shobjidl_core"))]
impl INameSpaceTreeControlEvents_Vtbl {
    pub const fn new<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnItemClick<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, nstcehittest: NSTCEHITTEST, nstceclicktype: NSTCECLICKTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnItemClick(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&nstcehittest), core::mem::transmute_copy(&nstceclicktype)).into()
            }
        }
        unsafe extern "system" fn OnPropertyItemCommit<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnPropertyItemCommit(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn OnItemStateChanging<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, nstcismask: super::shobjidl_core::NSTCITEMSTATE, nstcisstate: super::shobjidl_core::NSTCITEMSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnItemStateChanging(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&nstcismask), core::mem::transmute_copy(&nstcisstate)).into()
            }
        }
        unsafe extern "system" fn OnItemStateChanged<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, nstcismask: super::shobjidl_core::NSTCITEMSTATE, nstcisstate: super::shobjidl_core::NSTCITEMSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnItemStateChanged(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&nstcismask), core::mem::transmute_copy(&nstcisstate)).into()
            }
        }
        unsafe extern "system" fn OnSelectionChanged<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psiaselection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnSelectionChanged(this, core::mem::transmute_copy(&psiaselection)).into()
            }
        }
        unsafe extern "system" fn OnKeyboardInput<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnKeyboardInput(this, core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn OnBeforeExpand<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnBeforeExpand(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn OnAfterExpand<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnAfterExpand(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn OnBeginLabelEdit<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnBeginLabelEdit(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn OnEndLabelEdit<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnEndLabelEdit(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn OnGetToolTip<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, psztip: windows_core::PWSTR, cchtip: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnGetToolTip(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&psztip), core::mem::transmute_copy(&cchtip)).into()
            }
        }
        unsafe extern "system" fn OnBeforeItemDelete<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnBeforeItemDelete(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn OnItemAdded<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, fisroot: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnItemAdded(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&fisroot)).into()
            }
        }
        unsafe extern "system" fn OnItemDeleted<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, fisroot: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnItemDeleted(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&fisroot)).into()
            }
        }
        unsafe extern "system" fn OnBeforeContextMenu<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnBeforeContextMenu(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn OnAfterContextMenu<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, pcmin: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnAfterContextMenu(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&pcmin), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn OnBeforeStateImageChange<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnBeforeStateImageChange(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn OnGetDefaultIconIndex<Identity: INameSpaceTreeControlEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void, pidefaulticon: *mut i32, piopenicon: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INameSpaceTreeControlEvents_Impl::OnGetDefaultIconIndex(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&pidefaulticon), core::mem::transmute_copy(&piopenicon)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnItemClick: OnItemClick::<Identity, OFFSET>,
            OnPropertyItemCommit: OnPropertyItemCommit::<Identity, OFFSET>,
            OnItemStateChanging: OnItemStateChanging::<Identity, OFFSET>,
            OnItemStateChanged: OnItemStateChanged::<Identity, OFFSET>,
            OnSelectionChanged: OnSelectionChanged::<Identity, OFFSET>,
            OnKeyboardInput: OnKeyboardInput::<Identity, OFFSET>,
            OnBeforeExpand: OnBeforeExpand::<Identity, OFFSET>,
            OnAfterExpand: OnAfterExpand::<Identity, OFFSET>,
            OnBeginLabelEdit: OnBeginLabelEdit::<Identity, OFFSET>,
            OnEndLabelEdit: OnEndLabelEdit::<Identity, OFFSET>,
            OnGetToolTip: OnGetToolTip::<Identity, OFFSET>,
            OnBeforeItemDelete: OnBeforeItemDelete::<Identity, OFFSET>,
            OnItemAdded: OnItemAdded::<Identity, OFFSET>,
            OnItemDeleted: OnItemDeleted::<Identity, OFFSET>,
            OnBeforeContextMenu: OnBeforeContextMenu::<Identity, OFFSET>,
            OnAfterContextMenu: OnAfterContextMenu::<Identity, OFFSET>,
            OnBeforeStateImageChange: OnBeforeStateImageChange::<Identity, OFFSET>,
            OnGetDefaultIconIndex: OnGetDefaultIconIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INameSpaceTreeControlEvents as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shobjidl_core"))]
impl windows_core::RuntimeName for INameSpaceTreeControlEvents {}
windows_core::imp::define_interface!(IPreviousVersionsInfo, IPreviousVersionsInfo_Vtbl, 0x76e54780_ad74_48e3_a695_3ba9a0aff10d);
windows_core::imp::interface_hierarchy!(IPreviousVersionsInfo, windows_core::IUnknown);
impl IPreviousVersionsInfo {
    pub unsafe fn AreSnapshotsAvailable<P0>(&self, pszpath: P0, foktobeslow: bool) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreSnapshotsAvailable)(windows_core::Interface::as_raw(self), pszpath.param().abi(), foktobeslow.into(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreviousVersionsInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AreSnapshotsAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPreviousVersionsInfo_Impl: windows_core::IUnknownImpl {
    fn AreSnapshotsAvailable(&self, pszpath: &windows_core::PCWSTR, foktobeslow: windows_core::BOOL) -> windows_core::Result<windows_core::BOOL>;
}
impl IPreviousVersionsInfo_Vtbl {
    pub const fn new<Identity: IPreviousVersionsInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AreSnapshotsAvailable<Identity: IPreviousVersionsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, foktobeslow: windows_core::BOOL, pfavailable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPreviousVersionsInfo_Impl::AreSnapshotsAvailable(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&foktobeslow)) {
                    Ok(ok__) => {
                        pfavailable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AreSnapshotsAvailable: AreSnapshotsAvailable::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPreviousVersionsInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPreviousVersionsInfo {}
windows_core::imp::define_interface!(IPublishingWizard, IPublishingWizard_Vtbl, 0xaa9198bb_ccec_472d_beed_19a4f6733f7a);
impl core::ops::Deref for IPublishingWizard {
    type Target = IWizardExtension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPublishingWizard, windows_core::IUnknown, IWizardExtension);
impl IPublishingWizard {
    #[cfg(feature = "objidl")]
    pub unsafe fn Initialize<P0, P2>(&self, pdo: P0, dwoptions: u32, pszservicescope: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IDataObject>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pdo.param().abi(), dwoptions, pszservicescope.param().abi()) }
    }
    #[cfg(all(feature = "msxml", feature = "oaidl"))]
    pub unsafe fn GetTransferManifest(&self, phrfromtransfer: Option<*mut windows_core::HRESULT>, pdocmanifest: *mut Option<super::msxml::IXMLDOMDocument>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransferManifest)(windows_core::Interface::as_raw(self), phrfromtransfer.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pdocmanifest)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPublishingWizard_Vtbl {
    pub base__: IWizardExtension_Vtbl,
    #[cfg(feature = "objidl")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    Initialize: usize,
    #[cfg(all(feature = "msxml", feature = "oaidl"))]
    pub GetTransferManifest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "msxml", feature = "oaidl")))]
    GetTransferManifest: usize,
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "objidl", feature = "prsht"))]
pub trait IPublishingWizard_Impl: IWizardExtension_Impl {
    fn Initialize(&self, pdo: windows_core::Ref<super::objidl::IDataObject>, dwoptions: u32, pszservicescope: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTransferManifest(&self, phrfromtransfer: *mut windows_core::HRESULT, pdocmanifest: windows_core::OutRef<super::msxml::IXMLDOMDocument>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "objidl", feature = "prsht"))]
impl IPublishingWizard_Vtbl {
    pub const fn new<Identity: IPublishingWizard_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IPublishingWizard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdo: *mut core::ffi::c_void, dwoptions: u32, pszservicescope: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPublishingWizard_Impl::Initialize(this, core::mem::transmute_copy(&pdo), core::mem::transmute_copy(&dwoptions), core::mem::transmute(&pszservicescope)).into()
            }
        }
        unsafe extern "system" fn GetTransferManifest<Identity: IPublishingWizard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrfromtransfer: *mut windows_core::HRESULT, pdocmanifest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPublishingWizard_Impl::GetTransferManifest(this, core::mem::transmute_copy(&phrfromtransfer), core::mem::transmute_copy(&pdocmanifest)).into()
            }
        }
        Self {
            base__: IWizardExtension_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetTransferManifest: GetTransferManifest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPublishingWizard as windows_core::Interface>::IID || iid == &<IWizardExtension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "objidl", feature = "prsht"))]
impl windows_core::RuntimeName for IPublishingWizard {}
windows_core::imp::define_interface!(IQueryCancelAutoPlay, IQueryCancelAutoPlay_Vtbl, 0xddefe873_6997_4e68_be26_39b633adbe12);
windows_core::imp::interface_hierarchy!(IQueryCancelAutoPlay, windows_core::IUnknown);
impl IQueryCancelAutoPlay {
    pub unsafe fn AllowAutoPlay<P0, P2>(&self, pszpath: P0, dwcontenttype: u32, pszlabel: P2, dwserialnumber: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AllowAutoPlay)(windows_core::Interface::as_raw(self), pszpath.param().abi(), dwcontenttype, pszlabel.param().abi(), dwserialnumber) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryCancelAutoPlay_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllowAutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
pub trait IQueryCancelAutoPlay_Impl: windows_core::IUnknownImpl {
    fn AllowAutoPlay(&self, pszpath: &windows_core::PCWSTR, dwcontenttype: u32, pszlabel: &windows_core::PCWSTR, dwserialnumber: u32) -> windows_core::Result<()>;
}
impl IQueryCancelAutoPlay_Vtbl {
    pub const fn new<Identity: IQueryCancelAutoPlay_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllowAutoPlay<Identity: IQueryCancelAutoPlay_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, dwcontenttype: u32, pszlabel: windows_core::PCWSTR, dwserialnumber: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryCancelAutoPlay_Impl::AllowAutoPlay(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&dwcontenttype), core::mem::transmute(&pszlabel), core::mem::transmute_copy(&dwserialnumber)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AllowAutoPlay: AllowAutoPlay::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryCancelAutoPlay as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IQueryCancelAutoPlay {}
windows_core::imp::define_interface!(IQueryCodePage, IQueryCodePage_Vtbl, 0xc7b236ce_ee80_11d0_985f_006008059382);
windows_core::imp::interface_hierarchy!(IQueryCodePage, windows_core::IUnknown);
impl IQueryCodePage {
    pub unsafe fn GetCodePage(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCodePage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCodePage(&self, uicodepage: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCodePage)(windows_core::Interface::as_raw(self), uicodepage) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryCodePage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IQueryCodePage_Impl: windows_core::IUnknownImpl {
    fn GetCodePage(&self) -> windows_core::Result<u32>;
    fn SetCodePage(&self, uicodepage: u32) -> windows_core::Result<()>;
}
impl IQueryCodePage_Vtbl {
    pub const fn new<Identity: IQueryCodePage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCodePage<Identity: IQueryCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puicodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryCodePage_Impl::GetCodePage(this) {
                    Ok(ok__) => {
                        puicodepage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCodePage<Identity: IQueryCodePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryCodePage_Impl::SetCodePage(this, core::mem::transmute_copy(&uicodepage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCodePage: GetCodePage::<Identity, OFFSET>,
            SetCodePage: SetCodePage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryCodePage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IQueryCodePage {}
windows_core::imp::define_interface!(IResultsFolder, IResultsFolder_Vtbl, 0x96e5ae6d_6ae1_4b1c_900c_c6480eaa8828);
windows_core::imp::interface_hierarchy!(IResultsFolder, windows_core::IUnknown);
impl IResultsFolder {
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn AddItem<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn AddIDList(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<super::shtypes::LPITEMIDLIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddIDList)(windows_core::Interface::as_raw(self), pidl, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn RemoveItem<P0>(&self, psi: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), psi.param().abi()) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn RemoveIDList(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveIDList)(windows_core::Interface::as_raw(self), pidl) }
    }
    pub unsafe fn RemoveAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultsFolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shobjidl_core")]
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    AddItem: usize,
    #[cfg(feature = "shtypes")]
    pub AddIDList: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    AddIDList: usize,
    #[cfg(feature = "shobjidl_core")]
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    RemoveItem: usize,
    #[cfg(feature = "shtypes")]
    pub RemoveIDList: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    RemoveIDList: usize,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
pub trait IResultsFolder_Impl: windows_core::IUnknownImpl {
    fn AddItem(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn AddIDList(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<super::shtypes::LPITEMIDLIST>;
    fn RemoveItem(&self, psi: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
    fn RemoveIDList(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<()>;
    fn RemoveAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
impl IResultsFolder_Vtbl {
    pub const fn new<Identity: IResultsFolder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddItem<Identity: IResultsFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultsFolder_Impl::AddItem(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn AddIDList<Identity: IResultsFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, ppidladded: *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResultsFolder_Impl::AddIDList(this, core::mem::transmute_copy(&pidl)) {
                    Ok(ok__) => {
                        ppidladded.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: IResultsFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultsFolder_Impl::RemoveItem(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn RemoveIDList<Identity: IResultsFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultsFolder_Impl::RemoveIDList(this, core::mem::transmute_copy(&pidl)).into()
            }
        }
        unsafe extern "system" fn RemoveAll<Identity: IResultsFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResultsFolder_Impl::RemoveAll(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddItem: AddItem::<Identity, OFFSET>,
            AddIDList: AddIDList::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
            RemoveIDList: RemoveIDList::<Identity, OFFSET>,
            RemoveAll: RemoveAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResultsFolder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
impl windows_core::RuntimeName for IResultsFolder {}
windows_core::imp::define_interface!(ISearchBoxInfo, ISearchBoxInfo_Vtbl, 0x6af6e03f_d664_4ef4_9626_f7e0ed36755e);
windows_core::imp::interface_hierarchy!(ISearchBoxInfo, windows_core::IUnknown);
impl ISearchBoxInfo {
    pub unsafe fn GetCondition<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCondition)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetText(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchBoxInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ISearchBoxInfo_Impl: windows_core::IUnknownImpl {
    fn GetCondition(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetText(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl ISearchBoxInfo_Vtbl {
    pub const fn new<Identity: ISearchBoxInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCondition<Identity: ISearchBoxInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchBoxInfo_Impl::GetCondition(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ISearchBoxInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsz: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchBoxInfo_Impl::GetText(this) {
                    Ok(ok__) => {
                        ppsz.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCondition: GetCondition::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchBoxInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISearchBoxInfo {}
windows_core::imp::define_interface!(IShellRunDll, IShellRunDll_Vtbl, 0xfce4bde0_4b68_4b80_8e9c_7426315a7388);
windows_core::imp::interface_hierarchy!(IShellRunDll, windows_core::IUnknown);
impl IShellRunDll {
    pub unsafe fn Run<P0>(&self, pszargs: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), pszargs.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellRunDll_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IShellRunDll_Impl: windows_core::IUnknownImpl {
    fn Run(&self, pszargs: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IShellRunDll_Vtbl {
    pub const fn new<Identity: IShellRunDll_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Run<Identity: IShellRunDll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszargs: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellRunDll_Impl::Run(this, core::mem::transmute(&pszargs)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Run: Run::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellRunDll as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IShellRunDll {}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
windows_core::imp::define_interface!(IShellView3, IShellView3_Vtbl, 0xec39fa88_f8af_41c5_8421_38bed28f4673);
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
impl core::ops::Deref for IShellView3 {
    type Target = super::shobjidl_core::IShellView2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
windows_core::imp::interface_hierarchy!(IShellView3, windows_core::IUnknown, super::oleidl::IOleWindow, super::shobjidl_core::IShellView, super::shobjidl_core::IShellView2);
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
impl IShellView3 {
    #[cfg(feature = "windef")]
    pub unsafe fn CreateViewWindow3<P0, P1>(&self, psbowner: P0, psvprev: P1, dwviewflags: SV3CVW3_FLAGS, dwmask: super::shobjidl_core::FOLDERFLAGS, dwflags: super::shobjidl_core::FOLDERFLAGS, fvmode: super::shobjidl_core::FOLDERVIEWMODE, pvid: *const super::shobjidl_core::SHELLVIEWID, prcview: *const super::windef::RECT) -> windows_core::Result<super::windef::HWND>
    where
        P0: windows_core::Param<super::shobjidl_core::IShellBrowser>,
        P1: windows_core::Param<super::shobjidl_core::IShellView>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateViewWindow3)(windows_core::Interface::as_raw(self), psbowner.param().abi(), psvprev.param().abi(), dwviewflags, dwmask, dwflags, fvmode, pvid, prcview, &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
#[repr(C)]
#[doc(hidden)]
pub struct IShellView3_Vtbl {
    pub base__: super::shobjidl_core::IShellView2_Vtbl,
    #[cfg(feature = "windef")]
    pub CreateViewWindow3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, SV3CVW3_FLAGS, super::shobjidl_core::FOLDERFLAGS, super::shobjidl_core::FOLDERFLAGS, super::shobjidl_core::FOLDERVIEWMODE, *const super::shobjidl_core::SHELLVIEWID, *const super::windef::RECT, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CreateViewWindow3: usize,
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "prsht", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser"))]
pub trait IShellView3_Impl: super::shobjidl_core::IShellView2_Impl {
    fn CreateViewWindow3(&self, psbowner: windows_core::Ref<super::shobjidl_core::IShellBrowser>, psvprev: windows_core::Ref<super::shobjidl_core::IShellView>, dwviewflags: SV3CVW3_FLAGS, dwmask: super::shobjidl_core::FOLDERFLAGS, dwflags: super::shobjidl_core::FOLDERFLAGS, fvmode: super::shobjidl_core::FOLDERVIEWMODE, pvid: *const super::shobjidl_core::SHELLVIEWID, prcview: *const super::windef::RECT) -> windows_core::Result<super::windef::HWND>;
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "prsht", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser"))]
impl IShellView3_Vtbl {
    pub const fn new<Identity: IShellView3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateViewWindow3<Identity: IShellView3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psbowner: *mut core::ffi::c_void, psvprev: *mut core::ffi::c_void, dwviewflags: SV3CVW3_FLAGS, dwmask: super::shobjidl_core::FOLDERFLAGS, dwflags: super::shobjidl_core::FOLDERFLAGS, fvmode: super::shobjidl_core::FOLDERVIEWMODE, pvid: *const super::shobjidl_core::SHELLVIEWID, prcview: *const super::windef::RECT, phwndview: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellView3_Impl::CreateViewWindow3(this, core::mem::transmute_copy(&psbowner), core::mem::transmute_copy(&psvprev), core::mem::transmute_copy(&dwviewflags), core::mem::transmute_copy(&dwmask), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&fvmode), core::mem::transmute_copy(&pvid), core::mem::transmute_copy(&prcview)) {
                    Ok(ok__) => {
                        phwndview.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::shobjidl_core::IShellView2_Vtbl::new::<Identity, OFFSET>(), CreateViewWindow3: CreateViewWindow3::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellView3 as windows_core::Interface>::IID || iid == &<super::oleidl::IOleWindow as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IShellView as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IShellView2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "prsht", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IShellView3 {}
windows_core::imp::define_interface!(IStartMenuPinnedList, IStartMenuPinnedList_Vtbl, 0x4cd19ada_25a5_4a32_b3b7_347bee5be36b);
windows_core::imp::interface_hierarchy!(IStartMenuPinnedList, windows_core::IUnknown);
impl IStartMenuPinnedList {
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn RemoveFromList<P0>(&self, pitem: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveFromList)(windows_core::Interface::as_raw(self), pitem.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartMenuPinnedList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shobjidl_core")]
    pub RemoveFromList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    RemoveFromList: usize,
}
#[cfg(feature = "shobjidl_core")]
pub trait IStartMenuPinnedList_Impl: windows_core::IUnknownImpl {
    fn RemoveFromList(&self, pitem: windows_core::Ref<super::shobjidl_core::IShellItem>) -> windows_core::Result<()>;
}
#[cfg(feature = "shobjidl_core")]
impl IStartMenuPinnedList_Vtbl {
    pub const fn new<Identity: IStartMenuPinnedList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemoveFromList<Identity: IStartMenuPinnedList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStartMenuPinnedList_Impl::RemoveFromList(this, core::mem::transmute_copy(&pitem)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RemoveFromList: RemoveFromList::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStartMenuPinnedList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shobjidl_core")]
impl windows_core::RuntimeName for IStartMenuPinnedList {}
windows_core::imp::define_interface!(IStorageProviderBanners, IStorageProviderBanners_Vtbl, 0x5efb46d7_47c0_4b68_acda_ded47c90ec91);
windows_core::imp::interface_hierarchy!(IStorageProviderBanners, windows_core::IUnknown);
impl IStorageProviderBanners {
    pub unsafe fn SetBanner<P0, P1, P2>(&self, provideridentity: P0, subscriptionid: P1, contentid: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBanner)(windows_core::Interface::as_raw(self), provideridentity.param().abi(), subscriptionid.param().abi(), contentid.param().abi()) }
    }
    pub unsafe fn ClearBanner<P0, P1>(&self, provideridentity: P0, subscriptionid: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ClearBanner)(windows_core::Interface::as_raw(self), provideridentity.param().abi(), subscriptionid.param().abi()) }
    }
    pub unsafe fn ClearAllBanners<P0>(&self, provideridentity: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ClearAllBanners)(windows_core::Interface::as_raw(self), provideridentity.param().abi()) }
    }
    pub unsafe fn GetBanner<P0, P1>(&self, provideridentity: P0, subscriptionid: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBanner)(windows_core::Interface::as_raw(self), provideridentity.param().abi(), subscriptionid.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderBanners_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBanner: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ClearBanner: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ClearAllBanners: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetBanner: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IStorageProviderBanners_Impl: windows_core::IUnknownImpl {
    fn SetBanner(&self, provideridentity: &windows_core::PCWSTR, subscriptionid: &windows_core::PCWSTR, contentid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ClearBanner(&self, provideridentity: &windows_core::PCWSTR, subscriptionid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ClearAllBanners(&self, provideridentity: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBanner(&self, provideridentity: &windows_core::PCWSTR, subscriptionid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl IStorageProviderBanners_Vtbl {
    pub const fn new<Identity: IStorageProviderBanners_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBanner<Identity: IStorageProviderBanners_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provideridentity: windows_core::PCWSTR, subscriptionid: windows_core::PCWSTR, contentid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderBanners_Impl::SetBanner(this, core::mem::transmute(&provideridentity), core::mem::transmute(&subscriptionid), core::mem::transmute(&contentid)).into()
            }
        }
        unsafe extern "system" fn ClearBanner<Identity: IStorageProviderBanners_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provideridentity: windows_core::PCWSTR, subscriptionid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderBanners_Impl::ClearBanner(this, core::mem::transmute(&provideridentity), core::mem::transmute(&subscriptionid)).into()
            }
        }
        unsafe extern "system" fn ClearAllBanners<Identity: IStorageProviderBanners_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provideridentity: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageProviderBanners_Impl::ClearAllBanners(this, core::mem::transmute(&provideridentity)).into()
            }
        }
        unsafe extern "system" fn GetBanner<Identity: IStorageProviderBanners_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provideridentity: windows_core::PCWSTR, subscriptionid: windows_core::PCWSTR, contentid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderBanners_Impl::GetBanner(this, core::mem::transmute(&provideridentity), core::mem::transmute(&subscriptionid)) {
                    Ok(ok__) => {
                        contentid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBanner: SetBanner::<Identity, OFFSET>,
            ClearBanner: ClearBanner::<Identity, OFFSET>,
            ClearAllBanners: ClearAllBanners::<Identity, OFFSET>,
            GetBanner: GetBanner::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderBanners as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStorageProviderBanners {}
windows_core::imp::define_interface!(IStorageProviderCopyHook, IStorageProviderCopyHook_Vtbl, 0x7bf992a9_af7a_4dba_b2e5_4d080b1ecbc6);
windows_core::imp::interface_hierarchy!(IStorageProviderCopyHook, windows_core::IUnknown);
impl IStorageProviderCopyHook {
    #[cfg(feature = "windef")]
    pub unsafe fn CopyCallback<P3, P5>(&self, hwnd: super::windef::HWND, operation: u32, flags: u32, srcfile: P3, srcattribs: u32, destfile: P5, destattribs: u32) -> windows_core::Result<u32>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyCallback)(windows_core::Interface::as_raw(self), hwnd, operation, flags, srcfile.param().abi(), srcattribs, destfile.param().abi(), destattribs, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderCopyHook_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub CopyCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, u32, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CopyCallback: usize,
}
#[cfg(feature = "windef")]
pub trait IStorageProviderCopyHook_Impl: windows_core::IUnknownImpl {
    fn CopyCallback(&self, hwnd: super::windef::HWND, operation: u32, flags: u32, srcfile: &windows_core::PCWSTR, srcattribs: u32, destfile: &windows_core::PCWSTR, destattribs: u32) -> windows_core::Result<u32>;
}
#[cfg(feature = "windef")]
impl IStorageProviderCopyHook_Vtbl {
    pub const fn new<Identity: IStorageProviderCopyHook_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopyCallback<Identity: IStorageProviderCopyHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, operation: u32, flags: u32, srcfile: windows_core::PCWSTR, srcattribs: u32, destfile: windows_core::PCWSTR, destattribs: u32, result: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageProviderCopyHook_Impl::CopyCallback(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&operation), core::mem::transmute_copy(&flags), core::mem::transmute(&srcfile), core::mem::transmute_copy(&srcattribs), core::mem::transmute(&destfile), core::mem::transmute_copy(&destattribs)) {
                    Ok(ok__) => {
                        result.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CopyCallback: CopyCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageProviderCopyHook as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IStorageProviderCopyHook {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IStreamAsync, IStreamAsync_Vtbl, 0xfe0b6665_e0ca_49b9_a178_2b5cb48d92a5);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IStreamAsync {
    type Target = super::objidlbase::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IStreamAsync, windows_core::IUnknown, super::objidlbase::ISequentialStream, super::objidlbase::IStream);
#[cfg(feature = "objidlbase")]
impl IStreamAsync {
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub unsafe fn ReadAsync(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: Option<*mut u32>, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadAsync)(windows_core::Interface::as_raw(self), pv as _, cb, pcbread.unwrap_or(core::mem::zeroed()) as _, lpoverlapped) }
    }
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub unsafe fn WriteAsync(&self, lpbuffer: *const core::ffi::c_void, cb: u32, pcbwritten: Option<*mut u32>, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteAsync)(windows_core::Interface::as_raw(self), lpbuffer, cb, pcbwritten.unwrap_or(core::mem::zeroed()) as _, lpoverlapped) }
    }
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub unsafe fn OverlappedResult(&self, lpoverlapped: *const super::minwinbase::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OverlappedResult)(windows_core::Interface::as_raw(self), lpoverlapped, lpnumberofbytestransferred as _, bwait.into()) }
    }
    pub unsafe fn CancelIo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelIo)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamAsync_Vtbl {
    pub base__: super::objidlbase::IStream_Vtbl,
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub ReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32, *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "winnt")))]
    ReadAsync: usize,
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub WriteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32, *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "winnt")))]
    WriteAsync: usize,
    #[cfg(all(feature = "minwinbase", feature = "winnt"))]
    pub OverlappedResult: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::minwinbase::OVERLAPPED, *mut u32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "winnt")))]
    OverlappedResult: usize,
    pub CancelIo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidlbase", feature = "winnt"))]
pub trait IStreamAsync_Impl: super::objidlbase::IStream_Impl {
    fn ReadAsync(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::Result<()>;
    fn WriteAsync(&self, lpbuffer: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::Result<()>;
    fn OverlappedResult(&self, lpoverlapped: *const super::minwinbase::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: windows_core::BOOL) -> windows_core::Result<()>;
    fn CancelIo(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidlbase", feature = "winnt"))]
impl IStreamAsync_Vtbl {
    pub const fn new<Identity: IStreamAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadAsync<Identity: IStreamAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamAsync_Impl::ReadAsync(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn WriteAsync<Identity: IStreamAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbuffer: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32, lpoverlapped: *const super::minwinbase::OVERLAPPED) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamAsync_Impl::WriteAsync(this, core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten), core::mem::transmute_copy(&lpoverlapped)).into()
            }
        }
        unsafe extern "system" fn OverlappedResult<Identity: IStreamAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpoverlapped: *const super::minwinbase::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamAsync_Impl::OverlappedResult(this, core::mem::transmute_copy(&lpoverlapped), core::mem::transmute_copy(&lpnumberofbytestransferred), core::mem::transmute_copy(&bwait)).into()
            }
        }
        unsafe extern "system" fn CancelIo<Identity: IStreamAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamAsync_Impl::CancelIo(this).into()
            }
        }
        Self {
            base__: super::objidlbase::IStream_Vtbl::new::<Identity, OFFSET>(),
            ReadAsync: ReadAsync::<Identity, OFFSET>,
            WriteAsync: WriteAsync::<Identity, OFFSET>,
            OverlappedResult: OverlappedResult::<Identity, OFFSET>,
            CancelIo: CancelIo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamAsync as windows_core::Interface>::IID || iid == &<super::objidlbase::ISequentialStream as windows_core::Interface>::IID || iid == &<super::objidlbase::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "objidlbase", feature = "winnt"))]
impl windows_core::RuntimeName for IStreamAsync {}
windows_core::imp::define_interface!(IStreamUnbufferedInfo, IStreamUnbufferedInfo_Vtbl, 0x8a68fdda_1fdc_4c20_8ceb_416643b5a625);
windows_core::imp::interface_hierarchy!(IStreamUnbufferedInfo, windows_core::IUnknown);
impl IStreamUnbufferedInfo {
    pub unsafe fn GetSectorSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSectorSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamUnbufferedInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSectorSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IStreamUnbufferedInfo_Impl: windows_core::IUnknownImpl {
    fn GetSectorSize(&self) -> windows_core::Result<u32>;
}
impl IStreamUnbufferedInfo_Vtbl {
    pub const fn new<Identity: IStreamUnbufferedInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSectorSize<Identity: IStreamUnbufferedInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsectorsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStreamUnbufferedInfo_Impl::GetSectorSize(this) {
                    Ok(ok__) => {
                        pcbsectorsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSectorSize: GetSectorSize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamUnbufferedInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStreamUnbufferedInfo {}
windows_core::imp::define_interface!(ITrayDeskBand, ITrayDeskBand_Vtbl, 0x6d67e846_5b9c_4db8_9cbc_dde12f4254f1);
windows_core::imp::interface_hierarchy!(ITrayDeskBand, windows_core::IUnknown);
impl ITrayDeskBand {
    pub unsafe fn ShowDeskBand(&self, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowDeskBand)(windows_core::Interface::as_raw(self), clsid) }
    }
    pub unsafe fn HideDeskBand(&self, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HideDeskBand)(windows_core::Interface::as_raw(self), clsid) }
    }
    pub unsafe fn IsDeskBandShown(&self, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDeskBandShown)(windows_core::Interface::as_raw(self), clsid) }
    }
    pub unsafe fn DeskBandRegistrationChanged(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeskBandRegistrationChanged)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrayDeskBand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShowDeskBand: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub HideDeskBand: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub IsDeskBandShown: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub DeskBandRegistrationChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITrayDeskBand_Impl: windows_core::IUnknownImpl {
    fn ShowDeskBand(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn HideDeskBand(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn IsDeskBandShown(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeskBandRegistrationChanged(&self) -> windows_core::Result<()>;
}
impl ITrayDeskBand_Vtbl {
    pub const fn new<Identity: ITrayDeskBand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowDeskBand<Identity: ITrayDeskBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrayDeskBand_Impl::ShowDeskBand(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn HideDeskBand<Identity: ITrayDeskBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrayDeskBand_Impl::HideDeskBand(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn IsDeskBandShown<Identity: ITrayDeskBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrayDeskBand_Impl::IsDeskBandShown(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn DeskBandRegistrationChanged<Identity: ITrayDeskBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrayDeskBand_Impl::DeskBandRegistrationChanged(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ShowDeskBand: ShowDeskBand::<Identity, OFFSET>,
            HideDeskBand: HideDeskBand::<Identity, OFFSET>,
            IsDeskBandShown: IsDeskBandShown::<Identity, OFFSET>,
            DeskBandRegistrationChanged: DeskBandRegistrationChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrayDeskBand as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITrayDeskBand {}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::define_interface!(IUseToBrowseItem, IUseToBrowseItem_Vtbl, 0x05edda5c_98a3_4717_8adb_c5e7da991eb1);
#[cfg(feature = "shobjidl_core")]
impl core::ops::Deref for IUseToBrowseItem {
    type Target = super::shobjidl_core::IRelatedItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::interface_hierarchy!(IUseToBrowseItem, windows_core::IUnknown, super::shobjidl_core::IRelatedItem);
#[cfg(feature = "shobjidl_core")]
#[repr(C)]
#[doc(hidden)]
pub struct IUseToBrowseItem_Vtbl {
    pub base__: super::shobjidl_core::IRelatedItem_Vtbl,
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
pub trait IUseToBrowseItem_Impl: super::shobjidl_core::IRelatedItem_Impl {}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
impl IUseToBrowseItem_Vtbl {
    pub const fn new<Identity: IUseToBrowseItem_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::shobjidl_core::IRelatedItem_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUseToBrowseItem as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IRelatedItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
impl windows_core::RuntimeName for IUseToBrowseItem {}
windows_core::imp::define_interface!(IUserAccountChangeCallback, IUserAccountChangeCallback_Vtbl, 0xa561e69a_b4b8_4113_91a5_64c6bcca3430);
windows_core::imp::interface_hierarchy!(IUserAccountChangeCallback, windows_core::IUnknown);
impl IUserAccountChangeCallback {
    pub unsafe fn OnPictureChange<P0>(&self, pszusername: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPictureChange)(windows_core::Interface::as_raw(self), pszusername.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAccountChangeCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnPictureChange: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IUserAccountChangeCallback_Impl: windows_core::IUnknownImpl {
    fn OnPictureChange(&self, pszusername: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IUserAccountChangeCallback_Vtbl {
    pub const fn new<Identity: IUserAccountChangeCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnPictureChange<Identity: IUserAccountChangeCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszusername: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserAccountChangeCallback_Impl::OnPictureChange(this, core::mem::transmute(&pszusername)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPictureChange: OnPictureChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserAccountChangeCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUserAccountChangeCallback {}
windows_core::imp::define_interface!(IUserNotification2, IUserNotification2_Vtbl, 0x215913cc_57eb_4fab_ab5a_e5fa7bea2a6c);
windows_core::imp::interface_hierarchy!(IUserNotification2, windows_core::IUnknown);
impl IUserNotification2 {
    pub unsafe fn SetBalloonInfo<P0, P1>(&self, psztitle: P0, psztext: P1, dwinfoflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBalloonInfo)(windows_core::Interface::as_raw(self), psztitle.param().abi(), psztext.param().abi(), dwinfoflags) }
    }
    pub unsafe fn SetBalloonRetry(&self, dwshowtime: u32, dwinterval: u32, cretrycount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBalloonRetry)(windows_core::Interface::as_raw(self), dwshowtime, dwinterval, cretrycount) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetIconInfo<P1>(&self, hicon: super::windef::HICON, psztooltip: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIconInfo)(windows_core::Interface::as_raw(self), hicon, psztooltip.param().abi()) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn Show<P0, P2>(&self, pqc: P0, dwcontinuepollinterval: u32, psink: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IQueryContinue>,
        P2: windows_core::Param<IUserNotificationCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), pqc.param().abi(), dwcontinuepollinterval, psink.param().abi()) }
    }
    pub unsafe fn PlaySound<P0>(&self, pszsoundname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PlaySound)(windows_core::Interface::as_raw(self), pszsoundname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotification2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBalloonInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetBalloonRetry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetIconInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HICON, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetIconInfo: usize,
    #[cfg(feature = "shobjidl_core")]
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    Show: usize,
    pub PlaySound: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
pub trait IUserNotification2_Impl: windows_core::IUnknownImpl {
    fn SetBalloonInfo(&self, psztitle: &windows_core::PCWSTR, psztext: &windows_core::PCWSTR, dwinfoflags: u32) -> windows_core::Result<()>;
    fn SetBalloonRetry(&self, dwshowtime: u32, dwinterval: u32, cretrycount: u32) -> windows_core::Result<()>;
    fn SetIconInfo(&self, hicon: super::windef::HICON, psztooltip: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Show(&self, pqc: windows_core::Ref<super::shobjidl_core::IQueryContinue>, dwcontinuepollinterval: u32, psink: windows_core::Ref<IUserNotificationCallback>) -> windows_core::Result<()>;
    fn PlaySound(&self, pszsoundname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
impl IUserNotification2_Vtbl {
    pub const fn new<Identity: IUserNotification2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBalloonInfo<Identity: IUserNotification2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztitle: windows_core::PCWSTR, psztext: windows_core::PCWSTR, dwinfoflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserNotification2_Impl::SetBalloonInfo(this, core::mem::transmute(&psztitle), core::mem::transmute(&psztext), core::mem::transmute_copy(&dwinfoflags)).into()
            }
        }
        unsafe extern "system" fn SetBalloonRetry<Identity: IUserNotification2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwshowtime: u32, dwinterval: u32, cretrycount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserNotification2_Impl::SetBalloonRetry(this, core::mem::transmute_copy(&dwshowtime), core::mem::transmute_copy(&dwinterval), core::mem::transmute_copy(&cretrycount)).into()
            }
        }
        unsafe extern "system" fn SetIconInfo<Identity: IUserNotification2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: super::windef::HICON, psztooltip: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserNotification2_Impl::SetIconInfo(this, core::mem::transmute_copy(&hicon), core::mem::transmute(&psztooltip)).into()
            }
        }
        unsafe extern "system" fn Show<Identity: IUserNotification2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqc: *mut core::ffi::c_void, dwcontinuepollinterval: u32, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserNotification2_Impl::Show(this, core::mem::transmute_copy(&pqc), core::mem::transmute_copy(&dwcontinuepollinterval), core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn PlaySound<Identity: IUserNotification2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsoundname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserNotification2_Impl::PlaySound(this, core::mem::transmute(&pszsoundname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBalloonInfo: SetBalloonInfo::<Identity, OFFSET>,
            SetBalloonRetry: SetBalloonRetry::<Identity, OFFSET>,
            SetIconInfo: SetIconInfo::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            PlaySound: PlaySound::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserNotification2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "windef"))]
impl windows_core::RuntimeName for IUserNotification2 {}
windows_core::imp::define_interface!(IUserNotificationCallback, IUserNotificationCallback_Vtbl, 0x19108294_0441_4aff_8013_fa0a730b0bea);
windows_core::imp::interface_hierarchy!(IUserNotificationCallback, windows_core::IUnknown);
impl IUserNotificationCallback {
    #[cfg(feature = "windef")]
    pub unsafe fn OnBalloonUserClick(&self, pt: *const super::windef::POINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnBalloonUserClick)(windows_core::Interface::as_raw(self), pt) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn OnLeftClick(&self, pt: *const super::windef::POINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLeftClick)(windows_core::Interface::as_raw(self), pt) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn OnContextMenu(&self, pt: *const super::windef::POINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnContextMenu)(windows_core::Interface::as_raw(self), pt) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub OnBalloonUserClick: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::POINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    OnBalloonUserClick: usize,
    #[cfg(feature = "windef")]
    pub OnLeftClick: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::POINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    OnLeftClick: usize,
    #[cfg(feature = "windef")]
    pub OnContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::POINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    OnContextMenu: usize,
}
#[cfg(feature = "windef")]
pub trait IUserNotificationCallback_Impl: windows_core::IUnknownImpl {
    fn OnBalloonUserClick(&self, pt: *const super::windef::POINT) -> windows_core::Result<()>;
    fn OnLeftClick(&self, pt: *const super::windef::POINT) -> windows_core::Result<()>;
    fn OnContextMenu(&self, pt: *const super::windef::POINT) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IUserNotificationCallback_Vtbl {
    pub const fn new<Identity: IUserNotificationCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnBalloonUserClick<Identity: IUserNotificationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: *const super::windef::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserNotificationCallback_Impl::OnBalloonUserClick(this, core::mem::transmute_copy(&pt)).into()
            }
        }
        unsafe extern "system" fn OnLeftClick<Identity: IUserNotificationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: *const super::windef::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserNotificationCallback_Impl::OnLeftClick(this, core::mem::transmute_copy(&pt)).into()
            }
        }
        unsafe extern "system" fn OnContextMenu<Identity: IUserNotificationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: *const super::windef::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserNotificationCallback_Impl::OnContextMenu(this, core::mem::transmute_copy(&pt)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnBalloonUserClick: OnBalloonUserClick::<Identity, OFFSET>,
            OnLeftClick: OnLeftClick::<Identity, OFFSET>,
            OnContextMenu: OnContextMenu::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserNotificationCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IUserNotificationCallback {}
windows_core::imp::define_interface!(IVisualProperties, IVisualProperties_Vtbl, 0xe693cf68_d967_4112_8763_99172aee5e5a);
windows_core::imp::interface_hierarchy!(IVisualProperties, windows_core::IUnknown);
impl IVisualProperties {
    #[cfg(feature = "windef")]
    pub unsafe fn SetWatermark(&self, hbmp: super::windef::HBITMAP, vpwf: VPWATERMARKFLAGS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWatermark)(windows_core::Interface::as_raw(self), hbmp, vpwf) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetColor(&self, vpcf: VPCOLORFLAGS, cr: super::windef::COLORREF) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), vpcf, cr) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetColor(&self, vpcf: VPCOLORFLAGS) -> windows_core::Result<super::windef::COLORREF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), vpcf, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetItemHeight(&self, cyiteminpixels: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetItemHeight)(windows_core::Interface::as_raw(self), cyiteminpixels) }
    }
    pub unsafe fn GetItemHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn SetFont(&self, plf: *const super::wingdi::LOGFONTW, bredraw: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFont)(windows_core::Interface::as_raw(self), plf, bredraw.into()) }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn GetFont(&self, plf: *mut super::wingdi::LOGFONTW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self), plf as _) }
    }
    pub unsafe fn SetTheme<P0, P1>(&self, pszsubappname: P0, pszsubidlist: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTheme)(windows_core::Interface::as_raw(self), pszsubappname.param().abi(), pszsubidlist.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub SetWatermark: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HBITMAP, VPWATERMARKFLAGS) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetWatermark: usize,
    #[cfg(feature = "windef")]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, VPCOLORFLAGS, super::windef::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetColor: usize,
    #[cfg(feature = "windef")]
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, VPCOLORFLAGS, *mut super::windef::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetColor: usize,
    pub SetItemHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetItemHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub SetFont: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wingdi::LOGFONTW, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    SetFont: usize,
    #[cfg(feature = "wingdi")]
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wingdi::LOGFONTW) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    GetFont: usize,
    pub SetTheme: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
pub trait IVisualProperties_Impl: windows_core::IUnknownImpl {
    fn SetWatermark(&self, hbmp: super::windef::HBITMAP, vpwf: VPWATERMARKFLAGS) -> windows_core::Result<()>;
    fn SetColor(&self, vpcf: VPCOLORFLAGS, cr: super::windef::COLORREF) -> windows_core::Result<()>;
    fn GetColor(&self, vpcf: VPCOLORFLAGS) -> windows_core::Result<super::windef::COLORREF>;
    fn SetItemHeight(&self, cyiteminpixels: i32) -> windows_core::Result<()>;
    fn GetItemHeight(&self) -> windows_core::Result<i32>;
    fn SetFont(&self, plf: *const super::wingdi::LOGFONTW, bredraw: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFont(&self, plf: *mut super::wingdi::LOGFONTW) -> windows_core::Result<()>;
    fn SetTheme(&self, pszsubappname: &windows_core::PCWSTR, pszsubidlist: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl IVisualProperties_Vtbl {
    pub const fn new<Identity: IVisualProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetWatermark<Identity: IVisualProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmp: super::windef::HBITMAP, vpwf: VPWATERMARKFLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualProperties_Impl::SetWatermark(this, core::mem::transmute_copy(&hbmp), core::mem::transmute_copy(&vpwf)).into()
            }
        }
        unsafe extern "system" fn SetColor<Identity: IVisualProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpcf: VPCOLORFLAGS, cr: super::windef::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualProperties_Impl::SetColor(this, core::mem::transmute_copy(&vpcf), core::mem::transmute_copy(&cr)).into()
            }
        }
        unsafe extern "system" fn GetColor<Identity: IVisualProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpcf: VPCOLORFLAGS, pcr: *mut super::windef::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualProperties_Impl::GetColor(this, core::mem::transmute_copy(&vpcf)) {
                    Ok(ok__) => {
                        pcr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetItemHeight<Identity: IVisualProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cyiteminpixels: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualProperties_Impl::SetItemHeight(this, core::mem::transmute_copy(&cyiteminpixels)).into()
            }
        }
        unsafe extern "system" fn GetItemHeight<Identity: IVisualProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cyiteminpixels: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualProperties_Impl::GetItemHeight(this) {
                    Ok(ok__) => {
                        cyiteminpixels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFont<Identity: IVisualProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plf: *const super::wingdi::LOGFONTW, bredraw: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualProperties_Impl::SetFont(this, core::mem::transmute_copy(&plf), core::mem::transmute_copy(&bredraw)).into()
            }
        }
        unsafe extern "system" fn GetFont<Identity: IVisualProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plf: *mut super::wingdi::LOGFONTW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualProperties_Impl::GetFont(this, core::mem::transmute_copy(&plf)).into()
            }
        }
        unsafe extern "system" fn SetTheme<Identity: IVisualProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubappname: windows_core::PCWSTR, pszsubidlist: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualProperties_Impl::SetTheme(this, core::mem::transmute(&pszsubappname), core::mem::transmute(&pszsubidlist)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetWatermark: SetWatermark::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            GetColor: GetColor::<Identity, OFFSET>,
            SetItemHeight: SetItemHeight::<Identity, OFFSET>,
            GetItemHeight: GetItemHeight::<Identity, OFFSET>,
            SetFont: SetFont::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
            SetTheme: SetTheme::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualProperties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IVisualProperties {}
windows_core::imp::define_interface!(IWebWizardExtension, IWebWizardExtension_Vtbl, 0x0e6b3f66_98d1_48c0_a222_fbde74e2fbc5);
impl core::ops::Deref for IWebWizardExtension {
    type Target = IWizardExtension;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebWizardExtension, windows_core::IUnknown, IWizardExtension);
impl IWebWizardExtension {
    pub unsafe fn SetInitialURL<P0>(&self, pszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInitialURL)(windows_core::Interface::as_raw(self), pszurl.param().abi()) }
    }
    pub unsafe fn SetErrorURL<P0>(&self, pszerrorurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetErrorURL)(windows_core::Interface::as_raw(self), pszerrorurl.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebWizardExtension_Vtbl {
    pub base__: IWizardExtension_Vtbl,
    pub SetInitialURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetErrorURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "prsht")]
pub trait IWebWizardExtension_Impl: IWizardExtension_Impl {
    fn SetInitialURL(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetErrorURL(&self, pszerrorurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "prsht")]
impl IWebWizardExtension_Vtbl {
    pub const fn new<Identity: IWebWizardExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInitialURL<Identity: IWebWizardExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebWizardExtension_Impl::SetInitialURL(this, core::mem::transmute(&pszurl)).into()
            }
        }
        unsafe extern "system" fn SetErrorURL<Identity: IWebWizardExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszerrorurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebWizardExtension_Impl::SetErrorURL(this, core::mem::transmute(&pszerrorurl)).into()
            }
        }
        Self {
            base__: IWizardExtension_Vtbl::new::<Identity, OFFSET>(),
            SetInitialURL: SetInitialURL::<Identity, OFFSET>,
            SetErrorURL: SetErrorURL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebWizardExtension as windows_core::Interface>::IID || iid == &<IWizardExtension as windows_core::Interface>::IID
    }
}
#[cfg(feature = "prsht")]
impl windows_core::RuntimeName for IWebWizardExtension {}
windows_core::imp::define_interface!(IWizardExtension, IWizardExtension_Vtbl, 0xc02ea696_86cc_491e_9b23_74394a0444a8);
windows_core::imp::interface_hierarchy!(IWizardExtension, windows_core::IUnknown);
impl IWizardExtension {
    #[cfg(feature = "prsht")]
    pub unsafe fn AddPages(&self, apages: &mut [super::prsht::HPROPSHEETPAGE], pnpagesadded: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddPages)(windows_core::Interface::as_raw(self), apages.as_mut_ptr(), apages.len().try_into().unwrap(), pnpagesadded as _) }
    }
    #[cfg(feature = "prsht")]
    pub unsafe fn GetFirstPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "prsht")]
    pub unsafe fn GetLastPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWizardExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "prsht")]
    pub AddPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::prsht::HPROPSHEETPAGE, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "prsht"))]
    AddPages: usize,
    #[cfg(feature = "prsht")]
    pub GetFirstPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "prsht"))]
    GetFirstPage: usize,
    #[cfg(feature = "prsht")]
    pub GetLastPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "prsht"))]
    GetLastPage: usize,
}
#[cfg(feature = "prsht")]
pub trait IWizardExtension_Impl: windows_core::IUnknownImpl {
    fn AddPages(&self, apages: *mut super::prsht::HPROPSHEETPAGE, cpages: u32, pnpagesadded: *mut u32) -> windows_core::Result<()>;
    fn GetFirstPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE>;
    fn GetLastPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE>;
}
#[cfg(feature = "prsht")]
impl IWizardExtension_Vtbl {
    pub const fn new<Identity: IWizardExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPages<Identity: IWizardExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apages: *mut super::prsht::HPROPSHEETPAGE, cpages: u32, pnpagesadded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWizardExtension_Impl::AddPages(this, core::mem::transmute_copy(&apages), core::mem::transmute_copy(&cpages), core::mem::transmute_copy(&pnpagesadded)).into()
            }
        }
        unsafe extern "system" fn GetFirstPage<Identity: IWizardExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phpage: *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWizardExtension_Impl::GetFirstPage(this) {
                    Ok(ok__) => {
                        phpage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastPage<Identity: IWizardExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phpage: *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWizardExtension_Impl::GetLastPage(this) {
                    Ok(ok__) => {
                        phpage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPages: AddPages::<Identity, OFFSET>,
            GetFirstPage: GetFirstPage::<Identity, OFFSET>,
            GetLastPage: GetLastPage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWizardExtension as windows_core::Interface>::IID
    }
}
#[cfg(feature = "prsht")]
impl windows_core::RuntimeName for IWizardExtension {}
windows_core::imp::define_interface!(IWizardSite, IWizardSite_Vtbl, 0x88960f5b_422f_4e7b_8013_73415381c3c3);
windows_core::imp::interface_hierarchy!(IWizardSite, windows_core::IUnknown);
impl IWizardSite {
    #[cfg(feature = "prsht")]
    pub unsafe fn GetPreviousPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "prsht")]
    pub unsafe fn GetNextPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "prsht")]
    pub unsafe fn GetCancelledPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCancelledPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWizardSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "prsht")]
    pub GetPreviousPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "prsht"))]
    GetPreviousPage: usize,
    #[cfg(feature = "prsht")]
    pub GetNextPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "prsht"))]
    GetNextPage: usize,
    #[cfg(feature = "prsht")]
    pub GetCancelledPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "prsht"))]
    GetCancelledPage: usize,
}
#[cfg(feature = "prsht")]
pub trait IWizardSite_Impl: windows_core::IUnknownImpl {
    fn GetPreviousPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE>;
    fn GetNextPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE>;
    fn GetCancelledPage(&self) -> windows_core::Result<super::prsht::HPROPSHEETPAGE>;
}
#[cfg(feature = "prsht")]
impl IWizardSite_Vtbl {
    pub const fn new<Identity: IWizardSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPreviousPage<Identity: IWizardSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phpage: *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWizardSite_Impl::GetPreviousPage(this) {
                    Ok(ok__) => {
                        phpage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextPage<Identity: IWizardSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phpage: *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWizardSite_Impl::GetNextPage(this) {
                    Ok(ok__) => {
                        phpage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCancelledPage<Identity: IWizardSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phpage: *mut super::prsht::HPROPSHEETPAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWizardSite_Impl::GetCancelledPage(this) {
                    Ok(ok__) => {
                        phpage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPreviousPage: GetPreviousPage::<Identity, OFFSET>,
            GetNextPage: GetNextPage::<Identity, OFFSET>,
            GetCancelledPage: GetCancelledPage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWizardSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "prsht")]
impl windows_core::RuntimeName for IWizardSite {}
pub const ImageProperties: windows_core::GUID = windows_core::GUID::from_u128(0x7ab770c7_0e23_4d7a_8aa2_19bfad479829);
pub const ImageRecompress: windows_core::GUID = windows_core::GUID::from_u128(0x6e33091c_d2f8_4740_b55e_2e11d1477a2c);
pub const InternetPrintOrdering: windows_core::GUID = windows_core::GUID::from_u128(0xadd36aa8_751a_4579_a266_d66f5202ccbb);
pub type LPVIEWSETTINGS = *mut i8;
pub const MergedCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0x8e827c11_33e7_4bc1_b242_8cd9a1c2b304);
#[repr(C)]
#[cfg(all(feature = "commctrl", feature = "shobjidl_core"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NSTCCUSTOMDRAW {
    pub psi: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellItem>>,
    pub uItemState: u32,
    pub nstcis: super::shobjidl_core::NSTCITEMSTATE,
    pub pszText: windows_core::PCWSTR,
    pub iImage: i32,
    pub himl: super::commctrl::HIMAGELIST,
    pub iLevel: i32,
    pub iIndent: i32,
}
pub const NSTCDHPOS_ONTOP: i32 = -1;
pub type NSTCECLICKTYPE = u32;
pub const NSTCECT_BUTTON: NSTCECLICKTYPE = 3;
pub const NSTCECT_DBLCLICK: NSTCECLICKTYPE = 4;
pub const NSTCECT_LBUTTON: NSTCECLICKTYPE = 1;
pub const NSTCECT_MBUTTON: NSTCECLICKTYPE = 2;
pub const NSTCECT_RBUTTON: NSTCECLICKTYPE = 3;
pub type NSTCEHITTEST = u32;
pub const NSTCEHT_NOWHERE: NSTCEHITTEST = 1;
pub const NSTCEHT_ONITEM: NSTCEHITTEST = 70;
pub const NSTCEHT_ONITEMBUTTON: NSTCEHITTEST = 16;
pub const NSTCEHT_ONITEMICON: NSTCEHITTEST = 2;
pub const NSTCEHT_ONITEMINDENT: NSTCEHITTEST = 8;
pub const NSTCEHT_ONITEMLABEL: NSTCEHITTEST = 4;
pub const NSTCEHT_ONITEMRIGHT: NSTCEHITTEST = 32;
pub const NSTCEHT_ONITEMSTATEICON: NSTCEHITTEST = 64;
pub const NSTCEHT_ONITEMTABBUTTON: NSTCEHITTEST = 4096;
pub const NSTCS2_ALLMASK: u32 = 7;
pub const NSTCS2_DEFAULT: NSTCSTYLE2 = 0;
pub const NSTCS2_DISPLAYPADDING: NSTCSTYLE2 = 4;
pub const NSTCS2_DISPLAYPINNEDONLY: NSTCSTYLE2 = 8;
pub const NSTCS2_INTERRUPTNOTIFICATIONS: NSTCSTYLE2 = 1;
pub const NSTCS2_SHOWNULLSPACEMENU: NSTCSTYLE2 = 2;
pub type NSTCSTYLE2 = u32;
pub const NTSCS2_NEVERINSERTNONENUMERATED: NSTCSTYLE2 = 32;
pub const NTSCS2_NOSINGLETONAUTOEXPAND: NSTCSTYLE2 = 16;
pub const NamespaceTreeControl: windows_core::GUID = windows_core::GUID::from_u128(0xae054212_3535_4430_83ed_d501aa6680e6);
pub const PROPSTR_EXTENSIONCOMPLETIONSTATE: windows_core::PCWSTR = windows_core::w!("ExtensionCompletionState");
pub const PreviousVersions: windows_core::GUID = windows_core::GUID::from_u128(0x596ab062_b4d2_4215_9f74_e9109b0a8153);
pub const PublishDropTarget: windows_core::GUID = windows_core::GUID::from_u128(0xcc6eeffb_43f6_46c5_9619_51d571967f7d);
pub const PublishingWizard: windows_core::GUID = windows_core::GUID::from_u128(0x6b33163c_76a5_4b6c_bf21_45de9cd503a1);
pub const QueryCancelAutoPlay: windows_core::GUID = windows_core::GUID::from_u128(0x331f1768_05a9_4ddd_b86e_dae34ddc998a);
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = 511;
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = 4;
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = 0;
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = 32;
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = 16;
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = 8;
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = 64;
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = 256;
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = 2;
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = 1;
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = 128;
pub const SHPWHF_ANYLOCATION: u32 = 256;
pub const SHPWHF_NOFILESELECTOR: u32 = 4;
pub const SHPWHF_NONETPLACECREATE: u32 = 2;
pub const SHPWHF_NORECOMPRESS: u32 = 1;
pub const SHPWHF_USEMRU: u32 = 8;
pub const SHPWHF_VALIDATEVIAWEBFOLDERS: u32 = 65536;
pub const SID_SCommandBarState: windows_core::GUID = windows_core::GUID::from_u128(0xb99eaa5c_3850_4400_bc33_2ce534048bf8);
pub const SV3CVW3_DEFAULT: SV3CVW3_FLAGS = 0;
pub type SV3CVW3_FLAGS = u32;
pub const SV3CVW3_FORCEFOLDERFLAGS: SV3CVW3_FLAGS = 4;
pub const SV3CVW3_FORCEVIEWMODE: SV3CVW3_FLAGS = 2;
pub const SV3CVW3_NONINTERACTIVE: SV3CVW3_FLAGS = 1;
pub type SYNC_ENGINE_STATE_FLAGS = u32;
pub const StartMenuPin: windows_core::GUID = windows_core::GUID::from_u128(0xa2a9545d_a0c2_42b4_9708_a0b2badd77c8);
pub const StorageProviderBanners: windows_core::GUID = windows_core::GUID::from_u128(0x7ccdf9f4_e576_455a_8bc7_f6ec68d6f063);
pub const TimeCategorizer: windows_core::GUID = windows_core::GUID::from_u128(0x3bb4118f_ddfd_4d30_a348_9fb5d6bf1afe);
pub const TrayBandSiteService: windows_core::GUID = windows_core::GUID::from_u128(0xf60ad0a0_e5e1_45cb_b51a_e15b9f8b2934);
pub const TrayDeskBand: windows_core::GUID = windows_core::GUID::from_u128(0xe6442437_6c68_4f52_94dd_2cfed267efb9);
pub type UNDOCK_REASON = i32;
pub const UR_MONITOR_DISCONNECT: UNDOCK_REASON = 1;
pub const UR_RESOLUTION_CHANGE: UNDOCK_REASON = 0;
pub const VPCF_BACKGROUND: VPCOLORFLAGS = 2;
pub const VPCF_SORTCOLUMN: VPCOLORFLAGS = 3;
pub const VPCF_SUBTEXT: VPCOLORFLAGS = 4;
pub const VPCF_TEXT: VPCOLORFLAGS = 1;
pub const VPCF_TEXTBACKGROUND: VPCOLORFLAGS = 5;
pub type VPCOLORFLAGS = i32;
pub type VPWATERMARKFLAGS = u32;
pub const VPWF_ALPHABLEND: VPWATERMARKFLAGS = 1;
pub const VPWF_DEFAULT: VPWATERMARKFLAGS = 0;
pub const VirtualDesktopManager: windows_core::GUID = windows_core::GUID::from_u128(0xaa509086_5ca9_4c25_8f95_589d3c07b48a);
pub const WebWizardHost: windows_core::GUID = windows_core::GUID::from_u128(0xc827f149_55c1_4d28_935e_57e47caed973);
pub type tagCDBURNINGEXTENSIONRET = i32;
