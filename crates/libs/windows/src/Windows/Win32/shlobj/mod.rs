#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DoPrivacyDlg<P1>(hwndowner: Option<super::windef::HWND>, pszurl: P1, pprivacyenum: *const IEnumPrivacyRecords, freportallsites: bool) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shdocvw.dll" "system" fn DoPrivacyDlg(hwndowner : super::windef::HWND, pszurl : windows_core::PCWSTR, pprivacyenum : *const IEnumPrivacyRecords, freportallsites : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DoPrivacyDlg(hwndowner.unwrap_or(core::mem::zeroed()) as _, pszurl.param().abi(), pprivacyenum, freportallsites.into()) }
}
#[inline]
pub unsafe fn ImportPrivacySettings<P0>(pszfilename: P0, pfparseprivacypreferences: *mut windows_core::BOOL, pfparsepersiterules: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shdocvw.dll" "system" fn ImportPrivacySettings(pszfilename : windows_core::PCWSTR, pfparseprivacypreferences : *mut windows_core::BOOL, pfparsepersiterules : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ImportPrivacySettings(pszfilename.param().abi(), pfparseprivacypreferences as _, pfparsepersiterules as _) }
}
#[inline]
pub unsafe fn PathIsSlowA<P0>(pszfile: P0, dwattr: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PathIsSlowA(pszfile : windows_core::PCSTR, dwattr : u32) -> windows_core::BOOL);
    unsafe { PathIsSlowA(pszfile.param().abi(), dwattr) }
}
#[inline]
pub unsafe fn PathIsSlowW<P0>(pszfile: P0, dwattr: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PathIsSlowW(pszfile : windows_core::PCWSTR, dwattr : u32) -> windows_core::BOOL);
    unsafe { PathIsSlowW(pszfile.param().abi(), dwattr) }
}
#[inline]
pub unsafe fn PathQualify(psz: windows_core::PWSTR) {
    windows_core::link!("shell32.dll" "system" fn PathQualify(psz : windows_core::PWSTR));
    unsafe { PathQualify(core::mem::transmute(psz)) }
}
#[cfg(feature = "shlobj_core")]
#[inline]
pub unsafe fn SHChangeNotifyRegisterThread(status: super::shlobj_core::SCNRT_STATUS) {
    windows_core::link!("shell32.dll" "system" fn SHChangeNotifyRegisterThread(status : super::shlobj_core::SCNRT_STATUS));
    unsafe { SHChangeNotifyRegisterThread(status) }
}
#[cfg(all(feature = "minwindef", feature = "shlobj_core"))]
#[inline]
pub unsafe fn SHCreatePropSheetExtArray<P1>(hkey: super::minwindef::HKEY, pszsubkey: P1, max_iface: u32) -> super::shlobj_core::HPSXA
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHCreatePropSheetExtArray(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR, max_iface : u32) -> super::shlobj_core::HPSXA);
    unsafe { SHCreatePropSheetExtArray(hkey, pszsubkey.param().abi(), max_iface) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn SHCreateQueryCancelAutoPlayMoniker() -> windows_core::Result<super::objidl::IMoniker> {
    windows_core::link!("shell32.dll" "system" fn SHCreateQueryCancelAutoPlayMoniker(ppmoniker : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCreateQueryCancelAutoPlayMoniker(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn SHMultiFileProperties<P0>(pdtobj: P0, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IDataObject>,
{
    windows_core::link!("shell32.dll" "system" fn SHMultiFileProperties(pdtobj : *mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SHMultiFileProperties(pdtobj.param().abi(), dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shobjidl_core"))]
#[inline]
pub unsafe fn SHOpenPropSheetW<P0, P4, P5, P6>(pszcaption: P0, ahkeys: Option<&[super::minwindef::HKEY]>, pclsiddefault: Option<*const windows_core::GUID>, pdtobj: P4, psb: P5, pstartpage: P6) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<super::objidl::IDataObject>,
    P5: windows_core::Param<super::shobjidl_core::IShellBrowser>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHOpenPropSheetW(pszcaption : windows_core::PCWSTR, ahkeys : *const super::minwindef::HKEY, ckeys : u32, pclsiddefault : *const windows_core::GUID, pdtobj : *mut core::ffi::c_void, psb : *mut core::ffi::c_void, pstartpage : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SHOpenPropSheetW(pszcaption.param().abi(), core::mem::transmute(ahkeys.map_or(core::ptr::null(), |slice| slice.as_ptr())), ahkeys.map_or(0, |slice| slice.len().try_into().unwrap()), pclsiddefault.unwrap_or(core::mem::zeroed()) as _, pdtobj.param().abi(), psb.param().abi(), pstartpage.param().abi()) }
}
#[cfg(all(feature = "urlmon", feature = "windef"))]
#[inline]
pub unsafe fn SoftwareUpdateMessageBox<P1>(hwnd: Option<super::windef::HWND>, pszdistunit: P1, dwflags: u32, psdi: Option<*mut super::urlmon::SOFTDISTINFO>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shdocvw.dll" "system" fn SoftwareUpdateMessageBox(hwnd : super::windef::HWND, pszdistunit : windows_core::PCWSTR, dwflags : u32, psdi : *mut super::urlmon::SOFTDISTINFO) -> u32);
    unsafe { SoftwareUpdateMessageBox(hwnd.unwrap_or(core::mem::zeroed()) as _, pszdistunit.param().abi(), dwflags, psdi.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AASHELLMENUFILENAME {
    pub cbTotal: i16,
    pub rgbReserved: [u8; 12],
    pub szFileName: [u16; 1],
}
impl Default for AASHELLMENUFILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AASHELLMENUITEM {
    pub lpReserved1: *mut core::ffi::c_void,
    pub iReserved: i32,
    pub uiReserved: u32,
    pub lpName: LPAASHELLMENUFILENAME,
    pub psz: windows_core::PWSTR,
}
impl Default for AASHELLMENUITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BANDINFOSFB {
    pub dwMask: u32,
    pub dwStateMask: u32,
    pub dwState: u32,
    pub crBkgnd: super::windef::COLORREF,
    pub crBtnLt: super::windef::COLORREF,
    pub crBtnDk: super::windef::COLORREF,
    pub wViewMode: u16,
    pub wAlign: u16,
    pub psf: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellFolder>>,
    pub pidl: super::shtypes::LPITEMIDLIST,
}
pub const BMICON_LARGE: i32 = 0;
pub const BMICON_SMALL: i32 = 1;
pub const CLOSEPROPS_DISCARD: u32 = 1;
pub const CLOSEPROPS_NONE: u32 = 0;
pub const DBCID_CLSIDOFBAR: i32 = 2;
pub const DBCID_EMPTY: i32 = 0;
pub const DBCID_GETBAR: i32 = 4;
pub const DBCID_ONDRAG: i32 = 1;
pub const DBCID_RESIZE: i32 = 3;
pub const DBCID_UPDATESIZE: i32 = 5;
pub const DBC_GS_IDEAL: u32 = 0;
pub const DBC_GS_SIZEDOWN: u32 = 1;
pub const DBC_HIDE: u32 = 0;
pub const DBC_SHOW: u32 = 1;
pub const DBC_SHOWOBSCURE: u32 = 2;
pub const DWFAF_AUTOHIDE: u32 = 16;
pub const DWFAF_GROUP1: u32 = 2;
pub const DWFAF_GROUP2: u32 = 4;
pub const DWFAF_HIDDEN: u32 = 1;
pub const DWFRF_DELETECONFIGDATA: u32 = 1;
pub const DWFRF_NORMAL: u32 = 0;
pub const FCIDM_STATUS: u32 = 40961;
pub const FCIDM_TOOLBAR: u32 = 40960;
pub const GADOF_DIRTY: u32 = 1;
pub const GETPROPS_NONE: u32 = 0;
windows_core::imp::define_interface!(IADesktopP2, IADesktopP2_Vtbl, 0xb22754e2_4574_11d1_9888_006097deacf9);
windows_core::imp::interface_hierarchy!(IADesktopP2, windows_core::IUnknown);
impl IADesktopP2 {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReReadWallpaper(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReReadWallpaper)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetADObjectFlags)(windows_core::Interface::as_raw(self), pdwflags as _, dwmask) }
    }
    pub unsafe fn UpdateAllDesktopSubscriptions(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateAllDesktopSubscriptions)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn MakeDynamicChanges<P0>(&self, poleobj: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oleidl::IOleObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).MakeDynamicChanges)(windows_core::Interface::as_raw(self), poleobj.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADesktopP2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub ReReadWallpaper: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetADObjectFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub UpdateAllDesktopSubscriptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oleidl")]
    pub MakeDynamicChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    MakeDynamicChanges: usize,
}
#[cfg(feature = "oleidl")]
pub trait IADesktopP2_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn ReReadWallpaper(&self) -> windows_core::Result<()>;
    fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> windows_core::Result<()>;
    fn UpdateAllDesktopSubscriptions(&self) -> windows_core::Result<()>;
    fn MakeDynamicChanges(&self, poleobj: windows_core::Ref<super::oleidl::IOleObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "oleidl")]
impl IADesktopP2_Vtbl {
    pub const fn new<Identity: IADesktopP2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::Release(this)
            }
        }
        unsafe extern "system" fn ReReadWallpaper<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::ReReadWallpaper(this).into()
            }
        }
        unsafe extern "system" fn GetADObjectFlags<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::GetADObjectFlags(this, core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&dwmask)).into()
            }
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::UpdateAllDesktopSubscriptions(this).into()
            }
        }
        unsafe extern "system" fn MakeDynamicChanges<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poleobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::MakeDynamicChanges(this, core::mem::transmute_copy(&poleobj)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            ReReadWallpaper: ReReadWallpaper::<Identity, OFFSET>,
            GetADObjectFlags: GetADObjectFlags::<Identity, OFFSET>,
            UpdateAllDesktopSubscriptions: UpdateAllDesktopSubscriptions::<Identity, OFFSET>,
            MakeDynamicChanges: MakeDynamicChanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADesktopP2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oleidl")]
impl windows_core::RuntimeName for IADesktopP2 {}
windows_core::imp::define_interface!(IActiveDesktopP, IActiveDesktopP_Vtbl, 0x52502ee0_ec80_11d0_89ab_00c04fc2972d);
windows_core::imp::interface_hierarchy!(IActiveDesktopP, windows_core::IUnknown);
impl IActiveDesktopP {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetSafeMode(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSafeMode)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn EnsureUpdateHTML(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnsureUpdateHTML)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetScheme<P0>(&self, pwszschemename: P0, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScheme)(windows_core::Interface::as_raw(self), pwszschemename.param().abi(), dwflags) }
    }
    pub unsafe fn GetScheme(&self, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScheme)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszschemename), pdwcchbuffer as _, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveDesktopP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub SetSafeMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnsureUpdateHTML: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32, u32) -> windows_core::HRESULT,
}
pub trait IActiveDesktopP_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn SetSafeMode(&self, dwflags: u32) -> windows_core::Result<()>;
    fn EnsureUpdateHTML(&self) -> windows_core::Result<()>;
    fn SetScheme(&self, pwszschemename: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetScheme(&self, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> windows_core::Result<()>;
}
impl IActiveDesktopP_Vtbl {
    pub const fn new<Identity: IActiveDesktopP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::Release(this)
            }
        }
        unsafe extern "system" fn SetSafeMode<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::SetSafeMode(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn EnsureUpdateHTML<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::EnsureUpdateHTML(this).into()
            }
        }
        unsafe extern "system" fn SetScheme<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszschemename: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::SetScheme(this, core::mem::transmute(&pwszschemename), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetScheme<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::GetScheme(this, core::mem::transmute_copy(&pwszschemename), core::mem::transmute_copy(&pdwcchbuffer), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            SetSafeMode: SetSafeMode::<Identity, OFFSET>,
            EnsureUpdateHTML: EnsureUpdateHTML::<Identity, OFFSET>,
            SetScheme: SetScheme::<Identity, OFFSET>,
            GetScheme: GetScheme::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveDesktopP as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveDesktopP {}
windows_core::imp::define_interface!(IBanneredBar, IBanneredBar_Vtbl, 0x596a9a94_013e_11d1_8d34_00a0c90f2719);
windows_core::imp::interface_hierarchy!(IBanneredBar, windows_core::IUnknown);
impl IBanneredBar {
    pub unsafe fn SetIconSize(&self, iicon: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIconSize)(windows_core::Interface::as_raw(self), iicon) }
    }
    pub unsafe fn GetIconSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIconSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetBitmap(&self, hbitmap: super::windef::HBITMAP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBitmap)(windows_core::Interface::as_raw(self), hbitmap) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetBitmap(&self) -> windows_core::Result<super::windef::HBITMAP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmap)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBanneredBar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIconSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetIconSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetBitmap: usize,
    #[cfg(feature = "windef")]
    pub GetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetBitmap: usize,
}
#[cfg(feature = "windef")]
pub trait IBanneredBar_Impl: windows_core::IUnknownImpl {
    fn SetIconSize(&self, iicon: u32) -> windows_core::Result<()>;
    fn GetIconSize(&self) -> windows_core::Result<u32>;
    fn SetBitmap(&self, hbitmap: super::windef::HBITMAP) -> windows_core::Result<()>;
    fn GetBitmap(&self) -> windows_core::Result<super::windef::HBITMAP>;
}
#[cfg(feature = "windef")]
impl IBanneredBar_Vtbl {
    pub const fn new<Identity: IBanneredBar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIconSize<Identity: IBanneredBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iicon: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBanneredBar_Impl::SetIconSize(this, core::mem::transmute_copy(&iicon)).into()
            }
        }
        unsafe extern "system" fn GetIconSize<Identity: IBanneredBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piicon: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBanneredBar_Impl::GetIconSize(this) {
                    Ok(ok__) => {
                        piicon.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBitmap<Identity: IBanneredBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbitmap: super::windef::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBanneredBar_Impl::SetBitmap(this, core::mem::transmute_copy(&hbitmap)).into()
            }
        }
        unsafe extern "system" fn GetBitmap<Identity: IBanneredBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phbitmap: *mut super::windef::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBanneredBar_Impl::GetBitmap(this) {
                    Ok(ok__) => {
                        phbitmap.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIconSize: SetIconSize::<Identity, OFFSET>,
            GetIconSize: GetIconSize::<Identity, OFFSET>,
            SetBitmap: SetBitmap::<Identity, OFFSET>,
            GetBitmap: GetBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBanneredBar as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IBanneredBar {}
windows_core::imp::define_interface!(IColumnProvider, IColumnProvider_Vtbl, 0xe8025004_1c42_11d2_be2c_00a0c9a83da1);
windows_core::imp::interface_hierarchy!(IColumnProvider, windows_core::IUnknown);
impl IColumnProvider {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Initialize(&self, psci: *const SHCOLUMNINIT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), psci) }
    }
    #[cfg(all(feature = "shtypes", feature = "wtypes"))]
    pub unsafe fn GetColumnInfo(&self, dwindex: u32, psci: *mut SHCOLUMNINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetColumnInfo)(windows_core::Interface::as_raw(self), dwindex, psci as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetItemData(&self, pscid: *const super::shtypes::SHCOLUMNID, pscd: *const SHCOLUMNDATA) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemData)(windows_core::Interface::as_raw(self), pscid, pscd, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IColumnProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const SHCOLUMNINIT) -> windows_core::HRESULT,
    #[cfg(all(feature = "shtypes", feature = "wtypes"))]
    pub GetColumnInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SHCOLUMNINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shtypes", feature = "wtypes")))]
    GetColumnInfo: usize,
    #[cfg(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
    pub GetItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::SHCOLUMNID, *const SHCOLUMNDATA, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase")))]
    GetItemData: usize,
}
#[cfg(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
pub trait IColumnProvider_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn Initialize(&self, psci: *const SHCOLUMNINIT) -> windows_core::Result<()>;
    fn GetColumnInfo(&self, dwindex: u32, psci: *mut SHCOLUMNINFO) -> windows_core::Result<()>;
    fn GetItemData(&self, pscid: *const super::shtypes::SHCOLUMNID, pscd: *const SHCOLUMNDATA) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
impl IColumnProvider_Vtbl {
    pub const fn new<Identity: IColumnProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IColumnProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnProvider_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IColumnProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnProvider_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IColumnProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnProvider_Impl::Release(this)
            }
        }
        unsafe extern "system" fn Initialize<Identity: IColumnProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psci: *const SHCOLUMNINIT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnProvider_Impl::Initialize(this, core::mem::transmute_copy(&psci)).into()
            }
        }
        unsafe extern "system" fn GetColumnInfo<Identity: IColumnProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, psci: *mut SHCOLUMNINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnProvider_Impl::GetColumnInfo(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&psci)).into()
            }
        }
        unsafe extern "system" fn GetItemData<Identity: IColumnProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscid: *const super::shtypes::SHCOLUMNID, pscd: *const SHCOLUMNDATA, pvardata: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IColumnProvider_Impl::GetItemData(this, core::mem::transmute_copy(&pscid), core::mem::transmute_copy(&pscd)) {
                    Ok(ok__) => {
                        pvardata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            GetColumnInfo: GetColumnInfo::<Identity, OFFSET>,
            GetItemData: GetItemData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IColumnProvider {}
windows_core::imp::define_interface!(ICopyHookA, ICopyHookA_Vtbl, 0x000214ef_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICopyHookA, windows_core::IUnknown);
impl ICopyHookA {
    #[cfg(feature = "windef")]
    pub unsafe fn CopyCallback<P3, P5>(&self, hwnd: Option<super::windef::HWND>, wfunc: u32, wflags: u32, pszsrcfile: P3, dwsrcattribs: u32, pszdestfile: P5, dwdestattribs: u32) -> u32
    where
        P3: windows_core::Param<windows_core::PCSTR>,
        P5: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyCallback)(windows_core::Interface::as_raw(self), hwnd.unwrap_or(core::mem::zeroed()) as _, wfunc, wflags, pszsrcfile.param().abi(), dwsrcattribs, pszdestfile.param().abi(), dwdestattribs) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICopyHookA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub CopyCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, u32, windows_core::PCSTR, u32, windows_core::PCSTR, u32) -> u32,
    #[cfg(not(feature = "windef"))]
    CopyCallback: usize,
}
#[cfg(feature = "windef")]
pub trait ICopyHookA_Impl: windows_core::IUnknownImpl {
    fn CopyCallback(&self, hwnd: super::windef::HWND, wfunc: u32, wflags: u32, pszsrcfile: &windows_core::PCSTR, dwsrcattribs: u32, pszdestfile: &windows_core::PCSTR, dwdestattribs: u32) -> u32;
}
#[cfg(feature = "windef")]
impl ICopyHookA_Vtbl {
    pub const fn new<Identity: ICopyHookA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopyCallback<Identity: ICopyHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, wfunc: u32, wflags: u32, pszsrcfile: windows_core::PCSTR, dwsrcattribs: u32, pszdestfile: windows_core::PCSTR, dwdestattribs: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICopyHookA_Impl::CopyCallback(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wfunc), core::mem::transmute_copy(&wflags), core::mem::transmute(&pszsrcfile), core::mem::transmute_copy(&dwsrcattribs), core::mem::transmute(&pszdestfile), core::mem::transmute_copy(&dwdestattribs))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CopyCallback: CopyCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICopyHookA as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ICopyHookA {}
windows_core::imp::define_interface!(ICopyHookW, ICopyHookW_Vtbl, 0x000214fc_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICopyHookW, windows_core::IUnknown);
impl ICopyHookW {
    #[cfg(feature = "windef")]
    pub unsafe fn CopyCallback<P3, P5>(&self, hwnd: Option<super::windef::HWND>, wfunc: u32, wflags: u32, pszsrcfile: P3, dwsrcattribs: u32, pszdestfile: P5, dwdestattribs: u32) -> u32
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyCallback)(windows_core::Interface::as_raw(self), hwnd.unwrap_or(core::mem::zeroed()) as _, wfunc, wflags, pszsrcfile.param().abi(), dwsrcattribs, pszdestfile.param().abi(), dwdestattribs) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICopyHookW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub CopyCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, u32, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32) -> u32,
    #[cfg(not(feature = "windef"))]
    CopyCallback: usize,
}
#[cfg(feature = "windef")]
pub trait ICopyHookW_Impl: windows_core::IUnknownImpl {
    fn CopyCallback(&self, hwnd: super::windef::HWND, wfunc: u32, wflags: u32, pszsrcfile: &windows_core::PCWSTR, dwsrcattribs: u32, pszdestfile: &windows_core::PCWSTR, dwdestattribs: u32) -> u32;
}
#[cfg(feature = "windef")]
impl ICopyHookW_Vtbl {
    pub const fn new<Identity: ICopyHookW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopyCallback<Identity: ICopyHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, wfunc: u32, wflags: u32, pszsrcfile: windows_core::PCWSTR, dwsrcattribs: u32, pszdestfile: windows_core::PCWSTR, dwdestattribs: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICopyHookW_Impl::CopyCallback(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wfunc), core::mem::transmute_copy(&wflags), core::mem::transmute(&pszsrcfile), core::mem::transmute_copy(&dwsrcattribs), core::mem::transmute(&pszdestfile), core::mem::transmute_copy(&dwdestattribs))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CopyCallback: CopyCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICopyHookW as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ICopyHookW {}
windows_core::imp::define_interface!(ICurrentWorkingDirectory, ICurrentWorkingDirectory_Vtbl, 0x91956d21_9276_11d1_921a_006097df5bd4);
windows_core::imp::interface_hierarchy!(ICurrentWorkingDirectory, windows_core::IUnknown);
impl ICurrentWorkingDirectory {
    pub unsafe fn GetDirectory(&self, pwzpath: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute(pwzpath.as_ptr()), pwzpath.len().try_into().unwrap()) }
    }
    pub unsafe fn SetDirectory<P0>(&self, pwzpath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDirectory)(windows_core::Interface::as_raw(self), pwzpath.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentWorkingDirectory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub SetDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ICurrentWorkingDirectory_Impl: windows_core::IUnknownImpl {
    fn GetDirectory(&self, pwzpath: windows_core::PWSTR, cchsize: u32) -> windows_core::Result<()>;
    fn SetDirectory(&self, pwzpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ICurrentWorkingDirectory_Vtbl {
    pub const fn new<Identity: ICurrentWorkingDirectory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDirectory<Identity: ICurrentWorkingDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzpath: windows_core::PWSTR, cchsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICurrentWorkingDirectory_Impl::GetDirectory(this, core::mem::transmute_copy(&pwzpath), core::mem::transmute_copy(&cchsize)).into()
            }
        }
        unsafe extern "system" fn SetDirectory<Identity: ICurrentWorkingDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICurrentWorkingDirectory_Impl::SetDirectory(this, core::mem::transmute(&pwzpath)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDirectory: GetDirectory::<Identity, OFFSET>,
            SetDirectory: SetDirectory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICurrentWorkingDirectory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICurrentWorkingDirectory {}
pub const IDC_OFFLINE_HAND: u32 = 103;
pub const IDC_PANTOOL_HAND_CLOSED: u32 = 105;
pub const IDC_PANTOOL_HAND_OPEN: u32 = 104;
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IDeskBarClient, IDeskBarClient_Vtbl, 0xeb0fe175_1a3a_11d0_89b3_00a0c90a90ac);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IDeskBarClient {
    type Target = super::oleidl::IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IDeskBarClient, windows_core::IUnknown, super::oleidl::IOleWindow);
#[cfg(feature = "oleidl")]
impl IDeskBarClient {
    pub unsafe fn SetDeskBarSite<P0>(&self, punksite: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDeskBarSite)(windows_core::Interface::as_raw(self), punksite.param().abi()) }
    }
    pub unsafe fn SetModeDBC(&self, dwmode: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModeDBC)(windows_core::Interface::as_raw(self), dwmode) }
    }
    pub unsafe fn UIActivateDBC(&self, dwstate: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UIActivateDBC)(windows_core::Interface::as_raw(self), dwstate) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetSize(&self, dwwhich: u32) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), dwwhich, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oleidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDeskBarClient_Vtbl {
    pub base__: super::oleidl::IOleWindow_Vtbl,
    pub SetDeskBarSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetModeDBC: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UIActivateDBC: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetSize: usize,
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
pub trait IDeskBarClient_Impl: super::oleidl::IOleWindow_Impl {
    fn SetDeskBarSite(&self, punksite: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetModeDBC(&self, dwmode: u32) -> windows_core::Result<()>;
    fn UIActivateDBC(&self, dwstate: u32) -> windows_core::Result<()>;
    fn GetSize(&self, dwwhich: u32) -> windows_core::Result<super::windef::RECT>;
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
impl IDeskBarClient_Vtbl {
    pub const fn new<Identity: IDeskBarClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDeskBarSite<Identity: IDeskBarClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeskBarClient_Impl::SetDeskBarSite(this, core::mem::transmute_copy(&punksite)).into()
            }
        }
        unsafe extern "system" fn SetModeDBC<Identity: IDeskBarClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeskBarClient_Impl::SetModeDBC(this, core::mem::transmute_copy(&dwmode)).into()
            }
        }
        unsafe extern "system" fn UIActivateDBC<Identity: IDeskBarClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeskBarClient_Impl::UIActivateDBC(this, core::mem::transmute_copy(&dwstate)).into()
            }
        }
        unsafe extern "system" fn GetSize<Identity: IDeskBarClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwhich: u32, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeskBarClient_Impl::GetSize(this, core::mem::transmute_copy(&dwwhich)) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oleidl::IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            SetDeskBarSite: SetDeskBarSite::<Identity, OFFSET>,
            SetModeDBC: SetModeDBC::<Identity, OFFSET>,
            UIActivateDBC: UIActivateDBC::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeskBarClient as windows_core::Interface>::IID || iid == &<super::oleidl::IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
impl windows_core::RuntimeName for IDeskBarClient {}
windows_core::imp::define_interface!(IDocViewSite, IDocViewSite_Vtbl, 0x87d605e0_c511_11cf_89a9_00a0c9054129);
windows_core::imp::interface_hierarchy!(IDocViewSite, windows_core::IUnknown);
impl IDocViewSite {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn OnSetTitle(&self, pvtitle: *const super::oaidl::VARIANTARG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSetTitle)(windows_core::Interface::as_raw(self), core::mem::transmute(pvtitle)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocViewSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub OnSetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANTARG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    OnSetTitle: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDocViewSite_Impl: windows_core::IUnknownImpl {
    fn OnSetTitle(&self, pvtitle: *const super::oaidl::VARIANTARG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IDocViewSite_Vtbl {
    pub const fn new<Identity: IDocViewSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetTitle<Identity: IDocViewSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtitle: *const super::oaidl::VARIANTARG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDocViewSite_Impl::OnSetTitle(this, core::mem::transmute_copy(&pvtitle)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSetTitle: OnSetTitle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDocViewSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDocViewSite {}
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IDockingWindowFrame, IDockingWindowFrame_Vtbl, 0x47d2657a_7b27_11d0_8ca9_00a0c92dbfe8);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IDockingWindowFrame {
    type Target = super::oleidl::IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IDockingWindowFrame, windows_core::IUnknown, super::oleidl::IOleWindow);
#[cfg(feature = "oleidl")]
impl IDockingWindowFrame {
    pub unsafe fn AddToolbar<P0, P1>(&self, punksrc: P0, pwszitem: P1, dwaddflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddToolbar)(windows_core::Interface::as_raw(self), punksrc.param().abi(), pwszitem.param().abi(), dwaddflags) }
    }
    pub unsafe fn RemoveToolbar<P0>(&self, punksrc: P0, dwremoveflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveToolbar)(windows_core::Interface::as_raw(self), punksrc.param().abi(), dwremoveflags) }
    }
    pub unsafe fn FindToolbar<P0>(&self, pwszitem: P0, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindToolbar)(windows_core::Interface::as_raw(self), pwszitem.param().abi(), riid, ppv as _) }
    }
}
#[cfg(feature = "oleidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDockingWindowFrame_Vtbl {
    pub base__: super::oleidl::IOleWindow_Vtbl,
    pub AddToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub RemoveToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FindToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
pub trait IDockingWindowFrame_Impl: super::oleidl::IOleWindow_Impl {
    fn AddToolbar(&self, punksrc: windows_core::Ref<windows_core::IUnknown>, pwszitem: &windows_core::PCWSTR, dwaddflags: u32) -> windows_core::Result<()>;
    fn RemoveToolbar(&self, punksrc: windows_core::Ref<windows_core::IUnknown>, dwremoveflags: u32) -> windows_core::Result<()>;
    fn FindToolbar(&self, pwszitem: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
impl IDockingWindowFrame_Vtbl {
    pub const fn new<Identity: IDockingWindowFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddToolbar<Identity: IDockingWindowFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksrc: *mut core::ffi::c_void, pwszitem: windows_core::PCWSTR, dwaddflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockingWindowFrame_Impl::AddToolbar(this, core::mem::transmute_copy(&punksrc), core::mem::transmute(&pwszitem), core::mem::transmute_copy(&dwaddflags)).into()
            }
        }
        unsafe extern "system" fn RemoveToolbar<Identity: IDockingWindowFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksrc: *mut core::ffi::c_void, dwremoveflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockingWindowFrame_Impl::RemoveToolbar(this, core::mem::transmute_copy(&punksrc), core::mem::transmute_copy(&dwremoveflags)).into()
            }
        }
        unsafe extern "system" fn FindToolbar<Identity: IDockingWindowFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszitem: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockingWindowFrame_Impl::FindToolbar(this, core::mem::transmute(&pwszitem), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: super::oleidl::IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            AddToolbar: AddToolbar::<Identity, OFFSET>,
            RemoveToolbar: RemoveToolbar::<Identity, OFFSET>,
            FindToolbar: FindToolbar::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDockingWindowFrame as windows_core::Interface>::IID || iid == &<super::oleidl::IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
impl windows_core::RuntimeName for IDockingWindowFrame {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IEnumPrivacyRecords(pub u8);
windows_core::imp::define_interface!(IInitializeObject, IInitializeObject_Vtbl, 0x4622ad16_ff23_11d0_8d34_00a0c90f2719);
windows_core::imp::interface_hierarchy!(IInitializeObject, windows_core::IUnknown);
impl IInitializeObject {
    pub unsafe fn Initialize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInitializeObject_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<()>;
}
impl IInitializeObject_Vtbl {
    pub const fn new<Identity: IInitializeObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IInitializeObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeObject_Impl::Initialize(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInitializeObject {}
windows_core::imp::define_interface!(INewShortcutHookA, INewShortcutHookA_Vtbl, 0x000214e1_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(INewShortcutHookA, windows_core::IUnknown);
impl INewShortcutHookA {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetReferent<P0>(&self, pcszreferent: P0, hwnd: Option<super::windef::HWND>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReferent)(windows_core::Interface::as_raw(self), pcszreferent.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetReferent(&self, pszreferent: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReferent)(windows_core::Interface::as_raw(self), core::mem::transmute(pszreferent.as_ptr()), pszreferent.len().try_into().unwrap()) }
    }
    pub unsafe fn SetFolder<P0>(&self, pcszfolder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFolder)(windows_core::Interface::as_raw(self), pcszfolder.param().abi()) }
    }
    pub unsafe fn GetFolder(&self, pszfolder: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), core::mem::transmute(pszfolder.as_ptr()), pszfolder.len().try_into().unwrap()) }
    }
    pub unsafe fn GetName(&self, pszname: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()) }
    }
    pub unsafe fn GetExtension(&self, pszextension: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), core::mem::transmute(pszextension.as_ptr()), pszextension.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INewShortcutHookA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "windef")]
    pub SetReferent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetReferent: usize,
    pub GetReferent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, i32) -> windows_core::HRESULT,
    pub SetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> windows_core::HRESULT,
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, i32) -> windows_core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait INewShortcutHookA_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn SetReferent(&self, pcszreferent: &windows_core::PCSTR, hwnd: super::windef::HWND) -> windows_core::Result<()>;
    fn GetReferent(&self, pszreferent: windows_core::PSTR, cchreferent: i32) -> windows_core::Result<()>;
    fn SetFolder(&self, pcszfolder: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn GetFolder(&self, pszfolder: windows_core::PSTR, cchfolder: i32) -> windows_core::Result<()>;
    fn GetName(&self, pszname: windows_core::PSTR, cchname: i32) -> windows_core::Result<()>;
    fn GetExtension(&self, pszextension: windows_core::PSTR, cchextension: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl INewShortcutHookA_Vtbl {
    pub const fn new<Identity: INewShortcutHookA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookA_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookA_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookA_Impl::Release(this)
            }
        }
        unsafe extern "system" fn SetReferent<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszreferent: windows_core::PCSTR, hwnd: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookA_Impl::SetReferent(this, core::mem::transmute(&pcszreferent), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn GetReferent<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszreferent: windows_core::PSTR, cchreferent: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookA_Impl::GetReferent(this, core::mem::transmute_copy(&pszreferent), core::mem::transmute_copy(&cchreferent)).into()
            }
        }
        unsafe extern "system" fn SetFolder<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszfolder: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookA_Impl::SetFolder(this, core::mem::transmute(&pcszfolder)).into()
            }
        }
        unsafe extern "system" fn GetFolder<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfolder: windows_core::PSTR, cchfolder: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookA_Impl::GetFolder(this, core::mem::transmute_copy(&pszfolder), core::mem::transmute_copy(&cchfolder)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PSTR, cchname: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookA_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchname)).into()
            }
        }
        unsafe extern "system" fn GetExtension<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszextension: windows_core::PSTR, cchextension: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookA_Impl::GetExtension(this, core::mem::transmute_copy(&pszextension), core::mem::transmute_copy(&cchextension)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            SetReferent: SetReferent::<Identity, OFFSET>,
            GetReferent: GetReferent::<Identity, OFFSET>,
            SetFolder: SetFolder::<Identity, OFFSET>,
            GetFolder: GetFolder::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetExtension: GetExtension::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INewShortcutHookA as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for INewShortcutHookA {}
windows_core::imp::define_interface!(INewShortcutHookW, INewShortcutHookW_Vtbl, 0x000214f7_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(INewShortcutHookW, windows_core::IUnknown);
impl INewShortcutHookW {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetReferent<P0>(&self, pcszreferent: P0, hwnd: Option<super::windef::HWND>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReferent)(windows_core::Interface::as_raw(self), pcszreferent.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetReferent(&self, pszreferent: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReferent)(windows_core::Interface::as_raw(self), core::mem::transmute(pszreferent.as_ptr()), pszreferent.len().try_into().unwrap()) }
    }
    pub unsafe fn SetFolder<P0>(&self, pcszfolder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFolder)(windows_core::Interface::as_raw(self), pcszfolder.param().abi()) }
    }
    pub unsafe fn GetFolder(&self, pszfolder: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), core::mem::transmute(pszfolder.as_ptr()), pszfolder.len().try_into().unwrap()) }
    }
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()) }
    }
    pub unsafe fn GetExtension(&self, pszextension: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), core::mem::transmute(pszextension.as_ptr()), pszextension.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INewShortcutHookW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "windef")]
    pub SetReferent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetReferent: usize,
    pub GetReferent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub SetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait INewShortcutHookW_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn SetReferent(&self, pcszreferent: &windows_core::PCWSTR, hwnd: super::windef::HWND) -> windows_core::Result<()>;
    fn GetReferent(&self, pszreferent: windows_core::PWSTR, cchreferent: i32) -> windows_core::Result<()>;
    fn SetFolder(&self, pcszfolder: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFolder(&self, pszfolder: windows_core::PWSTR, cchfolder: i32) -> windows_core::Result<()>;
    fn GetName(&self, pszname: windows_core::PWSTR, cchname: i32) -> windows_core::Result<()>;
    fn GetExtension(&self, pszextension: windows_core::PWSTR, cchextension: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl INewShortcutHookW_Vtbl {
    pub const fn new<Identity: INewShortcutHookW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookW_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookW_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookW_Impl::Release(this)
            }
        }
        unsafe extern "system" fn SetReferent<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszreferent: windows_core::PCWSTR, hwnd: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookW_Impl::SetReferent(this, core::mem::transmute(&pcszreferent), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn GetReferent<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszreferent: windows_core::PWSTR, cchreferent: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookW_Impl::GetReferent(this, core::mem::transmute_copy(&pszreferent), core::mem::transmute_copy(&cchreferent)).into()
            }
        }
        unsafe extern "system" fn SetFolder<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszfolder: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookW_Impl::SetFolder(this, core::mem::transmute(&pcszfolder)).into()
            }
        }
        unsafe extern "system" fn GetFolder<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfolder: windows_core::PWSTR, cchfolder: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookW_Impl::GetFolder(this, core::mem::transmute_copy(&pszfolder), core::mem::transmute_copy(&cchfolder)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchname: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookW_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchname)).into()
            }
        }
        unsafe extern "system" fn GetExtension<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszextension: windows_core::PWSTR, cchextension: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INewShortcutHookW_Impl::GetExtension(this, core::mem::transmute_copy(&pszextension), core::mem::transmute_copy(&cchextension)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            SetReferent: SetReferent::<Identity, OFFSET>,
            GetReferent: GetReferent::<Identity, OFFSET>,
            SetFolder: SetFolder::<Identity, OFFSET>,
            GetFolder: GetFolder::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetExtension: GetExtension::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INewShortcutHookW as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for INewShortcutHookW {}
pub const ISFBVIEWMODE_LARGEICONS: u32 = 2;
pub const ISFBVIEWMODE_SMALLICONS: u32 = 1;
pub const ISFB_MASK_BKCOLOR: u32 = 2;
pub const ISFB_MASK_COLORS: u32 = 32;
pub const ISFB_MASK_IDLIST: u32 = 16;
pub const ISFB_MASK_SHELLFOLDER: u32 = 8;
pub const ISFB_MASK_STATE: u32 = 1;
pub const ISFB_MASK_VIEWMODE: u32 = 4;
pub const ISFB_STATE_ALLOWRENAME: u32 = 2;
pub const ISFB_STATE_BTNMINSIZE: u32 = 256;
pub const ISFB_STATE_CHANNELBAR: u32 = 16;
pub const ISFB_STATE_DEBOSSED: u32 = 1;
pub const ISFB_STATE_DEFAULT: u32 = 0;
pub const ISFB_STATE_FULLOPEN: u32 = 64;
pub const ISFB_STATE_NONAMESORT: u32 = 128;
pub const ISFB_STATE_NOSHOWTEXT: u32 = 4;
pub const ISFB_STATE_QLINKSMODE: u32 = 32;
windows_core::imp::define_interface!(IShellFolderBand, IShellFolderBand_Vtbl, 0x7fe80cc8_c247_11d0_b93a_00a0c90312e1);
windows_core::imp::interface_hierarchy!(IShellFolderBand, windows_core::IUnknown);
impl IShellFolderBand {
    #[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
    pub unsafe fn InitializeSFB<P0>(&self, psf: P0, pidl: Option<*const super::shtypes::ITEMIDLIST>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellFolder>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeSFB)(windows_core::Interface::as_raw(self), psf.param().abi(), pidl.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub unsafe fn SetBandInfoSFB(&self, pbi: *const BANDINFOSFB) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBandInfoSFB)(windows_core::Interface::as_raw(self), core::mem::transmute(pbi)) }
    }
    #[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub unsafe fn GetBandInfoSFB(&self, pbi: *mut BANDINFOSFB) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBandInfoSFB)(windows_core::Interface::as_raw(self), core::mem::transmute(pbi)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellFolderBand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shobjidl_core", feature = "shtypes"))]
    pub InitializeSFB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shobjidl_core", feature = "shtypes")))]
    InitializeSFB: usize,
    #[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub SetBandInfoSFB: unsafe extern "system" fn(*mut core::ffi::c_void, *const BANDINFOSFB) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef")))]
    SetBandInfoSFB: usize,
    #[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub GetBandInfoSFB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BANDINFOSFB) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef")))]
    GetBandInfoSFB: usize,
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub trait IShellFolderBand_Impl: windows_core::IUnknownImpl {
    fn InitializeSFB(&self, psf: windows_core::Ref<super::shobjidl_core::IShellFolder>, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<()>;
    fn SetBandInfoSFB(&self, pbi: *const BANDINFOSFB) -> windows_core::Result<()>;
    fn GetBandInfoSFB(&self, pbi: *mut BANDINFOSFB) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl IShellFolderBand_Vtbl {
    pub const fn new<Identity: IShellFolderBand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeSFB<Identity: IShellFolderBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psf: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderBand_Impl::InitializeSFB(this, core::mem::transmute_copy(&psf), core::mem::transmute_copy(&pidl)).into()
            }
        }
        unsafe extern "system" fn SetBandInfoSFB<Identity: IShellFolderBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbi: *const BANDINFOSFB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderBand_Impl::SetBandInfoSFB(this, core::mem::transmute_copy(&pbi)).into()
            }
        }
        unsafe extern "system" fn GetBandInfoSFB<Identity: IShellFolderBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbi: *mut BANDINFOSFB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderBand_Impl::GetBandInfoSFB(this, core::mem::transmute_copy(&pbi)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeSFB: InitializeSFB::<Identity, OFFSET>,
            SetBandInfoSFB: SetBandInfoSFB::<Identity, OFFSET>,
            GetBandInfoSFB: GetBandInfoSFB::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellFolderBand as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl windows_core::RuntimeName for IShellFolderBand {}
windows_core::imp::define_interface!(IThumbnailCapture, IThumbnailCapture_Vtbl, 0x4ea39266_7211_409f_b622_f63dbd16c533);
windows_core::imp::interface_hierarchy!(IThumbnailCapture, windows_core::IUnknown);
impl IThumbnailCapture {
    #[cfg(feature = "windef")]
    pub unsafe fn CaptureThumbnail<P1>(&self, pmaxsize: *const super::windef::SIZE, phtmldoc2: P1) -> windows_core::Result<super::windef::HBITMAP>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaptureThumbnail)(windows_core::Interface::as_raw(self), pmaxsize, phtmldoc2.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailCapture_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub CaptureThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::SIZE, *mut core::ffi::c_void, *mut super::windef::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CaptureThumbnail: usize,
}
#[cfg(feature = "windef")]
pub trait IThumbnailCapture_Impl: windows_core::IUnknownImpl {
    fn CaptureThumbnail(&self, pmaxsize: *const super::windef::SIZE, phtmldoc2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<super::windef::HBITMAP>;
}
#[cfg(feature = "windef")]
impl IThumbnailCapture_Vtbl {
    pub const fn new<Identity: IThumbnailCapture_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CaptureThumbnail<Identity: IThumbnailCapture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxsize: *const super::windef::SIZE, phtmldoc2: *mut core::ffi::c_void, phbmthumbnail: *mut super::windef::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IThumbnailCapture_Impl::CaptureThumbnail(this, core::mem::transmute_copy(&pmaxsize), core::mem::transmute_copy(&phtmldoc2)) {
                    Ok(ok__) => {
                        phbmthumbnail.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CaptureThumbnail: CaptureThumbnail::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IThumbnailCapture as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IThumbnailCapture {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAASHELLMENUFILENAME(pub *mut AASHELLMENUFILENAME);
impl LPAASHELLMENUFILENAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPAASHELLMENUFILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAASHELLMENUITEM(pub *mut AASHELLMENUITEM);
impl LPAASHELLMENUITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPAASHELLMENUITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCSHCOLUMNDATA(pub *const SHCOLUMNDATA);
impl LPCSHCOLUMNDATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCSHCOLUMNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCSHCOLUMNINFO(pub *const SHCOLUMNINFO);
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
impl LPCSHCOLUMNINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
impl Default for LPCSHCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCSHCOLUMNINIT(pub *const SHCOLUMNINIT);
impl LPCSHCOLUMNINIT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCSHCOLUMNINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSFV_SETITEMPOS(pub *mut SFV_SETITEMPOS);
#[cfg(all(feature = "shtypes", feature = "windef"))]
impl LPSFV_SETITEMPOS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
impl Default for LPSFV_SETITEMPOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSHCOLUMNDATA(pub *mut SHCOLUMNDATA);
impl LPSHCOLUMNDATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSHCOLUMNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSHCOLUMNINFO(pub *mut SHCOLUMNINFO);
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
impl LPSHCOLUMNINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
impl Default for LPSHCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSHCOLUMNINIT(pub *mut SHCOLUMNINIT);
impl LPSHCOLUMNINIT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSHCOLUMNINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSHChangeProductKeyAsIDList(pub *mut SHChangeProductKeyAsIDList);
impl LPSHChangeProductKeyAsIDList {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSHChangeProductKeyAsIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTBINFO(pub *mut TBINFO);
impl LPTBINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTBINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPENPROPS_INHIBITPIF: u32 = 32768;
pub const OPENPROPS_NONE: u32 = 0;
pub const PANE_NAVIGATION: u32 = 5;
pub const PANE_NONE: u32 = 4294967295;
pub const PANE_OFFLINE: u32 = 2;
pub const PANE_PRINTER: u32 = 3;
pub const PANE_PRIVACY: u32 = 7;
pub const PANE_PROGRESS: u32 = 6;
pub const PANE_SSL: u32 = 4;
pub const PANE_ZONE: u32 = 1;
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBANDINFOSFB(pub *mut BANDINFOSFB);
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl PBANDINFOSFB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl Default for PBANDINFOSFB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCSFV_SETITEMPOS(pub *const SFV_SETITEMPOS);
#[cfg(all(feature = "shtypes", feature = "windef"))]
impl PCSFV_SETITEMPOS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
impl Default for PCSFV_SETITEMPOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCHEME_CREATE: u32 = 128;
pub const SCHEME_DISPLAY: u32 = 1;
pub const SCHEME_DONOTUSE: u32 = 64;
pub const SCHEME_EDIT: u32 = 2;
pub const SCHEME_GLOBAL: u32 = 8;
pub const SCHEME_LOCAL: u32 = 4;
pub const SCHEME_REFRESH: u32 = 16;
pub const SCHEME_UPDATE: u32 = 32;
pub const SETPROPS_NONE: u32 = 0;
pub const SFBID_PIDLCHANGED: i32 = 0;
pub const SFVM_ADDOBJECT: u32 = 3;
pub const SFVM_GETSELECTEDOBJECTS: u32 = 9;
pub const SFVM_REARRANGE: u32 = 1;
pub const SFVM_REMOVEOBJECT: u32 = 6;
pub const SFVM_SETCLIPBOARD: u32 = 16;
pub const SFVM_SETITEMPOS: u32 = 14;
pub const SFVM_SETPOINTS: u32 = 23;
pub const SFVM_UPDATEOBJECT: u32 = 7;
#[repr(C)]
#[cfg(all(feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SFV_SETITEMPOS {
    pub pidl: super::shtypes::LPCITEMIDLIST,
    pub pt: super::windef::POINT,
}
pub const SHCDF_UPDATEITEM: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHCOLUMNDATA {
    pub dwFlags: u32,
    pub dwFileAttributes: u32,
    pub dwReserved: u32,
    pub pwszExt: *mut u16,
    pub wszFile: [u16; 260],
}
impl Default for SHCOLUMNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub struct SHCOLUMNINFO {
    pub scid: super::shtypes::SHCOLUMNID,
    pub vt: super::wtypes::VARTYPE,
    pub fmt: u32,
    pub cChars: u32,
    pub csFlags: u32,
    pub wszTitle: [u16; 80],
    pub wszDescription: [u16; 128],
}
#[cfg(all(feature = "shtypes", feature = "wtypes"))]
impl Default for SHCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHCOLUMNINIT {
    pub dwFlags: u32,
    pub dwReserved: u32,
    pub wszFolder: [u16; 260],
}
impl Default for SHCOLUMNINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SHChangeProductKeyAsIDList {
    pub cb: u16,
    pub wszProductKey: [u16; 39],
    pub cbZero: u16,
}
impl Default for SHChangeProductKeyAsIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSM_CLEAR: u32 = 0;
pub const SSM_REFRESH: u32 = 2;
pub const SSM_SET: u32 = 1;
pub const SSM_UPDATE: u32 = 4;
pub const TBIF_APPEND: u32 = 0;
pub const TBIF_DEFAULT: u32 = 0;
pub const TBIF_INTERNETBAR: u32 = 65536;
pub const TBIF_NOTOOLBAR: u32 = 196608;
pub const TBIF_PREPEND: u32 = 1;
pub const TBIF_REPLACE: u32 = 2;
pub const TBIF_STANDARDTOOLBAR: u32 = 131072;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TBINFO {
    pub cbuttons: u32,
    pub uFlags: u32,
}
