pub const CARET_CUSTOM: CARET_FLAGS = 1;
pub type CARET_FLAGS = i32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union CARET_INFO {
    pub hbitmap: super::HBITMAP,
    pub caretFlags: CARET_FLAGS,
}
#[cfg(feature = "windef")]
impl Default for CARET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARET_ITALIC: CARET_FLAGS = 32;
pub const CARET_NONE: CARET_FLAGS = 0;
pub const CARET_NULL: CARET_FLAGS = 64;
pub const CARET_ROTATE90: CARET_FLAGS = 128;
pub const CARET_RTL: CARET_FLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANGENOTIFY {
    pub dwChangeType: u32,
    pub pvCookieData: *mut core::ffi::c_void,
}
impl Default for CHANGENOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CHANGETYPE = i32;
pub const CN_GENERIC: CHANGETYPE = 0;
pub const CN_NEWREDO: CHANGETYPE = 4;
pub const CN_NEWUNDO: CHANGETYPE = 2;
pub const CN_TEXTCHANGED: CHANGETYPE = 1;
windows_core::imp::define_interface!(IRichEditUiaInformation, IRichEditUiaInformation_Vtbl, 0x23969a9d_8546_4032_a1bb_73750cbf3333);
windows_core::imp::interface_hierarchy!(IRichEditUiaInformation, windows_core::IUnknown);
impl IRichEditUiaInformation {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetBoundaryRectangle(&self, puiarect: *mut super::UiaRect) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBoundaryRectangle)(windows_core::Interface::as_raw(self), puiarect as _) }
    }
    pub unsafe fn IsVisible(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsVisible)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditUiaInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub GetBoundaryRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UiaRect) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetBoundaryRectangle: usize,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IRichEditUiaInformation_Impl: windows_core::IUnknownImpl {
    fn GetBoundaryRectangle(&self, puiarect: *mut super::UiaRect) -> windows_core::Result<()>;
    fn IsVisible(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IRichEditUiaInformation_Vtbl {
    pub const fn new<Identity: IRichEditUiaInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBoundaryRectangle<Identity: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiarect: *mut super::UiaRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditUiaInformation_Impl::GetBoundaryRectangle(this, core::mem::transmute_copy(&puiarect)).into()
            }
        }
        unsafe extern "system" fn IsVisible<Identity: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditUiaInformation_Impl::IsVisible(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBoundaryRectangle: GetBoundaryRectangle::<Identity, OFFSET>,
            IsVisible: IsVisible::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichEditUiaInformation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IRichEditUiaInformation {}
windows_core::imp::define_interface!(IRicheditUiaOverrides, IRicheditUiaOverrides_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IRicheditUiaOverrides, windows_core::IUnknown);
impl IRicheditUiaOverrides {
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPropertyOverrideValue(&self, propertyid: super::PROPERTYID) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyOverrideValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRicheditUiaOverrides_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPropertyOverrideValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    GetPropertyOverrideValue: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRicheditUiaOverrides_Impl: windows_core::IUnknownImpl {
    fn GetPropertyOverrideValue(&self, propertyid: super::PROPERTYID) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
impl IRicheditUiaOverrides_Vtbl {
    pub const fn new<Identity: IRicheditUiaOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyOverrideValue<Identity: IRicheditUiaOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID, pretvalue: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRicheditUiaOverrides_Impl::GetPropertyOverrideValue(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        pretvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPropertyOverrideValue: GetPropertyOverrideValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRicheditUiaOverrides as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRicheditUiaOverrides {}
windows_core::imp::define_interface!(IRicheditWindowlessAccessibility, IRicheditWindowlessAccessibility_Vtbl, 0x983e572d_20cd_460b_9104_83111592dd10);
windows_core::imp::interface_hierarchy!(IRicheditWindowlessAccessibility, windows_core::IUnknown);
impl IRicheditWindowlessAccessibility {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CreateProvider<P0>(&self, psite: P0) -> windows_core::Result<super::IRawElementProviderSimple>
    where
        P0: windows_core::Param<super::IRawElementProviderWindowlessSite>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProvider)(windows_core::Interface::as_raw(self), psite.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRicheditWindowlessAccessibility_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub CreateProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CreateProvider: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IRicheditWindowlessAccessibility_Impl: windows_core::IUnknownImpl {
    fn CreateProvider(&self, psite: windows_core::Ref<super::IRawElementProviderWindowlessSite>) -> windows_core::Result<super::IRawElementProviderSimple>;
}
#[cfg(feature = "uiautomationcore")]
impl IRicheditWindowlessAccessibility_Vtbl {
    pub const fn new<Identity: IRicheditWindowlessAccessibility_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateProvider<Identity: IRicheditWindowlessAccessibility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psite: *mut core::ffi::c_void, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRicheditWindowlessAccessibility_Impl::CreateProvider(this, core::mem::transmute_copy(&psite)) {
                    Ok(ok__) => {
                        ppprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateProvider: CreateProvider::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRicheditWindowlessAccessibility as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IRicheditWindowlessAccessibility {}
windows_core::imp::define_interface!(ITextHost, ITextHost_Vtbl, 0);
windows_core::imp::interface_hierarchy!(ITextHost, windows_core::IUnknown);
impl ITextHost {
    #[cfg(feature = "windef")]
    pub unsafe fn TxGetDC(&self) -> super::HDC {
        unsafe { (windows_core::Interface::vtable(self).TxGetDC)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxReleaseDC(&self, hdc: super::HDC) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).TxReleaseDC)(windows_core::Interface::as_raw(self), hdc) }
    }
    pub unsafe fn TxShowScrollBar(&self, fnbar: i32, fshow: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxShowScrollBar)(windows_core::Interface::as_raw(self), fnbar, fshow.into()) }
    }
    pub unsafe fn TxEnableScrollBar(&self, fusbflags: i32, fuarrowflags: i32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxEnableScrollBar)(windows_core::Interface::as_raw(self), fusbflags, fuarrowflags) }
    }
    pub unsafe fn TxSetScrollRange(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxSetScrollRange)(windows_core::Interface::as_raw(self), fnbar, nminpos, nmaxpos, fredraw.into()) }
    }
    pub unsafe fn TxSetScrollPos(&self, fnbar: i32, npos: i32, fredraw: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxSetScrollPos)(windows_core::Interface::as_raw(self), fnbar, npos, fredraw.into()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxInvalidateRect(&self, prc: *const super::RECT, fmode: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).TxInvalidateRect)(windows_core::Interface::as_raw(self), prc, fmode.into());
        }
    }
    pub unsafe fn TxViewChange(&self, fupdate: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).TxViewChange)(windows_core::Interface::as_raw(self), fupdate.into());
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxCreateCaret(&self, hbmp: super::HBITMAP, xwidth: i32, yheight: i32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxCreateCaret)(windows_core::Interface::as_raw(self), hbmp, xwidth, yheight) }
    }
    pub unsafe fn TxShowCaret(&self, fshow: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxShowCaret)(windows_core::Interface::as_raw(self), fshow.into()) }
    }
    pub unsafe fn TxSetCaretPos(&self, x: i32, y: i32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxSetCaretPos)(windows_core::Interface::as_raw(self), x, y) }
    }
    pub unsafe fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxSetTimer)(windows_core::Interface::as_raw(self), idtimer, utimeout) }
    }
    pub unsafe fn TxKillTimer(&self, idtimer: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).TxKillTimer)(windows_core::Interface::as_raw(self), idtimer);
        }
    }
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub unsafe fn TxScrollWindowEx(&self, dx: i32, dy: i32, lprcscroll: *const super::RECT, lprcclip: *const super::RECT, hrgnupdate: super::HRGN, lprcupdate: *mut super::RECT, fuscroll: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).TxScrollWindowEx)(windows_core::Interface::as_raw(self), dx, dy, lprcscroll, lprcclip, hrgnupdate, lprcupdate as _, fuscroll);
        }
    }
    pub unsafe fn TxSetCapture(&self, fcapture: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).TxSetCapture)(windows_core::Interface::as_raw(self), fcapture.into());
        }
    }
    pub unsafe fn TxSetFocus(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).TxSetFocus)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxSetCursor(&self, hcur: super::HCURSOR, ftext: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).TxSetCursor)(windows_core::Interface::as_raw(self), hcur, ftext.into());
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxScreenToClient(&self, lppt: *mut super::POINT) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxScreenToClient)(windows_core::Interface::as_raw(self), lppt as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxClientToScreen(&self, lppt: *mut super::POINT) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxClientToScreen)(windows_core::Interface::as_raw(self), lppt as _) }
    }
    pub unsafe fn TxActivate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxActivate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TxDeactivate(&self, lnewstate: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxDeactivate)(windows_core::Interface::as_raw(self), lnewstate) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxGetClientRect(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetClientRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxGetViewInset(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetViewInset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "richedit", feature = "windef"))]
    pub unsafe fn TxGetCharFormat(&self) -> windows_core::Result<*mut super::CHARFORMATW> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetCharFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "richedit")]
    pub unsafe fn TxGetParaFormat(&self) -> windows_core::Result<*mut super::PARAFORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetParaFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxGetSysColor(&self, nindex: i32) -> super::COLORREF {
        unsafe { (windows_core::Interface::vtable(self).TxGetSysColor)(windows_core::Interface::as_raw(self), nindex) }
    }
    pub unsafe fn TxGetBackStyle(&self) -> windows_core::Result<TXTBACKSTYLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetBackStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TxGetMaxLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetMaxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TxGetScrollBars(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetScrollBars)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn TxGetPasswordChar(&self) -> windows_core::Result<super::TCHAR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetPasswordChar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TxGetAcceleratorPos(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetAcceleratorPos)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxGetExtent(&self) -> windows_core::Result<super::SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetExtent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "richedit", feature = "windef"))]
    pub unsafe fn OnTxCharFormatChange(&self, pcf: *const super::CHARFORMATW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTxCharFormatChange)(windows_core::Interface::as_raw(self), pcf) }
    }
    #[cfg(feature = "richedit")]
    pub unsafe fn OnTxParaFormatChange(&self, ppf: *const super::PARAFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTxParaFormatChange)(windows_core::Interface::as_raw(self), ppf) }
    }
    pub unsafe fn TxGetPropertyBits(&self, dwmask: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetPropertyBits)(windows_core::Interface::as_raw(self), dwmask, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TxNotify(&self, inotify: u32, pv: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxNotify)(windows_core::Interface::as_raw(self), inotify, pv as _) }
    }
    #[cfg(feature = "imm")]
    pub unsafe fn TxImmGetContext(&self) -> super::HIMC {
        unsafe { (windows_core::Interface::vtable(self).TxImmGetContext)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "imm")]
    pub unsafe fn TxImmReleaseContext(&self, himc: super::HIMC) {
        unsafe {
            (windows_core::Interface::vtable(self).TxImmReleaseContext)(windows_core::Interface::as_raw(self), himc);
        }
    }
    pub unsafe fn TxGetSelectionBarWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetSelectionBarWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub TxGetDC: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HDC,
    #[cfg(not(feature = "windef"))]
    TxGetDC: usize,
    #[cfg(feature = "windef")]
    pub TxReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC) -> i32,
    #[cfg(not(feature = "windef"))]
    TxReleaseDC: usize,
    pub TxShowScrollBar: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::BOOL) -> windows_core::BOOL,
    pub TxEnableScrollBar: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::BOOL,
    pub TxSetScrollRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, windows_core::BOOL) -> windows_core::BOOL,
    pub TxSetScrollPos: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::BOOL) -> windows_core::BOOL,
    #[cfg(feature = "windef")]
    pub TxInvalidateRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT, windows_core::BOOL),
    #[cfg(not(feature = "windef"))]
    TxInvalidateRect: usize,
    pub TxViewChange: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
    #[cfg(feature = "windef")]
    pub TxCreateCaret: unsafe extern "system" fn(*mut core::ffi::c_void, super::HBITMAP, i32, i32) -> windows_core::BOOL,
    #[cfg(not(feature = "windef"))]
    TxCreateCaret: usize,
    pub TxShowCaret: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    pub TxSetCaretPos: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::BOOL,
    pub TxSetTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::BOOL,
    pub TxKillTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub TxScrollWindowEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const super::RECT, *const super::RECT, super::HRGN, *mut super::RECT, u32),
    #[cfg(not(all(feature = "minwindef", feature = "windef")))]
    TxScrollWindowEx: usize,
    pub TxSetCapture: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
    pub TxSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "windef")]
    pub TxSetCursor: unsafe extern "system" fn(*mut core::ffi::c_void, super::HCURSOR, windows_core::BOOL),
    #[cfg(not(feature = "windef"))]
    TxSetCursor: usize,
    #[cfg(feature = "windef")]
    pub TxScreenToClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::POINT) -> windows_core::BOOL,
    #[cfg(not(feature = "windef"))]
    TxScreenToClient: usize,
    #[cfg(feature = "windef")]
    pub TxClientToScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::POINT) -> windows_core::BOOL,
    #[cfg(not(feature = "windef"))]
    TxClientToScreen: usize,
    pub TxActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TxDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub TxGetClientRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    TxGetClientRect: usize,
    #[cfg(feature = "windef")]
    pub TxGetViewInset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    TxGetViewInset: usize,
    #[cfg(all(feature = "richedit", feature = "windef"))]
    pub TxGetCharFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::CHARFORMATW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "richedit", feature = "windef")))]
    TxGetCharFormat: usize,
    #[cfg(feature = "richedit")]
    pub TxGetParaFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::PARAFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "richedit"))]
    TxGetParaFormat: usize,
    #[cfg(feature = "windef")]
    pub TxGetSysColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> super::COLORREF,
    #[cfg(not(feature = "windef"))]
    TxGetSysColor: usize,
    pub TxGetBackStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TXTBACKSTYLE) -> windows_core::HRESULT,
    pub TxGetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TxGetScrollBars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub TxGetPasswordChar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::TCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    TxGetPasswordChar: usize,
    pub TxGetAcceleratorPos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub TxGetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    TxGetExtent: usize,
    #[cfg(all(feature = "richedit", feature = "windef"))]
    pub OnTxCharFormatChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::CHARFORMATW) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "richedit", feature = "windef")))]
    OnTxCharFormatChange: usize,
    #[cfg(feature = "richedit")]
    pub OnTxParaFormatChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PARAFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "richedit"))]
    OnTxParaFormatChange: usize,
    pub TxGetPropertyBits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub TxNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "imm")]
    pub TxImmGetContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HIMC,
    #[cfg(not(feature = "imm"))]
    TxImmGetContext: usize,
    #[cfg(feature = "imm")]
    pub TxImmReleaseContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::HIMC),
    #[cfg(not(feature = "imm"))]
    TxImmReleaseContext: usize,
    pub TxGetSelectionBarWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "imm", feature = "minwindef", feature = "richedit", feature = "windef", feature = "winnt"))]
pub trait ITextHost_Impl: windows_core::IUnknownImpl {
    fn TxGetDC(&self) -> super::HDC;
    fn TxReleaseDC(&self, hdc: super::HDC) -> i32;
    fn TxShowScrollBar(&self, fnbar: i32, fshow: windows_core::BOOL) -> windows_core::BOOL;
    fn TxEnableScrollBar(&self, fusbflags: i32, fuarrowflags: i32) -> windows_core::BOOL;
    fn TxSetScrollRange(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: windows_core::BOOL) -> windows_core::BOOL;
    fn TxSetScrollPos(&self, fnbar: i32, npos: i32, fredraw: windows_core::BOOL) -> windows_core::BOOL;
    fn TxInvalidateRect(&self, prc: *const super::RECT, fmode: windows_core::BOOL);
    fn TxViewChange(&self, fupdate: windows_core::BOOL);
    fn TxCreateCaret(&self, hbmp: super::HBITMAP, xwidth: i32, yheight: i32) -> windows_core::BOOL;
    fn TxShowCaret(&self, fshow: windows_core::BOOL) -> windows_core::BOOL;
    fn TxSetCaretPos(&self, x: i32, y: i32) -> windows_core::BOOL;
    fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> windows_core::BOOL;
    fn TxKillTimer(&self, idtimer: u32);
    fn TxScrollWindowEx(&self, dx: i32, dy: i32, lprcscroll: *const super::RECT, lprcclip: *const super::RECT, hrgnupdate: super::HRGN, lprcupdate: *mut super::RECT, fuscroll: u32);
    fn TxSetCapture(&self, fcapture: windows_core::BOOL);
    fn TxSetFocus(&self);
    fn TxSetCursor(&self, hcur: super::HCURSOR, ftext: windows_core::BOOL);
    fn TxScreenToClient(&self, lppt: *mut super::POINT) -> windows_core::BOOL;
    fn TxClientToScreen(&self, lppt: *mut super::POINT) -> windows_core::BOOL;
    fn TxActivate(&self) -> windows_core::Result<i32>;
    fn TxDeactivate(&self, lnewstate: i32) -> windows_core::Result<()>;
    fn TxGetClientRect(&self) -> windows_core::Result<super::RECT>;
    fn TxGetViewInset(&self) -> windows_core::Result<super::RECT>;
    fn TxGetCharFormat(&self) -> windows_core::Result<*mut super::CHARFORMATW>;
    fn TxGetParaFormat(&self) -> windows_core::Result<*mut super::PARAFORMAT>;
    fn TxGetSysColor(&self, nindex: i32) -> super::COLORREF;
    fn TxGetBackStyle(&self) -> windows_core::Result<TXTBACKSTYLE>;
    fn TxGetMaxLength(&self) -> windows_core::Result<u32>;
    fn TxGetScrollBars(&self) -> windows_core::Result<u32>;
    fn TxGetPasswordChar(&self) -> windows_core::Result<super::TCHAR>;
    fn TxGetAcceleratorPos(&self) -> windows_core::Result<i32>;
    fn TxGetExtent(&self) -> windows_core::Result<super::SIZE>;
    fn OnTxCharFormatChange(&self, pcf: *const super::CHARFORMATW) -> windows_core::Result<()>;
    fn OnTxParaFormatChange(&self, ppf: *const super::PARAFORMAT) -> windows_core::Result<()>;
    fn TxGetPropertyBits(&self, dwmask: u32) -> windows_core::Result<u32>;
    fn TxNotify(&self, inotify: u32, pv: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn TxImmGetContext(&self) -> super::HIMC;
    fn TxImmReleaseContext(&self, himc: super::HIMC);
    fn TxGetSelectionBarWidth(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "imm", feature = "minwindef", feature = "richedit", feature = "windef", feature = "winnt"))]
impl ITextHost_Vtbl {
    pub const fn new<Identity: ITextHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TxGetDC<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HDC {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetDC(this)
            }
        }
        unsafe extern "system" fn TxReleaseDC<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::HDC) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxReleaseDC(this, core::mem::transmute_copy(&hdc))
            }
        }
        unsafe extern "system" fn TxShowScrollBar<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnbar: i32, fshow: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxShowScrollBar(this, core::mem::transmute_copy(&fnbar), core::mem::transmute_copy(&fshow))
            }
        }
        unsafe extern "system" fn TxEnableScrollBar<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fusbflags: i32, fuarrowflags: i32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxEnableScrollBar(this, core::mem::transmute_copy(&fusbflags), core::mem::transmute_copy(&fuarrowflags))
            }
        }
        unsafe extern "system" fn TxSetScrollRange<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetScrollRange(this, core::mem::transmute_copy(&fnbar), core::mem::transmute_copy(&nminpos), core::mem::transmute_copy(&nmaxpos), core::mem::transmute_copy(&fredraw))
            }
        }
        unsafe extern "system" fn TxSetScrollPos<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnbar: i32, npos: i32, fredraw: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetScrollPos(this, core::mem::transmute_copy(&fnbar), core::mem::transmute_copy(&npos), core::mem::transmute_copy(&fredraw))
            }
        }
        unsafe extern "system" fn TxInvalidateRect<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *const super::RECT, fmode: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxInvalidateRect(this, core::mem::transmute_copy(&prc), core::mem::transmute_copy(&fmode));
            }
        }
        unsafe extern "system" fn TxViewChange<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fupdate: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxViewChange(this, core::mem::transmute_copy(&fupdate));
            }
        }
        unsafe extern "system" fn TxCreateCaret<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmp: super::HBITMAP, xwidth: i32, yheight: i32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxCreateCaret(this, core::mem::transmute_copy(&hbmp), core::mem::transmute_copy(&xwidth), core::mem::transmute_copy(&yheight))
            }
        }
        unsafe extern "system" fn TxShowCaret<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxShowCaret(this, core::mem::transmute_copy(&fshow))
            }
        }
        unsafe extern "system" fn TxSetCaretPos<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetCaretPos(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y))
            }
        }
        unsafe extern "system" fn TxSetTimer<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idtimer: u32, utimeout: u32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetTimer(this, core::mem::transmute_copy(&idtimer), core::mem::transmute_copy(&utimeout))
            }
        }
        unsafe extern "system" fn TxKillTimer<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idtimer: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxKillTimer(this, core::mem::transmute_copy(&idtimer));
            }
        }
        unsafe extern "system" fn TxScrollWindowEx<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *const super::RECT, lprcclip: *const super::RECT, hrgnupdate: super::HRGN, lprcupdate: *mut super::RECT, fuscroll: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxScrollWindowEx(this, core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy), core::mem::transmute_copy(&lprcscroll), core::mem::transmute_copy(&lprcclip), core::mem::transmute_copy(&hrgnupdate), core::mem::transmute_copy(&lprcupdate), core::mem::transmute_copy(&fuscroll));
            }
        }
        unsafe extern "system" fn TxSetCapture<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcapture: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetCapture(this, core::mem::transmute_copy(&fcapture));
            }
        }
        unsafe extern "system" fn TxSetFocus<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetFocus(this);
            }
        }
        unsafe extern "system" fn TxSetCursor<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcur: super::HCURSOR, ftext: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxSetCursor(this, core::mem::transmute_copy(&hcur), core::mem::transmute_copy(&ftext));
            }
        }
        unsafe extern "system" fn TxScreenToClient<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppt: *mut super::POINT) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxScreenToClient(this, core::mem::transmute_copy(&lppt))
            }
        }
        unsafe extern "system" fn TxClientToScreen<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lppt: *mut super::POINT) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxClientToScreen(this, core::mem::transmute_copy(&lppt))
            }
        }
        unsafe extern "system" fn TxActivate<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploldstate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxActivate(this) {
                    Ok(ok__) => {
                        ploldstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxDeactivate<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnewstate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxDeactivate(this, core::mem::transmute_copy(&lnewstate)).into()
            }
        }
        unsafe extern "system" fn TxGetClientRect<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetClientRect(this) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetViewInset<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetViewInset(this) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetCharFormat<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcf: *mut *mut super::CHARFORMATW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetCharFormat(this) {
                    Ok(ok__) => {
                        ppcf.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetParaFormat<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppf: *mut *mut super::PARAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetParaFormat(this) {
                    Ok(ok__) => {
                        pppf.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetSysColor<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32) -> super::COLORREF {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxGetSysColor(this, core::mem::transmute_copy(&nindex))
            }
        }
        unsafe extern "system" fn TxGetBackStyle<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetBackStyle(this) {
                    Ok(ok__) => {
                        pstyle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetMaxLength<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetMaxLength(this) {
                    Ok(ok__) => {
                        plength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetScrollBars<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscrollbar: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetScrollBars(this) {
                    Ok(ok__) => {
                        pdwscrollbar.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetPasswordChar<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pch: *mut super::TCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetPasswordChar(this) {
                    Ok(ok__) => {
                        pch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetAcceleratorPos<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcp: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetAcceleratorPos(this) {
                    Ok(ok__) => {
                        pcp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetExtent<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpextent: *mut super::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetExtent(this) {
                    Ok(ok__) => {
                        lpextent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnTxCharFormatChange<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *const super::CHARFORMATW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::OnTxCharFormatChange(this, core::mem::transmute_copy(&pcf)).into()
            }
        }
        unsafe extern "system" fn OnTxParaFormatChange<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppf: *const super::PARAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::OnTxParaFormatChange(this, core::mem::transmute_copy(&ppf)).into()
            }
        }
        unsafe extern "system" fn TxGetPropertyBits<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetPropertyBits(this, core::mem::transmute_copy(&dwmask)) {
                    Ok(ok__) => {
                        pdwbits.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxNotify<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inotify: u32, pv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxNotify(this, core::mem::transmute_copy(&inotify), core::mem::transmute_copy(&pv)).into()
            }
        }
        unsafe extern "system" fn TxImmGetContext<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HIMC {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxImmGetContext(this)
            }
        }
        unsafe extern "system" fn TxImmReleaseContext<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: super::HIMC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost_Impl::TxImmReleaseContext(this, core::mem::transmute_copy(&himc));
            }
        }
        unsafe extern "system" fn TxGetSelectionBarWidth<Identity: ITextHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lselbarwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost_Impl::TxGetSelectionBarWidth(this) {
                    Ok(ok__) => {
                        lselbarwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TxGetDC: TxGetDC::<Identity, OFFSET>,
            TxReleaseDC: TxReleaseDC::<Identity, OFFSET>,
            TxShowScrollBar: TxShowScrollBar::<Identity, OFFSET>,
            TxEnableScrollBar: TxEnableScrollBar::<Identity, OFFSET>,
            TxSetScrollRange: TxSetScrollRange::<Identity, OFFSET>,
            TxSetScrollPos: TxSetScrollPos::<Identity, OFFSET>,
            TxInvalidateRect: TxInvalidateRect::<Identity, OFFSET>,
            TxViewChange: TxViewChange::<Identity, OFFSET>,
            TxCreateCaret: TxCreateCaret::<Identity, OFFSET>,
            TxShowCaret: TxShowCaret::<Identity, OFFSET>,
            TxSetCaretPos: TxSetCaretPos::<Identity, OFFSET>,
            TxSetTimer: TxSetTimer::<Identity, OFFSET>,
            TxKillTimer: TxKillTimer::<Identity, OFFSET>,
            TxScrollWindowEx: TxScrollWindowEx::<Identity, OFFSET>,
            TxSetCapture: TxSetCapture::<Identity, OFFSET>,
            TxSetFocus: TxSetFocus::<Identity, OFFSET>,
            TxSetCursor: TxSetCursor::<Identity, OFFSET>,
            TxScreenToClient: TxScreenToClient::<Identity, OFFSET>,
            TxClientToScreen: TxClientToScreen::<Identity, OFFSET>,
            TxActivate: TxActivate::<Identity, OFFSET>,
            TxDeactivate: TxDeactivate::<Identity, OFFSET>,
            TxGetClientRect: TxGetClientRect::<Identity, OFFSET>,
            TxGetViewInset: TxGetViewInset::<Identity, OFFSET>,
            TxGetCharFormat: TxGetCharFormat::<Identity, OFFSET>,
            TxGetParaFormat: TxGetParaFormat::<Identity, OFFSET>,
            TxGetSysColor: TxGetSysColor::<Identity, OFFSET>,
            TxGetBackStyle: TxGetBackStyle::<Identity, OFFSET>,
            TxGetMaxLength: TxGetMaxLength::<Identity, OFFSET>,
            TxGetScrollBars: TxGetScrollBars::<Identity, OFFSET>,
            TxGetPasswordChar: TxGetPasswordChar::<Identity, OFFSET>,
            TxGetAcceleratorPos: TxGetAcceleratorPos::<Identity, OFFSET>,
            TxGetExtent: TxGetExtent::<Identity, OFFSET>,
            OnTxCharFormatChange: OnTxCharFormatChange::<Identity, OFFSET>,
            OnTxParaFormatChange: OnTxParaFormatChange::<Identity, OFFSET>,
            TxGetPropertyBits: TxGetPropertyBits::<Identity, OFFSET>,
            TxNotify: TxNotify::<Identity, OFFSET>,
            TxImmGetContext: TxImmGetContext::<Identity, OFFSET>,
            TxImmReleaseContext: TxImmReleaseContext::<Identity, OFFSET>,
            TxGetSelectionBarWidth: TxGetSelectionBarWidth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "imm", feature = "minwindef", feature = "richedit", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ITextHost {}
windows_core::imp::define_interface!(ITextHost2, ITextHost2_Vtbl, 0);
impl core::ops::Deref for ITextHost2 {
    type Target = ITextHost;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextHost2, windows_core::IUnknown, ITextHost);
impl ITextHost2 {
    pub unsafe fn TxIsDoubleClickPending(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).TxIsDoubleClickPending)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxGetWindow(&self) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TxSetForegroundWindow(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxSetForegroundWindow)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxGetPalette(&self) -> super::HPALETTE {
        unsafe { (windows_core::Interface::vtable(self).TxGetPalette)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TxGetEastAsianFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetEastAsianFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxSetCursor2(&self, hcur: super::HCURSOR, btext: bool) -> super::HCURSOR {
        unsafe { (windows_core::Interface::vtable(self).TxSetCursor2)(windows_core::Interface::as_raw(self), hcur, btext.into()) }
    }
    pub unsafe fn TxFreeTextServicesNotification(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).TxFreeTextServicesNotification)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn TxGetEditStyle(&self, dwitem: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetEditStyle)(windows_core::Interface::as_raw(self), dwitem, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TxGetWindowStyles(&self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxGetWindowStyles)(windows_core::Interface::as_raw(self), pdwstyle as _, pdwexstyle as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn TxShowDropCaret(&self, fshow: bool, hdc: super::HDC, prc: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxShowDropCaret)(windows_core::Interface::as_raw(self), fshow.into(), hdc, prc) }
    }
    pub unsafe fn TxDestroyCaret(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxDestroyCaret)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TxGetHorzExtent(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetHorzExtent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHost2_Vtbl {
    pub base__: ITextHost_Vtbl,
    pub TxIsDoubleClickPending: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(feature = "windef")]
    pub TxGetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    TxGetWindow: usize,
    pub TxSetForegroundWindow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub TxGetPalette: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::HPALETTE,
    #[cfg(not(feature = "windef"))]
    TxGetPalette: usize,
    pub TxGetEastAsianFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub TxSetCursor2: unsafe extern "system" fn(*mut core::ffi::c_void, super::HCURSOR, windows_core::BOOL) -> super::HCURSOR,
    #[cfg(not(feature = "windef"))]
    TxSetCursor2: usize,
    pub TxFreeTextServicesNotification: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub TxGetEditStyle: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub TxGetWindowStyles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub TxShowDropCaret: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::HDC, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    TxShowDropCaret: usize,
    pub TxDestroyCaret: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TxGetHorzExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "imm", feature = "minwindef", feature = "richedit", feature = "windef", feature = "winnt"))]
pub trait ITextHost2_Impl: ITextHost_Impl {
    fn TxIsDoubleClickPending(&self) -> windows_core::BOOL;
    fn TxGetWindow(&self) -> windows_core::Result<super::HWND>;
    fn TxSetForegroundWindow(&self) -> windows_core::Result<()>;
    fn TxGetPalette(&self) -> super::HPALETTE;
    fn TxGetEastAsianFlags(&self) -> windows_core::Result<i32>;
    fn TxSetCursor2(&self, hcur: super::HCURSOR, btext: windows_core::BOOL) -> super::HCURSOR;
    fn TxFreeTextServicesNotification(&self);
    fn TxGetEditStyle(&self, dwitem: u32) -> windows_core::Result<u32>;
    fn TxGetWindowStyles(&self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> windows_core::Result<()>;
    fn TxShowDropCaret(&self, fshow: windows_core::BOOL, hdc: super::HDC, prc: *const super::RECT) -> windows_core::Result<()>;
    fn TxDestroyCaret(&self) -> windows_core::Result<()>;
    fn TxGetHorzExtent(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "imm", feature = "minwindef", feature = "richedit", feature = "windef", feature = "winnt"))]
impl ITextHost2_Vtbl {
    pub const fn new<Identity: ITextHost2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TxIsDoubleClickPending<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxIsDoubleClickPending(this)
            }
        }
        unsafe extern "system" fn TxGetWindow<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost2_Impl::TxGetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxSetForegroundWindow<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxSetForegroundWindow(this).into()
            }
        }
        unsafe extern "system" fn TxGetPalette<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HPALETTE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxGetPalette(this)
            }
        }
        unsafe extern "system" fn TxGetEastAsianFlags<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost2_Impl::TxGetEastAsianFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxSetCursor2<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcur: super::HCURSOR, btext: windows_core::BOOL) -> super::HCURSOR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxSetCursor2(this, core::mem::transmute_copy(&hcur), core::mem::transmute_copy(&btext))
            }
        }
        unsafe extern "system" fn TxFreeTextServicesNotification<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxFreeTextServicesNotification(this);
            }
        }
        unsafe extern "system" fn TxGetEditStyle<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwitem: u32, pdwdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost2_Impl::TxGetEditStyle(this, core::mem::transmute_copy(&dwitem)) {
                    Ok(ok__) => {
                        pdwdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetWindowStyles<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxGetWindowStyles(this, core::mem::transmute_copy(&pdwstyle), core::mem::transmute_copy(&pdwexstyle)).into()
            }
        }
        unsafe extern "system" fn TxShowDropCaret<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL, hdc: super::HDC, prc: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxShowDropCaret(this, core::mem::transmute_copy(&fshow), core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn TxDestroyCaret<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHost2_Impl::TxDestroyCaret(this).into()
            }
        }
        unsafe extern "system" fn TxGetHorzExtent<Identity: ITextHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhorzextent: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHost2_Impl::TxGetHorzExtent(this) {
                    Ok(ok__) => {
                        plhorzextent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITextHost_Vtbl::new::<Identity, OFFSET>(),
            TxIsDoubleClickPending: TxIsDoubleClickPending::<Identity, OFFSET>,
            TxGetWindow: TxGetWindow::<Identity, OFFSET>,
            TxSetForegroundWindow: TxSetForegroundWindow::<Identity, OFFSET>,
            TxGetPalette: TxGetPalette::<Identity, OFFSET>,
            TxGetEastAsianFlags: TxGetEastAsianFlags::<Identity, OFFSET>,
            TxSetCursor2: TxSetCursor2::<Identity, OFFSET>,
            TxFreeTextServicesNotification: TxFreeTextServicesNotification::<Identity, OFFSET>,
            TxGetEditStyle: TxGetEditStyle::<Identity, OFFSET>,
            TxGetWindowStyles: TxGetWindowStyles::<Identity, OFFSET>,
            TxShowDropCaret: TxShowDropCaret::<Identity, OFFSET>,
            TxDestroyCaret: TxDestroyCaret::<Identity, OFFSET>,
            TxGetHorzExtent: TxGetHorzExtent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextHost2 as windows_core::Interface>::IID || iid == &<ITextHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "imm", feature = "minwindef", feature = "richedit", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ITextHost2 {}
windows_core::imp::define_interface!(ITextServices, ITextServices_Vtbl, 0);
windows_core::imp::interface_hierarchy!(ITextServices, windows_core::IUnknown);
impl ITextServices {
    #[cfg(feature = "minwindef")]
    pub unsafe fn TxSendMessage(&self, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::Result<super::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxSendMessage)(windows_core::Interface::as_raw(self), msg, wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn TxDraw(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::DVTARGETDEVICE, hdcdraw: super::HDC, hictargetdev: super::HDC, lprcbounds: *const super::RECTL, lprcwbounds: *const super::RECTL, lprcupdate: *mut super::RECT, pfncontinue: *mut u8, dwcontinue: u32, lviewid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxDraw)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect as _, ptd as _, hdcdraw, hictargetdev, lprcbounds, lprcwbounds, lprcupdate as _, pfncontinue as _, dwcontinue, lviewid) }
    }
    pub unsafe fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxGetHScroll)(windows_core::Interface::as_raw(self), plmin as _, plmax as _, plpos as _, plpage as _, pfenabled as _) }
    }
    pub unsafe fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxGetVScroll)(windows_core::Interface::as_raw(self), plmin as _, plmax as _, plpos as _, plpage as _, pfenabled as _) }
    }
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn OnTxSetCursor(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::DVTARGETDEVICE, hdcdraw: super::HDC, hictargetdev: super::HDC, lprcclient: *const super::RECT, x: i32, y: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTxSetCursor)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect as _, ptd as _, hdcdraw, hictargetdev, lprcclient, x, y) }
    }
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn TxQueryHitPoint(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::DVTARGETDEVICE, hdcdraw: super::HDC, hictargetdev: super::HDC, lprcclient: *const super::RECT, x: i32, y: i32, phitresult: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxQueryHitPoint)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect as _, ptd as _, hdcdraw, hictargetdev, lprcclient, x, y, phitresult as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn OnTxInPlaceActivate(&self, prcclient: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTxInPlaceActivate)(windows_core::Interface::as_raw(self), prcclient) }
    }
    pub unsafe fn OnTxInPlaceDeactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTxInPlaceDeactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnTxUIActivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTxUIActivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnTxUIDeactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTxUIDeactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TxGetText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TxSetText<P0>(&self, psztext: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).TxSetText)(windows_core::Interface::as_raw(self), psztext.param().abi()) }
    }
    pub unsafe fn TxGetCurTargetX(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetCurTargetX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TxGetBaseLinePos(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetBaseLinePos)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn TxGetNaturalSize(&self, dwaspect: u32, hdcdraw: super::HDC, hictargetdev: super::HDC, ptd: *mut super::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::SIZEL, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxGetNaturalSize)(windows_core::Interface::as_raw(self), dwaspect, hdcdraw, hictargetdev, ptd as _, dwmode, psizelextent, pwidth as _, pheight as _) }
    }
    #[cfg(feature = "oleidl")]
    pub unsafe fn TxGetDropTarget(&self) -> windows_core::Result<super::IDropTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TxGetDropTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTxPropertyBitsChange)(windows_core::Interface::as_raw(self), dwmask, dwbits) }
    }
    pub unsafe fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxGetCachedSize)(windows_core::Interface::as_raw(self), pdwwidth as _, pdwheight as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub TxSendMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::WPARAM, super::LPARAM, *mut super::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    TxSendMessage: usize,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub TxDraw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *mut core::ffi::c_void, *mut super::DVTARGETDEVICE, super::HDC, super::HDC, *const super::RECTL, *const super::RECTL, *mut super::RECT, *mut u8, u32, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    TxDraw: usize,
    pub TxGetHScroll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub TxGetVScroll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub OnTxSetCursor: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *mut core::ffi::c_void, *mut super::DVTARGETDEVICE, super::HDC, super::HDC, *const super::RECT, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    OnTxSetCursor: usize,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub TxQueryHitPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *mut core::ffi::c_void, *mut super::DVTARGETDEVICE, super::HDC, super::HDC, *const super::RECT, i32, i32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    TxQueryHitPoint: usize,
    #[cfg(feature = "windef")]
    pub OnTxInPlaceActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    OnTxInPlaceActivate: usize,
    pub OnTxInPlaceDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnTxUIActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnTxUIDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TxGetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TxSetText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub TxGetCurTargetX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TxGetBaseLinePos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub TxGetNaturalSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::HDC, super::HDC, *mut super::DVTARGETDEVICE, u32, *const super::SIZEL, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    TxGetNaturalSize: usize,
    #[cfg(feature = "oleidl")]
    pub TxGetDropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oleidl"))]
    TxGetDropTarget: usize,
    pub OnTxPropertyBitsChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub TxGetCachedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "windef"))]
pub trait ITextServices_Impl: windows_core::IUnknownImpl {
    fn TxSendMessage(&self, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::Result<super::LRESULT>;
    fn TxDraw(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::DVTARGETDEVICE, hdcdraw: super::HDC, hictargetdev: super::HDC, lprcbounds: *const super::RECTL, lprcwbounds: *const super::RECTL, lprcupdate: *mut super::RECT, pfncontinue: *mut u8, dwcontinue: u32, lviewid: i32) -> windows_core::Result<()>;
    fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn OnTxSetCursor(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::DVTARGETDEVICE, hdcdraw: super::HDC, hictargetdev: super::HDC, lprcclient: *const super::RECT, x: i32, y: i32) -> windows_core::Result<()>;
    fn TxQueryHitPoint(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::DVTARGETDEVICE, hdcdraw: super::HDC, hictargetdev: super::HDC, lprcclient: *const super::RECT, x: i32, y: i32, phitresult: *mut u32) -> windows_core::Result<()>;
    fn OnTxInPlaceActivate(&self, prcclient: *const super::RECT) -> windows_core::Result<()>;
    fn OnTxInPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn OnTxUIActivate(&self) -> windows_core::Result<()>;
    fn OnTxUIDeactivate(&self) -> windows_core::Result<()>;
    fn TxGetText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TxSetText(&self, psztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn TxGetCurTargetX(&self) -> windows_core::Result<i32>;
    fn TxGetBaseLinePos(&self) -> windows_core::Result<i32>;
    fn TxGetNaturalSize(&self, dwaspect: u32, hdcdraw: super::HDC, hictargetdev: super::HDC, ptd: *mut super::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::SIZEL, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn TxGetDropTarget(&self) -> windows_core::Result<super::IDropTarget>;
    fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> windows_core::Result<()>;
    fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "windef"))]
impl ITextServices_Vtbl {
    pub const fn new<Identity: ITextServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TxSendMessage<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::WPARAM, lparam: super::LPARAM, plresult: *mut super::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextServices_Impl::TxSendMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        plresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxDraw<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::DVTARGETDEVICE, hdcdraw: super::HDC, hictargetdev: super::HDC, lprcbounds: *const super::RECTL, lprcwbounds: *const super::RECTL, lprcupdate: *mut super::RECT, pfncontinue: *mut u8, dwcontinue: u32, lviewid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxDraw(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&lprcbounds), core::mem::transmute_copy(&lprcwbounds), core::mem::transmute_copy(&lprcupdate), core::mem::transmute_copy(&pfncontinue), core::mem::transmute_copy(&dwcontinue), core::mem::transmute_copy(&lviewid)).into()
            }
        }
        unsafe extern "system" fn TxGetHScroll<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetHScroll(this, core::mem::transmute_copy(&plmin), core::mem::transmute_copy(&plmax), core::mem::transmute_copy(&plpos), core::mem::transmute_copy(&plpage), core::mem::transmute_copy(&pfenabled)).into()
            }
        }
        unsafe extern "system" fn TxGetVScroll<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetVScroll(this, core::mem::transmute_copy(&plmin), core::mem::transmute_copy(&plmax), core::mem::transmute_copy(&plpos), core::mem::transmute_copy(&plpage), core::mem::transmute_copy(&pfenabled)).into()
            }
        }
        unsafe extern "system" fn OnTxSetCursor<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::DVTARGETDEVICE, hdcdraw: super::HDC, hictargetdev: super::HDC, lprcclient: *const super::RECT, x: i32, y: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxSetCursor(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&lprcclient), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn TxQueryHitPoint<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *mut super::DVTARGETDEVICE, hdcdraw: super::HDC, hictargetdev: super::HDC, lprcclient: *const super::RECT, x: i32, y: i32, phitresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxQueryHitPoint(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&lprcclient), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&phitresult)).into()
            }
        }
        unsafe extern "system" fn OnTxInPlaceActivate<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcclient: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxInPlaceActivate(this, core::mem::transmute_copy(&prcclient)).into()
            }
        }
        unsafe extern "system" fn OnTxInPlaceDeactivate<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxInPlaceDeactivate(this).into()
            }
        }
        unsafe extern "system" fn OnTxUIActivate<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxUIActivate(this).into()
            }
        }
        unsafe extern "system" fn OnTxUIDeactivate<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxUIDeactivate(this).into()
            }
        }
        unsafe extern "system" fn TxGetText<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextServices_Impl::TxGetText(this) {
                    Ok(ok__) => {
                        pbstrtext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxSetText<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxSetText(this, core::mem::transmute(&psztext)).into()
            }
        }
        unsafe extern "system" fn TxGetCurTargetX<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextServices_Impl::TxGetCurTargetX(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetBaseLinePos<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextServices_Impl::TxGetBaseLinePos(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TxGetNaturalSize<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, hdcdraw: super::HDC, hictargetdev: super::HDC, ptd: *mut super::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::SIZEL, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetNaturalSize(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&psizelextent), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn TxGetDropTarget<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdroptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextServices_Impl::TxGetDropTarget(this) {
                    Ok(ok__) => {
                        ppdroptarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnTxPropertyBitsChange<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32, dwbits: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::OnTxPropertyBitsChange(this, core::mem::transmute_copy(&dwmask), core::mem::transmute_copy(&dwbits)).into()
            }
        }
        unsafe extern "system" fn TxGetCachedSize<Identity: ITextServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices_Impl::TxGetCachedSize(this, core::mem::transmute_copy(&pdwwidth), core::mem::transmute_copy(&pdwheight)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TxSendMessage: TxSendMessage::<Identity, OFFSET>,
            TxDraw: TxDraw::<Identity, OFFSET>,
            TxGetHScroll: TxGetHScroll::<Identity, OFFSET>,
            TxGetVScroll: TxGetVScroll::<Identity, OFFSET>,
            OnTxSetCursor: OnTxSetCursor::<Identity, OFFSET>,
            TxQueryHitPoint: TxQueryHitPoint::<Identity, OFFSET>,
            OnTxInPlaceActivate: OnTxInPlaceActivate::<Identity, OFFSET>,
            OnTxInPlaceDeactivate: OnTxInPlaceDeactivate::<Identity, OFFSET>,
            OnTxUIActivate: OnTxUIActivate::<Identity, OFFSET>,
            OnTxUIDeactivate: OnTxUIDeactivate::<Identity, OFFSET>,
            TxGetText: TxGetText::<Identity, OFFSET>,
            TxSetText: TxSetText::<Identity, OFFSET>,
            TxGetCurTargetX: TxGetCurTargetX::<Identity, OFFSET>,
            TxGetBaseLinePos: TxGetBaseLinePos::<Identity, OFFSET>,
            TxGetNaturalSize: TxGetNaturalSize::<Identity, OFFSET>,
            TxGetDropTarget: TxGetDropTarget::<Identity, OFFSET>,
            OnTxPropertyBitsChange: OnTxPropertyBitsChange::<Identity, OFFSET>,
            TxGetCachedSize: TxGetCachedSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "windef"))]
impl windows_core::RuntimeName for ITextServices {}
windows_core::imp::define_interface!(ITextServices2, ITextServices2_Vtbl, 0);
impl core::ops::Deref for ITextServices2 {
    type Target = ITextServices;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextServices2, windows_core::IUnknown, ITextServices);
impl ITextServices2 {
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn TxGetNaturalSize2(&self, dwaspect: u32, hdcdraw: super::HDC, hictargetdev: super::HDC, ptd: *mut super::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::SIZEL, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TxGetNaturalSize2)(windows_core::Interface::as_raw(self), dwaspect, hdcdraw, hictargetdev, ptd as _, dwmode, psizelextent, pwidth as _, pheight as _, pascent as _) }
    }
    #[cfg(all(feature = "d2d", feature = "windef"))]
    pub unsafe fn TxDrawD2D<P0>(&self, prendertarget: P0, lprcbounds: *const super::RECTL, lprcupdate: *mut super::RECT, lviewid: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ID2D1RenderTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).TxDrawD2D)(windows_core::Interface::as_raw(self), prendertarget.param().abi(), lprcbounds, lprcupdate as _, lviewid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServices2_Vtbl {
    pub base__: ITextServices_Vtbl,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub TxGetNaturalSize2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::HDC, super::HDC, *mut super::DVTARGETDEVICE, u32, *const super::SIZEL, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    TxGetNaturalSize2: usize,
    #[cfg(all(feature = "d2d", feature = "windef"))]
    pub TxDrawD2D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::RECTL, *mut super::RECT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "d2d", feature = "windef")))]
    TxDrawD2D: usize,
}
#[cfg(all(feature = "d2d", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "windef"))]
pub trait ITextServices2_Impl: ITextServices_Impl {
    fn TxGetNaturalSize2(&self, dwaspect: u32, hdcdraw: super::HDC, hictargetdev: super::HDC, ptd: *mut super::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::SIZEL, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> windows_core::Result<()>;
    fn TxDrawD2D(&self, prendertarget: windows_core::Ref<super::ID2D1RenderTarget>, lprcbounds: *const super::RECTL, lprcupdate: *mut super::RECT, lviewid: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "d2d", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "windef"))]
impl ITextServices2_Vtbl {
    pub const fn new<Identity: ITextServices2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TxGetNaturalSize2<Identity: ITextServices2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, hdcdraw: super::HDC, hictargetdev: super::HDC, ptd: *mut super::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::SIZEL, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices2_Impl::TxGetNaturalSize2(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&psizelextent), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight), core::mem::transmute_copy(&pascent)).into()
            }
        }
        unsafe extern "system" fn TxDrawD2D<Identity: ITextServices2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, lprcbounds: *const super::RECTL, lprcupdate: *mut super::RECT, lviewid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextServices2_Impl::TxDrawD2D(this, core::mem::transmute_copy(&prendertarget), core::mem::transmute_copy(&lprcbounds), core::mem::transmute_copy(&lprcupdate), core::mem::transmute_copy(&lviewid)).into()
            }
        }
        Self {
            base__: ITextServices_Vtbl::new::<Identity, OFFSET>(),
            TxGetNaturalSize2: TxGetNaturalSize2::<Identity, OFFSET>,
            TxDrawD2D: TxDrawD2D::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextServices2 as windows_core::Interface>::IID || iid == &<ITextServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "d2d", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "windef"))]
impl windows_core::RuntimeName for ITextServices2 {}
pub type PCreateTextServices = Option<unsafe extern "system" fn(punkouter: windows_core::Ref<windows_core::IUnknown>, pitexthost: windows_core::Ref<ITextHost>, ppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::HRESULT>;
pub type PShutdownTextServices = Option<unsafe extern "system" fn(ptextservices: windows_core::Ref<windows_core::IUnknown>) -> windows_core::HRESULT>;
pub const S_MSG_KEY_IGNORED: u32 = 262657;
pub const TXES_ISDIALOG: u32 = 1;
pub type TXTBACKSTYLE = i32;
pub const TXTBACK_OPAQUE: TXTBACKSTYLE = 1;
pub const TXTBACK_TRANSPARENT: TXTBACKSTYLE = 0;
pub const TXTBIT_ADVANCEDINPUT: u32 = 536870912;
pub const TXTBIT_ALLOWBEEP: u32 = 2048;
pub const TXTBIT_AUTOWORDSEL: u32 = 128;
pub const TXTBIT_BACKSTYLECHANGE: u32 = 16384;
pub const TXTBIT_CHARFORMATCHANGE: u32 = 131072;
pub const TXTBIT_CLIENTRECTCHANGE: u32 = 1048576;
pub const TXTBIT_D2DDWRITE: u32 = 16777216;
pub const TXTBIT_D2DPIXELSNAPPED: u32 = 67108864;
pub const TXTBIT_D2DSIMPLETYPOGRAPHY: u32 = 33554432;
pub const TXTBIT_D2DSUBPIXELLINES: u32 = 134217728;
pub const TXTBIT_DISABLEDRAG: u32 = 4096;
pub const TXTBIT_EXTENTCHANGE: u32 = 524288;
pub const TXTBIT_FLASHLASTPASSWORDCHAR: u32 = 268435456;
pub const TXTBIT_HIDESELECTION: u32 = 32;
pub const TXTBIT_MAXLENGTHCHANGE: u32 = 32768;
pub const TXTBIT_MULTILINE: u32 = 2;
pub const TXTBIT_NOTHREADREFCOUNT: u32 = 4194304;
pub const TXTBIT_PARAFORMATCHANGE: u32 = 262144;
pub const TXTBIT_READONLY: u32 = 4;
pub const TXTBIT_RICHTEXT: u32 = 1;
pub const TXTBIT_SAVESELECTION: u32 = 64;
pub const TXTBIT_SCROLLBARCHANGE: u32 = 65536;
pub const TXTBIT_SELBARCHANGE: u32 = 512;
pub const TXTBIT_SHOWACCELERATOR: u32 = 8;
pub const TXTBIT_SHOWPASSWORD: u32 = 8388608;
pub const TXTBIT_USECURRENTBKG: u32 = 2097152;
pub const TXTBIT_USEPASSWORD: u32 = 16;
pub const TXTBIT_VERTICAL: u32 = 256;
pub const TXTBIT_VIEWINSETCHANGE: u32 = 8192;
pub const TXTBIT_WORDWRAP: u32 = 1024;
pub type TXTHITRESULT = i32;
pub const TXTHITRESULT_CLOSE: TXTHITRESULT = 2;
pub const TXTHITRESULT_HIT: TXTHITRESULT = 3;
pub const TXTHITRESULT_NOHIT: TXTHITRESULT = 0;
pub const TXTHITRESULT_TRANSPARENT: TXTHITRESULT = 1;
pub type TXTNATURALSIZE = i32;
pub const TXTNS_EMU: TXTNATURALSIZE = -2147483648;
pub const TXTNS_FITTOCONTENT: TXTNATURALSIZE = 1;
pub const TXTNS_FITTOCONTENT2: TXTNATURALSIZE = 0;
pub const TXTNS_FITTOCONTENT3: TXTNATURALSIZE = 3;
pub const TXTNS_FITTOCONTENTWSP: TXTNATURALSIZE = 4;
pub const TXTNS_INCLUDELASTLINE: TXTNATURALSIZE = 1073741824;
pub const TXTNS_ROUNDTOLINE: TXTNATURALSIZE = 2;
pub type TXTVIEW = i32;
pub const TXTVIEW_ACTIVE: TXTVIEW = 0;
pub const TXTVIEW_INACTIVE: TXTVIEW = -1;
