pub const Closed: VisualState = 4;
pub type CorrectionMode = i32;
pub const CorrectionMode_NotVisible: CorrectionMode = 0;
pub const CorrectionMode_PostInsertionCollapsed: CorrectionMode = 2;
pub const CorrectionMode_PostInsertionExpanded: CorrectionMode = 3;
pub const CorrectionMode_PreInsertion: CorrectionMode = 1;
pub type CorrectionPosition = i32;
pub const CorrectionPosition_Auto: CorrectionPosition = 0;
pub const CorrectionPosition_Bottom: CorrectionPosition = 1;
pub const CorrectionPosition_Top: CorrectionPosition = 2;
pub const DISPID_PIPAttachedEditWindow: DISPID_PenInputPanel = 0;
pub const DISPID_PIPAutoShow: DISPID_PenInputPanel = 16;
pub const DISPID_PIPBusy: DISPID_PenInputPanel = 12;
pub const DISPID_PIPCommitPendingInput: DISPID_PenInputPanel = 10;
pub const DISPID_PIPCurrentPanel: DISPID_PenInputPanel = 2;
pub const DISPID_PIPDefaultPanel: DISPID_PenInputPanel = 3;
pub const DISPID_PIPEInputFailed: DISPID_PenInputPanelEvents = 2;
pub const DISPID_PIPEPanelChanged: DISPID_PenInputPanelEvents = 1;
pub const DISPID_PIPEPanelMoving: DISPID_PenInputPanelEvents = 3;
pub const DISPID_PIPEVisibleChanged: DISPID_PenInputPanelEvents = 0;
pub const DISPID_PIPEnableTsf: DISPID_PenInputPanel = 15;
pub const DISPID_PIPFactoid: DISPID_PenInputPanel = 1;
pub const DISPID_PIPHeight: DISPID_PenInputPanel = 8;
pub const DISPID_PIPHorizontalOffset: DISPID_PenInputPanel = 14;
pub const DISPID_PIPLeft: DISPID_PenInputPanel = 6;
pub const DISPID_PIPMoveTo: DISPID_PenInputPanel = 9;
pub const DISPID_PIPRefresh: DISPID_PenInputPanel = 11;
pub const DISPID_PIPTop: DISPID_PenInputPanel = 5;
pub const DISPID_PIPVerticalOffset: DISPID_PenInputPanel = 13;
pub const DISPID_PIPVisible: DISPID_PenInputPanel = 4;
pub const DISPID_PIPWidth: DISPID_PenInputPanel = 7;
pub type DISPID_PenInputPanel = i32;
pub type DISPID_PenInputPanelEvents = i32;
pub const DockedBottom: VisualState = 3;
pub const DockedTop: VisualState = 2;
pub type EventMask = i32;
pub const EventMask_All: EventMask = 4095;
pub const EventMask_CorrectionModeChanged: EventMask = 128;
pub const EventMask_CorrectionModeChanging: EventMask = 64;
pub const EventMask_InPlaceSizeChanged: EventMask = 8;
pub const EventMask_InPlaceSizeChanging: EventMask = 4;
pub const EventMask_InPlaceStateChanged: EventMask = 2;
pub const EventMask_InPlaceStateChanging: EventMask = 1;
pub const EventMask_InPlaceVisibilityChanged: EventMask = 512;
pub const EventMask_InPlaceVisibilityChanging: EventMask = 256;
pub const EventMask_InputAreaChanged: EventMask = 32;
pub const EventMask_InputAreaChanging: EventMask = 16;
pub const EventMask_TextInserted: EventMask = 2048;
pub const EventMask_TextInserting: EventMask = 1024;
pub const Floating: VisualState = 1;
pub const HandwrittenTextInsertion: windows_core::GUID = windows_core::GUID::from_u128(0x9f074ee2_e6e9_4d8a_a047_eb5b5c3c55da);
windows_core::imp::define_interface!(IHandwrittenTextInsertion, IHandwrittenTextInsertion_Vtbl, 0x56fdea97_ecd6_43e7_aa3a_816be7785860);
windows_core::imp::interface_hierarchy!(IHandwrittenTextInsertion, windows_core::IUnknown);
impl IHandwrittenTextInsertion {
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
    pub unsafe fn InsertRecognitionResultsArray(&self, psaalternates: *const super::oaidl::SAFEARRAY, locale: super::winnt::LCID, falternatecontainsautospacinginformation: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertRecognitionResultsArray)(windows_core::Interface::as_raw(self), psaalternates, locale, falternatecontainsautospacinginformation.into()) }
    }
    #[cfg(all(feature = "Win32_msinkaut", feature = "Win32_oaidl", feature = "Win32_winnt"))]
    pub unsafe fn InsertInkRecognitionResult<P0>(&self, piinkrecoresult: P0, locale: super::winnt::LCID, falternatecontainsautospacinginformation: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::msinkaut::IInkRecognitionResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertInkRecognitionResult)(windows_core::Interface::as_raw(self), piinkrecoresult.param().abi(), locale, falternatecontainsautospacinginformation.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandwrittenTextInsertion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
    pub InsertRecognitionResultsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY, super::winnt::LCID, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_winnt")))]
    InsertRecognitionResultsArray: usize,
    #[cfg(all(feature = "Win32_msinkaut", feature = "Win32_oaidl", feature = "Win32_winnt"))]
    pub InsertInkRecognitionResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::winnt::LCID, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_msinkaut", feature = "Win32_oaidl", feature = "Win32_winnt")))]
    InsertInkRecognitionResult: usize,
}
#[cfg(all(feature = "Win32_msinkaut", feature = "Win32_oaidl", feature = "Win32_winnt"))]
pub trait IHandwrittenTextInsertion_Impl: windows_core::IUnknownImpl {
    fn InsertRecognitionResultsArray(&self, psaalternates: *const super::oaidl::SAFEARRAY, locale: super::winnt::LCID, falternatecontainsautospacinginformation: windows_core::BOOL) -> windows_core::Result<()>;
    fn InsertInkRecognitionResult(&self, piinkrecoresult: windows_core::Ref<super::msinkaut::IInkRecognitionResult>, locale: super::winnt::LCID, falternatecontainsautospacinginformation: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_msinkaut", feature = "Win32_oaidl", feature = "Win32_winnt"))]
impl IHandwrittenTextInsertion_Vtbl {
    pub const fn new<Identity: IHandwrittenTextInsertion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertRecognitionResultsArray<Identity: IHandwrittenTextInsertion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psaalternates: *const super::oaidl::SAFEARRAY, locale: super::winnt::LCID, falternatecontainsautospacinginformation: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHandwrittenTextInsertion_Impl::InsertRecognitionResultsArray(this, core::mem::transmute_copy(&psaalternates), core::mem::transmute_copy(&locale), core::mem::transmute_copy(&falternatecontainsautospacinginformation)).into()
            }
        }
        unsafe extern "system" fn InsertInkRecognitionResult<Identity: IHandwrittenTextInsertion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piinkrecoresult: *mut core::ffi::c_void, locale: super::winnt::LCID, falternatecontainsautospacinginformation: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHandwrittenTextInsertion_Impl::InsertInkRecognitionResult(this, core::mem::transmute_copy(&piinkrecoresult), core::mem::transmute_copy(&locale), core::mem::transmute_copy(&falternatecontainsautospacinginformation)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertRecognitionResultsArray: InsertRecognitionResultsArray::<Identity, OFFSET>,
            InsertInkRecognitionResult: InsertInkRecognitionResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHandwrittenTextInsertion as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msinkaut", feature = "Win32_oaidl", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IHandwrittenTextInsertion {}
windows_core::imp::define_interface!(IInputPanelWindowHandle, IInputPanelWindowHandle_Vtbl, 0x4af81847_fdc4_4fc3_ad0b_422479c1b935);
windows_core::imp::interface_hierarchy!(IInputPanelWindowHandle, windows_core::IUnknown);
impl IInputPanelWindowHandle {
    pub unsafe fn AttachedEditWindow32(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttachedEditWindow32)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAttachedEditWindow32(&self, attachededitwindow: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAttachedEditWindow32)(windows_core::Interface::as_raw(self), attachededitwindow) }
    }
    pub unsafe fn AttachedEditWindow64(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttachedEditWindow64)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAttachedEditWindow64(&self, attachededitwindow: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAttachedEditWindow64)(windows_core::Interface::as_raw(self), attachededitwindow) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPanelWindowHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AttachedEditWindow32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAttachedEditWindow32: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AttachedEditWindow64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub SetAttachedEditWindow64: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
pub trait IInputPanelWindowHandle_Impl: windows_core::IUnknownImpl {
    fn AttachedEditWindow32(&self) -> windows_core::Result<i32>;
    fn SetAttachedEditWindow32(&self, attachededitwindow: i32) -> windows_core::Result<()>;
    fn AttachedEditWindow64(&self) -> windows_core::Result<i64>;
    fn SetAttachedEditWindow64(&self, attachededitwindow: i64) -> windows_core::Result<()>;
}
impl IInputPanelWindowHandle_Vtbl {
    pub const fn new<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachedEditWindow32<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPanelWindowHandle_Impl::AttachedEditWindow32(this) {
                    Ok(ok__) => {
                        attachededitwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow32<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPanelWindowHandle_Impl::SetAttachedEditWindow32(this, core::mem::transmute_copy(&attachededitwindow)).into()
            }
        }
        unsafe extern "system" fn AttachedEditWindow64<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPanelWindowHandle_Impl::AttachedEditWindow64(this) {
                    Ok(ok__) => {
                        attachededitwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow64<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPanelWindowHandle_Impl::SetAttachedEditWindow64(this, core::mem::transmute_copy(&attachededitwindow)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AttachedEditWindow32: AttachedEditWindow32::<Identity, OFFSET>,
            SetAttachedEditWindow32: SetAttachedEditWindow32::<Identity, OFFSET>,
            AttachedEditWindow64: AttachedEditWindow64::<Identity, OFFSET>,
            SetAttachedEditWindow64: SetAttachedEditWindow64::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPanelWindowHandle as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInputPanelWindowHandle {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IPenInputPanel, IPenInputPanel_Vtbl, 0xfa7a4083_5747_4040_a182_0b0e9fd4fac7);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IPenInputPanel {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IPenInputPanel, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IPenInputPanel {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Busy(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Busy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Factoid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Factoid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFactoid(&self, factoid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFactoid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(factoid)) }
    }
    pub unsafe fn AttachedEditWindow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttachedEditWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAttachedEditWindow(&self, attachededitwindow: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAttachedEditWindow)(windows_core::Interface::as_raw(self), attachededitwindow) }
    }
    pub unsafe fn CurrentPanel(&self) -> windows_core::Result<PanelType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPanel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCurrentPanel(&self, currentpanel: PanelType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPanel)(windows_core::Interface::as_raw(self), currentpanel) }
    }
    pub unsafe fn DefaultPanel(&self) -> windows_core::Result<PanelType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultPanel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultPanel(&self, defaultpanel: PanelType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultPanel)(windows_core::Interface::as_raw(self), defaultpanel) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Visible(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetVisible(&self, visible: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVisible)(windows_core::Interface::as_raw(self), visible) }
    }
    pub unsafe fn Top(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Top)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Left(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Left)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Height(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VerticalOffset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticalOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetVerticalOffset(&self, verticaloffset: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVerticalOffset)(windows_core::Interface::as_raw(self), verticaloffset) }
    }
    pub unsafe fn HorizontalOffset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HorizontalOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHorizontalOffset(&self, horizontaloffset: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHorizontalOffset)(windows_core::Interface::as_raw(self), horizontaloffset) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn AutoShow(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoShow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetAutoShow(&self, autoshow: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoShow)(windows_core::Interface::as_raw(self), autoshow) }
    }
    pub unsafe fn MoveTo(&self, left: i32, top: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveTo)(windows_core::Interface::as_raw(self), left, top) }
    }
    pub unsafe fn CommitPendingInput(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitPendingInput)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Refresh(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn EnableTsf(&self, enable: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableTsf)(windows_core::Interface::as_raw(self), enable) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPenInputPanel_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub Busy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Busy: usize,
    pub Factoid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFactoid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttachedEditWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAttachedEditWindow: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CurrentPanel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PanelType) -> windows_core::HRESULT,
    pub SetCurrentPanel: unsafe extern "system" fn(*mut core::ffi::c_void, PanelType) -> windows_core::HRESULT,
    pub DefaultPanel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PanelType) -> windows_core::HRESULT,
    pub SetDefaultPanel: unsafe extern "system" fn(*mut core::ffi::c_void, PanelType) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Visible: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetVisible: usize,
    pub Top: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Left: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub AutoShow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    AutoShow: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetAutoShow: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetAutoShow: usize,
    pub MoveTo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub CommitPendingInput: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub EnableTsf: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    EnableTsf: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IPenInputPanel_Impl: super::oaidl::IDispatch_Impl {
    fn Busy(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn Factoid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFactoid(&self, factoid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AttachedEditWindow(&self) -> windows_core::Result<i32>;
    fn SetAttachedEditWindow(&self, attachededitwindow: i32) -> windows_core::Result<()>;
    fn CurrentPanel(&self) -> windows_core::Result<PanelType>;
    fn SetCurrentPanel(&self, currentpanel: PanelType) -> windows_core::Result<()>;
    fn DefaultPanel(&self) -> windows_core::Result<PanelType>;
    fn SetDefaultPanel(&self, defaultpanel: PanelType) -> windows_core::Result<()>;
    fn Visible(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetVisible(&self, visible: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Top(&self) -> windows_core::Result<i32>;
    fn Left(&self) -> windows_core::Result<i32>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn VerticalOffset(&self) -> windows_core::Result<i32>;
    fn SetVerticalOffset(&self, verticaloffset: i32) -> windows_core::Result<()>;
    fn HorizontalOffset(&self) -> windows_core::Result<i32>;
    fn SetHorizontalOffset(&self, horizontaloffset: i32) -> windows_core::Result<()>;
    fn AutoShow(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetAutoShow(&self, autoshow: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MoveTo(&self, left: i32, top: i32) -> windows_core::Result<()>;
    fn CommitPendingInput(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn EnableTsf(&self, enable: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IPenInputPanel_Vtbl {
    pub const fn new<Identity: IPenInputPanel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Busy<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busy: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::Busy(this) {
                    Ok(ok__) => {
                        busy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Factoid<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::Factoid(this) {
                    Ok(ok__) => {
                        factoid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFactoid<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::SetFactoid(this, core::mem::transmute(&factoid)).into()
            }
        }
        unsafe extern "system" fn AttachedEditWindow<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::AttachedEditWindow(this) {
                    Ok(ok__) => {
                        attachededitwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::SetAttachedEditWindow(this, core::mem::transmute_copy(&attachededitwindow)).into()
            }
        }
        unsafe extern "system" fn CurrentPanel<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentpanel: *mut PanelType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::CurrentPanel(this) {
                    Ok(ok__) => {
                        currentpanel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentPanel<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentpanel: PanelType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::SetCurrentPanel(this, core::mem::transmute_copy(&currentpanel)).into()
            }
        }
        unsafe extern "system" fn DefaultPanel<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefaultpanel: *mut PanelType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::DefaultPanel(this) {
                    Ok(ok__) => {
                        pdefaultpanel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultPanel<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaultpanel: PanelType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::SetDefaultPanel(this, core::mem::transmute_copy(&defaultpanel)).into()
            }
        }
        unsafe extern "system" fn Visible<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::Visible(this) {
                    Ok(ok__) => {
                        visible.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVisible<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::SetVisible(this, core::mem::transmute_copy(&visible)).into()
            }
        }
        unsafe extern "system" fn Top<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::Top(this) {
                    Ok(ok__) => {
                        top.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Left<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::Left(this) {
                    Ok(ok__) => {
                        left.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Width<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::Width(this) {
                    Ok(ok__) => {
                        width.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Height<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::Height(this) {
                    Ok(ok__) => {
                        height.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VerticalOffset<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, verticaloffset: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::VerticalOffset(this) {
                    Ok(ok__) => {
                        verticaloffset.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, verticaloffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::SetVerticalOffset(this, core::mem::transmute_copy(&verticaloffset)).into()
            }
        }
        unsafe extern "system" fn HorizontalOffset<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontaloffset: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::HorizontalOffset(this) {
                    Ok(ok__) => {
                        horizontaloffset.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontaloffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::SetHorizontalOffset(this, core::mem::transmute_copy(&horizontaloffset)).into()
            }
        }
        unsafe extern "system" fn AutoShow<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pautoshow: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::AutoShow(this) {
                    Ok(ok__) => {
                        pautoshow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoShow<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoshow: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::SetAutoShow(this, core::mem::transmute_copy(&autoshow)).into()
            }
        }
        unsafe extern "system" fn MoveTo<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32, top: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::MoveTo(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn CommitPendingInput<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::CommitPendingInput(this).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn EnableTsf<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::EnableTsf(this, core::mem::transmute_copy(&enable)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Busy: Busy::<Identity, OFFSET>,
            Factoid: Factoid::<Identity, OFFSET>,
            SetFactoid: SetFactoid::<Identity, OFFSET>,
            AttachedEditWindow: AttachedEditWindow::<Identity, OFFSET>,
            SetAttachedEditWindow: SetAttachedEditWindow::<Identity, OFFSET>,
            CurrentPanel: CurrentPanel::<Identity, OFFSET>,
            SetCurrentPanel: SetCurrentPanel::<Identity, OFFSET>,
            DefaultPanel: DefaultPanel::<Identity, OFFSET>,
            SetDefaultPanel: SetDefaultPanel::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
            Top: Top::<Identity, OFFSET>,
            Left: Left::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            VerticalOffset: VerticalOffset::<Identity, OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Identity, OFFSET>,
            HorizontalOffset: HorizontalOffset::<Identity, OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Identity, OFFSET>,
            AutoShow: AutoShow::<Identity, OFFSET>,
            SetAutoShow: SetAutoShow::<Identity, OFFSET>,
            MoveTo: MoveTo::<Identity, OFFSET>,
            CommitPendingInput: CommitPendingInput::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            EnableTsf: EnableTsf::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPenInputPanel as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IPenInputPanel {}
windows_core::imp::define_interface!(ITextInputPanel, ITextInputPanel_Vtbl, 0x6b6a65a5_6af3_46c2_b6ea_56cd1f80df71);
windows_core::imp::interface_hierarchy!(ITextInputPanel, windows_core::IUnknown);
impl ITextInputPanel {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn AttachedEditWindow(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttachedEditWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SetAttachedEditWindow(&self, attachededitwindow: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAttachedEditWindow)(windows_core::Interface::as_raw(self), attachededitwindow) }
    }
    pub unsafe fn CurrentInteractionMode(&self) -> windows_core::Result<InteractionMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentInteractionMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DefaultInPlaceState(&self) -> windows_core::Result<InPlaceState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultInPlaceState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultInPlaceState(&self, state: InPlaceState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultInPlaceState)(windows_core::Interface::as_raw(self), state) }
    }
    pub unsafe fn CurrentInPlaceState(&self) -> windows_core::Result<InPlaceState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentInPlaceState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DefaultInputArea(&self) -> windows_core::Result<PanelInputArea> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultInputArea)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultInputArea(&self, area: PanelInputArea) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultInputArea)(windows_core::Interface::as_raw(self), area) }
    }
    pub unsafe fn CurrentInputArea(&self) -> windows_core::Result<PanelInputArea> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentInputArea)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentCorrectionMode(&self) -> windows_core::Result<CorrectionMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCorrectionMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PreferredInPlaceDirection(&self) -> windows_core::Result<InPlaceDirection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredInPlaceDirection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPreferredInPlaceDirection(&self, direction: InPlaceDirection) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPreferredInPlaceDirection)(windows_core::Interface::as_raw(self), direction) }
    }
    pub unsafe fn ExpandPostInsertionCorrection(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpandPostInsertionCorrection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExpandPostInsertionCorrection(&self, expand: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExpandPostInsertionCorrection)(windows_core::Interface::as_raw(self), expand.into()) }
    }
    pub unsafe fn InPlaceVisibleOnFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InPlaceVisibleOnFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetInPlaceVisibleOnFocus(&self, visible: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInPlaceVisibleOnFocus)(windows_core::Interface::as_raw(self), visible.into()) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn InPlaceBoundingRectangle(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InPlaceBoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PopUpCorrectionHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PopUpCorrectionHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PopDownCorrectionHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PopDownCorrectionHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CommitPendingInput(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitPendingInput)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetInPlaceVisibility(&self, visible: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInPlaceVisibility)(windows_core::Interface::as_raw(self), visible.into()) }
    }
    pub unsafe fn SetInPlacePosition(&self, xposition: i32, yposition: i32, position: CorrectionPosition) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInPlacePosition)(windows_core::Interface::as_raw(self), xposition, yposition, position) }
    }
    pub unsafe fn SetInPlaceHoverTargetPosition(&self, xposition: i32, yposition: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInPlaceHoverTargetPosition)(windows_core::Interface::as_raw(self), xposition, yposition) }
    }
    pub unsafe fn Advise<P0>(&self, eventsink: P0, eventmask: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextInputPanelEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), eventsink.param().abi(), eventmask) }
    }
    pub unsafe fn Unadvise<P0>(&self, eventsink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextInputPanelEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), eventsink.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub AttachedEditWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    AttachedEditWindow: usize,
    #[cfg(feature = "Win32_windef")]
    pub SetAttachedEditWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SetAttachedEditWindow: usize,
    pub CurrentInteractionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionMode) -> windows_core::HRESULT,
    pub DefaultInPlaceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InPlaceState) -> windows_core::HRESULT,
    pub SetDefaultInPlaceState: unsafe extern "system" fn(*mut core::ffi::c_void, InPlaceState) -> windows_core::HRESULT,
    pub CurrentInPlaceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InPlaceState) -> windows_core::HRESULT,
    pub DefaultInputArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PanelInputArea) -> windows_core::HRESULT,
    pub SetDefaultInputArea: unsafe extern "system" fn(*mut core::ffi::c_void, PanelInputArea) -> windows_core::HRESULT,
    pub CurrentInputArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PanelInputArea) -> windows_core::HRESULT,
    pub CurrentCorrectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CorrectionMode) -> windows_core::HRESULT,
    pub PreferredInPlaceDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InPlaceDirection) -> windows_core::HRESULT,
    pub SetPreferredInPlaceDirection: unsafe extern "system" fn(*mut core::ffi::c_void, InPlaceDirection) -> windows_core::HRESULT,
    pub ExpandPostInsertionCorrection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetExpandPostInsertionCorrection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub InPlaceVisibleOnFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetInPlaceVisibleOnFocus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub InPlaceBoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    InPlaceBoundingRectangle: usize,
    pub PopUpCorrectionHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PopDownCorrectionHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CommitPendingInput: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInPlaceVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetInPlacePosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, CorrectionPosition) -> windows_core::HRESULT,
    pub SetInPlaceHoverTargetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait ITextInputPanel_Impl: windows_core::IUnknownImpl {
    fn AttachedEditWindow(&self) -> windows_core::Result<super::windef::HWND>;
    fn SetAttachedEditWindow(&self, attachededitwindow: super::windef::HWND) -> windows_core::Result<()>;
    fn CurrentInteractionMode(&self) -> windows_core::Result<InteractionMode>;
    fn DefaultInPlaceState(&self) -> windows_core::Result<InPlaceState>;
    fn SetDefaultInPlaceState(&self, state: InPlaceState) -> windows_core::Result<()>;
    fn CurrentInPlaceState(&self) -> windows_core::Result<InPlaceState>;
    fn DefaultInputArea(&self) -> windows_core::Result<PanelInputArea>;
    fn SetDefaultInputArea(&self, area: PanelInputArea) -> windows_core::Result<()>;
    fn CurrentInputArea(&self) -> windows_core::Result<PanelInputArea>;
    fn CurrentCorrectionMode(&self) -> windows_core::Result<CorrectionMode>;
    fn PreferredInPlaceDirection(&self) -> windows_core::Result<InPlaceDirection>;
    fn SetPreferredInPlaceDirection(&self, direction: InPlaceDirection) -> windows_core::Result<()>;
    fn ExpandPostInsertionCorrection(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetExpandPostInsertionCorrection(&self, expand: windows_core::BOOL) -> windows_core::Result<()>;
    fn InPlaceVisibleOnFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetInPlaceVisibleOnFocus(&self, visible: windows_core::BOOL) -> windows_core::Result<()>;
    fn InPlaceBoundingRectangle(&self) -> windows_core::Result<super::windef::RECT>;
    fn PopUpCorrectionHeight(&self) -> windows_core::Result<i32>;
    fn PopDownCorrectionHeight(&self) -> windows_core::Result<i32>;
    fn CommitPendingInput(&self) -> windows_core::Result<()>;
    fn SetInPlaceVisibility(&self, visible: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetInPlacePosition(&self, xposition: i32, yposition: i32, position: CorrectionPosition) -> windows_core::Result<()>;
    fn SetInPlaceHoverTargetPosition(&self, xposition: i32, yposition: i32) -> windows_core::Result<()>;
    fn Advise(&self, eventsink: windows_core::Ref<ITextInputPanelEventSink>, eventmask: u32) -> windows_core::Result<()>;
    fn Unadvise(&self, eventsink: windows_core::Ref<ITextInputPanelEventSink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl ITextInputPanel_Vtbl {
    pub const fn new<Identity: ITextInputPanel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachedEditWindow<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::AttachedEditWindow(this) {
                    Ok(ok__) => {
                        attachededitwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttachedEditWindow<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::SetAttachedEditWindow(this, core::mem::transmute_copy(&attachededitwindow)).into()
            }
        }
        unsafe extern "system" fn CurrentInteractionMode<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentinteractionmode: *mut InteractionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::CurrentInteractionMode(this) {
                    Ok(ok__) => {
                        currentinteractionmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultInPlaceState<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut InPlaceState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::DefaultInPlaceState(this) {
                    Ok(ok__) => {
                        state.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultInPlaceState<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: InPlaceState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::SetDefaultInPlaceState(this, core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn CurrentInPlaceState<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut InPlaceState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::CurrentInPlaceState(this) {
                    Ok(ok__) => {
                        state.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultInputArea<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, area: *mut PanelInputArea) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::DefaultInputArea(this) {
                    Ok(ok__) => {
                        area.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultInputArea<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, area: PanelInputArea) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::SetDefaultInputArea(this, core::mem::transmute_copy(&area)).into()
            }
        }
        unsafe extern "system" fn CurrentInputArea<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, area: *mut PanelInputArea) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::CurrentInputArea(this) {
                    Ok(ok__) => {
                        area.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentCorrectionMode<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut CorrectionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::CurrentCorrectionMode(this) {
                    Ok(ok__) => {
                        mode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PreferredInPlaceDirection<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: *mut InPlaceDirection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::PreferredInPlaceDirection(this) {
                    Ok(ok__) => {
                        direction.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreferredInPlaceDirection<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: InPlaceDirection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::SetPreferredInPlaceDirection(this, core::mem::transmute_copy(&direction)).into()
            }
        }
        unsafe extern "system" fn ExpandPostInsertionCorrection<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expand: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::ExpandPostInsertionCorrection(this) {
                    Ok(ok__) => {
                        expand.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExpandPostInsertionCorrection<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expand: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::SetExpandPostInsertionCorrection(this, core::mem::transmute_copy(&expand)).into()
            }
        }
        unsafe extern "system" fn InPlaceVisibleOnFocus<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::InPlaceVisibleOnFocus(this) {
                    Ok(ok__) => {
                        visible.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInPlaceVisibleOnFocus<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::SetInPlaceVisibleOnFocus(this, core::mem::transmute_copy(&visible)).into()
            }
        }
        unsafe extern "system" fn InPlaceBoundingRectangle<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundingrectangle: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::InPlaceBoundingRectangle(this) {
                    Ok(ok__) => {
                        boundingrectangle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PopUpCorrectionHeight<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::PopUpCorrectionHeight(this) {
                    Ok(ok__) => {
                        height.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PopDownCorrectionHeight<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::PopDownCorrectionHeight(this) {
                    Ok(ok__) => {
                        height.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommitPendingInput<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::CommitPendingInput(this).into()
            }
        }
        unsafe extern "system" fn SetInPlaceVisibility<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::SetInPlaceVisibility(this, core::mem::transmute_copy(&visible)).into()
            }
        }
        unsafe extern "system" fn SetInPlacePosition<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xposition: i32, yposition: i32, position: CorrectionPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::SetInPlacePosition(this, core::mem::transmute_copy(&xposition), core::mem::transmute_copy(&yposition), core::mem::transmute_copy(&position)).into()
            }
        }
        unsafe extern "system" fn SetInPlaceHoverTargetPosition<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xposition: i32, yposition: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::SetInPlaceHoverTargetPosition(this, core::mem::transmute_copy(&xposition), core::mem::transmute_copy(&yposition)).into()
            }
        }
        unsafe extern "system" fn Advise<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventsink: *mut core::ffi::c_void, eventmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::Advise(this, core::mem::transmute_copy(&eventsink), core::mem::transmute_copy(&eventmask)).into()
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::Unadvise(this, core::mem::transmute_copy(&eventsink)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AttachedEditWindow: AttachedEditWindow::<Identity, OFFSET>,
            SetAttachedEditWindow: SetAttachedEditWindow::<Identity, OFFSET>,
            CurrentInteractionMode: CurrentInteractionMode::<Identity, OFFSET>,
            DefaultInPlaceState: DefaultInPlaceState::<Identity, OFFSET>,
            SetDefaultInPlaceState: SetDefaultInPlaceState::<Identity, OFFSET>,
            CurrentInPlaceState: CurrentInPlaceState::<Identity, OFFSET>,
            DefaultInputArea: DefaultInputArea::<Identity, OFFSET>,
            SetDefaultInputArea: SetDefaultInputArea::<Identity, OFFSET>,
            CurrentInputArea: CurrentInputArea::<Identity, OFFSET>,
            CurrentCorrectionMode: CurrentCorrectionMode::<Identity, OFFSET>,
            PreferredInPlaceDirection: PreferredInPlaceDirection::<Identity, OFFSET>,
            SetPreferredInPlaceDirection: SetPreferredInPlaceDirection::<Identity, OFFSET>,
            ExpandPostInsertionCorrection: ExpandPostInsertionCorrection::<Identity, OFFSET>,
            SetExpandPostInsertionCorrection: SetExpandPostInsertionCorrection::<Identity, OFFSET>,
            InPlaceVisibleOnFocus: InPlaceVisibleOnFocus::<Identity, OFFSET>,
            SetInPlaceVisibleOnFocus: SetInPlaceVisibleOnFocus::<Identity, OFFSET>,
            InPlaceBoundingRectangle: InPlaceBoundingRectangle::<Identity, OFFSET>,
            PopUpCorrectionHeight: PopUpCorrectionHeight::<Identity, OFFSET>,
            PopDownCorrectionHeight: PopDownCorrectionHeight::<Identity, OFFSET>,
            CommitPendingInput: CommitPendingInput::<Identity, OFFSET>,
            SetInPlaceVisibility: SetInPlaceVisibility::<Identity, OFFSET>,
            SetInPlacePosition: SetInPlacePosition::<Identity, OFFSET>,
            SetInPlaceHoverTargetPosition: SetInPlaceHoverTargetPosition::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextInputPanel as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for ITextInputPanel {}
windows_core::imp::define_interface!(ITextInputPanelEventSink, ITextInputPanelEventSink_Vtbl, 0x27560408_8e64_4fe1_804e_421201584b31);
windows_core::imp::interface_hierarchy!(ITextInputPanelEventSink, windows_core::IUnknown);
impl ITextInputPanelEventSink {
    pub unsafe fn InPlaceStateChanging(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceStateChanging)(windows_core::Interface::as_raw(self), oldinplacestate, newinplacestate) }
    }
    pub unsafe fn InPlaceStateChanged(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceStateChanged)(windows_core::Interface::as_raw(self), oldinplacestate, newinplacestate) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn InPlaceSizeChanging(&self, oldboundingrectangle: super::windef::RECT, newboundingrectangle: super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceSizeChanging)(windows_core::Interface::as_raw(self), core::mem::transmute(oldboundingrectangle), core::mem::transmute(newboundingrectangle)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn InPlaceSizeChanged(&self, oldboundingrectangle: super::windef::RECT, newboundingrectangle: super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceSizeChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(oldboundingrectangle), core::mem::transmute(newboundingrectangle)) }
    }
    pub unsafe fn InputAreaChanging(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InputAreaChanging)(windows_core::Interface::as_raw(self), oldinputarea, newinputarea) }
    }
    pub unsafe fn InputAreaChanged(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InputAreaChanged)(windows_core::Interface::as_raw(self), oldinputarea, newinputarea) }
    }
    pub unsafe fn CorrectionModeChanging(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CorrectionModeChanging)(windows_core::Interface::as_raw(self), oldcorrectionmode, newcorrectionmode) }
    }
    pub unsafe fn CorrectionModeChanged(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CorrectionModeChanged)(windows_core::Interface::as_raw(self), oldcorrectionmode, newcorrectionmode) }
    }
    pub unsafe fn InPlaceVisibilityChanging(&self, oldvisible: bool, newvisible: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceVisibilityChanging)(windows_core::Interface::as_raw(self), oldvisible.into(), newvisible.into()) }
    }
    pub unsafe fn InPlaceVisibilityChanged(&self, oldvisible: bool, newvisible: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceVisibilityChanged)(windows_core::Interface::as_raw(self), oldvisible.into(), newvisible.into()) }
    }
    #[cfg(feature = "Win32_oaidl")]
    pub unsafe fn TextInserting(&self, ink: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TextInserting)(windows_core::Interface::as_raw(self), ink) }
    }
    #[cfg(feature = "Win32_oaidl")]
    pub unsafe fn TextInserted(&self, ink: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TextInserted)(windows_core::Interface::as_raw(self), ink) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanelEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InPlaceStateChanging: unsafe extern "system" fn(*mut core::ffi::c_void, InPlaceState, InPlaceState) -> windows_core::HRESULT,
    pub InPlaceStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, InPlaceState, InPlaceState) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub InPlaceSizeChanging: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::RECT, super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    InPlaceSizeChanging: usize,
    #[cfg(feature = "Win32_windef")]
    pub InPlaceSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::RECT, super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    InPlaceSizeChanged: usize,
    pub InputAreaChanging: unsafe extern "system" fn(*mut core::ffi::c_void, PanelInputArea, PanelInputArea) -> windows_core::HRESULT,
    pub InputAreaChanged: unsafe extern "system" fn(*mut core::ffi::c_void, PanelInputArea, PanelInputArea) -> windows_core::HRESULT,
    pub CorrectionModeChanging: unsafe extern "system" fn(*mut core::ffi::c_void, CorrectionMode, CorrectionMode) -> windows_core::HRESULT,
    pub CorrectionModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, CorrectionMode, CorrectionMode) -> windows_core::HRESULT,
    pub InPlaceVisibilityChanging: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub InPlaceVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_oaidl")]
    pub TextInserting: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_oaidl"))]
    TextInserting: usize,
    #[cfg(feature = "Win32_oaidl")]
    pub TextInserted: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_oaidl"))]
    TextInserted: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef"))]
pub trait ITextInputPanelEventSink_Impl: windows_core::IUnknownImpl {
    fn InPlaceStateChanging(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::Result<()>;
    fn InPlaceStateChanged(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::Result<()>;
    fn InPlaceSizeChanging(&self, oldboundingrectangle: &super::windef::RECT, newboundingrectangle: &super::windef::RECT) -> windows_core::Result<()>;
    fn InPlaceSizeChanged(&self, oldboundingrectangle: &super::windef::RECT, newboundingrectangle: &super::windef::RECT) -> windows_core::Result<()>;
    fn InputAreaChanging(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::Result<()>;
    fn InputAreaChanged(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::Result<()>;
    fn CorrectionModeChanging(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::Result<()>;
    fn CorrectionModeChanged(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::Result<()>;
    fn InPlaceVisibilityChanging(&self, oldvisible: windows_core::BOOL, newvisible: windows_core::BOOL) -> windows_core::Result<()>;
    fn InPlaceVisibilityChanged(&self, oldvisible: windows_core::BOOL, newvisible: windows_core::BOOL) -> windows_core::Result<()>;
    fn TextInserting(&self, ink: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn TextInserted(&self, ink: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef"))]
impl ITextInputPanelEventSink_Vtbl {
    pub const fn new<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InPlaceStateChanging<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::InPlaceStateChanging(this, core::mem::transmute_copy(&oldinplacestate), core::mem::transmute_copy(&newinplacestate)).into()
            }
        }
        unsafe extern "system" fn InPlaceStateChanged<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::InPlaceStateChanged(this, core::mem::transmute_copy(&oldinplacestate), core::mem::transmute_copy(&newinplacestate)).into()
            }
        }
        unsafe extern "system" fn InPlaceSizeChanging<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldboundingrectangle: super::windef::RECT, newboundingrectangle: super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::InPlaceSizeChanging(this, core::mem::transmute(&oldboundingrectangle), core::mem::transmute(&newboundingrectangle)).into()
            }
        }
        unsafe extern "system" fn InPlaceSizeChanged<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldboundingrectangle: super::windef::RECT, newboundingrectangle: super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::InPlaceSizeChanged(this, core::mem::transmute(&oldboundingrectangle), core::mem::transmute(&newboundingrectangle)).into()
            }
        }
        unsafe extern "system" fn InputAreaChanging<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::InputAreaChanging(this, core::mem::transmute_copy(&oldinputarea), core::mem::transmute_copy(&newinputarea)).into()
            }
        }
        unsafe extern "system" fn InputAreaChanged<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::InputAreaChanged(this, core::mem::transmute_copy(&oldinputarea), core::mem::transmute_copy(&newinputarea)).into()
            }
        }
        unsafe extern "system" fn CorrectionModeChanging<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::CorrectionModeChanging(this, core::mem::transmute_copy(&oldcorrectionmode), core::mem::transmute_copy(&newcorrectionmode)).into()
            }
        }
        unsafe extern "system" fn CorrectionModeChanged<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::CorrectionModeChanged(this, core::mem::transmute_copy(&oldcorrectionmode), core::mem::transmute_copy(&newcorrectionmode)).into()
            }
        }
        unsafe extern "system" fn InPlaceVisibilityChanging<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldvisible: windows_core::BOOL, newvisible: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::InPlaceVisibilityChanging(this, core::mem::transmute_copy(&oldvisible), core::mem::transmute_copy(&newvisible)).into()
            }
        }
        unsafe extern "system" fn InPlaceVisibilityChanged<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldvisible: windows_core::BOOL, newvisible: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::InPlaceVisibilityChanged(this, core::mem::transmute_copy(&oldvisible), core::mem::transmute_copy(&newvisible)).into()
            }
        }
        unsafe extern "system" fn TextInserting<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::TextInserting(this, core::mem::transmute_copy(&ink)).into()
            }
        }
        unsafe extern "system" fn TextInserted<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::TextInserted(this, core::mem::transmute_copy(&ink)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InPlaceStateChanging: InPlaceStateChanging::<Identity, OFFSET>,
            InPlaceStateChanged: InPlaceStateChanged::<Identity, OFFSET>,
            InPlaceSizeChanging: InPlaceSizeChanging::<Identity, OFFSET>,
            InPlaceSizeChanged: InPlaceSizeChanged::<Identity, OFFSET>,
            InputAreaChanging: InputAreaChanging::<Identity, OFFSET>,
            InputAreaChanged: InputAreaChanged::<Identity, OFFSET>,
            CorrectionModeChanging: CorrectionModeChanging::<Identity, OFFSET>,
            CorrectionModeChanged: CorrectionModeChanged::<Identity, OFFSET>,
            InPlaceVisibilityChanging: InPlaceVisibilityChanging::<Identity, OFFSET>,
            InPlaceVisibilityChanged: InPlaceVisibilityChanged::<Identity, OFFSET>,
            TextInserting: TextInserting::<Identity, OFFSET>,
            TextInserted: TextInserted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextInputPanelEventSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ITextInputPanelEventSink {}
windows_core::imp::define_interface!(ITextInputPanelRunInfo, ITextInputPanelRunInfo_Vtbl, 0x9f424568_1920_48cc_9811_a993cbf5adba);
windows_core::imp::interface_hierarchy!(ITextInputPanelRunInfo, windows_core::IUnknown);
impl ITextInputPanelRunInfo {
    pub unsafe fn IsTipRunning(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTipRunning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanelRunInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsTipRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITextInputPanelRunInfo_Impl: windows_core::IUnknownImpl {
    fn IsTipRunning(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl ITextInputPanelRunInfo_Vtbl {
    pub const fn new<Identity: ITextInputPanelRunInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsTipRunning<Identity: ITextInputPanelRunInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrunning: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanelRunInfo_Impl::IsTipRunning(this) {
                    Ok(ok__) => {
                        pfrunning.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsTipRunning: IsTipRunning::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextInputPanelRunInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextInputPanelRunInfo {}
pub const InPlace: VisualState = 0;
pub type InPlaceDirection = i32;
pub const InPlaceDirection_Auto: InPlaceDirection = 0;
pub const InPlaceDirection_Bottom: InPlaceDirection = 1;
pub const InPlaceDirection_Top: InPlaceDirection = 2;
pub type InPlaceState = i32;
pub const InPlaceState_Auto: InPlaceState = 0;
pub const InPlaceState_Expanded: InPlaceState = 2;
pub const InPlaceState_HoverTarget: InPlaceState = 1;
pub type InteractionMode = i32;
pub const InteractionMode_DockedBottom: InteractionMode = 3;
pub const InteractionMode_DockedTop: InteractionMode = 2;
pub const InteractionMode_Floating: InteractionMode = 1;
pub const InteractionMode_InPlace: InteractionMode = 0;
pub const MICROSOFT_PENINPUT_PANEL_PROPERTY_T: windows_core::PCWSTR = windows_core::w!("Microsoft PenInputPanel 1.5");
pub const PT_Default: PanelType = 0;
pub const PT_Handwriting: PanelType = 2;
pub const PT_Inactive: PanelType = 1;
pub const PT_Keyboard: PanelType = 3;
pub type PanelInputArea = i32;
pub const PanelInputArea_Auto: PanelInputArea = 0;
pub const PanelInputArea_CharacterPad: PanelInputArea = 3;
pub const PanelInputArea_Keyboard: PanelInputArea = 1;
pub const PanelInputArea_WritingPad: PanelInputArea = 2;
pub type PanelType = i32;
pub const PenInputPanel: windows_core::GUID = windows_core::GUID::from_u128(0xf744e496_1b5a_489e_81dc_fbd7ac6298a8);
pub const PenInputPanel_Internal: windows_core::GUID = windows_core::GUID::from_u128(0x802b1fb9_056b_4720_b0cc_80d23b71171e);
pub const TextInputPanel: windows_core::GUID = windows_core::GUID::from_u128(0xf9b189d7_228b_4f2b_8650_b97f59e02c8c);
pub type VisualState = i32;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(_IPenInputPanelEvents, _IPenInputPanelEvents_Vtbl, 0xb7e489da_3719_439f_848f_e7acbd820f17);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for _IPenInputPanelEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(_IPenInputPanelEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct _IPenInputPanelEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait _IPenInputPanelEvents_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl _IPenInputPanelEvents_Vtbl {
    pub const fn new<Identity: _IPenInputPanelEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IPenInputPanelEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for _IPenInputPanelEvents {}
