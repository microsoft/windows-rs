#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type BASEBROWSERDATA = BASEBROWSERDATALH;
#[repr(C)]
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BASEBROWSERDATALH {
    pub _hwnd: super::windef::HWND,
    pub _ptl: core::mem::ManuallyDrop<Option<ITravelLog>>,
    pub _phlf: core::mem::ManuallyDrop<Option<super::hlink::IHlinkFrame>>,
    pub _pautoWB2: core::mem::ManuallyDrop<Option<super::exdisp::IWebBrowser2>>,
    pub _pautoEDS: core::mem::ManuallyDrop<Option<IExpDispSupport>>,
    pub _pautoSS: core::mem::ManuallyDrop<Option<IShellService>>,
    pub _eSecureLockIcon: i32,
    pub _bitfield: u32,
    pub _uActivateState: u32,
    pub _pidlViewState: super::shtypes::LPCITEMIDLIST,
    pub _pctView: core::mem::ManuallyDrop<Option<super::docobj::IOleCommandTarget>>,
    pub _pidlCur: super::shtypes::LPITEMIDLIST,
    pub _psv: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellView>>,
    pub _psf: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellFolder>>,
    pub _hwndView: super::windef::HWND,
    pub _pszTitleCur: windows_core::PWSTR,
    pub _pidlPending: super::shtypes::LPITEMIDLIST,
    pub _psvPending: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellView>>,
    pub _psfPending: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellFolder>>,
    pub _hwndViewPending: super::windef::HWND,
    pub _pszTitlePending: windows_core::PWSTR,
    pub _fIsViewMSHTML: windows_core::BOOL,
    pub _fPrivacyImpacted: windows_core::BOOL,
    pub _clsidView: windows_core::GUID,
    pub _clsidViewPending: windows_core::GUID,
    pub _hwndFrame: super::windef::HWND,
    pub _lPhishingFilterStatus: i32,
}
#[repr(C)]
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BASEBROWSERDATAXP {
    pub _hwnd: super::windef::HWND,
    pub _ptl: core::mem::ManuallyDrop<Option<ITravelLog>>,
    pub _phlf: core::mem::ManuallyDrop<Option<super::hlink::IHlinkFrame>>,
    pub _pautoWB2: core::mem::ManuallyDrop<Option<super::exdisp::IWebBrowser2>>,
    pub _pautoEDS: core::mem::ManuallyDrop<Option<IExpDispSupportXP>>,
    pub _pautoSS: core::mem::ManuallyDrop<Option<IShellService>>,
    pub _eSecureLockIcon: i32,
    pub _bitfield: u32,
    pub _uActivateState: u32,
    pub _pidlViewState: super::shtypes::LPCITEMIDLIST,
    pub _pctView: core::mem::ManuallyDrop<Option<super::docobj::IOleCommandTarget>>,
    pub _pidlCur: super::shtypes::LPITEMIDLIST,
    pub _psv: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellView>>,
    pub _psf: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellFolder>>,
    pub _hwndView: super::windef::HWND,
    pub _pszTitleCur: windows_core::PWSTR,
    pub _pidlPending: super::shtypes::LPITEMIDLIST,
    pub _psvPending: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellView>>,
    pub _psfPending: core::mem::ManuallyDrop<Option<super::shobjidl_core::IShellFolder>>,
    pub _hwndViewPending: super::windef::HWND,
    pub _pszTitlePending: windows_core::PWSTR,
    pub _fIsViewMSHTML: windows_core::BOOL,
    pub _fPrivacyImpacted: windows_core::BOOL,
    pub _clsidView: windows_core::GUID,
    pub _clsidViewPending: windows_core::GUID,
    pub _hwndFrame: super::windef::HWND,
}
pub type BNSTATE = i32;
pub const BNS_BEGIN_NAVIGATE: BNSTATE = 1;
pub const BNS_NAVIGATE: BNSTATE = 2;
pub const BNS_NORMAL: BNSTATE = 0;
pub const BSF_CANMAXIMIZE: u32 = 1024;
pub const BSF_DELEGATEDNAVIGATION: u32 = 65536;
pub const BSF_DONTSHOWNAVCANCELPAGE: u32 = 16384;
pub const BSF_FEEDNAVIGATION: u32 = 524288;
pub const BSF_FEEDSUBSCRIBED: u32 = 1048576;
pub const BSF_HTMLNAVCANCELED: u32 = 8192;
pub const BSF_MERGEDMENUS: u32 = 262144;
pub const BSF_NAVNOHISTORY: u32 = 4096;
pub const BSF_NOLOCALFILEWARNING: u32 = 16;
pub const BSF_REGISTERASDROPTARGET: u32 = 1;
pub const BSF_RESIZABLE: u32 = 512;
pub const BSF_SETNAVIGATABLECODEPAGE: u32 = 32768;
pub const BSF_THEATERMODE: u32 = 2;
pub const BSF_TOPBROWSER: u32 = 2048;
pub const BSF_TRUSTEDFORACTIVEX: u32 = 131072;
pub const BSF_UISETBYAUTOMATION: u32 = 256;
#[cfg(feature = "ocidl")]
windows_core::imp::define_interface!(CIE4ConnectionPoint, CIE4ConnectionPoint_Vtbl, 0);
#[cfg(feature = "ocidl")]
impl core::ops::Deref for CIE4ConnectionPoint {
    type Target = super::ocidl::IConnectionPoint;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "ocidl")]
windows_core::imp::interface_hierarchy!(CIE4ConnectionPoint, windows_core::IUnknown, super::ocidl::IConnectionPoint);
#[cfg(feature = "ocidl")]
impl CIE4ConnectionPoint {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn DoInvokeIE4(&self, pf: *mut windows_core::BOOL, ppv: *mut *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pdispparams: *mut super::oaidl::DISPPARAMS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoInvokeIE4)(windows_core::Interface::as_raw(self), pf as _, ppv as _, dispid, pdispparams as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "shtypes"))]
    pub unsafe fn DoInvokePIDLIE4(&self, dispid: super::oaidl::DISPID, pidl: *const super::shtypes::ITEMIDLIST, fcancancel: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoInvokePIDLIE4)(windows_core::Interface::as_raw(self), dispid, pidl, fcancancel.into()) }
    }
}
#[cfg(feature = "ocidl")]
#[repr(C)]
#[doc(hidden)]
pub struct CIE4ConnectionPoint_Vtbl {
    pub base__: super::ocidl::IConnectionPoint_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub DoInvokeIE4: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void, super::oaidl::DISPID, *mut super::oaidl::DISPPARAMS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    DoInvokeIE4: usize,
    #[cfg(all(feature = "oaidl", feature = "shtypes"))]
    pub DoInvokePIDLIE4: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *const super::shtypes::ITEMIDLIST, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shtypes")))]
    DoInvokePIDLIE4: usize,
}
#[cfg(all(feature = "oaidl", feature = "ocidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
pub trait CIE4ConnectionPoint_Impl: super::ocidl::IConnectionPoint_Impl {
    fn DoInvokeIE4(&self, pf: *mut windows_core::BOOL, ppv: *mut *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pdispparams: *mut super::oaidl::DISPPARAMS) -> windows_core::Result<()>;
    fn DoInvokePIDLIE4(&self, dispid: super::oaidl::DISPID, pidl: *const super::shtypes::ITEMIDLIST, fcancancel: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "ocidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
impl CIE4ConnectionPoint_Vtbl {
    pub const fn new<Identity: CIE4ConnectionPoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoInvokeIE4<Identity: CIE4ConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pf: *mut windows_core::BOOL, ppv: *mut *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pdispparams: *mut super::oaidl::DISPPARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                CIE4ConnectionPoint_Impl::DoInvokeIE4(this, core::mem::transmute_copy(&pf), core::mem::transmute_copy(&ppv), core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&pdispparams)).into()
            }
        }
        unsafe extern "system" fn DoInvokePIDLIE4<Identity: CIE4ConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pidl: *const super::shtypes::ITEMIDLIST, fcancancel: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                CIE4ConnectionPoint_Impl::DoInvokePIDLIE4(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&pidl), core::mem::transmute_copy(&fcancancel)).into()
            }
        }
        Self {
            base__: super::ocidl::IConnectionPoint_Vtbl::new::<Identity, OFFSET>(),
            DoInvokeIE4: DoInvokeIE4::<Identity, OFFSET>,
            DoInvokePIDLIE4: DoInvokePIDLIE4::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<CIE4ConnectionPoint as windows_core::Interface>::IID || iid == &<super::ocidl::IConnectionPoint as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "ocidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for CIE4ConnectionPoint {}
#[repr(C)]
#[cfg(feature = "shobjidl_core")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FOLDERSETDATA {
    pub _fs: super::shobjidl_core::FOLDERSETTINGS,
    pub _vidRestore: super::shobjidl_core::SHELLVIEWID,
    pub _dwViewPriority: u32,
}
pub const HLNF_ALLOW_AUTONAVIGATE: u32 = 536870912;
pub const HLNF_CALLERUNTRUSTED: u32 = 2097152;
pub const HLNF_DISABLEWINDOWRESTRICTIONS: u32 = 8388608;
pub const HLNF_EXTERNALNAVIGATE: u32 = 268435456;
pub const HLNF_NEWWINDOWSMANAGED: u32 = 2147483648;
pub const HLNF_TRUSTEDFORACTIVEX: u32 = 4194304;
pub const HLNF_TRUSTFIRSTDOWNLOAD: u32 = 16777216;
pub const HLNF_UNTRUSTEDFORDOWNLOAD: u32 = 33554432;
windows_core::imp::define_interface!(IBrowserService, IBrowserService_Vtbl, 0x02ba3b52_0547_11d1_b833_00c04fc9b31f);
windows_core::imp::interface_hierarchy!(IBrowserService, windows_core::IUnknown);
impl IBrowserService {
    #[cfg(feature = "oleidl")]
    pub unsafe fn GetParentSite(&self) -> windows_core::Result<super::oleidl::IOleInPlaceSite> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentSite)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
    pub unsafe fn SetTitle<P0, P1>(&self, psv: P0, pszname: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellView>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), psv.param().abi(), pszname.param().abi()) }
    }
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
    pub unsafe fn GetTitle<P0>(&self, psv: P0, pszname: &mut [u16]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellView>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTitle)(windows_core::Interface::as_raw(self), psv.param().abi(), core::mem::transmute(pszname.as_mut_ptr()), pszname.len().try_into().unwrap()) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn GetOleObject(&self) -> windows_core::Result<super::oleidl::IOleObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOleObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTravelLog(&self) -> windows_core::Result<ITravelLog> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTravelLog)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ShowControlWindow(&self, id: u32, fshow: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowControlWindow)(windows_core::Interface::as_raw(self), id, fshow.into()) }
    }
    pub unsafe fn IsControlWindowShown(&self, id: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsControlWindowShown)(windows_core::Interface::as_raw(self), id, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn IEGetDisplayName(&self, pidl: *const super::shtypes::ITEMIDLIST, pwszname: windows_core::PWSTR, uflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IEGetDisplayName)(windows_core::Interface::as_raw(self), pidl, core::mem::transmute(pwszname), uflags) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn IEParseDisplayName<P1>(&self, uicp: u32, pwszpath: P1) -> windows_core::Result<super::shtypes::LPITEMIDLIST>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IEParseDisplayName)(windows_core::Interface::as_raw(self), uicp, pwszpath.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DisplayParseError<P1>(&self, hres: windows_core::HRESULT, pwszpath: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DisplayParseError)(windows_core::Interface::as_raw(self), hres, pwszpath.param().abi()) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn NavigateToPidl(&self, pidl: *const super::shtypes::ITEMIDLIST, grfhlnf: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NavigateToPidl)(windows_core::Interface::as_raw(self), pidl, grfhlnf) }
    }
    pub unsafe fn SetNavigateState(&self, bnstate: BNSTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNavigateState)(windows_core::Interface::as_raw(self), bnstate) }
    }
    pub unsafe fn GetNavigateState(&self) -> windows_core::Result<BNSTATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNavigateState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "shtypes"))]
    pub unsafe fn NotifyRedirect<P0>(&self, psv: P0, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<super::shobjidl_core::IShellView>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NotifyRedirect)(windows_core::Interface::as_raw(self), psv.param().abi(), pidl, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateWindowList(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateWindowList)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn UpdateBackForwardState(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateBackForwardState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetFlags(&self, dwflags: u32, dwflagmask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), dwflags, dwflagmask) }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanNavigateNow(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CanNavigateNow)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn GetPidl(&self) -> windows_core::Result<super::shtypes::LPITEMIDLIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPidl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn SetReferrer(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReferrer)(windows_core::Interface::as_raw(self), pidl) }
    }
    pub unsafe fn GetBrowserIndex(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetBrowserIndex)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBrowserByIndex(&self, dwid: u32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBrowserByIndex)(windows_core::Interface::as_raw(self), dwid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidl", feature = "objidlbase", feature = "oleidl"))]
    pub unsafe fn GetHistoryObject(&self, ppole: *mut Option<super::oleidl::IOleObject>, pstm: *mut Option<super::objidlbase::IStream>, ppbc: *mut Option<super::objidl::IBindCtx>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHistoryObject)(windows_core::Interface::as_raw(self), core::mem::transmute(ppole), core::mem::transmute(pstm), core::mem::transmute(ppbc)) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn SetHistoryObject<P0>(&self, pole: P0, fislocalanchor: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oleidl::IOleObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHistoryObject)(windows_core::Interface::as_raw(self), pole.param().abi(), fislocalanchor.into()) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn CacheOLEServer<P0>(&self, pole: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oleidl::IOleObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).CacheOLEServer)(windows_core::Interface::as_raw(self), pole.param().abi()) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetSetCodePage(&self, pvarin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSetCodePage)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarin), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn OnHttpEquiv<P0>(&self, psv: P0, fdone: bool, pvarargin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT>
    where
        P0: windows_core::Param<super::shobjidl_core::IShellView>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnHttpEquiv)(windows_core::Interface::as_raw(self), psv.param().abi(), fdone.into(), core::mem::transmute(pvarargin), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetPalette(&self) -> windows_core::Result<super::windef::HPALETTE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPalette)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterWindow(&self, fforceregister: bool, swc: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterWindow)(windows_core::Interface::as_raw(self), fforceregister.into(), swc) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrowserService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oleidl")]
    pub GetParentSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    GetParentSite: usize,
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oleidl", feature = "shobjidl_core")))]
    SetTitle: usize,
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core"))]
    pub GetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oleidl", feature = "shobjidl_core")))]
    GetTitle: usize,
    #[cfg(feature = "oleidl")]
    pub GetOleObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    GetOleObject: usize,
    pub GetTravelLog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowControlWindow: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsControlWindowShown: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub IEGetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    IEGetDisplayName: usize,
    #[cfg(feature = "shtypes")]
    pub IEParseDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    IEParseDisplayName: usize,
    pub DisplayParseError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub NavigateToPidl: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    NavigateToPidl: usize,
    pub SetNavigateState: unsafe extern "system" fn(*mut core::ffi::c_void, BNSTATE) -> windows_core::HRESULT,
    pub GetNavigateState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BNSTATE) -> windows_core::HRESULT,
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "shtypes"))]
    pub NotifyRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oleidl", feature = "shobjidl_core", feature = "shtypes")))]
    NotifyRedirect: usize,
    pub UpdateWindowList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateBackForwardState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CanNavigateNow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub GetPidl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    GetPidl: usize,
    #[cfg(feature = "shtypes")]
    pub SetReferrer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    SetReferrer: usize,
    pub GetBrowserIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetBrowserByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "objidlbase", feature = "oleidl"))]
    pub GetHistoryObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "objidlbase", feature = "oleidl")))]
    GetHistoryObject: usize,
    #[cfg(feature = "oleidl")]
    pub SetHistoryObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    SetHistoryObject: usize,
    #[cfg(feature = "oleidl")]
    pub CacheOLEServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    CacheOLEServer: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetSetCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetSetCodePage: usize,
    #[cfg(all(feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "wtypes", feature = "wtypesbase"))]
    pub OnHttpEquiv: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *const super::oaidl::VARIANT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "wtypes", feature = "wtypesbase")))]
    OnHttpEquiv: usize,
    #[cfg(feature = "windef")]
    pub GetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HPALETTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetPalette: usize,
    pub RegisterWindow: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBrowserService_Impl: windows_core::IUnknownImpl {
    fn GetParentSite(&self) -> windows_core::Result<super::oleidl::IOleInPlaceSite>;
    fn SetTitle(&self, psv: windows_core::Ref<super::shobjidl_core::IShellView>, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTitle(&self, psv: windows_core::Ref<super::shobjidl_core::IShellView>, pszname: windows_core::PWSTR, cchname: u32) -> windows_core::Result<()>;
    fn GetOleObject(&self) -> windows_core::Result<super::oleidl::IOleObject>;
    fn GetTravelLog(&self) -> windows_core::Result<ITravelLog>;
    fn ShowControlWindow(&self, id: u32, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsControlWindowShown(&self, id: u32) -> windows_core::Result<windows_core::BOOL>;
    fn IEGetDisplayName(&self, pidl: *const super::shtypes::ITEMIDLIST, pwszname: windows_core::PWSTR, uflags: u32) -> windows_core::Result<()>;
    fn IEParseDisplayName(&self, uicp: u32, pwszpath: &windows_core::PCWSTR) -> windows_core::Result<super::shtypes::LPITEMIDLIST>;
    fn DisplayParseError(&self, hres: windows_core::HRESULT, pwszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn NavigateToPidl(&self, pidl: *const super::shtypes::ITEMIDLIST, grfhlnf: u32) -> windows_core::Result<()>;
    fn SetNavigateState(&self, bnstate: BNSTATE) -> windows_core::Result<()>;
    fn GetNavigateState(&self) -> windows_core::Result<BNSTATE>;
    fn NotifyRedirect(&self, psv: windows_core::Ref<super::shobjidl_core::IShellView>, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<windows_core::BOOL>;
    fn UpdateWindowList(&self) -> windows_core::Result<()>;
    fn UpdateBackForwardState(&self) -> windows_core::Result<()>;
    fn SetFlags(&self, dwflags: u32, dwflagmask: u32) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn CanNavigateNow(&self) -> windows_core::Result<()>;
    fn GetPidl(&self) -> windows_core::Result<super::shtypes::LPITEMIDLIST>;
    fn SetReferrer(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<()>;
    fn GetBrowserIndex(&self) -> u32;
    fn GetBrowserByIndex(&self, dwid: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn GetHistoryObject(&self, ppole: windows_core::OutRef<super::oleidl::IOleObject>, pstm: windows_core::OutRef<super::objidlbase::IStream>, ppbc: windows_core::OutRef<super::objidl::IBindCtx>) -> windows_core::Result<()>;
    fn SetHistoryObject(&self, pole: windows_core::Ref<super::oleidl::IOleObject>, fislocalanchor: windows_core::BOOL) -> windows_core::Result<()>;
    fn CacheOLEServer(&self, pole: windows_core::Ref<super::oleidl::IOleObject>) -> windows_core::Result<()>;
    fn GetSetCodePage(&self, pvarin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT>;
    fn OnHttpEquiv(&self, psv: windows_core::Ref<super::shobjidl_core::IShellView>, fdone: windows_core::BOOL, pvarargin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetPalette(&self) -> windows_core::Result<super::windef::HPALETTE>;
    fn RegisterWindow(&self, fforceregister: windows_core::BOOL, swc: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IBrowserService_Vtbl {
    pub const fn new<Identity: IBrowserService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParentSite<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppipsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::GetParentSite(this) {
                    Ok(ok__) => {
                        ppipsite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTitle<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psv: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::SetTitle(this, core::mem::transmute_copy(&psv), core::mem::transmute(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetTitle<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psv: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchname: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::GetTitle(this, core::mem::transmute_copy(&psv), core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchname)).into()
            }
        }
        unsafe extern "system" fn GetOleObject<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobjv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::GetOleObject(this) {
                    Ok(ok__) => {
                        ppobjv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTravelLog<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::GetTravelLog(this) {
                    Ok(ok__) => {
                        pptl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShowControlWindow<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u32, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::ShowControlWindow(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn IsControlWindowShown<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u32, pfshown: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::IsControlWindowShown(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        pfshown.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IEGetDisplayName<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, pwszname: windows_core::PWSTR, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::IEGetDisplayName(this, core::mem::transmute_copy(&pidl), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&uflags)).into()
            }
        }
        unsafe extern "system" fn IEParseDisplayName<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicp: u32, pwszpath: windows_core::PCWSTR, ppidlout: *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::IEParseDisplayName(this, core::mem::transmute_copy(&uicp), core::mem::transmute(&pwszpath)) {
                    Ok(ok__) => {
                        ppidlout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayParseError<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hres: windows_core::HRESULT, pwszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::DisplayParseError(this, core::mem::transmute_copy(&hres), core::mem::transmute(&pwszpath)).into()
            }
        }
        unsafe extern "system" fn NavigateToPidl<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, grfhlnf: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::NavigateToPidl(this, core::mem::transmute_copy(&pidl), core::mem::transmute_copy(&grfhlnf)).into()
            }
        }
        unsafe extern "system" fn SetNavigateState<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnstate: BNSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::SetNavigateState(this, core::mem::transmute_copy(&bnstate)).into()
            }
        }
        unsafe extern "system" fn GetNavigateState<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnstate: *mut BNSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::GetNavigateState(this) {
                    Ok(ok__) => {
                        pbnstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NotifyRedirect<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psv: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, pfdidbrowse: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::NotifyRedirect(this, core::mem::transmute_copy(&psv), core::mem::transmute_copy(&pidl)) {
                    Ok(ok__) => {
                        pfdidbrowse.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateWindowList<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::UpdateWindowList(this).into()
            }
        }
        unsafe extern "system" fn UpdateBackForwardState<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::UpdateBackForwardState(this).into()
            }
        }
        unsafe extern "system" fn SetFlags<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwflagmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::SetFlags(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwflagmask)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanNavigateNow<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::CanNavigateNow(this).into()
            }
        }
        unsafe extern "system" fn GetPidl<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidl: *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::GetPidl(this) {
                    Ok(ok__) => {
                        ppidl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReferrer<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::SetReferrer(this, core::mem::transmute_copy(&pidl)).into()
            }
        }
        unsafe extern "system" fn GetBrowserIndex<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::GetBrowserIndex(this)
            }
        }
        unsafe extern "system" fn GetBrowserByIndex<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::GetBrowserByIndex(this, core::mem::transmute_copy(&dwid)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHistoryObject<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppole: *mut *mut core::ffi::c_void, pstm: *mut *mut core::ffi::c_void, ppbc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::GetHistoryObject(this, core::mem::transmute_copy(&ppole), core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&ppbc)).into()
            }
        }
        unsafe extern "system" fn SetHistoryObject<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pole: *mut core::ffi::c_void, fislocalanchor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::SetHistoryObject(this, core::mem::transmute_copy(&pole), core::mem::transmute_copy(&fislocalanchor)).into()
            }
        }
        unsafe extern "system" fn CacheOLEServer<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pole: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::CacheOLEServer(this, core::mem::transmute_copy(&pole)).into()
            }
        }
        unsafe extern "system" fn GetSetCodePage<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarin: *const super::oaidl::VARIANT, pvarout: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::GetSetCodePage(this, core::mem::transmute_copy(&pvarin)) {
                    Ok(ok__) => {
                        pvarout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnHttpEquiv<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psv: *mut core::ffi::c_void, fdone: windows_core::BOOL, pvarargin: *const super::oaidl::VARIANT, pvarargout: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::OnHttpEquiv(this, core::mem::transmute_copy(&psv), core::mem::transmute_copy(&fdone), core::mem::transmute_copy(&pvarargin)) {
                    Ok(ok__) => {
                        pvarargout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPalette<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpal: *mut super::windef::HPALETTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService_Impl::GetPalette(this) {
                    Ok(ok__) => {
                        hpal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterWindow<Identity: IBrowserService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforceregister: windows_core::BOOL, swc: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService_Impl::RegisterWindow(this, core::mem::transmute_copy(&fforceregister), core::mem::transmute_copy(&swc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParentSite: GetParentSite::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            GetTitle: GetTitle::<Identity, OFFSET>,
            GetOleObject: GetOleObject::<Identity, OFFSET>,
            GetTravelLog: GetTravelLog::<Identity, OFFSET>,
            ShowControlWindow: ShowControlWindow::<Identity, OFFSET>,
            IsControlWindowShown: IsControlWindowShown::<Identity, OFFSET>,
            IEGetDisplayName: IEGetDisplayName::<Identity, OFFSET>,
            IEParseDisplayName: IEParseDisplayName::<Identity, OFFSET>,
            DisplayParseError: DisplayParseError::<Identity, OFFSET>,
            NavigateToPidl: NavigateToPidl::<Identity, OFFSET>,
            SetNavigateState: SetNavigateState::<Identity, OFFSET>,
            GetNavigateState: GetNavigateState::<Identity, OFFSET>,
            NotifyRedirect: NotifyRedirect::<Identity, OFFSET>,
            UpdateWindowList: UpdateWindowList::<Identity, OFFSET>,
            UpdateBackForwardState: UpdateBackForwardState::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            CanNavigateNow: CanNavigateNow::<Identity, OFFSET>,
            GetPidl: GetPidl::<Identity, OFFSET>,
            SetReferrer: SetReferrer::<Identity, OFFSET>,
            GetBrowserIndex: GetBrowserIndex::<Identity, OFFSET>,
            GetBrowserByIndex: GetBrowserByIndex::<Identity, OFFSET>,
            GetHistoryObject: GetHistoryObject::<Identity, OFFSET>,
            SetHistoryObject: SetHistoryObject::<Identity, OFFSET>,
            CacheOLEServer: CacheOLEServer::<Identity, OFFSET>,
            GetSetCodePage: GetSetCodePage::<Identity, OFFSET>,
            OnHttpEquiv: OnHttpEquiv::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            RegisterWindow: RegisterWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBrowserService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBrowserService {}
windows_core::imp::define_interface!(IBrowserService2, IBrowserService2_Vtbl, 0x68bd21cc_438b_11d2_a560_00a0c92dbfe8);
impl core::ops::Deref for IBrowserService2 {
    type Target = IBrowserService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBrowserService2, windows_core::IUnknown, IBrowserService);
impl IBrowserService2 {
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub unsafe fn WndProcBS(&self, hwnd: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
        unsafe { (windows_core::Interface::vtable(self).WndProcBS)(windows_core::Interface::as_raw(self), hwnd, umsg, wparam, lparam) }
    }
    pub unsafe fn SetAsDefFolderSettings(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAsDefFolderSettings)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetViewRect(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnSize(&self, wparam: super::minwindef::WPARAM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSize)(windows_core::Interface::as_raw(self), wparam) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn OnCreate(&self, pcs: *const super::winuser::CREATESTRUCTW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnCreate)(windows_core::Interface::as_raw(self), pcs) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnCommand(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnCommand)(windows_core::Interface::as_raw(self), wparam, lparam) }
    }
    pub unsafe fn OnDestroy(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnDestroy)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn OnNotify(&self, pnm: *const super::winuser::NMHDR) -> super::minwindef::LRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), pnm) }
    }
    pub unsafe fn OnSetFocus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSetFocus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnFrameWindowActivateBS(&self, factive: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnFrameWindowActivateBS)(windows_core::Interface::as_raw(self), factive.into()) }
    }
    pub unsafe fn ReleaseShellView(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseShellView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ActivatePendingView(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivatePendingView)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
    pub unsafe fn CreateViewWindow<P0, P1>(&self, psvnew: P0, psvold: P1, prcview: *const super::windef::RECT) -> windows_core::Result<super::windef::HWND>
    where
        P0: windows_core::Param<super::shobjidl_core::IShellView>,
        P1: windows_core::Param<super::shobjidl_core::IShellView>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateViewWindow)(windows_core::Interface::as_raw(self), psvnew.param().abi(), psvold.param().abi(), prcview, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateBrowserPropSheetExt<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateBrowserPropSheetExt)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetViewWindow(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub unsafe fn GetBaseBrowserData(&self) -> windows_core::Result<LPCBASEBROWSERDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBaseBrowserData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub unsafe fn PutBaseBrowserData(&self) -> LPBASEBROWSERDATA {
        unsafe { (windows_core::Interface::vtable(self).PutBaseBrowserData)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn InitializeTravelLog<P0>(&self, ptl: P0, dw: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITravelLog>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeTravelLog)(windows_core::Interface::as_raw(self), ptl.param().abi(), dw) }
    }
    pub unsafe fn SetTopBrowser(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTopBrowser)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Offline(&self, icmd: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Offline)(windows_core::Interface::as_raw(self), icmd) }
    }
    pub unsafe fn AllowViewResize(&self, f: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AllowViewResize)(windows_core::Interface::as_raw(self), f.into()) }
    }
    pub unsafe fn SetActivateState(&self, u: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetActivateState)(windows_core::Interface::as_raw(self), u) }
    }
    pub unsafe fn UpdateSecureLockIcon(&self, esecurelock: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateSecureLockIcon)(windows_core::Interface::as_raw(self), esecurelock) }
    }
    pub unsafe fn InitializeDownloadManager(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeDownloadManager)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn InitializeTransitionSite(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeTransitionSite)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn _Initialize<P1>(&self, hwnd: super::windef::HWND, pauto: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self)._Initialize)(windows_core::Interface::as_raw(self), hwnd, pauto.param().abi()) }
    }
    pub unsafe fn _CancelPendingNavigationAsync(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._CancelPendingNavigationAsync)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn _CancelPendingView(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._CancelPendingView)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn _MaySaveChanges(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._MaySaveChanges)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn _PauseOrResumeView(&self, fpaused: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._PauseOrResumeView)(windows_core::Interface::as_raw(self), fpaused.into()) }
    }
    pub unsafe fn _DisableModeless(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._DisableModeless)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn _NavigateToPidl(&self, pidl: *const super::shtypes::ITEMIDLIST, grfhlnf: u32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._NavigateToPidl)(windows_core::Interface::as_raw(self), pidl, grfhlnf, dwflags) }
    }
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "shtypes"))]
    pub unsafe fn _TryShell2Rename<P0>(&self, psv: P0, pidlnew: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::shobjidl_core::IShellView>,
    {
        unsafe { (windows_core::Interface::vtable(self)._TryShell2Rename)(windows_core::Interface::as_raw(self), psv.param().abi(), pidlnew) }
    }
    pub unsafe fn _SwitchActivationNow(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._SwitchActivationNow)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn _ExecChildren<P0>(&self, punkbar: P0, fbroadcast: bool, pguidcmdgroup: Option<*const windows_core::GUID>, ncmdid: u32, ncmdexecopt: u32, pvarargin: Option<*const super::oaidl::VARIANTARG>, pvarargout: Option<*mut super::oaidl::VARIANTARG>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self)._ExecChildren)(windows_core::Interface::as_raw(self), punkbar.param().abi(), fbroadcast.into(), pguidcmdgroup.unwrap_or(core::mem::zeroed()) as _, ncmdid, ncmdexecopt, pvarargin.unwrap_or(core::mem::zeroed()) as _, pvarargout.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub unsafe fn _SendChildren(&self, hwndbar: super::windef::HWND, fbroadcast: bool, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._SendChildren)(windows_core::Interface::as_raw(self), hwndbar, fbroadcast.into(), umsg, wparam, lparam) }
    }
    #[cfg(feature = "shobjidl_core")]
    pub unsafe fn GetFolderSetData(&self, pfsd: *mut FOLDERSETDATA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFolderSetData)(windows_core::Interface::as_raw(self), pfsd as _) }
    }
    pub unsafe fn _OnFocusChange(&self, itb: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._OnFocusChange)(windows_core::Interface::as_raw(self), itb) }
    }
    pub unsafe fn v_ShowHideChildWindows(&self, fchildonly: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).v_ShowHideChildWindows)(windows_core::Interface::as_raw(self), fchildonly.into()) }
    }
    pub unsafe fn _get_itbLastFocus(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self)._get_itbLastFocus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn _put_itbLastFocus(&self, itblastfocus: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._put_itbLastFocus)(windows_core::Interface::as_raw(self), itblastfocus) }
    }
    pub unsafe fn _UIActivateView(&self, ustate: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._UIActivateView)(windows_core::Interface::as_raw(self), ustate) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn _GetViewBorderRect(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._GetViewBorderRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _UpdateViewRectSize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._UpdateViewRectSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn _ResizeNextBorder(&self, itb: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._ResizeNextBorder)(windows_core::Interface::as_raw(self), itb) }
    }
    pub unsafe fn _ResizeView(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._ResizeView)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn _GetEffectiveClientArea(&self, lprectborder: *mut super::windef::RECT, hmon: super::windef::HMONITOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._GetEffectiveClientArea)(windows_core::Interface::as_raw(self), lprectborder as _, hmon) }
    }
    #[cfg(all(feature = "objidlbase", feature = "shtypes"))]
    pub unsafe fn v_GetViewStream<P2>(&self, pidl: *const super::shtypes::ITEMIDLIST, grfmode: u32, pwszname: P2) -> Option<super::objidlbase::IStream>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).v_GetViewStream)(windows_core::Interface::as_raw(self), pidl, grfmode, pwszname.param().abi()) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn ForwardViewMsg(&self, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
        unsafe { (windows_core::Interface::vtable(self).ForwardViewMsg)(windows_core::Interface::as_raw(self), umsg, wparam, lparam) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetAcceleratorMenu(&self, hacc: super::windef::HACCEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAcceleratorMenu)(windows_core::Interface::as_raw(self), hacc) }
    }
    pub unsafe fn _GetToolbarCount(&self) -> i32 {
        unsafe { (windows_core::Interface::vtable(self)._GetToolbarCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
    pub unsafe fn _GetToolbarItem(&self, itb: i32) -> LPTOOLBARITEM {
        unsafe { (windows_core::Interface::vtable(self)._GetToolbarItem)(windows_core::Interface::as_raw(self), itb) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn _SaveToolbars<P0>(&self, pstm: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self)._SaveToolbars)(windows_core::Interface::as_raw(self), pstm.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn _LoadToolbars<P0>(&self, pstm: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self)._LoadToolbars)(windows_core::Interface::as_raw(self), pstm.param().abi()) }
    }
    pub unsafe fn _CloseAndReleaseToolbars(&self, fclose: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._CloseAndReleaseToolbars)(windows_core::Interface::as_raw(self), fclose.into()) }
    }
    #[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "windef", feature = "winuser"))]
    pub unsafe fn v_MayGetNextToolbarFocus(&self, lpmsg: *const super::winuser::MSG, itbnext: u32, citb: i32, pptbi: *mut LPTOOLBARITEM, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).v_MayGetNextToolbarFocus)(windows_core::Interface::as_raw(self), lpmsg, itbnext, citb, pptbi as _, phwnd as _) }
    }
    pub unsafe fn _ResizeNextBorderHelper(&self, itb: u32, busehmonitor: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._ResizeNextBorderHelper)(windows_core::Interface::as_raw(self), itb, busehmonitor.into()) }
    }
    pub unsafe fn _FindTBar<P0>(&self, punksrc: P0) -> u32
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self)._FindTBar)(windows_core::Interface::as_raw(self), punksrc.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "windef", feature = "winuser"))]
    pub unsafe fn _SetFocus(&self, ptbi: *const TOOLBARITEM, hwnd: super::windef::HWND, lpmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._SetFocus)(windows_core::Interface::as_raw(self), core::mem::transmute(ptbi), hwnd, lpmsg) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn v_MayTranslateAccelerator(&self, pmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).v_MayTranslateAccelerator)(windows_core::Interface::as_raw(self), pmsg) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn _GetBorderDWHelper<P0>(&self, punksrc: P0, lprectborder: *mut super::windef::RECT, busehmonitor: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self)._GetBorderDWHelper)(windows_core::Interface::as_raw(self), punksrc.param().abi(), lprectborder as _, busehmonitor.into()) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn v_CheckZoneCrossing(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).v_CheckZoneCrossing)(windows_core::Interface::as_raw(self), pidl) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrowserService2_Vtbl {
    pub base__: IBrowserService_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub WndProcBS: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, super::minwindef::WPARAM, super::minwindef::LPARAM) -> super::minwindef::LRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef")))]
    WndProcBS: usize,
    pub SetAsDefFolderSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetViewRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetViewRect: usize,
    #[cfg(feature = "minwindef")]
    pub OnSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnSize: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub OnCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::CREATESTRUCTW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    OnCreate: usize,
    #[cfg(feature = "minwindef")]
    pub OnCommand: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM) -> super::minwindef::LRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnCommand: usize,
    pub OnDestroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::NMHDR) -> super::minwindef::LRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    OnNotify: usize,
    pub OnSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnFrameWindowActivateBS: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ReleaseShellView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivatePendingView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
    pub CreateViewWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::windef::RECT, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef")))]
    CreateViewWindow: usize,
    pub CreateBrowserPropSheetExt: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetViewWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetViewWindow: usize,
    #[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub GetBaseBrowserData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LPCBASEBROWSERDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef")))]
    GetBaseBrowserData: usize,
    #[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
    pub PutBaseBrowserData: unsafe extern "system" fn(*mut core::ffi::c_void) -> LPBASEBROWSERDATA,
    #[cfg(not(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef")))]
    PutBaseBrowserData: usize,
    pub InitializeTravelLog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetTopBrowser: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Offline: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AllowViewResize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetActivateState: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateSecureLockIcon: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub InitializeDownloadManager: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeTransitionSite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub _Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    _Initialize: usize,
    pub _CancelPendingNavigationAsync: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _CancelPendingView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _MaySaveChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _PauseOrResumeView: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub _DisableModeless: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub _NavigateToPidl: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    _NavigateToPidl: usize,
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "shtypes"))]
    pub _TryShell2Rename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oleidl", feature = "shobjidl_core", feature = "shtypes")))]
    _TryShell2Rename: usize,
    pub _SwitchActivationNow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub _ExecChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID, u32, u32, *const super::oaidl::VARIANTARG, *mut super::oaidl::VARIANTARG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    _ExecChildren: usize,
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub _SendChildren: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, windows_core::BOOL, u32, super::minwindef::WPARAM, super::minwindef::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef")))]
    _SendChildren: usize,
    #[cfg(feature = "shobjidl_core")]
    pub GetFolderSetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FOLDERSETDATA) -> windows_core::HRESULT,
    #[cfg(not(feature = "shobjidl_core"))]
    GetFolderSetData: usize,
    pub _OnFocusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub v_ShowHideChildWindows: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub _get_itbLastFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub _put_itbLastFocus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub _UIActivateView: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub _GetViewBorderRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    _GetViewBorderRect: usize,
    pub _UpdateViewRectSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _ResizeNextBorder: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub _ResizeView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub _GetEffectiveClientArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT, super::windef::HMONITOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    _GetEffectiveClientArea: usize,
    #[cfg(all(feature = "objidlbase", feature = "shtypes"))]
    pub v_GetViewStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, u32, windows_core::PCWSTR) -> Option<super::objidlbase::IStream>,
    #[cfg(not(all(feature = "objidlbase", feature = "shtypes")))]
    v_GetViewStream: usize,
    #[cfg(feature = "minwindef")]
    pub ForwardViewMsg: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::minwindef::WPARAM, super::minwindef::LPARAM) -> super::minwindef::LRESULT,
    #[cfg(not(feature = "minwindef"))]
    ForwardViewMsg: usize,
    #[cfg(feature = "windef")]
    pub SetAcceleratorMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HACCEL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetAcceleratorMenu: usize,
    pub _GetToolbarCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    #[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
    pub _GetToolbarItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> LPTOOLBARITEM,
    #[cfg(not(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef")))]
    _GetToolbarItem: usize,
    #[cfg(feature = "objidlbase")]
    pub _SaveToolbars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    _SaveToolbars: usize,
    #[cfg(feature = "objidlbase")]
    pub _LoadToolbars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    _LoadToolbars: usize,
    pub _CloseAndReleaseToolbars: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "windef", feature = "winuser"))]
    pub v_MayGetNextToolbarFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG, u32, i32, *mut LPTOOLBARITEM, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "windef", feature = "winuser")))]
    v_MayGetNextToolbarFocus: usize,
    pub _ResizeNextBorderHelper: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub _FindTBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "windef", feature = "winuser"))]
    pub _SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *const TOOLBARITEM, super::windef::HWND, *const super::winuser::MSG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oleidl", feature = "shobjidl_core", feature = "windef", feature = "winuser")))]
    _SetFocus: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub v_MayTranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    v_MayTranslateAccelerator: usize,
    #[cfg(feature = "windef")]
    pub _GetBorderDWHelper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::windef::RECT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    _GetBorderDWHelper: usize,
    #[cfg(feature = "shtypes")]
    pub v_CheckZoneCrossing: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    v_CheckZoneCrossing: usize,
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBrowserService2_Impl: IBrowserService_Impl {
    fn WndProcBS(&self, hwnd: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT;
    fn SetAsDefFolderSettings(&self) -> windows_core::Result<()>;
    fn GetViewRect(&self) -> windows_core::Result<super::windef::RECT>;
    fn OnSize(&self, wparam: super::minwindef::WPARAM) -> windows_core::Result<()>;
    fn OnCreate(&self, pcs: *const super::winuser::CREATESTRUCTW) -> windows_core::Result<()>;
    fn OnCommand(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT;
    fn OnDestroy(&self) -> windows_core::Result<()>;
    fn OnNotify(&self, pnm: *const super::winuser::NMHDR) -> super::minwindef::LRESULT;
    fn OnSetFocus(&self) -> windows_core::Result<()>;
    fn OnFrameWindowActivateBS(&self, factive: windows_core::BOOL) -> windows_core::Result<()>;
    fn ReleaseShellView(&self) -> windows_core::Result<()>;
    fn ActivatePendingView(&self) -> windows_core::Result<()>;
    fn CreateViewWindow(&self, psvnew: windows_core::Ref<super::shobjidl_core::IShellView>, psvold: windows_core::Ref<super::shobjidl_core::IShellView>, prcview: *const super::windef::RECT) -> windows_core::Result<super::windef::HWND>;
    fn CreateBrowserPropSheetExt(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetViewWindow(&self) -> windows_core::Result<super::windef::HWND>;
    fn GetBaseBrowserData(&self) -> windows_core::Result<LPCBASEBROWSERDATA>;
    fn PutBaseBrowserData(&self) -> LPBASEBROWSERDATA;
    fn InitializeTravelLog(&self, ptl: windows_core::Ref<ITravelLog>, dw: u32) -> windows_core::Result<()>;
    fn SetTopBrowser(&self) -> windows_core::Result<()>;
    fn Offline(&self, icmd: i32) -> windows_core::Result<()>;
    fn AllowViewResize(&self, f: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetActivateState(&self, u: u32) -> windows_core::Result<()>;
    fn UpdateSecureLockIcon(&self, esecurelock: i32) -> windows_core::Result<()>;
    fn InitializeDownloadManager(&self) -> windows_core::Result<()>;
    fn InitializeTransitionSite(&self) -> windows_core::Result<()>;
    fn _Initialize(&self, hwnd: super::windef::HWND, pauto: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn _CancelPendingNavigationAsync(&self) -> windows_core::Result<()>;
    fn _CancelPendingView(&self) -> windows_core::Result<()>;
    fn _MaySaveChanges(&self) -> windows_core::Result<()>;
    fn _PauseOrResumeView(&self, fpaused: windows_core::BOOL) -> windows_core::Result<()>;
    fn _DisableModeless(&self) -> windows_core::Result<()>;
    fn _NavigateToPidl(&self, pidl: *const super::shtypes::ITEMIDLIST, grfhlnf: u32, dwflags: u32) -> windows_core::Result<()>;
    fn _TryShell2Rename(&self, psv: windows_core::Ref<super::shobjidl_core::IShellView>, pidlnew: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<()>;
    fn _SwitchActivationNow(&self) -> windows_core::Result<()>;
    fn _ExecChildren(&self, punkbar: windows_core::Ref<windows_core::IUnknown>, fbroadcast: windows_core::BOOL, pguidcmdgroup: *const windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvarargin: *const super::oaidl::VARIANTARG, pvarargout: *mut super::oaidl::VARIANTARG) -> windows_core::Result<()>;
    fn _SendChildren(&self, hwndbar: super::windef::HWND, fbroadcast: windows_core::BOOL, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<()>;
    fn GetFolderSetData(&self, pfsd: *mut FOLDERSETDATA) -> windows_core::Result<()>;
    fn _OnFocusChange(&self, itb: u32) -> windows_core::Result<()>;
    fn v_ShowHideChildWindows(&self, fchildonly: windows_core::BOOL) -> windows_core::Result<()>;
    fn _get_itbLastFocus(&self) -> u32;
    fn _put_itbLastFocus(&self, itblastfocus: u32) -> windows_core::Result<()>;
    fn _UIActivateView(&self, ustate: u32) -> windows_core::Result<()>;
    fn _GetViewBorderRect(&self) -> windows_core::Result<super::windef::RECT>;
    fn _UpdateViewRectSize(&self) -> windows_core::Result<()>;
    fn _ResizeNextBorder(&self, itb: u32) -> windows_core::Result<()>;
    fn _ResizeView(&self) -> windows_core::Result<()>;
    fn _GetEffectiveClientArea(&self, lprectborder: *mut super::windef::RECT, hmon: super::windef::HMONITOR) -> windows_core::Result<()>;
    fn v_GetViewStream(&self, pidl: *const super::shtypes::ITEMIDLIST, grfmode: u32, pwszname: &windows_core::PCWSTR) -> Option<super::objidlbase::IStream>;
    fn ForwardViewMsg(&self, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT;
    fn SetAcceleratorMenu(&self, hacc: super::windef::HACCEL) -> windows_core::Result<()>;
    fn _GetToolbarCount(&self) -> i32;
    fn _GetToolbarItem(&self, itb: i32) -> LPTOOLBARITEM;
    fn _SaveToolbars(&self, pstm: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn _LoadToolbars(&self, pstm: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn _CloseAndReleaseToolbars(&self, fclose: windows_core::BOOL) -> windows_core::Result<()>;
    fn v_MayGetNextToolbarFocus(&self, lpmsg: *const super::winuser::MSG, itbnext: u32, citb: i32, pptbi: *mut LPTOOLBARITEM, phwnd: *mut super::windef::HWND) -> windows_core::Result<()>;
    fn _ResizeNextBorderHelper(&self, itb: u32, busehmonitor: windows_core::BOOL) -> windows_core::Result<()>;
    fn _FindTBar(&self, punksrc: windows_core::Ref<windows_core::IUnknown>) -> u32;
    fn _SetFocus(&self, ptbi: *const TOOLBARITEM, hwnd: super::windef::HWND, lpmsg: *const super::winuser::MSG) -> windows_core::Result<()>;
    fn v_MayTranslateAccelerator(&self, pmsg: *const super::winuser::MSG) -> windows_core::Result<()>;
    fn _GetBorderDWHelper(&self, punksrc: windows_core::Ref<windows_core::IUnknown>, lprectborder: *mut super::windef::RECT, busehmonitor: windows_core::BOOL) -> windows_core::Result<()>;
    fn v_CheckZoneCrossing(&self, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<()>;
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl IBrowserService2_Vtbl {
    pub const fn new<Identity: IBrowserService2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WndProcBS<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::WndProcBS(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam))
            }
        }
        unsafe extern "system" fn SetAsDefFolderSettings<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::SetAsDefFolderSettings(this).into()
            }
        }
        unsafe extern "system" fn GetViewRect<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService2_Impl::GetViewRect(this) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnSize<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::OnSize(this, core::mem::transmute_copy(&wparam)).into()
            }
        }
        unsafe extern "system" fn OnCreate<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcs: *const super::winuser::CREATESTRUCTW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::OnCreate(this, core::mem::transmute_copy(&pcs)).into()
            }
        }
        unsafe extern "system" fn OnCommand<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::OnCommand(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam))
            }
        }
        unsafe extern "system" fn OnDestroy<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::OnDestroy(this).into()
            }
        }
        unsafe extern "system" fn OnNotify<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnm: *const super::winuser::NMHDR) -> super::minwindef::LRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::OnNotify(this, core::mem::transmute_copy(&pnm))
            }
        }
        unsafe extern "system" fn OnSetFocus<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::OnSetFocus(this).into()
            }
        }
        unsafe extern "system" fn OnFrameWindowActivateBS<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factive: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::OnFrameWindowActivateBS(this, core::mem::transmute_copy(&factive)).into()
            }
        }
        unsafe extern "system" fn ReleaseShellView<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::ReleaseShellView(this).into()
            }
        }
        unsafe extern "system" fn ActivatePendingView<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::ActivatePendingView(this).into()
            }
        }
        unsafe extern "system" fn CreateViewWindow<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psvnew: *mut core::ffi::c_void, psvold: *mut core::ffi::c_void, prcview: *const super::windef::RECT, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService2_Impl::CreateViewWindow(this, core::mem::transmute_copy(&psvnew), core::mem::transmute_copy(&psvold), core::mem::transmute_copy(&prcview)) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBrowserPropSheetExt<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::CreateBrowserPropSheetExt(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetViewWindow<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwndview: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService2_Impl::GetViewWindow(this) {
                    Ok(ok__) => {
                        phwndview.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBaseBrowserData<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbd: *mut LPCBASEBROWSERDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService2_Impl::GetBaseBrowserData(this) {
                    Ok(ok__) => {
                        pbbd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutBaseBrowserData<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> LPBASEBROWSERDATA {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::PutBaseBrowserData(this)
            }
        }
        unsafe extern "system" fn InitializeTravelLog<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptl: *mut core::ffi::c_void, dw: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::InitializeTravelLog(this, core::mem::transmute_copy(&ptl), core::mem::transmute_copy(&dw)).into()
            }
        }
        unsafe extern "system" fn SetTopBrowser<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::SetTopBrowser(this).into()
            }
        }
        unsafe extern "system" fn Offline<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmd: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::Offline(this, core::mem::transmute_copy(&icmd)).into()
            }
        }
        unsafe extern "system" fn AllowViewResize<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, f: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::AllowViewResize(this, core::mem::transmute_copy(&f)).into()
            }
        }
        unsafe extern "system" fn SetActivateState<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::SetActivateState(this, core::mem::transmute_copy(&u)).into()
            }
        }
        unsafe extern "system" fn UpdateSecureLockIcon<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esecurelock: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::UpdateSecureLockIcon(this, core::mem::transmute_copy(&esecurelock)).into()
            }
        }
        unsafe extern "system" fn InitializeDownloadManager<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::InitializeDownloadManager(this).into()
            }
        }
        unsafe extern "system" fn InitializeTransitionSite<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::InitializeTransitionSite(this).into()
            }
        }
        unsafe extern "system" fn _Initialize<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, pauto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_Initialize(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pauto)).into()
            }
        }
        unsafe extern "system" fn _CancelPendingNavigationAsync<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_CancelPendingNavigationAsync(this).into()
            }
        }
        unsafe extern "system" fn _CancelPendingView<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_CancelPendingView(this).into()
            }
        }
        unsafe extern "system" fn _MaySaveChanges<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_MaySaveChanges(this).into()
            }
        }
        unsafe extern "system" fn _PauseOrResumeView<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpaused: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_PauseOrResumeView(this, core::mem::transmute_copy(&fpaused)).into()
            }
        }
        unsafe extern "system" fn _DisableModeless<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_DisableModeless(this).into()
            }
        }
        unsafe extern "system" fn _NavigateToPidl<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, grfhlnf: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_NavigateToPidl(this, core::mem::transmute_copy(&pidl), core::mem::transmute_copy(&grfhlnf), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn _TryShell2Rename<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psv: *mut core::ffi::c_void, pidlnew: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_TryShell2Rename(this, core::mem::transmute_copy(&psv), core::mem::transmute_copy(&pidlnew)).into()
            }
        }
        unsafe extern "system" fn _SwitchActivationNow<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_SwitchActivationNow(this).into()
            }
        }
        unsafe extern "system" fn _ExecChildren<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkbar: *mut core::ffi::c_void, fbroadcast: windows_core::BOOL, pguidcmdgroup: *const windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvarargin: *const super::oaidl::VARIANTARG, pvarargout: *mut super::oaidl::VARIANTARG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_ExecChildren(this, core::mem::transmute_copy(&punkbar), core::mem::transmute_copy(&fbroadcast), core::mem::transmute_copy(&pguidcmdgroup), core::mem::transmute_copy(&ncmdid), core::mem::transmute_copy(&ncmdexecopt), core::mem::transmute_copy(&pvarargin), core::mem::transmute_copy(&pvarargout)).into()
            }
        }
        unsafe extern "system" fn _SendChildren<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndbar: super::windef::HWND, fbroadcast: windows_core::BOOL, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_SendChildren(this, core::mem::transmute_copy(&hwndbar), core::mem::transmute_copy(&fbroadcast), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn GetFolderSetData<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsd: *mut FOLDERSETDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::GetFolderSetData(this, core::mem::transmute_copy(&pfsd)).into()
            }
        }
        unsafe extern "system" fn _OnFocusChange<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itb: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_OnFocusChange(this, core::mem::transmute_copy(&itb)).into()
            }
        }
        unsafe extern "system" fn v_ShowHideChildWindows<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fchildonly: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::v_ShowHideChildWindows(this, core::mem::transmute_copy(&fchildonly)).into()
            }
        }
        unsafe extern "system" fn _get_itbLastFocus<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_get_itbLastFocus(this)
            }
        }
        unsafe extern "system" fn _put_itbLastFocus<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itblastfocus: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_put_itbLastFocus(this, core::mem::transmute_copy(&itblastfocus)).into()
            }
        }
        unsafe extern "system" fn _UIActivateView<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ustate: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_UIActivateView(this, core::mem::transmute_copy(&ustate)).into()
            }
        }
        unsafe extern "system" fn _GetViewBorderRect<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService2_Impl::_GetViewBorderRect(this) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _UpdateViewRectSize<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_UpdateViewRectSize(this).into()
            }
        }
        unsafe extern "system" fn _ResizeNextBorder<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itb: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_ResizeNextBorder(this, core::mem::transmute_copy(&itb)).into()
            }
        }
        unsafe extern "system" fn _ResizeView<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_ResizeView(this).into()
            }
        }
        unsafe extern "system" fn _GetEffectiveClientArea<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprectborder: *mut super::windef::RECT, hmon: super::windef::HMONITOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_GetEffectiveClientArea(this, core::mem::transmute_copy(&lprectborder), core::mem::transmute_copy(&hmon)).into()
            }
        }
        unsafe extern "system" fn v_GetViewStream<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, grfmode: u32, pwszname: windows_core::PCWSTR) -> Option<super::objidlbase::IStream> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::v_GetViewStream(this, core::mem::transmute_copy(&pidl), core::mem::transmute_copy(&grfmode), core::mem::transmute(&pwszname))
            }
        }
        unsafe extern "system" fn ForwardViewMsg<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::ForwardViewMsg(this, core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam))
            }
        }
        unsafe extern "system" fn SetAcceleratorMenu<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hacc: super::windef::HACCEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::SetAcceleratorMenu(this, core::mem::transmute_copy(&hacc)).into()
            }
        }
        unsafe extern "system" fn _GetToolbarCount<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_GetToolbarCount(this)
            }
        }
        unsafe extern "system" fn _GetToolbarItem<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itb: i32) -> LPTOOLBARITEM {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_GetToolbarItem(this, core::mem::transmute_copy(&itb))
            }
        }
        unsafe extern "system" fn _SaveToolbars<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_SaveToolbars(this, core::mem::transmute_copy(&pstm)).into()
            }
        }
        unsafe extern "system" fn _LoadToolbars<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_LoadToolbars(this, core::mem::transmute_copy(&pstm)).into()
            }
        }
        unsafe extern "system" fn _CloseAndReleaseToolbars<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fclose: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_CloseAndReleaseToolbars(this, core::mem::transmute_copy(&fclose)).into()
            }
        }
        unsafe extern "system" fn v_MayGetNextToolbarFocus<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsg: *const super::winuser::MSG, itbnext: u32, citb: i32, pptbi: *mut LPTOOLBARITEM, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::v_MayGetNextToolbarFocus(this, core::mem::transmute_copy(&lpmsg), core::mem::transmute_copy(&itbnext), core::mem::transmute_copy(&citb), core::mem::transmute_copy(&pptbi), core::mem::transmute_copy(&phwnd)).into()
            }
        }
        unsafe extern "system" fn _ResizeNextBorderHelper<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itb: u32, busehmonitor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_ResizeNextBorderHelper(this, core::mem::transmute_copy(&itb), core::mem::transmute_copy(&busehmonitor)).into()
            }
        }
        unsafe extern "system" fn _FindTBar<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksrc: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_FindTBar(this, core::mem::transmute_copy(&punksrc))
            }
        }
        unsafe extern "system" fn _SetFocus<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptbi: *const TOOLBARITEM, hwnd: super::windef::HWND, lpmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_SetFocus(this, core::mem::transmute_copy(&ptbi), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&lpmsg)).into()
            }
        }
        unsafe extern "system" fn v_MayTranslateAccelerator<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::v_MayTranslateAccelerator(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        unsafe extern "system" fn _GetBorderDWHelper<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksrc: *mut core::ffi::c_void, lprectborder: *mut super::windef::RECT, busehmonitor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::_GetBorderDWHelper(this, core::mem::transmute_copy(&punksrc), core::mem::transmute_copy(&lprectborder), core::mem::transmute_copy(&busehmonitor)).into()
            }
        }
        unsafe extern "system" fn v_CheckZoneCrossing<Identity: IBrowserService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService2_Impl::v_CheckZoneCrossing(this, core::mem::transmute_copy(&pidl)).into()
            }
        }
        Self {
            base__: IBrowserService_Vtbl::new::<Identity, OFFSET>(),
            WndProcBS: WndProcBS::<Identity, OFFSET>,
            SetAsDefFolderSettings: SetAsDefFolderSettings::<Identity, OFFSET>,
            GetViewRect: GetViewRect::<Identity, OFFSET>,
            OnSize: OnSize::<Identity, OFFSET>,
            OnCreate: OnCreate::<Identity, OFFSET>,
            OnCommand: OnCommand::<Identity, OFFSET>,
            OnDestroy: OnDestroy::<Identity, OFFSET>,
            OnNotify: OnNotify::<Identity, OFFSET>,
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnFrameWindowActivateBS: OnFrameWindowActivateBS::<Identity, OFFSET>,
            ReleaseShellView: ReleaseShellView::<Identity, OFFSET>,
            ActivatePendingView: ActivatePendingView::<Identity, OFFSET>,
            CreateViewWindow: CreateViewWindow::<Identity, OFFSET>,
            CreateBrowserPropSheetExt: CreateBrowserPropSheetExt::<Identity, OFFSET>,
            GetViewWindow: GetViewWindow::<Identity, OFFSET>,
            GetBaseBrowserData: GetBaseBrowserData::<Identity, OFFSET>,
            PutBaseBrowserData: PutBaseBrowserData::<Identity, OFFSET>,
            InitializeTravelLog: InitializeTravelLog::<Identity, OFFSET>,
            SetTopBrowser: SetTopBrowser::<Identity, OFFSET>,
            Offline: Offline::<Identity, OFFSET>,
            AllowViewResize: AllowViewResize::<Identity, OFFSET>,
            SetActivateState: SetActivateState::<Identity, OFFSET>,
            UpdateSecureLockIcon: UpdateSecureLockIcon::<Identity, OFFSET>,
            InitializeDownloadManager: InitializeDownloadManager::<Identity, OFFSET>,
            InitializeTransitionSite: InitializeTransitionSite::<Identity, OFFSET>,
            _Initialize: _Initialize::<Identity, OFFSET>,
            _CancelPendingNavigationAsync: _CancelPendingNavigationAsync::<Identity, OFFSET>,
            _CancelPendingView: _CancelPendingView::<Identity, OFFSET>,
            _MaySaveChanges: _MaySaveChanges::<Identity, OFFSET>,
            _PauseOrResumeView: _PauseOrResumeView::<Identity, OFFSET>,
            _DisableModeless: _DisableModeless::<Identity, OFFSET>,
            _NavigateToPidl: _NavigateToPidl::<Identity, OFFSET>,
            _TryShell2Rename: _TryShell2Rename::<Identity, OFFSET>,
            _SwitchActivationNow: _SwitchActivationNow::<Identity, OFFSET>,
            _ExecChildren: _ExecChildren::<Identity, OFFSET>,
            _SendChildren: _SendChildren::<Identity, OFFSET>,
            GetFolderSetData: GetFolderSetData::<Identity, OFFSET>,
            _OnFocusChange: _OnFocusChange::<Identity, OFFSET>,
            v_ShowHideChildWindows: v_ShowHideChildWindows::<Identity, OFFSET>,
            _get_itbLastFocus: _get_itbLastFocus::<Identity, OFFSET>,
            _put_itbLastFocus: _put_itbLastFocus::<Identity, OFFSET>,
            _UIActivateView: _UIActivateView::<Identity, OFFSET>,
            _GetViewBorderRect: _GetViewBorderRect::<Identity, OFFSET>,
            _UpdateViewRectSize: _UpdateViewRectSize::<Identity, OFFSET>,
            _ResizeNextBorder: _ResizeNextBorder::<Identity, OFFSET>,
            _ResizeView: _ResizeView::<Identity, OFFSET>,
            _GetEffectiveClientArea: _GetEffectiveClientArea::<Identity, OFFSET>,
            v_GetViewStream: v_GetViewStream::<Identity, OFFSET>,
            ForwardViewMsg: ForwardViewMsg::<Identity, OFFSET>,
            SetAcceleratorMenu: SetAcceleratorMenu::<Identity, OFFSET>,
            _GetToolbarCount: _GetToolbarCount::<Identity, OFFSET>,
            _GetToolbarItem: _GetToolbarItem::<Identity, OFFSET>,
            _SaveToolbars: _SaveToolbars::<Identity, OFFSET>,
            _LoadToolbars: _LoadToolbars::<Identity, OFFSET>,
            _CloseAndReleaseToolbars: _CloseAndReleaseToolbars::<Identity, OFFSET>,
            v_MayGetNextToolbarFocus: v_MayGetNextToolbarFocus::<Identity, OFFSET>,
            _ResizeNextBorderHelper: _ResizeNextBorderHelper::<Identity, OFFSET>,
            _FindTBar: _FindTBar::<Identity, OFFSET>,
            _SetFocus: _SetFocus::<Identity, OFFSET>,
            v_MayTranslateAccelerator: v_MayTranslateAccelerator::<Identity, OFFSET>,
            _GetBorderDWHelper: _GetBorderDWHelper::<Identity, OFFSET>,
            v_CheckZoneCrossing: v_CheckZoneCrossing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBrowserService2 as windows_core::Interface>::IID || iid == &<IBrowserService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBrowserService2 {}
windows_core::imp::define_interface!(IBrowserService3, IBrowserService3_Vtbl, 0x27d7ce21_762d_48f3_86f3_40e2fd3749c4);
impl core::ops::Deref for IBrowserService3 {
    type Target = IBrowserService2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBrowserService3, windows_core::IUnknown, IBrowserService, IBrowserService2);
impl IBrowserService3 {
    #[cfg(feature = "windef")]
    pub unsafe fn _PositionViewWindow(&self, hwnd: super::windef::HWND, prc: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._PositionViewWindow)(windows_core::Interface::as_raw(self), hwnd, prc) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn IEParseDisplayNameEx<P1>(&self, uicp: u32, pwszpath: P1, dwflags: u32) -> windows_core::Result<super::shtypes::LPITEMIDLIST>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IEParseDisplayNameEx)(windows_core::Interface::as_raw(self), uicp, pwszpath.param().abi(), dwflags, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrowserService3_Vtbl {
    pub base__: IBrowserService2_Vtbl,
    #[cfg(feature = "windef")]
    pub _PositionViewWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    _PositionViewWindow: usize,
    #[cfg(feature = "shtypes")]
    pub IEParseDisplayNameEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    IEParseDisplayNameEx: usize,
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBrowserService3_Impl: IBrowserService2_Impl {
    fn _PositionViewWindow(&self, hwnd: super::windef::HWND, prc: *const super::windef::RECT) -> windows_core::Result<()>;
    fn IEParseDisplayNameEx(&self, uicp: u32, pwszpath: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<super::shtypes::LPITEMIDLIST>;
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl IBrowserService3_Vtbl {
    pub const fn new<Identity: IBrowserService3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _PositionViewWindow<Identity: IBrowserService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, prc: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService3_Impl::_PositionViewWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn IEParseDisplayNameEx<Identity: IBrowserService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicp: u32, pwszpath: windows_core::PCWSTR, dwflags: u32, ppidlout: *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrowserService3_Impl::IEParseDisplayNameEx(this, core::mem::transmute_copy(&uicp), core::mem::transmute(&pwszpath), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppidlout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IBrowserService2_Vtbl::new::<Identity, OFFSET>(),
            _PositionViewWindow: _PositionViewWindow::<Identity, OFFSET>,
            IEParseDisplayNameEx: IEParseDisplayNameEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBrowserService3 as windows_core::Interface>::IID || iid == &<IBrowserService as windows_core::Interface>::IID || iid == &<IBrowserService2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBrowserService3 {}
windows_core::imp::define_interface!(IBrowserService4, IBrowserService4_Vtbl, 0x639f1bff_e135_4096_abd8_e0f504d649a4);
impl core::ops::Deref for IBrowserService4 {
    type Target = IBrowserService3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBrowserService4, windows_core::IUnknown, IBrowserService, IBrowserService2, IBrowserService3);
impl IBrowserService4 {
    pub unsafe fn ActivateView(&self, fpendingview: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateView)(windows_core::Interface::as_raw(self), fpendingview.into()) }
    }
    pub unsafe fn SaveViewState(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveViewState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn _ResizeAllBorders(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self)._ResizeAllBorders)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrowserService4_Vtbl {
    pub base__: IBrowserService3_Vtbl,
    pub ActivateView: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SaveViewState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _ResizeAllBorders: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBrowserService4_Impl: IBrowserService3_Impl {
    fn ActivateView(&self, fpendingview: windows_core::BOOL) -> windows_core::Result<()>;
    fn SaveViewState(&self) -> windows_core::Result<()>;
    fn _ResizeAllBorders(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl IBrowserService4_Vtbl {
    pub const fn new<Identity: IBrowserService4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateView<Identity: IBrowserService4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpendingview: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService4_Impl::ActivateView(this, core::mem::transmute_copy(&fpendingview)).into()
            }
        }
        unsafe extern "system" fn SaveViewState<Identity: IBrowserService4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService4_Impl::SaveViewState(this).into()
            }
        }
        unsafe extern "system" fn _ResizeAllBorders<Identity: IBrowserService4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBrowserService4_Impl::_ResizeAllBorders(this).into()
            }
        }
        Self {
            base__: IBrowserService3_Vtbl::new::<Identity, OFFSET>(),
            ActivateView: ActivateView::<Identity, OFFSET>,
            SaveViewState: SaveViewState::<Identity, OFFSET>,
            _ResizeAllBorders: _ResizeAllBorders::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBrowserService4 as windows_core::Interface>::IID || iid == &<IBrowserService as windows_core::Interface>::IID || iid == &<IBrowserService2 as windows_core::Interface>::IID || iid == &<IBrowserService3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBrowserService4 {}
pub type IEPDNFLAGS = i32;
pub const IEPDN_BINDINGUI: IEPDNFLAGS = 1;
windows_core::imp::define_interface!(IExpDispSupport, IExpDispSupport_Vtbl, 0x0d7d1d00_6fc0_11d0_a974_00c04fd705a2);
windows_core::imp::interface_hierarchy!(IExpDispSupport, windows_core::IUnknown);
impl IExpDispSupport {
    #[cfg(feature = "ocidl")]
    pub unsafe fn FindConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<super::ocidl::IConnectionPoint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindConnectionPoint)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn OnTranslateAccelerator(&self, pmsg: *const super::winuser::MSG, grfmodifiers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTranslateAccelerator)(windows_core::Interface::as_raw(self), pmsg, grfmodifiers) }
    }
    #[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn OnInvoke(&self, dispidmember: super::oaidl::DISPID, iid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInvoke)(windows_core::Interface::as_raw(self), dispidmember, iid, lcid, wflags, pdispparams, core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo), puargerr as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpDispSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ocidl")]
    pub FindConnectionPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    FindConnectionPoint: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub OnTranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    OnTranslateAccelerator: usize,
    #[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub OnInvoke: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *const windows_core::GUID, super::winnt::LCID, u16, *const super::oaidl::DISPPARAMS, *mut super::oaidl::VARIANT, *mut super::oaidl::EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    OnInvoke: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "ocidl", feature = "windef", feature = "winnt", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
pub trait IExpDispSupport_Impl: windows_core::IUnknownImpl {
    fn FindConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<super::ocidl::IConnectionPoint>;
    fn OnTranslateAccelerator(&self, pmsg: *const super::winuser::MSG, grfmodifiers: u32) -> windows_core::Result<()>;
    fn OnInvoke(&self, dispidmember: super::oaidl::DISPID, iid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "ocidl", feature = "windef", feature = "winnt", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl IExpDispSupport_Vtbl {
    pub const fn new<Identity: IExpDispSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindConnectionPoint<Identity: IExpDispSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppccp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExpDispSupport_Impl::FindConnectionPoint(this, core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppccp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnTranslateAccelerator<Identity: IExpDispSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::winuser::MSG, grfmodifiers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExpDispSupport_Impl::OnTranslateAccelerator(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&grfmodifiers)).into()
            }
        }
        unsafe extern "system" fn OnInvoke<Identity: IExpDispSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: super::oaidl::DISPID, iid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExpDispSupport_Impl::OnInvoke(this, core::mem::transmute_copy(&dispidmember), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindConnectionPoint: FindConnectionPoint::<Identity, OFFSET>,
            OnTranslateAccelerator: OnTranslateAccelerator::<Identity, OFFSET>,
            OnInvoke: OnInvoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExpDispSupport as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "ocidl", feature = "windef", feature = "winnt", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IExpDispSupport {}
windows_core::imp::define_interface!(IExpDispSupportXP, IExpDispSupportXP_Vtbl, 0x2f0dd58c_f789_4f14_99fb_9293b3c9c212);
windows_core::imp::interface_hierarchy!(IExpDispSupportXP, windows_core::IUnknown);
impl IExpDispSupportXP {
    #[cfg(feature = "ocidl")]
    pub unsafe fn FindCIE4ConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<CIE4ConnectionPoint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindCIE4ConnectionPoint)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn OnTranslateAccelerator(&self, pmsg: *const super::winuser::MSG, grfmodifiers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTranslateAccelerator)(windows_core::Interface::as_raw(self), pmsg, grfmodifiers) }
    }
    #[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn OnInvoke(&self, dispidmember: super::oaidl::DISPID, iid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInvoke)(windows_core::Interface::as_raw(self), dispidmember, iid, lcid, wflags, pdispparams, core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo), puargerr as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpDispSupportXP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ocidl")]
    pub FindCIE4ConnectionPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    FindCIE4ConnectionPoint: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub OnTranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    OnTranslateAccelerator: usize,
    #[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub OnInvoke: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *const windows_core::GUID, super::winnt::LCID, u16, *const super::oaidl::DISPPARAMS, *mut super::oaidl::VARIANT, *mut super::oaidl::EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    OnInvoke: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "ocidl", feature = "windef", feature = "winnt", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
pub trait IExpDispSupportXP_Impl: windows_core::IUnknownImpl {
    fn FindCIE4ConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<CIE4ConnectionPoint>;
    fn OnTranslateAccelerator(&self, pmsg: *const super::winuser::MSG, grfmodifiers: u32) -> windows_core::Result<()>;
    fn OnInvoke(&self, dispidmember: super::oaidl::DISPID, iid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "ocidl", feature = "windef", feature = "winnt", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl IExpDispSupportXP_Vtbl {
    pub const fn new<Identity: IExpDispSupportXP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindCIE4ConnectionPoint<Identity: IExpDispSupportXP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppccp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExpDispSupportXP_Impl::FindCIE4ConnectionPoint(this, core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppccp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnTranslateAccelerator<Identity: IExpDispSupportXP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::winuser::MSG, grfmodifiers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExpDispSupportXP_Impl::OnTranslateAccelerator(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&grfmodifiers)).into()
            }
        }
        unsafe extern "system" fn OnInvoke<Identity: IExpDispSupportXP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: super::oaidl::DISPID, iid: *const windows_core::GUID, lcid: super::winnt::LCID, wflags: u16, pdispparams: *const super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExpDispSupportXP_Impl::OnInvoke(this, core::mem::transmute_copy(&dispidmember), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindCIE4ConnectionPoint: FindCIE4ConnectionPoint::<Identity, OFFSET>,
            OnTranslateAccelerator: OnTranslateAccelerator::<Identity, OFFSET>,
            OnInvoke: OnInvoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExpDispSupportXP as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "ocidl", feature = "windef", feature = "winnt", feature = "winuser", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IExpDispSupportXP {}
windows_core::imp::define_interface!(IShellService, IShellService_Vtbl, 0x5836fb00_8187_11cf_a12b_00aa004ae837);
windows_core::imp::interface_hierarchy!(IShellService, windows_core::IUnknown);
impl IShellService {
    pub unsafe fn SetOwner<P0>(&self, punkowner: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOwner)(windows_core::Interface::as_raw(self), punkowner.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IShellService_Impl: windows_core::IUnknownImpl {
    fn SetOwner(&self, punkowner: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IShellService_Vtbl {
    pub const fn new<Identity: IShellService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOwner<Identity: IShellService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkowner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellService_Impl::SetOwner(this, core::mem::transmute_copy(&punkowner)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetOwner: SetOwner::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IShellService {}
pub const ITB_VIEW: u32 = 4294967295;
#[cfg(feature = "shobjidl_core")]
windows_core::imp::define_interface!(ITrackShellMenu, ITrackShellMenu_Vtbl, 0x8278f932_2a3e_11d2_838f_00c04fd918d0);
#[cfg(feature = "shobjidl_core")]
impl core::ops::Deref for ITrackShellMenu {
    type Target = super::shobjidl_core::IShellMenu;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "shobjidl_core")]
windows_core::imp::interface_hierarchy!(ITrackShellMenu, windows_core::IUnknown, super::shobjidl_core::IShellMenu);
#[cfg(feature = "shobjidl_core")]
impl ITrackShellMenu {
    #[cfg(feature = "windef")]
    pub unsafe fn SetObscured<P1>(&self, hwndtb: super::windef::HWND, punkband: P1, dwsmsetflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetObscured)(windows_core::Interface::as_raw(self), hwndtb, punkband.param().abi(), dwsmsetflags) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Popup(&self, hwnd: super::windef::HWND, ppt: *const super::windef::POINTL, prcexclude: *const super::windef::RECTL, dwflags: super::shobjidl_core::MP_POPUPFLAGS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Popup)(windows_core::Interface::as_raw(self), hwnd, ppt, prcexclude, dwflags) }
    }
}
#[cfg(feature = "shobjidl_core")]
#[repr(C)]
#[doc(hidden)]
pub struct ITrackShellMenu_Vtbl {
    pub base__: super::shobjidl_core::IShellMenu_Vtbl,
    #[cfg(feature = "windef")]
    pub SetObscured: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetObscured: usize,
    #[cfg(feature = "windef")]
    pub Popup: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const super::windef::POINTL, *const super::windef::RECTL, super::shobjidl_core::MP_POPUPFLAGS) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Popup: usize,
}
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub trait ITrackShellMenu_Impl: super::shobjidl_core::IShellMenu_Impl {
    fn SetObscured(&self, hwndtb: super::windef::HWND, punkband: windows_core::Ref<windows_core::IUnknown>, dwsmsetflags: u32) -> windows_core::Result<()>;
    fn Popup(&self, hwnd: super::windef::HWND, ppt: *const super::windef::POINTL, prcexclude: *const super::windef::RECTL, dwflags: super::shobjidl_core::MP_POPUPFLAGS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl ITrackShellMenu_Vtbl {
    pub const fn new<Identity: ITrackShellMenu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetObscured<Identity: ITrackShellMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndtb: super::windef::HWND, punkband: *mut core::ffi::c_void, dwsmsetflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrackShellMenu_Impl::SetObscured(this, core::mem::transmute_copy(&hwndtb), core::mem::transmute_copy(&punkband), core::mem::transmute_copy(&dwsmsetflags)).into()
            }
        }
        unsafe extern "system" fn Popup<Identity: ITrackShellMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, ppt: *const super::windef::POINTL, prcexclude: *const super::windef::RECTL, dwflags: super::shobjidl_core::MP_POPUPFLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrackShellMenu_Impl::Popup(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&ppt), core::mem::transmute_copy(&prcexclude), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: super::shobjidl_core::IShellMenu_Vtbl::new::<Identity, OFFSET>(),
            SetObscured: SetObscured::<Identity, OFFSET>,
            Popup: Popup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrackShellMenu as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IShellMenu as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
impl windows_core::RuntimeName for ITrackShellMenu {}
windows_core::imp::define_interface!(ITravelEntry, ITravelEntry_Vtbl, 0xf46edb3b_bc2f_11d0_9412_00aa00a3ebd3);
windows_core::imp::interface_hierarchy!(ITravelEntry, windows_core::IUnknown);
impl ITravelEntry {
    pub unsafe fn Invoke<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn Update<P0>(&self, punk: P0, fislocalanchor: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), punk.param().abi(), fislocalanchor.into()) }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn GetPidl(&self) -> windows_core::Result<super::shtypes::LPITEMIDLIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPidl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITravelEntry_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub GetPidl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    GetPidl: usize,
}
#[cfg(feature = "shtypes")]
pub trait ITravelEntry_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Update(&self, punk: windows_core::Ref<windows_core::IUnknown>, fislocalanchor: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetPidl(&self) -> windows_core::Result<super::shtypes::LPITEMIDLIST>;
}
#[cfg(feature = "shtypes")]
impl ITravelEntry_Vtbl {
    pub const fn new<Identity: ITravelEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: ITravelEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelEntry_Impl::Invoke(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn Update<Identity: ITravelEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, fislocalanchor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelEntry_Impl::Update(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&fislocalanchor)).into()
            }
        }
        unsafe extern "system" fn GetPidl<Identity: ITravelEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidl: *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelEntry_Impl::GetPidl(this) {
                    Ok(ok__) => {
                        ppidl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            GetPidl: GetPidl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITravelEntry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shtypes")]
impl windows_core::RuntimeName for ITravelEntry {}
windows_core::imp::define_interface!(ITravelLog, ITravelLog_Vtbl, 0x66a9cb08_4802_11d2_a561_00a0c92dbfe8);
windows_core::imp::interface_hierarchy!(ITravelLog, windows_core::IUnknown);
impl ITravelLog {
    pub unsafe fn AddEntry<P0>(&self, punk: P0, fislocalanchor: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEntry)(windows_core::Interface::as_raw(self), punk.param().abi(), fislocalanchor.into()) }
    }
    pub unsafe fn UpdateEntry<P0>(&self, punk: P0, fislocalanchor: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateEntry)(windows_core::Interface::as_raw(self), punk.param().abi(), fislocalanchor.into()) }
    }
    pub unsafe fn UpdateExternal<P0, P1>(&self, punk: P0, punkhlbrowsecontext: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateExternal)(windows_core::Interface::as_raw(self), punk.param().abi(), punkhlbrowsecontext.param().abi()) }
    }
    pub unsafe fn Travel<P0>(&self, punk: P0, ioffset: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Travel)(windows_core::Interface::as_raw(self), punk.param().abi(), ioffset) }
    }
    pub unsafe fn GetTravelEntry<P0>(&self, punk: P0, ioffset: i32) -> windows_core::Result<ITravelEntry>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTravelEntry)(windows_core::Interface::as_raw(self), punk.param().abi(), ioffset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn FindTravelEntry<P0>(&self, punk: P0, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<ITravelEntry>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindTravelEntry)(windows_core::Interface::as_raw(self), punk.param().abi(), pidl, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetToolTipText<P0>(&self, punk: P0, ioffset: i32, idstemplate: i32, pwztext: windows_core::PWSTR, cchtext: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetToolTipText)(windows_core::Interface::as_raw(self), punk.param().abi(), ioffset, idstemplate, core::mem::transmute(pwztext), cchtext) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn InsertMenuEntries<P0>(&self, punk: P0, hmenu: super::windef::HMENU, npos: i32, idfirst: i32, idlast: i32, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertMenuEntries)(windows_core::Interface::as_raw(self), punk.param().abi(), hmenu, npos, idfirst, idlast, dwflags) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CountEntries<P0>(&self, punk: P0) -> u32
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CountEntries)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn Revert(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITravelLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub UpdateEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub UpdateExternal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Travel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetTravelEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub FindTravelEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::shtypes::ITEMIDLIST, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    FindTravelEntry: usize,
    pub GetToolTipText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub InsertMenuEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::windef::HMENU, i32, i32, i32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    InsertMenuEntries: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CountEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> u32,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
pub trait ITravelLog_Impl: windows_core::IUnknownImpl {
    fn AddEntry(&self, punk: windows_core::Ref<windows_core::IUnknown>, fislocalanchor: windows_core::BOOL) -> windows_core::Result<()>;
    fn UpdateEntry(&self, punk: windows_core::Ref<windows_core::IUnknown>, fislocalanchor: windows_core::BOOL) -> windows_core::Result<()>;
    fn UpdateExternal(&self, punk: windows_core::Ref<windows_core::IUnknown>, punkhlbrowsecontext: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Travel(&self, punk: windows_core::Ref<windows_core::IUnknown>, ioffset: i32) -> windows_core::Result<()>;
    fn GetTravelEntry(&self, punk: windows_core::Ref<windows_core::IUnknown>, ioffset: i32) -> windows_core::Result<ITravelEntry>;
    fn FindTravelEntry(&self, punk: windows_core::Ref<windows_core::IUnknown>, pidl: *const super::shtypes::ITEMIDLIST) -> windows_core::Result<ITravelEntry>;
    fn GetToolTipText(&self, punk: windows_core::Ref<windows_core::IUnknown>, ioffset: i32, idstemplate: i32, pwztext: windows_core::PWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn InsertMenuEntries(&self, punk: windows_core::Ref<windows_core::IUnknown>, hmenu: super::windef::HMENU, npos: i32, idfirst: i32, idlast: i32, dwflags: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ITravelLog>;
    fn CountEntries(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> u32;
    fn Revert(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
impl ITravelLog_Vtbl {
    pub const fn new<Identity: ITravelLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddEntry<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, fislocalanchor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLog_Impl::AddEntry(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&fislocalanchor)).into()
            }
        }
        unsafe extern "system" fn UpdateEntry<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, fislocalanchor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLog_Impl::UpdateEntry(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&fislocalanchor)).into()
            }
        }
        unsafe extern "system" fn UpdateExternal<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, punkhlbrowsecontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLog_Impl::UpdateExternal(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&punkhlbrowsecontext)).into()
            }
        }
        unsafe extern "system" fn Travel<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, ioffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLog_Impl::Travel(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&ioffset)).into()
            }
        }
        unsafe extern "system" fn GetTravelEntry<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, ioffset: i32, ppte: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLog_Impl::GetTravelEntry(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&ioffset)) {
                    Ok(ok__) => {
                        ppte.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindTravelEntry<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, pidl: *const super::shtypes::ITEMIDLIST, ppte: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLog_Impl::FindTravelEntry(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&pidl)) {
                    Ok(ok__) => {
                        ppte.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetToolTipText<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, ioffset: i32, idstemplate: i32, pwztext: windows_core::PWSTR, cchtext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLog_Impl::GetToolTipText(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&ioffset), core::mem::transmute_copy(&idstemplate), core::mem::transmute_copy(&pwztext), core::mem::transmute_copy(&cchtext)).into()
            }
        }
        unsafe extern "system" fn InsertMenuEntries<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, hmenu: super::windef::HMENU, npos: i32, idfirst: i32, idlast: i32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLog_Impl::InsertMenuEntries(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&hmenu), core::mem::transmute_copy(&npos), core::mem::transmute_copy(&idfirst), core::mem::transmute_copy(&idlast), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITravelLog_Impl::Clone(this) {
                    Ok(ok__) => {
                        pptl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CountEntries<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLog_Impl::CountEntries(this, core::mem::transmute_copy(&punk))
            }
        }
        unsafe extern "system" fn Revert<Identity: ITravelLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITravelLog_Impl::Revert(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddEntry: AddEntry::<Identity, OFFSET>,
            UpdateEntry: UpdateEntry::<Identity, OFFSET>,
            UpdateExternal: UpdateExternal::<Identity, OFFSET>,
            Travel: Travel::<Identity, OFFSET>,
            GetTravelEntry: GetTravelEntry::<Identity, OFFSET>,
            FindTravelEntry: FindTravelEntry::<Identity, OFFSET>,
            GetToolTipText: GetToolTipText::<Identity, OFFSET>,
            InsertMenuEntries: InsertMenuEntries::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            CountEntries: CountEntries::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITravelLog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shtypes", feature = "windef"))]
impl windows_core::RuntimeName for ITravelLog {}
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPBASEBROWSERDATA = *mut BASEBROWSERDATA;
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPBASEBROWSERDATALH = *mut BASEBROWSERDATALH;
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPBASEBROWSERDATAXP = *mut BASEBROWSERDATAXP;
#[cfg(all(feature = "docobj", feature = "exdisp", feature = "hlink", feature = "oaidl", feature = "oleidl", feature = "shobjidl_core", feature = "shtypes", feature = "windef"))]
pub type LPCBASEBROWSERDATA = *const BASEBROWSERDATA;
#[cfg(feature = "shobjidl_core")]
pub type LPFOLDERSETDATA = *mut FOLDERSETDATA;
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
pub type LPTOOLBARITEM = *mut TOOLBARITEM;
pub const SBSC_HIDE: SHELLBROWSERSHOWCONTROL = 0;
pub const SBSC_QUERY: SHELLBROWSERSHOWCONTROL = 3;
pub const SBSC_SHOW: SHELLBROWSERSHOWCONTROL = 1;
pub const SBSC_TOGGLE: SHELLBROWSERSHOWCONTROL = 2;
pub type SECURELOCKCODE = i32;
pub const SECURELOCK_FIRSTSUGGEST: SECURELOCKCODE = 7;
pub const SECURELOCK_NOCHANGE: SECURELOCKCODE = -1;
pub const SECURELOCK_SET_FORTEZZA: SECURELOCKCODE = 5;
pub const SECURELOCK_SET_MIXED: SECURELOCKCODE = 1;
pub const SECURELOCK_SET_SECURE128BIT: SECURELOCKCODE = 6;
pub const SECURELOCK_SET_SECURE40BIT: SECURELOCKCODE = 3;
pub const SECURELOCK_SET_SECURE56BIT: SECURELOCKCODE = 4;
pub const SECURELOCK_SET_SECUREUNKNOWNBIT: SECURELOCKCODE = 2;
pub const SECURELOCK_SET_UNSECURE: SECURELOCKCODE = 0;
pub const SECURELOCK_SUGGEST_FORTEZZA: SECURELOCKCODE = 12;
pub const SECURELOCK_SUGGEST_MIXED: SECURELOCKCODE = 8;
pub const SECURELOCK_SUGGEST_SECURE128BIT: SECURELOCKCODE = 13;
pub const SECURELOCK_SUGGEST_SECURE40BIT: SECURELOCKCODE = 10;
pub const SECURELOCK_SUGGEST_SECURE56BIT: SECURELOCKCODE = 11;
pub const SECURELOCK_SUGGEST_SECUREUNKNOWNBIT: SECURELOCKCODE = 9;
pub const SECURELOCK_SUGGEST_UNSECURE: SECURELOCKCODE = 7;
pub type SHELLBROWSERSHOWCONTROL = i32;
pub const SHHLNF_NOAUTOSELECT: u32 = 67108864;
pub const SHHLNF_WRITENOHISTORY: u32 = 134217728;
pub const TLMENUF_BACK: u32 = 16;
pub const TLMENUF_BACKANDFORTH: u32 = 49;
pub const TLMENUF_CHECKCURRENT: u32 = 3;
pub const TLMENUF_FORE: u32 = 32;
pub const TLMENUF_INCLUDECURRENT: u32 = 1;
pub const TLOG_BACK: i32 = -1;
pub const TLOG_CURRENT: u32 = 0;
pub const TLOG_FORE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "oleidl", feature = "shobjidl_core", feature = "windef"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TOOLBARITEM {
    pub ptbar: core::mem::ManuallyDrop<Option<super::shobjidl_core::IDockingWindow>>,
    pub rcBorderTool: super::oleidl::BORDERWIDTHS,
    pub pwszItem: windows_core::PWSTR,
    pub fShow: windows_core::BOOL,
    pub hMon: super::windef::HMONITOR,
}
pub const TrackShellMenu: windows_core::GUID = windows_core::GUID::from_u128(0x8278f931_2a3e_11d2_838f_00c04fd918d0);
pub const VIEW_PRIORITY_CACHEHIT: u32 = 80;
pub const VIEW_PRIORITY_CACHEMISS: u32 = 48;
pub const VIEW_PRIORITY_DESPERATE: u32 = 16;
pub const VIEW_PRIORITY_INHERIT: u32 = 32;
pub const VIEW_PRIORITY_NONE: u32 = 0;
pub const VIEW_PRIORITY_RESTRICTED: u32 = 112;
pub const VIEW_PRIORITY_SHELLEXT: u32 = 64;
pub const VIEW_PRIORITY_SHELLEXT_ASBACKUP: u32 = 21;
pub const VIEW_PRIORITY_STALECACHEHIT: u32 = 69;
pub const VIEW_PRIORITY_USEASDEFAULT: u32 = 67;
