#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DoPrivacyDlg<P1>(hwndowner: Option<super::HWND>, pszurl: P1, pprivacyenum: *const IEnumPrivacyRecords, freportallsites: bool) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shdocvw.dll" "system" fn DoPrivacyDlg(hwndowner : super::HWND, pszurl : windows_core::PCWSTR, pprivacyenum : *const IEnumPrivacyRecords, freportallsites : windows_core::BOOL) -> windows_core::HRESULT);
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
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathIsSlowA<P0>(pszfile: P0, dwattr: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PathIsSlowA(pszfile : windows_core::PCSTR, dwattr : super::DWORD) -> windows_core::BOOL);
    unsafe { PathIsSlowA(pszfile.param().abi(), dwattr) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PathIsSlowW<P0>(pszfile: P0, dwattr: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn PathIsSlowW(pszfile : windows_core::PCWSTR, dwattr : super::DWORD) -> windows_core::BOOL);
    unsafe { PathIsSlowW(pszfile.param().abi(), dwattr) }
}
#[inline]
pub unsafe fn PathQualify(psz: windows_core::PWSTR) {
    windows_core::link!("shell32.dll" "system" fn PathQualify(psz : windows_core::PWSTR));
    unsafe { PathQualify(psz) }
}
#[cfg(feature = "shlobj_core")]
#[inline]
pub unsafe fn SHChangeNotifyRegisterThread(status: super::SCNRT_STATUS) {
    windows_core::link!("shell32.dll" "system" fn SHChangeNotifyRegisterThread(status : super::SCNRT_STATUS));
    unsafe { SHChangeNotifyRegisterThread(status) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "shlobj_core"))]
#[inline]
pub unsafe fn SHCreatePropSheetExtArray<P1>(hkey: super::HKEY, pszsubkey: P1, max_iface: super::UINT) -> super::HPSXA
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHCreatePropSheetExtArray(hkey : super::HKEY, pszsubkey : windows_core::PCWSTR, max_iface : super::UINT) -> super::HPSXA);
    unsafe { SHCreatePropSheetExtArray(hkey, pszsubkey.param().abi(), max_iface) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn SHCreateQueryCancelAutoPlayMoniker(ppmoniker: *mut Option<super::IMoniker>) -> windows_core::HRESULT {
    windows_core::link!("shell32.dll" "system" fn SHCreateQueryCancelAutoPlayMoniker(ppmoniker : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SHCreateQueryCancelAutoPlayMoniker(core::mem::transmute(ppmoniker)) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn SHMultiFileProperties<P0>(pdtobj: P0, dwflags: super::DWORD) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDataObject>,
{
    windows_core::link!("shell32.dll" "system" fn SHMultiFileProperties(pdtobj : *mut core::ffi::c_void, dwflags : super::DWORD) -> windows_core::HRESULT);
    unsafe { SHMultiFileProperties(pdtobj.param().abi(), dwflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "shobjidl_core"))]
#[inline]
pub unsafe fn SHOpenPropSheetW<P0, P4, P5, P6>(pszcaption: P0, ahkeys: Option<&[super::HKEY]>, pclsiddefault: Option<*const super::CLSID>, pdtobj: P4, psb: P5, pstartpage: P6) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<super::IDataObject>,
    P5: windows_core::Param<super::IShellBrowser>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn SHOpenPropSheetW(pszcaption : windows_core::PCWSTR, ahkeys : *const super::HKEY, ckeys : super::UINT, pclsiddefault : *const super::CLSID, pdtobj : *mut core::ffi::c_void, psb : *mut core::ffi::c_void, pstartpage : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SHOpenPropSheetW(pszcaption.param().abi(), ahkeys.map_or(core::ptr::null(), |slice| slice.as_ptr()), ahkeys.map_or(0, |slice| slice.len().try_into().unwrap()), pclsiddefault.unwrap_or(core::mem::zeroed()) as _, pdtobj.param().abi(), psb.param().abi(), pstartpage.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "urlmon", feature = "windef"))]
#[inline]
pub unsafe fn SoftwareUpdateMessageBox<P1>(hwnd: Option<super::HWND>, pszdistunit: P1, dwflags: super::DWORD, psdi: Option<super::LPSOFTDISTINFO>) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shdocvw.dll" "system" fn SoftwareUpdateMessageBox(hwnd : super::HWND, pszdistunit : windows_core::PCWSTR, dwflags : super::DWORD, psdi : super::LPSOFTDISTINFO) -> super::DWORD);
    unsafe { SoftwareUpdateMessageBox(hwnd.unwrap_or(core::mem::zeroed()) as _, pszdistunit.param().abi(), dwflags, psdi.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AASHELLMENUFILENAME {
    pub cbTotal: super::SHORT,
    pub rgbReserved: [super::BYTE; 12],
    pub szFileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AASHELLMENUFILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct AASHELLMENUITEM {
    pub lpReserved1: *mut core::ffi::c_void,
    pub iReserved: i32,
    pub uiReserved: super::UINT,
    pub lpName: LPAASHELLMENUFILENAME,
    pub psz: windows_core::PWSTR,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Default)]
pub struct BANDINFOSFB {
    pub dwMask: super::DWORD,
    pub dwStateMask: super::DWORD,
    pub dwState: super::DWORD,
    pub crBkgnd: super::COLORREF,
    pub crBtnLt: super::COLORREF,
    pub crBtnDk: super::COLORREF,
    pub wViewMode: super::WORD,
    pub wAlign: super::WORD,
    pub psf: core::mem::ManuallyDrop<Option<super::IShellFolder>>,
    pub pidl: super::LPITEMIDLIST,
}
pub const BMICON_LARGE: i32 = 0;
pub const BMICON_SMALL: i32 = 1;
pub const CLOSEPROPS_DISCARD: i32 = 1;
pub const CLOSEPROPS_NONE: i32 = 0;
pub const DBCID_CLSIDOFBAR: i32 = 2;
pub const DBCID_EMPTY: i32 = 0;
pub const DBCID_GETBAR: i32 = 4;
pub const DBCID_ONDRAG: i32 = 1;
pub const DBCID_RESIZE: i32 = 3;
pub const DBCID_UPDATESIZE: i32 = 5;
pub const DBC_GS_IDEAL: i32 = 0;
pub const DBC_GS_SIZEDOWN: i32 = 1;
pub const DBC_HIDE: i32 = 0;
pub const DBC_SHOW: i32 = 1;
pub const DBC_SHOWOBSCURE: i32 = 2;
pub const DWFAF_AUTOHIDE: i32 = 16;
pub const DWFAF_GROUP1: i32 = 2;
pub const DWFAF_GROUP2: i32 = 4;
pub const DWFAF_HIDDEN: i32 = 1;
pub const DWFRF_DELETECONFIGDATA: i32 = 1;
pub const DWFRF_NORMAL: i32 = 0;
pub const FCIDM_STATUS: i32 = 40961;
pub const FCIDM_TOOLBAR: i32 = 40960;
pub const GADOF_DIRTY: i32 = 1;
pub const GETPROPS_NONE: i32 = 0;
windows_core::imp::define_interface!(IADesktopP2, IADesktopP2_Vtbl, 0xb22754e2_4574_11d1_9888_006097deacf9);
windows_core::imp::interface_hierarchy!(IADesktopP2, windows_core::IUnknown);
impl IADesktopP2 {
    pub unsafe fn ReReadWallpaper(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReReadWallpaper)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetADObjectFlags(&self, pdwflags: *mut super::DWORD, dwmask: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetADObjectFlags)(windows_core::Interface::as_raw(self), pdwflags as _, dwmask) }
    }
    pub unsafe fn UpdateAllDesktopSubscriptions(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateAllDesktopSubscriptions)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn MakeDynamicChanges<P0>(&self, poleobj: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IOleObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).MakeDynamicChanges)(windows_core::Interface::as_raw(self), poleobj.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IADesktopP2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReReadWallpaper: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetADObjectFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetADObjectFlags: usize,
    pub UpdateAllDesktopSubscriptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oleidl")]
    pub MakeDynamicChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    MakeDynamicChanges: usize,
}
#[cfg(all(feature = "oleidl", feature = "shared"))]
pub trait IADesktopP2_Impl: windows_core::IUnknownImpl {
    fn ReReadWallpaper(&self) -> windows_core::Result<()>;
    fn GetADObjectFlags(&self, pdwflags: *mut super::DWORD, dwmask: super::DWORD) -> windows_core::Result<()>;
    fn UpdateAllDesktopSubscriptions(&self) -> windows_core::Result<()>;
    fn MakeDynamicChanges(&self, poleobj: windows_core::Ref<super::IOleObject>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oleidl", feature = "shared"))]
impl IADesktopP2_Vtbl {
    pub const fn new<Identity: IADesktopP2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReReadWallpaper<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADesktopP2_Impl::ReReadWallpaper(this).into()
            }
        }
        unsafe extern "system" fn GetADObjectFlags<Identity: IADesktopP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut super::DWORD, dwmask: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oleidl", feature = "shared"))]
impl windows_core::RuntimeName for IADesktopP2 {}
windows_core::imp::define_interface!(IActiveDesktopP, IActiveDesktopP_Vtbl, 0x52502ee0_ec80_11d0_89ab_00c04fc2972d);
windows_core::imp::interface_hierarchy!(IActiveDesktopP, windows_core::IUnknown);
impl IActiveDesktopP {
    #[cfg(feature = "shared")]
    pub unsafe fn SetSafeMode(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSafeMode)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn EnsureUpdateHTML(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnsureUpdateHTML)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetScheme<P0>(&self, pwszschemename: P0, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScheme)(windows_core::Interface::as_raw(self), pwszschemename.param().abi(), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetScheme(&self, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScheme)(windows_core::Interface::as_raw(self), pwszschemename, pdwcchbuffer as _, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveDesktopP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetSafeMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSafeMode: usize,
    pub EnsureUpdateHTML: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetScheme: usize,
    #[cfg(feature = "shared")]
    pub GetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetScheme: usize,
}
#[cfg(feature = "shared")]
pub trait IActiveDesktopP_Impl: windows_core::IUnknownImpl {
    fn SetSafeMode(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn EnsureUpdateHTML(&self) -> windows_core::Result<()>;
    fn SetScheme(&self, pwszschemename: &windows_core::PCWSTR, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetScheme(&self, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut super::DWORD, dwflags: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IActiveDesktopP_Vtbl {
    pub const fn new<Identity: IActiveDesktopP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSafeMode<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetScheme<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszschemename: windows_core::PCWSTR, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::SetScheme(this, core::mem::transmute(&pwszschemename), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetScheme<Identity: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszschemename: windows_core::PWSTR, pdwcchbuffer: *mut super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveDesktopP_Impl::GetScheme(this, core::mem::transmute_copy(&pwszschemename), core::mem::transmute_copy(&pdwcchbuffer), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IActiveDesktopP {}
windows_core::imp::define_interface!(IBanneredBar, IBanneredBar_Vtbl, 0x596a9a94_013e_11d1_8d34_00a0c90f2719);
windows_core::imp::interface_hierarchy!(IBanneredBar, windows_core::IUnknown);
impl IBanneredBar {
    #[cfg(feature = "shared")]
    pub unsafe fn SetIconSize(&self, iicon: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIconSize)(windows_core::Interface::as_raw(self), iicon) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetIconSize(&self, piicon: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIconSize)(windows_core::Interface::as_raw(self), piicon as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetBitmap(&self, hbitmap: super::HBITMAP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBitmap)(windows_core::Interface::as_raw(self), hbitmap) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetBitmap(&self, phbitmap: *mut super::HBITMAP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBitmap)(windows_core::Interface::as_raw(self), phbitmap as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBanneredBar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetIconSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetIconSize: usize,
    #[cfg(feature = "shared")]
    pub GetIconSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetIconSize: usize,
    #[cfg(feature = "windef")]
    pub SetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, super::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetBitmap: usize,
    #[cfg(feature = "windef")]
    pub GetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetBitmap: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IBanneredBar_Impl: windows_core::IUnknownImpl {
    fn SetIconSize(&self, iicon: super::DWORD) -> windows_core::Result<()>;
    fn GetIconSize(&self, piicon: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetBitmap(&self, hbitmap: super::HBITMAP) -> windows_core::Result<()>;
    fn GetBitmap(&self, phbitmap: *mut super::HBITMAP) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IBanneredBar_Vtbl {
    pub const fn new<Identity: IBanneredBar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIconSize<Identity: IBanneredBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iicon: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBanneredBar_Impl::SetIconSize(this, core::mem::transmute_copy(&iicon)).into()
            }
        }
        unsafe extern "system" fn GetIconSize<Identity: IBanneredBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piicon: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBanneredBar_Impl::GetIconSize(this, core::mem::transmute_copy(&piicon)).into()
            }
        }
        unsafe extern "system" fn SetBitmap<Identity: IBanneredBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbitmap: super::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBanneredBar_Impl::SetBitmap(this, core::mem::transmute_copy(&hbitmap)).into()
            }
        }
        unsafe extern "system" fn GetBitmap<Identity: IBanneredBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phbitmap: *mut super::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBanneredBar_Impl::GetBitmap(this, core::mem::transmute_copy(&phbitmap)).into()
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IBanneredBar {}
windows_core::imp::define_interface!(IColumnProvider, IColumnProvider_Vtbl, 0xe8025004_1c42_11d2_be2c_00a0c9a83da1);
windows_core::imp::interface_hierarchy!(IColumnProvider, windows_core::IUnknown);
impl IColumnProvider {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Initialize(&self, psci: LPCSHCOLUMNINIT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), psci) }
    }
    #[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn GetColumnInfo(&self, dwindex: super::DWORD, psci: *mut SHCOLUMNINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetColumnInfo)(windows_core::Interface::as_raw(self), dwindex, psci as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetItemData(&self, pscid: super::LPCSHCOLUMNID, pscd: LPCSHCOLUMNDATA, pvardata: *mut super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemData)(windows_core::Interface::as_raw(self), pscid, pscd, pvardata) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IColumnProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, LPCSHCOLUMNINIT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Initialize: usize,
    #[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
    pub GetColumnInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut SHCOLUMNINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes")))]
    GetColumnInfo: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetItemData: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCSHCOLUMNID, LPCSHCOLUMNDATA, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetItemData: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IColumnProvider_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, psci: LPCSHCOLUMNINIT) -> windows_core::Result<()>;
    fn GetColumnInfo(&self, dwindex: super::DWORD, psci: *mut SHCOLUMNINFO) -> windows_core::Result<()>;
    fn GetItemData(&self, pscid: super::LPCSHCOLUMNID, pscd: LPCSHCOLUMNDATA, pvardata: *mut super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IColumnProvider_Vtbl {
    pub const fn new<Identity: IColumnProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IColumnProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psci: LPCSHCOLUMNINIT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnProvider_Impl::Initialize(this, core::mem::transmute_copy(&psci)).into()
            }
        }
        unsafe extern "system" fn GetColumnInfo<Identity: IColumnProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, psci: *mut SHCOLUMNINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnProvider_Impl::GetColumnInfo(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&psci)).into()
            }
        }
        unsafe extern "system" fn GetItemData<Identity: IColumnProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscid: super::LPCSHCOLUMNID, pscd: LPCSHCOLUMNDATA, pvardata: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IColumnProvider_Impl::GetItemData(this, core::mem::transmute_copy(&pscid), core::mem::transmute_copy(&pscd), core::mem::transmute_copy(&pvardata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetColumnInfo: GetColumnInfo::<Identity, OFFSET>,
            GetItemData: GetItemData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IColumnProvider {}
windows_core::imp::define_interface!(ICopyHookA, ICopyHookA_Vtbl, 0x000214ef_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICopyHookA, windows_core::IUnknown);
impl ICopyHookA {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn CopyCallback<P3, P5>(&self, hwnd: Option<super::HWND>, wfunc: super::UINT, wflags: super::UINT, pszsrcfile: P3, dwsrcattribs: super::DWORD, pszdestfile: P5, dwdestattribs: super::DWORD) -> super::UINT
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
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub CopyCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::UINT, super::UINT, windows_core::PCSTR, super::DWORD, windows_core::PCSTR, super::DWORD) -> super::UINT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    CopyCallback: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait ICopyHookA_Impl: windows_core::IUnknownImpl {
    fn CopyCallback(&self, hwnd: super::HWND, wfunc: super::UINT, wflags: super::UINT, pszsrcfile: &windows_core::PCSTR, dwsrcattribs: super::DWORD, pszdestfile: &windows_core::PCSTR, dwdestattribs: super::DWORD) -> super::UINT;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl ICopyHookA_Vtbl {
    pub const fn new<Identity: ICopyHookA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopyCallback<Identity: ICopyHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, wfunc: super::UINT, wflags: super::UINT, pszsrcfile: windows_core::PCSTR, dwsrcattribs: super::DWORD, pszdestfile: windows_core::PCSTR, dwdestattribs: super::DWORD) -> super::UINT {
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for ICopyHookA {}
windows_core::imp::define_interface!(ICopyHookW, ICopyHookW_Vtbl, 0x000214fc_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICopyHookW, windows_core::IUnknown);
impl ICopyHookW {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn CopyCallback<P3, P5>(&self, hwnd: Option<super::HWND>, wfunc: super::UINT, wflags: super::UINT, pszsrcfile: P3, dwsrcattribs: super::DWORD, pszdestfile: P5, dwdestattribs: super::DWORD) -> super::UINT
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
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub CopyCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::UINT, super::UINT, windows_core::PCWSTR, super::DWORD, windows_core::PCWSTR, super::DWORD) -> super::UINT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    CopyCallback: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait ICopyHookW_Impl: windows_core::IUnknownImpl {
    fn CopyCallback(&self, hwnd: super::HWND, wfunc: super::UINT, wflags: super::UINT, pszsrcfile: &windows_core::PCWSTR, dwsrcattribs: super::DWORD, pszdestfile: &windows_core::PCWSTR, dwdestattribs: super::DWORD) -> super::UINT;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl ICopyHookW_Vtbl {
    pub const fn new<Identity: ICopyHookW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopyCallback<Identity: ICopyHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, wfunc: super::UINT, wflags: super::UINT, pszsrcfile: windows_core::PCWSTR, dwsrcattribs: super::DWORD, pszdestfile: windows_core::PCWSTR, dwdestattribs: super::DWORD) -> super::UINT {
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for ICopyHookW {}
windows_core::imp::define_interface!(ICurrentWorkingDirectory, ICurrentWorkingDirectory_Vtbl, 0x91956d21_9276_11d1_921a_006097df5bd4);
windows_core::imp::interface_hierarchy!(ICurrentWorkingDirectory, windows_core::IUnknown);
impl ICurrentWorkingDirectory {
    #[cfg(feature = "shared")]
    pub unsafe fn GetDirectory(&self, pwzpath: windows_core::PWSTR, cchsize: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDirectory)(windows_core::Interface::as_raw(self), pwzpath, cchsize) }
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
    #[cfg(feature = "shared")]
    pub GetDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDirectory: usize,
    pub SetDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait ICurrentWorkingDirectory_Impl: windows_core::IUnknownImpl {
    fn GetDirectory(&self, pwzpath: windows_core::PWSTR, cchsize: super::DWORD) -> windows_core::Result<()>;
    fn SetDirectory(&self, pwzpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ICurrentWorkingDirectory_Vtbl {
    pub const fn new<Identity: ICurrentWorkingDirectory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDirectory<Identity: ICurrentWorkingDirectory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzpath: windows_core::PWSTR, cchsize: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ICurrentWorkingDirectory {}
pub const IDC_OFFLINE_HAND: i32 = 103;
pub const IDC_PANTOOL_HAND_CLOSED: i32 = 105;
pub const IDC_PANTOOL_HAND_OPEN: i32 = 104;
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IDeskBarClient, IDeskBarClient_Vtbl, 0xeb0fe175_1a3a_11d0_89b3_00a0c90a90ac);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IDeskBarClient {
    type Target = super::IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IDeskBarClient, windows_core::IUnknown, super::IOleWindow);
#[cfg(feature = "oleidl")]
impl IDeskBarClient {
    pub unsafe fn SetDeskBarSite<P0>(&self, punksite: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDeskBarSite)(windows_core::Interface::as_raw(self), punksite.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetModeDBC(&self, dwmode: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModeDBC)(windows_core::Interface::as_raw(self), dwmode) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UIActivateDBC(&self, dwstate: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UIActivateDBC)(windows_core::Interface::as_raw(self), dwstate) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetSize(&self, dwwhich: super::DWORD, prc: super::LPRECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), dwwhich, prc as _) }
    }
}
#[cfg(feature = "oleidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDeskBarClient_Vtbl {
    pub base__: super::IOleWindow_Vtbl,
    pub SetDeskBarSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetModeDBC: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetModeDBC: usize,
    #[cfg(feature = "shared")]
    pub UIActivateDBC: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UIActivateDBC: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LPRECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetSize: usize,
}
#[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
pub trait IDeskBarClient_Impl: super::IOleWindow_Impl {
    fn SetDeskBarSite(&self, punksite: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetModeDBC(&self, dwmode: super::DWORD) -> windows_core::Result<()>;
    fn UIActivateDBC(&self, dwstate: super::DWORD) -> windows_core::Result<()>;
    fn GetSize(&self, dwwhich: super::DWORD, prc: super::LPRECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
impl IDeskBarClient_Vtbl {
    pub const fn new<Identity: IDeskBarClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDeskBarSite<Identity: IDeskBarClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeskBarClient_Impl::SetDeskBarSite(this, core::mem::transmute_copy(&punksite)).into()
            }
        }
        unsafe extern "system" fn SetModeDBC<Identity: IDeskBarClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeskBarClient_Impl::SetModeDBC(this, core::mem::transmute_copy(&dwmode)).into()
            }
        }
        unsafe extern "system" fn UIActivateDBC<Identity: IDeskBarClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeskBarClient_Impl::UIActivateDBC(this, core::mem::transmute_copy(&dwstate)).into()
            }
        }
        unsafe extern "system" fn GetSize<Identity: IDeskBarClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwhich: super::DWORD, prc: super::LPRECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeskBarClient_Impl::GetSize(this, core::mem::transmute_copy(&dwwhich), core::mem::transmute_copy(&prc)).into()
            }
        }
        Self {
            base__: super::IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            SetDeskBarSite: SetDeskBarSite::<Identity, OFFSET>,
            SetModeDBC: SetModeDBC::<Identity, OFFSET>,
            UIActivateDBC: UIActivateDBC::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeskBarClient as windows_core::Interface>::IID || iid == &<super::IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IDeskBarClient {}
windows_core::imp::define_interface!(IDocViewSite, IDocViewSite_Vtbl, 0x87d605e0_c511_11cf_89a9_00a0c9054129);
windows_core::imp::interface_hierarchy!(IDocViewSite, windows_core::IUnknown);
impl IDocViewSite {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn OnSetTitle(&self, pvtitle: *const super::VARIANTARG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSetTitle)(windows_core::Interface::as_raw(self), pvtitle) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocViewSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub OnSetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VARIANTARG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    OnSetTitle: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDocViewSite_Impl: windows_core::IUnknownImpl {
    fn OnSetTitle(&self, pvtitle: *const super::VARIANTARG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDocViewSite_Vtbl {
    pub const fn new<Identity: IDocViewSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetTitle<Identity: IDocViewSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtitle: *const super::VARIANTARG) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDocViewSite {}
#[cfg(feature = "oleidl")]
windows_core::imp::define_interface!(IDockingWindowFrame, IDockingWindowFrame_Vtbl, 0x47d2657a_7b27_11d0_8ca9_00a0c92dbfe8);
#[cfg(feature = "oleidl")]
impl core::ops::Deref for IDockingWindowFrame {
    type Target = super::IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oleidl")]
windows_core::imp::interface_hierarchy!(IDockingWindowFrame, windows_core::IUnknown, super::IOleWindow);
#[cfg(feature = "oleidl")]
impl IDockingWindowFrame {
    #[cfg(feature = "shared")]
    pub unsafe fn AddToolbar<P0, P1>(&self, punksrc: P0, pwszitem: P1, dwaddflags: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddToolbar)(windows_core::Interface::as_raw(self), punksrc.param().abi(), pwszitem.param().abi(), dwaddflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveToolbar<P0>(&self, punksrc: P0, dwremoveflags: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveToolbar)(windows_core::Interface::as_raw(self), punksrc.param().abi(), dwremoveflags) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn FindToolbar<P0>(&self, pwszitem: P0, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
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
    pub base__: super::IOleWindow_Vtbl,
    #[cfg(feature = "shared")]
    pub AddToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddToolbar: usize,
    #[cfg(feature = "shared")]
    pub RemoveToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveToolbar: usize,
    #[cfg(feature = "guiddef")]
    pub FindToolbar: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    FindToolbar: usize,
}
#[cfg(all(feature = "guiddef", feature = "oleidl", feature = "shared", feature = "windef"))]
pub trait IDockingWindowFrame_Impl: super::IOleWindow_Impl {
    fn AddToolbar(&self, punksrc: windows_core::Ref<windows_core::IUnknown>, pwszitem: &windows_core::PCWSTR, dwaddflags: super::DWORD) -> windows_core::Result<()>;
    fn RemoveToolbar(&self, punksrc: windows_core::Ref<windows_core::IUnknown>, dwremoveflags: super::DWORD) -> windows_core::Result<()>;
    fn FindToolbar(&self, pwszitem: &windows_core::PCWSTR, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "oleidl", feature = "shared", feature = "windef"))]
impl IDockingWindowFrame_Vtbl {
    pub const fn new<Identity: IDockingWindowFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddToolbar<Identity: IDockingWindowFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksrc: *mut core::ffi::c_void, pwszitem: windows_core::PCWSTR, dwaddflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockingWindowFrame_Impl::AddToolbar(this, core::mem::transmute_copy(&punksrc), core::mem::transmute(&pwszitem), core::mem::transmute_copy(&dwaddflags)).into()
            }
        }
        unsafe extern "system" fn RemoveToolbar<Identity: IDockingWindowFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksrc: *mut core::ffi::c_void, dwremoveflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockingWindowFrame_Impl::RemoveToolbar(this, core::mem::transmute_copy(&punksrc), core::mem::transmute_copy(&dwremoveflags)).into()
            }
        }
        unsafe extern "system" fn FindToolbar<Identity: IDockingWindowFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszitem: windows_core::PCWSTR, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockingWindowFrame_Impl::FindToolbar(this, core::mem::transmute(&pwszitem), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: super::IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            AddToolbar: AddToolbar::<Identity, OFFSET>,
            RemoveToolbar: RemoveToolbar::<Identity, OFFSET>,
            FindToolbar: FindToolbar::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDockingWindowFrame as windows_core::Interface>::IID || iid == &<super::IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "oleidl", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IDockingWindowFrame {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    #[cfg(feature = "windef")]
    pub unsafe fn SetReferent<P0>(&self, pcszreferent: P0, hwnd: Option<super::HWND>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReferent)(windows_core::Interface::as_raw(self), pcszreferent.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetReferent(&self, pszreferent: windows_core::PSTR, cchreferent: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReferent)(windows_core::Interface::as_raw(self), pszreferent, cchreferent) }
    }
    pub unsafe fn SetFolder<P0>(&self, pcszfolder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFolder)(windows_core::Interface::as_raw(self), pcszfolder.param().abi()) }
    }
    pub unsafe fn GetFolder(&self, pszfolder: windows_core::PSTR, cchfolder: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), pszfolder, cchfolder) }
    }
    pub unsafe fn GetName(&self, pszname: windows_core::PSTR, cchname: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), pszname, cchname) }
    }
    pub unsafe fn GetExtension(&self, pszextension: windows_core::PSTR, cchextension: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), pszextension, cchextension) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INewShortcutHookA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub SetReferent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, super::HWND) -> windows_core::HRESULT,
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
    fn SetReferent(&self, pcszreferent: &windows_core::PCSTR, hwnd: super::HWND) -> windows_core::Result<()>;
    fn GetReferent(&self, pszreferent: windows_core::PSTR, cchreferent: i32) -> windows_core::Result<()>;
    fn SetFolder(&self, pcszfolder: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn GetFolder(&self, pszfolder: windows_core::PSTR, cchfolder: i32) -> windows_core::Result<()>;
    fn GetName(&self, pszname: windows_core::PSTR, cchname: i32) -> windows_core::Result<()>;
    fn GetExtension(&self, pszextension: windows_core::PSTR, cchextension: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl INewShortcutHookA_Vtbl {
    pub const fn new<Identity: INewShortcutHookA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetReferent<Identity: INewShortcutHookA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszreferent: windows_core::PCSTR, hwnd: super::HWND) -> windows_core::HRESULT {
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
    #[cfg(feature = "windef")]
    pub unsafe fn SetReferent<P0>(&self, pcszreferent: P0, hwnd: Option<super::HWND>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReferent)(windows_core::Interface::as_raw(self), pcszreferent.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetReferent(&self, pszreferent: windows_core::PWSTR, cchreferent: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReferent)(windows_core::Interface::as_raw(self), pszreferent, cchreferent) }
    }
    pub unsafe fn SetFolder<P0>(&self, pcszfolder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFolder)(windows_core::Interface::as_raw(self), pcszfolder.param().abi()) }
    }
    pub unsafe fn GetFolder(&self, pszfolder: windows_core::PWSTR, cchfolder: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), pszfolder, cchfolder) }
    }
    pub unsafe fn GetName(&self, pszname: windows_core::PWSTR, cchname: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), pszname, cchname) }
    }
    pub unsafe fn GetExtension(&self, pszextension: windows_core::PWSTR, cchextension: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), pszextension, cchextension) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INewShortcutHookW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub SetReferent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::HWND) -> windows_core::HRESULT,
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
    fn SetReferent(&self, pcszreferent: &windows_core::PCWSTR, hwnd: super::HWND) -> windows_core::Result<()>;
    fn GetReferent(&self, pszreferent: windows_core::PWSTR, cchreferent: i32) -> windows_core::Result<()>;
    fn SetFolder(&self, pcszfolder: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFolder(&self, pszfolder: windows_core::PWSTR, cchfolder: i32) -> windows_core::Result<()>;
    fn GetName(&self, pszname: windows_core::PWSTR, cchname: i32) -> windows_core::Result<()>;
    fn GetExtension(&self, pszextension: windows_core::PWSTR, cchextension: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl INewShortcutHookW_Vtbl {
    pub const fn new<Identity: INewShortcutHookW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetReferent<Identity: INewShortcutHookW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszreferent: windows_core::PCWSTR, hwnd: super::HWND) -> windows_core::HRESULT {
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
pub const ISFBVIEWMODE_LARGEICONS: i32 = 2;
pub const ISFBVIEWMODE_SMALLICONS: i32 = 1;
pub const ISFB_MASK_BKCOLOR: i32 = 2;
pub const ISFB_MASK_COLORS: i32 = 32;
pub const ISFB_MASK_IDLIST: i32 = 16;
pub const ISFB_MASK_SHELLFOLDER: i32 = 8;
pub const ISFB_MASK_STATE: i32 = 1;
pub const ISFB_MASK_VIEWMODE: i32 = 4;
pub const ISFB_STATE_ALLOWRENAME: i32 = 2;
pub const ISFB_STATE_BTNMINSIZE: i32 = 256;
pub const ISFB_STATE_CHANNELBAR: i32 = 16;
pub const ISFB_STATE_DEBOSSED: i32 = 1;
pub const ISFB_STATE_DEFAULT: i32 = 0;
pub const ISFB_STATE_FULLOPEN: i32 = 64;
pub const ISFB_STATE_NONAMESORT: i32 = 128;
pub const ISFB_STATE_NOSHOWTEXT: i32 = 4;
pub const ISFB_STATE_QLINKSMODE: i32 = 32;
windows_core::imp::define_interface!(IShellFolderBand, IShellFolderBand_Vtbl, 0x7fe80cc8_c247_11d0_b93a_00a0c90312e1);
windows_core::imp::interface_hierarchy!(IShellFolderBand, windows_core::IUnknown);
impl IShellFolderBand {
    #[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
    pub unsafe fn InitializeSFB<P0>(&self, psf: P0, pidl: Option<super::LPCITEMIDLIST>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IShellFolder>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeSFB)(windows_core::Interface::as_raw(self), psf.param().abi(), pidl.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub unsafe fn SetBandInfoSFB(&self, pbi: PBANDINFOSFB) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBandInfoSFB)(windows_core::Interface::as_raw(self), pbi) }
    }
    #[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub unsafe fn GetBandInfoSFB(&self, pbi: PBANDINFOSFB) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBandInfoSFB)(windows_core::Interface::as_raw(self), pbi) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellFolderBand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes"))]
    pub InitializeSFB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LPCITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes")))]
    InitializeSFB: usize,
    #[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub SetBandInfoSFB: unsafe extern "system" fn(*mut core::ffi::c_void, PBANDINFOSFB) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef")))]
    SetBandInfoSFB: usize,
    #[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub GetBandInfoSFB: unsafe extern "system" fn(*mut core::ffi::c_void, PBANDINFOSFB) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef")))]
    GetBandInfoSFB: usize,
}
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub trait IShellFolderBand_Impl: windows_core::IUnknownImpl {
    fn InitializeSFB(&self, psf: windows_core::Ref<super::IShellFolder>, pidl: super::LPCITEMIDLIST) -> windows_core::Result<()>;
    fn SetBandInfoSFB(&self, pbi: PBANDINFOSFB) -> windows_core::Result<()>;
    fn GetBandInfoSFB(&self, pbi: PBANDINFOSFB) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl IShellFolderBand_Vtbl {
    pub const fn new<Identity: IShellFolderBand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeSFB<Identity: IShellFolderBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psf: *mut core::ffi::c_void, pidl: super::LPCITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderBand_Impl::InitializeSFB(this, core::mem::transmute_copy(&psf), core::mem::transmute_copy(&pidl)).into()
            }
        }
        unsafe extern "system" fn SetBandInfoSFB<Identity: IShellFolderBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbi: PBANDINFOSFB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderBand_Impl::SetBandInfoSFB(this, core::mem::transmute_copy(&pbi)).into()
            }
        }
        unsafe extern "system" fn GetBandInfoSFB<Identity: IShellFolderBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbi: PBANDINFOSFB) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl windows_core::RuntimeName for IShellFolderBand {}
windows_core::imp::define_interface!(IThumbnailCapture, IThumbnailCapture_Vtbl, 0x4ea39266_7211_409f_b622_f63dbd16c533);
windows_core::imp::interface_hierarchy!(IThumbnailCapture, windows_core::IUnknown);
impl IThumbnailCapture {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn CaptureThumbnail<P1>(&self, pmaxsize: *const super::SIZE, phtmldoc2: P1, phbmthumbnail: *mut super::HBITMAP) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CaptureThumbnail)(windows_core::Interface::as_raw(self), pmaxsize, phtmldoc2.param().abi(), phbmthumbnail as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailCapture_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub CaptureThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SIZE, *mut core::ffi::c_void, *mut super::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    CaptureThumbnail: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IThumbnailCapture_Impl: windows_core::IUnknownImpl {
    fn CaptureThumbnail(&self, pmaxsize: *const super::SIZE, phtmldoc2: windows_core::Ref<windows_core::IUnknown>, phbmthumbnail: *mut super::HBITMAP) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IThumbnailCapture_Vtbl {
    pub const fn new<Identity: IThumbnailCapture_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CaptureThumbnail<Identity: IThumbnailCapture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxsize: *const super::SIZE, phtmldoc2: *mut core::ffi::c_void, phbmthumbnail: *mut super::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThumbnailCapture_Impl::CaptureThumbnail(this, core::mem::transmute_copy(&pmaxsize), core::mem::transmute_copy(&phtmldoc2), core::mem::transmute_copy(&phbmthumbnail)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CaptureThumbnail: CaptureThumbnail::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IThumbnailCapture as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IThumbnailCapture {}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPAASHELLMENUFILENAME = *mut AASHELLMENUFILENAME;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPAASHELLMENUITEM = *mut AASHELLMENUITEM;
pub type LPACTIVEDESKTOPP = IActiveDesktopP;
pub type LPADESKTOPP2 = IADesktopP2;
pub type LPCOPYHOOKA = ICopyHookA;
pub type LPCOPYHOOKW = ICopyHookW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCSHCOLUMNDATA = *const SHCOLUMNDATA;
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
pub type LPCSHCOLUMNINFO = *const SHCOLUMNINFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCSHCOLUMNINIT = *const SHCOLUMNINIT;
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef"))]
pub type LPSFV_SETITEMPOS = *mut SFV_SETITEMPOS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHCOLUMNDATA = *mut SHCOLUMNDATA;
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
pub type LPSHCOLUMNINFO = *mut SHCOLUMNINFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHCOLUMNINIT = *mut SHCOLUMNINIT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSHChangeProductKeyAsIDList = *mut SHChangeProductKeyAsIDList;
#[cfg(feature = "shared")]
pub type LPTBINFO = *mut TBINFO;
pub type LPTHUMBNAILCAPTURE = IThumbnailCapture;
pub const OPENPROPS_INHIBITPIF: i32 = 32768;
pub const OPENPROPS_NONE: i32 = 0;
pub const PANE_NAVIGATION: i32 = 5;
#[cfg(feature = "shared")]
pub const PANE_NONE: super::DWORD = 4294967295;
pub const PANE_OFFLINE: i32 = 2;
pub const PANE_PRINTER: i32 = 3;
pub const PANE_PRIVACY: i32 = 7;
pub const PANE_PROGRESS: i32 = 6;
pub const PANE_SSL: i32 = 4;
pub const PANE_ZONE: i32 = 1;
#[cfg(all(feature = "shared", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type PBANDINFOSFB = *mut BANDINFOSFB;
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef"))]
pub type PCSFV_SETITEMPOS = *const SFV_SETITEMPOS;
pub const SCHEME_CREATE: i32 = 128;
pub const SCHEME_DISPLAY: i32 = 1;
pub const SCHEME_DONOTUSE: i32 = 64;
pub const SCHEME_EDIT: i32 = 2;
pub const SCHEME_GLOBAL: i32 = 8;
pub const SCHEME_LOCAL: i32 = 4;
pub const SCHEME_REFRESH: i32 = 16;
pub const SCHEME_UPDATE: i32 = 32;
pub const SETPROPS_NONE: i32 = 0;
pub const SFBID_PIDLCHANGED: i32 = 0;
pub const SFVM_ADDOBJECT: i32 = 3;
pub const SFVM_GETSELECTEDOBJECTS: i32 = 9;
pub const SFVM_REARRANGE: i32 = 1;
pub const SFVM_REMOVEOBJECT: i32 = 6;
pub const SFVM_SETCLIPBOARD: i32 = 16;
pub const SFVM_SETITEMPOS: i32 = 14;
pub const SFVM_SETPOINTS: i32 = 23;
pub const SFVM_UPDATEOBJECT: i32 = 7;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct SFV_SETITEMPOS {
    pub pidl: super::LPCITEMIDLIST,
    pub pt: super::POINT,
}
pub const SHCDF_UPDATEITEM: i32 = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SHCOLUMNDATA {
    pub dwFlags: super::ULONG,
    pub dwFileAttributes: super::DWORD,
    pub dwReserved: super::ULONG,
    pub pwszExt: *mut super::WCHAR,
    pub wszFile: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SHCOLUMNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
#[derive(Clone, Copy)]
pub struct SHCOLUMNINFO {
    pub scid: super::SHCOLUMNID,
    pub vt: super::VARTYPE,
    pub fmt: super::DWORD,
    pub cChars: super::UINT,
    pub csFlags: super::DWORD,
    pub wszTitle: [super::WCHAR; 80],
    pub wszDescription: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes"))]
impl Default for SHCOLUMNINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SHCOLUMNINIT {
    pub dwFlags: super::ULONG,
    pub dwReserved: super::ULONG,
    pub wszFolder: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SHCOLUMNINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SHChangeProductKeyAsIDList {
    pub cb: super::USHORT,
    pub wszProductKey: [super::WCHAR; 39],
    pub cbZero: super::USHORT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SHChangeProductKeyAsIDList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSM_CLEAR: i32 = 0;
pub const SSM_REFRESH: i32 = 2;
pub const SSM_SET: i32 = 1;
pub const SSM_UPDATE: i32 = 4;
pub const TBIF_APPEND: i32 = 0;
pub const TBIF_DEFAULT: i32 = 0;
pub const TBIF_INTERNETBAR: i32 = 65536;
pub const TBIF_NOTOOLBAR: i32 = 196608;
pub const TBIF_PREPEND: i32 = 1;
pub const TBIF_REPLACE: i32 = 2;
pub const TBIF_STANDARDTOOLBAR: i32 = 131072;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TBINFO {
    pub cbuttons: super::UINT,
    pub uFlags: super::UINT,
}
