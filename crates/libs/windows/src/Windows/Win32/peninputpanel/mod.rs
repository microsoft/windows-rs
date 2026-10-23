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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn InsertRecognitionResultsArray(&self, psaalternates: *const super::SAFEARRAY, locale: super::LCID, falternatecontainsautospacinginformation: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertRecognitionResultsArray)(windows_core::Interface::as_raw(self), psaalternates, locale, falternatecontainsautospacinginformation.into()) }
    }
    #[cfg(all(feature = "msinkaut", feature = "oaidl", feature = "shared"))]
    pub unsafe fn InsertInkRecognitionResult<P0>(&self, piinkrecoresult: P0, locale: super::LCID, falternatecontainsautospacinginformation: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IInkRecognitionResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertInkRecognitionResult)(windows_core::Interface::as_raw(self), piinkrecoresult.param().abi(), locale, falternatecontainsautospacinginformation.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandwrittenTextInsertion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub InsertRecognitionResultsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY, super::LCID, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    InsertRecognitionResultsArray: usize,
    #[cfg(all(feature = "msinkaut", feature = "oaidl", feature = "shared"))]
    pub InsertInkRecognitionResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LCID, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "msinkaut", feature = "oaidl", feature = "shared")))]
    InsertInkRecognitionResult: usize,
}
#[cfg(all(feature = "msinkaut", feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IHandwrittenTextInsertion_Impl: windows_core::IUnknownImpl {
    fn InsertRecognitionResultsArray(&self, psaalternates: *const super::SAFEARRAY, locale: super::LCID, falternatecontainsautospacinginformation: windows_core::BOOL) -> windows_core::Result<()>;
    fn InsertInkRecognitionResult(&self, piinkrecoresult: windows_core::Ref<super::IInkRecognitionResult>, locale: super::LCID, falternatecontainsautospacinginformation: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "msinkaut", feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IHandwrittenTextInsertion_Vtbl {
    pub const fn new<Identity: IHandwrittenTextInsertion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertRecognitionResultsArray<Identity: IHandwrittenTextInsertion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psaalternates: *const super::SAFEARRAY, locale: super::LCID, falternatecontainsautospacinginformation: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHandwrittenTextInsertion_Impl::InsertRecognitionResultsArray(this, core::mem::transmute_copy(&psaalternates), core::mem::transmute_copy(&locale), core::mem::transmute_copy(&falternatecontainsautospacinginformation)).into()
            }
        }
        unsafe extern "system" fn InsertInkRecognitionResult<Identity: IHandwrittenTextInsertion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piinkrecoresult: *mut core::ffi::c_void, locale: super::LCID, falternatecontainsautospacinginformation: windows_core::BOOL) -> windows_core::HRESULT {
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
#[cfg(all(feature = "msinkaut", feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IHandwrittenTextInsertion {}
windows_core::imp::define_interface!(IInputPanelWindowHandle, IInputPanelWindowHandle_Vtbl, 0x4af81847_fdc4_4fc3_ad0b_422479c1b935);
windows_core::imp::interface_hierarchy!(IInputPanelWindowHandle, windows_core::IUnknown);
impl IInputPanelWindowHandle {
    #[cfg(feature = "basetsd")]
    pub unsafe fn get_AttachedEditWindow32(&self) -> windows_core::Result<super::LONG32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AttachedEditWindow32)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "basetsd")]
    pub unsafe fn put_AttachedEditWindow32(&self, attachededitwindow: super::LONG32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AttachedEditWindow32)(windows_core::Interface::as_raw(self), attachededitwindow) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_AttachedEditWindow64(&self) -> windows_core::Result<super::LONG64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AttachedEditWindow64)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_AttachedEditWindow64(&self, attachededitwindow: super::LONG64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AttachedEditWindow64)(windows_core::Interface::as_raw(self), attachededitwindow) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPanelWindowHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "basetsd")]
    pub get_AttachedEditWindow32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG32) -> windows_core::HRESULT,
    #[cfg(not(feature = "basetsd"))]
    get_AttachedEditWindow32: usize,
    #[cfg(feature = "basetsd")]
    pub put_AttachedEditWindow32: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG32) -> windows_core::HRESULT,
    #[cfg(not(feature = "basetsd"))]
    put_AttachedEditWindow32: usize,
    #[cfg(feature = "shared")]
    pub get_AttachedEditWindow64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG64) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_AttachedEditWindow64: usize,
    #[cfg(feature = "shared")]
    pub put_AttachedEditWindow64: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG64) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_AttachedEditWindow64: usize,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub trait IInputPanelWindowHandle_Impl: windows_core::IUnknownImpl {
    fn get_AttachedEditWindow32(&self) -> windows_core::Result<super::LONG32>;
    fn put_AttachedEditWindow32(&self, attachededitwindow: super::LONG32) -> windows_core::Result<()>;
    fn get_AttachedEditWindow64(&self) -> windows_core::Result<super::LONG64>;
    fn put_AttachedEditWindow64(&self, attachededitwindow: super::LONG64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl IInputPanelWindowHandle_Vtbl {
    pub const fn new<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AttachedEditWindow32<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut super::LONG32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPanelWindowHandle_Impl::get_AttachedEditWindow32(this) {
                    Ok(ok__) => {
                        attachededitwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AttachedEditWindow32<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: super::LONG32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPanelWindowHandle_Impl::put_AttachedEditWindow32(this, core::mem::transmute_copy(&attachededitwindow)).into()
            }
        }
        unsafe extern "system" fn get_AttachedEditWindow64<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut super::LONG64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPanelWindowHandle_Impl::get_AttachedEditWindow64(this) {
                    Ok(ok__) => {
                        attachededitwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AttachedEditWindow64<Identity: IInputPanelWindowHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: super::LONG64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPanelWindowHandle_Impl::put_AttachedEditWindow64(this, core::mem::transmute_copy(&attachededitwindow)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AttachedEditWindow32: get_AttachedEditWindow32::<Identity, OFFSET>,
            put_AttachedEditWindow32: put_AttachedEditWindow32::<Identity, OFFSET>,
            get_AttachedEditWindow64: get_AttachedEditWindow64::<Identity, OFFSET>,
            put_AttachedEditWindow64: put_AttachedEditWindow64::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPanelWindowHandle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl windows_core::RuntimeName for IInputPanelWindowHandle {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IPenInputPanel, IPenInputPanel_Vtbl, 0xfa7a4083_5747_4040_a182_0b0e9fd4fac7);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IPenInputPanel {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IPenInputPanel, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IPenInputPanel {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Busy(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Busy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Factoid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Factoid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Factoid(&self, factoid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Factoid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(factoid)) }
    }
    #[cfg(feature = "basetsd")]
    pub unsafe fn get_AttachedEditWindow(&self) -> windows_core::Result<super::LONG32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AttachedEditWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "basetsd")]
    pub unsafe fn put_AttachedEditWindow(&self, attachededitwindow: super::LONG32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AttachedEditWindow)(windows_core::Interface::as_raw(self), attachededitwindow) }
    }
    pub unsafe fn get_CurrentPanel(&self) -> windows_core::Result<PanelType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentPanel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_CurrentPanel(&self, currentpanel: PanelType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CurrentPanel)(windows_core::Interface::as_raw(self), currentpanel) }
    }
    pub unsafe fn get_DefaultPanel(&self) -> windows_core::Result<PanelType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultPanel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DefaultPanel(&self, defaultpanel: PanelType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DefaultPanel)(windows_core::Interface::as_raw(self), defaultpanel) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Visible(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Visible(&self, visible: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Visible)(windows_core::Interface::as_raw(self), visible) }
    }
    pub unsafe fn get_Top(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Top)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Left(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Left)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Height(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_VerticalOffset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VerticalOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_VerticalOffset(&self, verticaloffset: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_VerticalOffset)(windows_core::Interface::as_raw(self), verticaloffset) }
    }
    pub unsafe fn get_HorizontalOffset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HorizontalOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_HorizontalOffset(&self, horizontaloffset: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HorizontalOffset)(windows_core::Interface::as_raw(self), horizontaloffset) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AutoShow(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoShow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AutoShow(&self, autoshow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AutoShow)(windows_core::Interface::as_raw(self), autoshow) }
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
    #[cfg(feature = "wtypes")]
    pub unsafe fn EnableTsf(&self, enable: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableTsf)(windows_core::Interface::as_raw(self), enable) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPenInputPanel_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_Busy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Busy: usize,
    pub get_Factoid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Factoid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "basetsd")]
    pub get_AttachedEditWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG32) -> windows_core::HRESULT,
    #[cfg(not(feature = "basetsd"))]
    get_AttachedEditWindow: usize,
    #[cfg(feature = "basetsd")]
    pub put_AttachedEditWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG32) -> windows_core::HRESULT,
    #[cfg(not(feature = "basetsd"))]
    put_AttachedEditWindow: usize,
    pub get_CurrentPanel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PanelType) -> windows_core::HRESULT,
    pub put_CurrentPanel: unsafe extern "system" fn(*mut core::ffi::c_void, PanelType) -> windows_core::HRESULT,
    pub get_DefaultPanel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PanelType) -> windows_core::HRESULT,
    pub put_DefaultPanel: unsafe extern "system" fn(*mut core::ffi::c_void, PanelType) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Visible: usize,
    #[cfg(feature = "wtypes")]
    pub put_Visible: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Visible: usize,
    pub get_Top: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Left: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_VerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_VerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_HorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_HorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_AutoShow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AutoShow: usize,
    #[cfg(feature = "wtypes")]
    pub put_AutoShow: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AutoShow: usize,
    pub MoveTo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub CommitPendingInput: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub EnableTsf: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    EnableTsf: usize,
}
#[cfg(all(feature = "basetsd", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPenInputPanel_Impl: super::IDispatch_Impl {
    fn get_Busy(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Factoid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Factoid(&self, factoid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AttachedEditWindow(&self) -> windows_core::Result<super::LONG32>;
    fn put_AttachedEditWindow(&self, attachededitwindow: super::LONG32) -> windows_core::Result<()>;
    fn get_CurrentPanel(&self) -> windows_core::Result<PanelType>;
    fn put_CurrentPanel(&self, currentpanel: PanelType) -> windows_core::Result<()>;
    fn get_DefaultPanel(&self) -> windows_core::Result<PanelType>;
    fn put_DefaultPanel(&self, defaultpanel: PanelType) -> windows_core::Result<()>;
    fn get_Visible(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Visible(&self, visible: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Top(&self) -> windows_core::Result<i32>;
    fn get_Left(&self) -> windows_core::Result<i32>;
    fn get_Width(&self) -> windows_core::Result<i32>;
    fn get_Height(&self) -> windows_core::Result<i32>;
    fn get_VerticalOffset(&self) -> windows_core::Result<i32>;
    fn put_VerticalOffset(&self, verticaloffset: i32) -> windows_core::Result<()>;
    fn get_HorizontalOffset(&self) -> windows_core::Result<i32>;
    fn put_HorizontalOffset(&self, horizontaloffset: i32) -> windows_core::Result<()>;
    fn get_AutoShow(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AutoShow(&self, autoshow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MoveTo(&self, left: i32, top: i32) -> windows_core::Result<()>;
    fn CommitPendingInput(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn EnableTsf(&self, enable: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "basetsd", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPenInputPanel_Vtbl {
    pub const fn new<Identity: IPenInputPanel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Busy<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busy: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_Busy(this) {
                    Ok(ok__) => {
                        busy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Factoid<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_Factoid(this) {
                    Ok(ok__) => {
                        factoid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Factoid<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::put_Factoid(this, core::mem::transmute(&factoid)).into()
            }
        }
        unsafe extern "system" fn get_AttachedEditWindow<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut super::LONG32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_AttachedEditWindow(this) {
                    Ok(ok__) => {
                        attachededitwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AttachedEditWindow<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: super::LONG32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::put_AttachedEditWindow(this, core::mem::transmute_copy(&attachededitwindow)).into()
            }
        }
        unsafe extern "system" fn get_CurrentPanel<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentpanel: *mut PanelType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_CurrentPanel(this) {
                    Ok(ok__) => {
                        currentpanel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CurrentPanel<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentpanel: PanelType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::put_CurrentPanel(this, core::mem::transmute_copy(&currentpanel)).into()
            }
        }
        unsafe extern "system" fn get_DefaultPanel<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefaultpanel: *mut PanelType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_DefaultPanel(this) {
                    Ok(ok__) => {
                        pdefaultpanel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DefaultPanel<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaultpanel: PanelType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::put_DefaultPanel(this, core::mem::transmute_copy(&defaultpanel)).into()
            }
        }
        unsafe extern "system" fn get_Visible<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_Visible(this) {
                    Ok(ok__) => {
                        visible.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Visible<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::put_Visible(this, core::mem::transmute_copy(&visible)).into()
            }
        }
        unsafe extern "system" fn get_Top<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_Top(this) {
                    Ok(ok__) => {
                        top.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Left<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_Left(this) {
                    Ok(ok__) => {
                        left.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Width<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_Width(this) {
                    Ok(ok__) => {
                        width.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Height<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_Height(this) {
                    Ok(ok__) => {
                        height.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VerticalOffset<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, verticaloffset: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_VerticalOffset(this) {
                    Ok(ok__) => {
                        verticaloffset.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_VerticalOffset<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, verticaloffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::put_VerticalOffset(this, core::mem::transmute_copy(&verticaloffset)).into()
            }
        }
        unsafe extern "system" fn get_HorizontalOffset<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontaloffset: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_HorizontalOffset(this) {
                    Ok(ok__) => {
                        horizontaloffset.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HorizontalOffset<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontaloffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::put_HorizontalOffset(this, core::mem::transmute_copy(&horizontaloffset)).into()
            }
        }
        unsafe extern "system" fn get_AutoShow<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pautoshow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenInputPanel_Impl::get_AutoShow(this) {
                    Ok(ok__) => {
                        pautoshow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AutoShow<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoshow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::put_AutoShow(this, core::mem::transmute_copy(&autoshow)).into()
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
        unsafe extern "system" fn EnableTsf<Identity: IPenInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPenInputPanel_Impl::EnableTsf(this, core::mem::transmute_copy(&enable)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Busy: get_Busy::<Identity, OFFSET>,
            get_Factoid: get_Factoid::<Identity, OFFSET>,
            put_Factoid: put_Factoid::<Identity, OFFSET>,
            get_AttachedEditWindow: get_AttachedEditWindow::<Identity, OFFSET>,
            put_AttachedEditWindow: put_AttachedEditWindow::<Identity, OFFSET>,
            get_CurrentPanel: get_CurrentPanel::<Identity, OFFSET>,
            put_CurrentPanel: put_CurrentPanel::<Identity, OFFSET>,
            get_DefaultPanel: get_DefaultPanel::<Identity, OFFSET>,
            put_DefaultPanel: put_DefaultPanel::<Identity, OFFSET>,
            get_Visible: get_Visible::<Identity, OFFSET>,
            put_Visible: put_Visible::<Identity, OFFSET>,
            get_Top: get_Top::<Identity, OFFSET>,
            get_Left: get_Left::<Identity, OFFSET>,
            get_Width: get_Width::<Identity, OFFSET>,
            get_Height: get_Height::<Identity, OFFSET>,
            get_VerticalOffset: get_VerticalOffset::<Identity, OFFSET>,
            put_VerticalOffset: put_VerticalOffset::<Identity, OFFSET>,
            get_HorizontalOffset: get_HorizontalOffset::<Identity, OFFSET>,
            put_HorizontalOffset: put_HorizontalOffset::<Identity, OFFSET>,
            get_AutoShow: get_AutoShow::<Identity, OFFSET>,
            put_AutoShow: put_AutoShow::<Identity, OFFSET>,
            MoveTo: MoveTo::<Identity, OFFSET>,
            CommitPendingInput: CommitPendingInput::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            EnableTsf: EnableTsf::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPenInputPanel as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "basetsd", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPenInputPanel {}
windows_core::imp::define_interface!(ITextInputPanel, ITextInputPanel_Vtbl, 0x6b6a65a5_6af3_46c2_b6ea_56cd1f80df71);
windows_core::imp::interface_hierarchy!(ITextInputPanel, windows_core::IUnknown);
impl ITextInputPanel {
    #[cfg(feature = "windef")]
    pub unsafe fn get_AttachedEditWindow(&self) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AttachedEditWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn put_AttachedEditWindow(&self, attachededitwindow: super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AttachedEditWindow)(windows_core::Interface::as_raw(self), attachededitwindow) }
    }
    pub unsafe fn get_CurrentInteractionMode(&self) -> windows_core::Result<InteractionMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentInteractionMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_DefaultInPlaceState(&self) -> windows_core::Result<InPlaceState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultInPlaceState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DefaultInPlaceState(&self, state: InPlaceState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DefaultInPlaceState)(windows_core::Interface::as_raw(self), state) }
    }
    pub unsafe fn get_CurrentInPlaceState(&self) -> windows_core::Result<InPlaceState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentInPlaceState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_DefaultInputArea(&self) -> windows_core::Result<PanelInputArea> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultInputArea)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DefaultInputArea(&self, area: PanelInputArea) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DefaultInputArea)(windows_core::Interface::as_raw(self), area) }
    }
    pub unsafe fn get_CurrentInputArea(&self) -> windows_core::Result<PanelInputArea> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentInputArea)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentCorrectionMode(&self) -> windows_core::Result<CorrectionMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCorrectionMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_PreferredInPlaceDirection(&self) -> windows_core::Result<InPlaceDirection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PreferredInPlaceDirection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_PreferredInPlaceDirection(&self, direction: InPlaceDirection) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PreferredInPlaceDirection)(windows_core::Interface::as_raw(self), direction) }
    }
    pub unsafe fn get_ExpandPostInsertionCorrection(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExpandPostInsertionCorrection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ExpandPostInsertionCorrection(&self, expand: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExpandPostInsertionCorrection)(windows_core::Interface::as_raw(self), expand.into()) }
    }
    pub unsafe fn get_InPlaceVisibleOnFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InPlaceVisibleOnFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_InPlaceVisibleOnFocus(&self, visible: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InPlaceVisibleOnFocus)(windows_core::Interface::as_raw(self), visible.into()) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn get_InPlaceBoundingRectangle(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InPlaceBoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_PopUpCorrectionHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PopUpCorrectionHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_PopDownCorrectionHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PopDownCorrectionHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CommitPendingInput(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitPendingInput)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetInPlaceVisibility(&self, visible: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInPlaceVisibility)(windows_core::Interface::as_raw(self), visible) }
    }
    pub unsafe fn SetInPlacePosition(&self, xposition: i32, yposition: i32, position: CorrectionPosition) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInPlacePosition)(windows_core::Interface::as_raw(self), xposition, yposition, position) }
    }
    pub unsafe fn SetInPlaceHoverTargetPosition(&self, xposition: i32, yposition: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInPlaceHoverTargetPosition)(windows_core::Interface::as_raw(self), xposition, yposition) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Advise<P0>(&self, eventsink: P0, eventmask: super::DWORD) -> windows_core::HRESULT
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
    #[cfg(feature = "windef")]
    pub get_AttachedEditWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_AttachedEditWindow: usize,
    #[cfg(feature = "windef")]
    pub put_AttachedEditWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    put_AttachedEditWindow: usize,
    pub get_CurrentInteractionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionMode) -> windows_core::HRESULT,
    pub get_DefaultInPlaceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InPlaceState) -> windows_core::HRESULT,
    pub put_DefaultInPlaceState: unsafe extern "system" fn(*mut core::ffi::c_void, InPlaceState) -> windows_core::HRESULT,
    pub get_CurrentInPlaceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InPlaceState) -> windows_core::HRESULT,
    pub get_DefaultInputArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PanelInputArea) -> windows_core::HRESULT,
    pub put_DefaultInputArea: unsafe extern "system" fn(*mut core::ffi::c_void, PanelInputArea) -> windows_core::HRESULT,
    pub get_CurrentInputArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PanelInputArea) -> windows_core::HRESULT,
    pub get_CurrentCorrectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CorrectionMode) -> windows_core::HRESULT,
    pub get_PreferredInPlaceDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InPlaceDirection) -> windows_core::HRESULT,
    pub put_PreferredInPlaceDirection: unsafe extern "system" fn(*mut core::ffi::c_void, InPlaceDirection) -> windows_core::HRESULT,
    pub get_ExpandPostInsertionCorrection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_ExpandPostInsertionCorrection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub get_InPlaceVisibleOnFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_InPlaceVisibleOnFocus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub get_InPlaceBoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    get_InPlaceBoundingRectangle: usize,
    pub get_PopUpCorrectionHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_PopDownCorrectionHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CommitPendingInput: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInPlaceVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetInPlacePosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, CorrectionPosition) -> windows_core::HRESULT,
    pub SetInPlaceHoverTargetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait ITextInputPanel_Impl: windows_core::IUnknownImpl {
    fn get_AttachedEditWindow(&self) -> windows_core::Result<super::HWND>;
    fn put_AttachedEditWindow(&self, attachededitwindow: super::HWND) -> windows_core::Result<()>;
    fn get_CurrentInteractionMode(&self) -> windows_core::Result<InteractionMode>;
    fn get_DefaultInPlaceState(&self) -> windows_core::Result<InPlaceState>;
    fn put_DefaultInPlaceState(&self, state: InPlaceState) -> windows_core::Result<()>;
    fn get_CurrentInPlaceState(&self) -> windows_core::Result<InPlaceState>;
    fn get_DefaultInputArea(&self) -> windows_core::Result<PanelInputArea>;
    fn put_DefaultInputArea(&self, area: PanelInputArea) -> windows_core::Result<()>;
    fn get_CurrentInputArea(&self) -> windows_core::Result<PanelInputArea>;
    fn get_CurrentCorrectionMode(&self) -> windows_core::Result<CorrectionMode>;
    fn get_PreferredInPlaceDirection(&self) -> windows_core::Result<InPlaceDirection>;
    fn put_PreferredInPlaceDirection(&self, direction: InPlaceDirection) -> windows_core::Result<()>;
    fn get_ExpandPostInsertionCorrection(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_ExpandPostInsertionCorrection(&self, expand: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_InPlaceVisibleOnFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_InPlaceVisibleOnFocus(&self, visible: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_InPlaceBoundingRectangle(&self) -> windows_core::Result<super::RECT>;
    fn get_PopUpCorrectionHeight(&self) -> windows_core::Result<i32>;
    fn get_PopDownCorrectionHeight(&self) -> windows_core::Result<i32>;
    fn CommitPendingInput(&self) -> windows_core::Result<()>;
    fn SetInPlaceVisibility(&self, visible: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetInPlacePosition(&self, xposition: i32, yposition: i32, position: CorrectionPosition) -> windows_core::Result<()>;
    fn SetInPlaceHoverTargetPosition(&self, xposition: i32, yposition: i32) -> windows_core::Result<()>;
    fn Advise(&self, eventsink: windows_core::Ref<ITextInputPanelEventSink>, eventmask: super::DWORD) -> windows_core::Result<()>;
    fn Unadvise(&self, eventsink: windows_core::Ref<ITextInputPanelEventSink>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl ITextInputPanel_Vtbl {
    pub const fn new<Identity: ITextInputPanel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AttachedEditWindow<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_AttachedEditWindow(this) {
                    Ok(ok__) => {
                        attachededitwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AttachedEditWindow<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachededitwindow: super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::put_AttachedEditWindow(this, core::mem::transmute_copy(&attachededitwindow)).into()
            }
        }
        unsafe extern "system" fn get_CurrentInteractionMode<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentinteractionmode: *mut InteractionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_CurrentInteractionMode(this) {
                    Ok(ok__) => {
                        currentinteractionmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DefaultInPlaceState<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut InPlaceState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_DefaultInPlaceState(this) {
                    Ok(ok__) => {
                        state.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DefaultInPlaceState<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: InPlaceState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::put_DefaultInPlaceState(this, core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn get_CurrentInPlaceState<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut InPlaceState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_CurrentInPlaceState(this) {
                    Ok(ok__) => {
                        state.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DefaultInputArea<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, area: *mut PanelInputArea) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_DefaultInputArea(this) {
                    Ok(ok__) => {
                        area.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DefaultInputArea<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, area: PanelInputArea) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::put_DefaultInputArea(this, core::mem::transmute_copy(&area)).into()
            }
        }
        unsafe extern "system" fn get_CurrentInputArea<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, area: *mut PanelInputArea) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_CurrentInputArea(this) {
                    Ok(ok__) => {
                        area.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentCorrectionMode<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut CorrectionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_CurrentCorrectionMode(this) {
                    Ok(ok__) => {
                        mode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PreferredInPlaceDirection<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: *mut InPlaceDirection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_PreferredInPlaceDirection(this) {
                    Ok(ok__) => {
                        direction.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PreferredInPlaceDirection<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: InPlaceDirection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::put_PreferredInPlaceDirection(this, core::mem::transmute_copy(&direction)).into()
            }
        }
        unsafe extern "system" fn get_ExpandPostInsertionCorrection<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expand: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_ExpandPostInsertionCorrection(this) {
                    Ok(ok__) => {
                        expand.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExpandPostInsertionCorrection<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expand: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::put_ExpandPostInsertionCorrection(this, core::mem::transmute_copy(&expand)).into()
            }
        }
        unsafe extern "system" fn get_InPlaceVisibleOnFocus<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_InPlaceVisibleOnFocus(this) {
                    Ok(ok__) => {
                        visible.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InPlaceVisibleOnFocus<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanel_Impl::put_InPlaceVisibleOnFocus(this, core::mem::transmute_copy(&visible)).into()
            }
        }
        unsafe extern "system" fn get_InPlaceBoundingRectangle<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundingrectangle: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_InPlaceBoundingRectangle(this) {
                    Ok(ok__) => {
                        boundingrectangle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PopUpCorrectionHeight<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_PopUpCorrectionHeight(this) {
                    Ok(ok__) => {
                        height.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PopDownCorrectionHeight<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextInputPanel_Impl::get_PopDownCorrectionHeight(this) {
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
        unsafe extern "system" fn Advise<Identity: ITextInputPanel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventsink: *mut core::ffi::c_void, eventmask: super::DWORD) -> windows_core::HRESULT {
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
            get_AttachedEditWindow: get_AttachedEditWindow::<Identity, OFFSET>,
            put_AttachedEditWindow: put_AttachedEditWindow::<Identity, OFFSET>,
            get_CurrentInteractionMode: get_CurrentInteractionMode::<Identity, OFFSET>,
            get_DefaultInPlaceState: get_DefaultInPlaceState::<Identity, OFFSET>,
            put_DefaultInPlaceState: put_DefaultInPlaceState::<Identity, OFFSET>,
            get_CurrentInPlaceState: get_CurrentInPlaceState::<Identity, OFFSET>,
            get_DefaultInputArea: get_DefaultInputArea::<Identity, OFFSET>,
            put_DefaultInputArea: put_DefaultInputArea::<Identity, OFFSET>,
            get_CurrentInputArea: get_CurrentInputArea::<Identity, OFFSET>,
            get_CurrentCorrectionMode: get_CurrentCorrectionMode::<Identity, OFFSET>,
            get_PreferredInPlaceDirection: get_PreferredInPlaceDirection::<Identity, OFFSET>,
            put_PreferredInPlaceDirection: put_PreferredInPlaceDirection::<Identity, OFFSET>,
            get_ExpandPostInsertionCorrection: get_ExpandPostInsertionCorrection::<Identity, OFFSET>,
            put_ExpandPostInsertionCorrection: put_ExpandPostInsertionCorrection::<Identity, OFFSET>,
            get_InPlaceVisibleOnFocus: get_InPlaceVisibleOnFocus::<Identity, OFFSET>,
            put_InPlaceVisibleOnFocus: put_InPlaceVisibleOnFocus::<Identity, OFFSET>,
            get_InPlaceBoundingRectangle: get_InPlaceBoundingRectangle::<Identity, OFFSET>,
            get_PopUpCorrectionHeight: get_PopUpCorrectionHeight::<Identity, OFFSET>,
            get_PopDownCorrectionHeight: get_PopDownCorrectionHeight::<Identity, OFFSET>,
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
#[cfg(all(feature = "shared", feature = "windef"))]
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
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn InPlaceSizeChanging(&self, oldboundingrectangle: super::RECT, newboundingrectangle: super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceSizeChanging)(windows_core::Interface::as_raw(self), oldboundingrectangle, newboundingrectangle) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn InPlaceSizeChanged(&self, oldboundingrectangle: super::RECT, newboundingrectangle: super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceSizeChanged)(windows_core::Interface::as_raw(self), oldboundingrectangle, newboundingrectangle) }
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn TextInserting(&self, ink: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TextInserting)(windows_core::Interface::as_raw(self), ink) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn TextInserted(&self, ink: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TextInserted)(windows_core::Interface::as_raw(self), ink) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanelEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InPlaceStateChanging: unsafe extern "system" fn(*mut core::ffi::c_void, InPlaceState, InPlaceState) -> windows_core::HRESULT,
    pub InPlaceStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, InPlaceState, InPlaceState) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub InPlaceSizeChanging: unsafe extern "system" fn(*mut core::ffi::c_void, super::RECT, super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    InPlaceSizeChanging: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub InPlaceSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::RECT, super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    InPlaceSizeChanged: usize,
    pub InputAreaChanging: unsafe extern "system" fn(*mut core::ffi::c_void, PanelInputArea, PanelInputArea) -> windows_core::HRESULT,
    pub InputAreaChanged: unsafe extern "system" fn(*mut core::ffi::c_void, PanelInputArea, PanelInputArea) -> windows_core::HRESULT,
    pub CorrectionModeChanging: unsafe extern "system" fn(*mut core::ffi::c_void, CorrectionMode, CorrectionMode) -> windows_core::HRESULT,
    pub CorrectionModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, CorrectionMode, CorrectionMode) -> windows_core::HRESULT,
    pub InPlaceVisibilityChanging: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub InPlaceVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub TextInserting: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    TextInserting: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub TextInserted: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    TextInserted: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ITextInputPanelEventSink_Impl: windows_core::IUnknownImpl {
    fn InPlaceStateChanging(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::Result<()>;
    fn InPlaceStateChanged(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> windows_core::Result<()>;
    fn InPlaceSizeChanging(&self, oldboundingrectangle: &super::RECT, newboundingrectangle: &super::RECT) -> windows_core::Result<()>;
    fn InPlaceSizeChanged(&self, oldboundingrectangle: &super::RECT, newboundingrectangle: &super::RECT) -> windows_core::Result<()>;
    fn InputAreaChanging(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::Result<()>;
    fn InputAreaChanged(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> windows_core::Result<()>;
    fn CorrectionModeChanging(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::Result<()>;
    fn CorrectionModeChanged(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> windows_core::Result<()>;
    fn InPlaceVisibilityChanging(&self, oldvisible: windows_core::BOOL, newvisible: windows_core::BOOL) -> windows_core::Result<()>;
    fn InPlaceVisibilityChanged(&self, oldvisible: windows_core::BOOL, newvisible: windows_core::BOOL) -> windows_core::Result<()>;
    fn TextInserting(&self, ink: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn TextInserted(&self, ink: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt"))]
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
        unsafe extern "system" fn InPlaceSizeChanging<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldboundingrectangle: super::RECT, newboundingrectangle: super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::InPlaceSizeChanging(this, core::mem::transmute(&oldboundingrectangle), core::mem::transmute(&newboundingrectangle)).into()
            }
        }
        unsafe extern "system" fn InPlaceSizeChanged<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldboundingrectangle: super::RECT, newboundingrectangle: super::RECT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn TextInserting<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextInputPanelEventSink_Impl::TextInserting(this, core::mem::transmute_copy(&ink)).into()
            }
        }
        unsafe extern "system" fn TextInserted<Identity: ITextInputPanelEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *const super::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt"))]
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
pub const MICROSOFT_TIP_COMBOBOXLIST_PROPERTY: windows_core::PCWSTR = windows_core::w!("Microsoft TIP ComboBox List Window Identifier");
pub const MICROSOFT_TIP_NO_INSERT_BUTTON_PROPERTY: windows_core::PCWSTR = windows_core::w!("Microsoft TIP No Insert Option");
pub const MICROSOFT_TIP_OPENING_MSG: windows_core::PCWSTR = windows_core::w!("TabletInputPanelOpening");
pub const MICROSOFT_URL_EXPERIENCE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Microsoft TIP URL Experience");
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
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(_IPenInputPanelEvents, _IPenInputPanelEvents_Vtbl, 0xb7e489da_3719_439f_848f_e7acbd820f17);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for _IPenInputPanelEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(_IPenInputPanelEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct _IPenInputPanelEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait _IPenInputPanelEvents_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl _IPenInputPanelEvents_Vtbl {
    pub const fn new<Identity: _IPenInputPanelEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IPenInputPanelEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for _IPenInputPanelEvents {}
