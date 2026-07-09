pub const GUID_TS_SERVICE_ACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0xf9786200_a5bf_4a0f_8c24_fb16f5d1aabb);
pub const GUID_TS_SERVICE_ACTIVEX: windows_core::GUID = windows_core::GUID::from_u128(0xea937a50_c9a6_4b7d_894a_49d99b784834);
pub const GUID_TS_SERVICE_DATAOBJECT: windows_core::GUID = windows_core::GUID::from_u128(0x6086fbb5_e225_46ce_a770_c1bbd3e05d7b);
pub const GXFPF_NEAREST: u32 = 2;
pub const GXFPF_ROUND_NEAREST: u32 = 1;
windows_core::imp::define_interface!(IAnchor, IAnchor_Vtbl, 0x0feb7e34_5a60_4356_8ef7_abdec2ff7cf8);
windows_core::imp::interface_hierarchy!(IAnchor, windows_core::IUnknown);
impl IAnchor {
    pub unsafe fn SetGravity(&self, gravity: TsGravity) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGravity)(windows_core::Interface::as_raw(self), gravity) }
    }
    pub unsafe fn GetGravity(&self) -> windows_core::Result<TsGravity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGravity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual<P0>(&self, pawith: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pawith.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Compare<P0>(&self, pawith: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), pawith.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Shift<P3>(&self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Shift)(windows_core::Interface::as_raw(self), dwflags, cchreq, pcch as _, pahaltanchor.param().abi()) }
    }
    pub unsafe fn ShiftTo<P0>(&self, pasite: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).ShiftTo)(windows_core::Interface::as_raw(self), pasite.param().abi()) }
    }
    pub unsafe fn ShiftRegion(&self, dwflags: u32, dir: TsShiftDir) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShiftRegion)(windows_core::Interface::as_raw(self), dwflags, dir, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChangeHistoryMask(&self, dwmask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChangeHistoryMask)(windows_core::Interface::as_raw(self), dwmask) }
    }
    pub unsafe fn GetChangeHistory(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChangeHistory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ClearChangeHistory(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearChangeHistory)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnchor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, TsGravity) -> windows_core::HRESULT,
    pub GetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TsGravity) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Shift: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *mut i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShiftTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShiftRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, TsShiftDir, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetChangeHistoryMask: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetChangeHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ClearChangeHistory: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAnchor_Impl: windows_core::IUnknownImpl {
    fn SetGravity(&self, gravity: TsGravity) -> windows_core::Result<()>;
    fn GetGravity(&self) -> windows_core::Result<TsGravity>;
    fn IsEqual(&self, pawith: windows_core::Ref<IAnchor>) -> windows_core::Result<windows_core::BOOL>;
    fn Compare(&self, pawith: windows_core::Ref<IAnchor>) -> windows_core::Result<i32>;
    fn Shift(&self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: windows_core::Ref<IAnchor>) -> windows_core::Result<()>;
    fn ShiftTo(&self, pasite: windows_core::Ref<IAnchor>) -> windows_core::Result<()>;
    fn ShiftRegion(&self, dwflags: u32, dir: TsShiftDir) -> windows_core::Result<windows_core::BOOL>;
    fn SetChangeHistoryMask(&self, dwmask: u32) -> windows_core::Result<()>;
    fn GetChangeHistory(&self) -> windows_core::Result<u32>;
    fn ClearChangeHistory(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IAnchor>;
}
impl IAnchor_Vtbl {
    pub const fn new<Identity: IAnchor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGravity<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gravity: TsGravity) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::SetGravity(this, core::mem::transmute_copy(&gravity)).into()
            }
        }
        unsafe extern "system" fn GetGravity<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgravity: *mut TsGravity) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnchor_Impl::GetGravity(this) {
                    Ok(ok__) => {
                        pgravity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pawith: *mut core::ffi::c_void, pfequal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnchor_Impl::IsEqual(this, core::mem::transmute_copy(&pawith)) {
                    Ok(ok__) => {
                        pfequal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Compare<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pawith: *mut core::ffi::c_void, plresult: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnchor_Impl::Compare(this, core::mem::transmute_copy(&pawith)) {
                    Ok(ok__) => {
                        plresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shift<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::Shift(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&pahaltanchor)).into()
            }
        }
        unsafe extern "system" fn ShiftTo<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::ShiftTo(this, core::mem::transmute_copy(&pasite)).into()
            }
        }
        unsafe extern "system" fn ShiftRegion<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnchor_Impl::ShiftRegion(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dir)) {
                    Ok(ok__) => {
                        pfnoregion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChangeHistoryMask<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::SetChangeHistoryMask(this, core::mem::transmute_copy(&dwmask)).into()
            }
        }
        unsafe extern "system" fn GetChangeHistory<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhistory: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnchor_Impl::GetChangeHistory(this) {
                    Ok(ok__) => {
                        pdwhistory.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClearChangeHistory<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::ClearChangeHistory(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaclone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnchor_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppaclone.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGravity: SetGravity::<Identity, OFFSET>,
            GetGravity: GetGravity::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Compare: Compare::<Identity, OFFSET>,
            Shift: Shift::<Identity, OFFSET>,
            ShiftTo: ShiftTo::<Identity, OFFSET>,
            ShiftRegion: ShiftRegion::<Identity, OFFSET>,
            SetChangeHistoryMask: SetChangeHistoryMask::<Identity, OFFSET>,
            GetChangeHistory: GetChangeHistory::<Identity, OFFSET>,
            ClearChangeHistory: ClearChangeHistory::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAnchor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAnchor {}
windows_core::imp::define_interface!(ITextStoreACP, ITextStoreACP_Vtbl, 0x28888fe3_c2a0_483a_a3ea_8cb1ce51ff3d);
windows_core::imp::interface_hierarchy!(ITextStoreACP, windows_core::IUnknown);
impl ITextStoreACP {
    pub unsafe fn AdviseSink<P1>(&self, riid: *const windows_core::GUID, punk: P1, dwmask: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AdviseSink)(windows_core::Interface::as_raw(self), riid, punk.param().abi(), dwmask) }
    }
    pub unsafe fn UnadviseSink<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnadviseSink)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestLock)(windows_core::Interface::as_raw(self), dwlockflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TS_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInsert)(windows_core::Interface::as_raw(self), acpteststart, acptestend, cch, pacpresultstart as _, pacpresultend as _) }
    }
    pub unsafe fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ulindex, ulcount, pselection as _, pcfetched as _) }
    }
    pub unsafe fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), ulcount, pselection) }
    }
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: *mut u16, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), acpstart, acpend, pchplain as _, cchplainreq, pcchplainret as _, prgruninfo as _, cruninforeq, pcruninforet as _, pacpnext as _) }
    }
    pub unsafe fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: *const u16, cch: u32) -> windows_core::Result<TS_TEXTCHANGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pchtext, cch, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidl")]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::objidl::IDataObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEmbedded<T>(&self, acppos: i32, rguidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), acppos, rguidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "Win32_objidl", feature = "Win32_wtypes"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidl")]
    pub unsafe fn InsertEmbedded<P3>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: P3) -> windows_core::Result<TS_TEXTCHANGE>
    where
        P3: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pdataobject.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertTextAtSelection(&self, dwflags: u32, pchtext: *const u16, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), dwflags, pchtext, cch, pacpstart as _, pacpend as _, pchange as _) }
    }
    #[cfg(feature = "Win32_objidl")]
    pub unsafe fn InsertEmbeddedAtSelection<P1>(&self, dwflags: u32, pdataobject: P1, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), dwflags, pdataobject.param().abi(), pacpstart as _, pacpend as _, pchange as _) }
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestSupportedAttrs)(windows_core::Interface::as_raw(self), dwflags, cfilterattrs, pafilterattrs) }
    }
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsAtPosition)(windows_core::Interface::as_raw(self), acppos, cfilterattrs, pafilterattrs, dwflags) }
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(windows_core::Interface::as_raw(self), acppos, cfilterattrs, pafilterattrs, dwflags) }
    }
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32, pacpnext: *mut i32, pffound: *mut windows_core::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindNextAttrTransition)(windows_core::Interface::as_raw(self), acpstart, acphalt, cfilterattrs, pafilterattrs, dwflags, pacpnext as _, pffound as _, plfoundoffset as _) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RetrieveRequestedAttrs)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(paattrvals), pcfetched as _) }
    }
    pub unsafe fn GetEndACP(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndACP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetActiveView(&self) -> windows_core::Result<TsViewCookie> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetACPFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetACPFromPoint)(windows_core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetTextExt(&self, vcview: TsViewCookie, acpstart: i32, acpend: i32, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), vcview, acpstart, acpend, prc as _, pfclipped as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetScreenExt(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetWnd(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut TS_SELECTION_ACP, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const TS_SELECTION_ACP) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut u16, u32, *mut u32, *mut TS_RUNINFO, u32, *mut u32, *mut i32) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, *const u16, u32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidl")]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidl"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_objidl", feature = "Win32_wtypes"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::objidl::FORMATETC, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_objidl", feature = "Win32_wtypes")))]
    QueryInsertEmbedded: usize,
    #[cfg(feature = "Win32_objidl")]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, *mut core::ffi::c_void, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidl"))]
    InsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, u32, *mut i32, *mut i32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidl")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut i32, *mut i32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidl"))]
    InsertEmbeddedAtSelection: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const TS_ATTRID) -> windows_core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const TS_ATTRID, u32) -> windows_core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const TS_ATTRID, u32) -> windows_core::HRESULT,
    pub FindNextAttrTransition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *const TS_ATTRID, u32, *mut i32, *mut windows_core::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TS_ATTRVAL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    RetrieveRequestedAttrs: usize,
    pub GetEndACP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TsViewCookie) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetACPFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *const super::windef::POINT, u32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetACPFromPoint: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, i32, i32, *mut super::windef::RECT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetScreenExt: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetWnd: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_windef", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextStoreACP_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<windows_core::IUnknown>, dwmask: u32) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::Result<()>;
    fn GetText(&self, acpstart: i32, acpend: i32, pchplain: *mut u16, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: *const u16, cch: u32) -> windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::objidl::IDataObject>;
    fn GetEmbedded(&self, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC) -> windows_core::Result<windows_core::BOOL>;
    fn InsertEmbedded(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: *const u16, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: windows_core::Ref<super::objidl::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32, pacpnext: *mut i32, pffound: *mut windows_core::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn GetEndACP(&self) -> windows_core::Result<i32>;
    fn GetActiveView(&self) -> windows_core::Result<TsViewCookie>;
    fn GetACPFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<i32>;
    fn GetTextExt(&self, vcview: TsViewCookie, acpstart: i32, acpend: i32, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::RECT>;
    fn GetWnd(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::HWND>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_windef", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextStoreACP_Vtbl {
    pub const fn new<Identity: ITextStoreACP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&dwmask)).into()
            }
        }
        unsafe extern "system" fn UnadviseSink<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::UnadviseSink(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn RequestLock<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags)) {
                    Ok(ok__) => {
                        phrsession.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdcs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::QueryInsert(this, core::mem::transmute_copy(&acpteststart), core::mem::transmute_copy(&acptestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpresultstart), core::mem::transmute_copy(&pacpresultend)).into()
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: *mut u16, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchplain), core::mem::transmute_copy(&cchplainreq), core::mem::transmute_copy(&pcchplainret), core::mem::transmute_copy(&prgruninfo), core::mem::transmute_copy(&cruninforeq), core::mem::transmute_copy(&pcruninforet), core::mem::transmute_copy(&pacpnext)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: *const u16, cch: u32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::SetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch)) {
                    Ok(ok__) => {
                        pchange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::GetFormattedText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                    Ok(ok__) => {
                        ppdataobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetEmbedded(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                    Ok(ok__) => {
                        pfinsertable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pdataobject)) {
                    Ok(ok__) => {
                        pchange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchtext: *const u16, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdataobject: *mut core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32, pacpnext: *mut i32, pffound: *mut windows_core::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acphalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacpnext), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
            }
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn GetEndACP<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacp: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::GetEndACP(this) {
                    Ok(ok__) => {
                        pacp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut TsViewCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::GetActiveView(this) {
                    Ok(ok__) => {
                        pvcview.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, ptscreen: *const super::windef::POINT, dwflags: u32, pacp: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        pacp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, acpstart: i32, acpend: i32, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
            }
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview)) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP_Impl::GetWnd(this, core::mem::transmute_copy(&vcview)) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
            RequestLock: RequestLock::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            QueryInsert: QueryInsert::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, OFFSET>,
            GetEndACP: GetEndACP::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACP as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_windef", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextStoreACP {}
windows_core::imp::define_interface!(ITextStoreACP2, ITextStoreACP2_Vtbl, 0xf86ad89f_5fe4_4b8d_bb9f_ef3797a84f1f);
windows_core::imp::interface_hierarchy!(ITextStoreACP2, windows_core::IUnknown);
impl ITextStoreACP2 {
    pub unsafe fn AdviseSink<P1>(&self, riid: *const windows_core::GUID, punk: P1, dwmask: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AdviseSink)(windows_core::Interface::as_raw(self), riid, punk.param().abi(), dwmask) }
    }
    pub unsafe fn UnadviseSink<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnadviseSink)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestLock)(windows_core::Interface::as_raw(self), dwlockflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TS_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInsert)(windows_core::Interface::as_raw(self), acpteststart, acptestend, cch, pacpresultstart as _, pacpresultend as _) }
    }
    pub unsafe fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ulindex, ulcount, pselection as _, pcfetched as _) }
    }
    pub unsafe fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), ulcount, pselection) }
    }
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: *mut u16, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), acpstart, acpend, pchplain as _, cchplainreq, pcchplainret as _, prgruninfo as _, cruninforeq, pcruninforet as _, pacpnext as _) }
    }
    pub unsafe fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: *const u16, cch: u32) -> windows_core::Result<TS_TEXTCHANGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pchtext, cch, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidl")]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::objidl::IDataObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEmbedded<T>(&self, acppos: i32, rguidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), acppos, rguidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "Win32_objidl", feature = "Win32_wtypes"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidl")]
    pub unsafe fn InsertEmbedded<P3>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: P3) -> windows_core::Result<TS_TEXTCHANGE>
    where
        P3: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pdataobject.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertTextAtSelection(&self, dwflags: u32, pchtext: *const u16, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), dwflags, pchtext, cch, pacpstart as _, pacpend as _, pchange as _) }
    }
    #[cfg(feature = "Win32_objidl")]
    pub unsafe fn InsertEmbeddedAtSelection<P1>(&self, dwflags: u32, pdataobject: P1, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), dwflags, pdataobject.param().abi(), pacpstart as _, pacpend as _, pchange as _) }
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestSupportedAttrs)(windows_core::Interface::as_raw(self), dwflags, cfilterattrs, pafilterattrs) }
    }
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsAtPosition)(windows_core::Interface::as_raw(self), acppos, cfilterattrs, pafilterattrs, dwflags) }
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(windows_core::Interface::as_raw(self), acppos, cfilterattrs, pafilterattrs, dwflags) }
    }
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32, pacpnext: *mut i32, pffound: *mut windows_core::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindNextAttrTransition)(windows_core::Interface::as_raw(self), acpstart, acphalt, cfilterattrs, pafilterattrs, dwflags, pacpnext as _, pffound as _, plfoundoffset as _) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RetrieveRequestedAttrs)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(paattrvals), pcfetched as _) }
    }
    pub unsafe fn GetEndACP(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndACP)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetActiveView(&self) -> windows_core::Result<TsViewCookie> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetACPFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetACPFromPoint)(windows_core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetTextExt(&self, vcview: TsViewCookie, acpstart: i32, acpend: i32, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), vcview, acpstart, acpend, prc as _, pfclipped as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetScreenExt(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACP2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut TS_SELECTION_ACP, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const TS_SELECTION_ACP) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut u16, u32, *mut u32, *mut TS_RUNINFO, u32, *mut u32, *mut i32) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, *const u16, u32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidl")]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidl"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_objidl", feature = "Win32_wtypes"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::objidl::FORMATETC, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_objidl", feature = "Win32_wtypes")))]
    QueryInsertEmbedded: usize,
    #[cfg(feature = "Win32_objidl")]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, i32, *mut core::ffi::c_void, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidl"))]
    InsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, u32, *mut i32, *mut i32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidl")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut i32, *mut i32, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidl"))]
    InsertEmbeddedAtSelection: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const TS_ATTRID) -> windows_core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const TS_ATTRID, u32) -> windows_core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *const TS_ATTRID, u32) -> windows_core::HRESULT,
    pub FindNextAttrTransition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *const TS_ATTRID, u32, *mut i32, *mut windows_core::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TS_ATTRVAL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    RetrieveRequestedAttrs: usize,
    pub GetEndACP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TsViewCookie) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetACPFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *const super::windef::POINT, u32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetACPFromPoint: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, i32, i32, *mut super::windef::RECT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetScreenExt: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_windef", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextStoreACP2_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<windows_core::IUnknown>, dwmask: u32) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::Result<()>;
    fn GetText(&self, acpstart: i32, acpend: i32, pchplain: *mut u16, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: *const u16, cch: u32) -> windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> windows_core::Result<super::objidl::IDataObject>;
    fn GetEmbedded(&self, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC) -> windows_core::Result<windows_core::BOOL>;
    fn InsertEmbedded(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: *const u16, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: windows_core::Ref<super::objidl::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32, pacpnext: *mut i32, pffound: *mut windows_core::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn GetEndACP(&self) -> windows_core::Result<i32>;
    fn GetActiveView(&self) -> windows_core::Result<TsViewCookie>;
    fn GetACPFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<i32>;
    fn GetTextExt(&self, vcview: TsViewCookie, acpstart: i32, acpend: i32, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::RECT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_windef", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextStoreACP2_Vtbl {
    pub const fn new<Identity: ITextStoreACP2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&dwmask)).into()
            }
        }
        unsafe extern "system" fn UnadviseSink<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::UnadviseSink(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn RequestLock<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags)) {
                    Ok(ok__) => {
                        phrsession.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdcs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::QueryInsert(this, core::mem::transmute_copy(&acpteststart), core::mem::transmute_copy(&acptestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpresultstart), core::mem::transmute_copy(&pacpresultend)).into()
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: *mut u16, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchplain), core::mem::transmute_copy(&cchplainreq), core::mem::transmute_copy(&pcchplainret), core::mem::transmute_copy(&prgruninfo), core::mem::transmute_copy(&cruninforeq), core::mem::transmute_copy(&pcruninforet), core::mem::transmute_copy(&pacpnext)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: *const u16, cch: u32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::SetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch)) {
                    Ok(ok__) => {
                        pchange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::GetFormattedText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                    Ok(ok__) => {
                        ppdataobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetEmbedded(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                    Ok(ok__) => {
                        pfinsertable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pdataobject)) {
                    Ok(ok__) => {
                        pchange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchtext: *const u16, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdataobject: *mut core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32, pacpnext: *mut i32, pffound: *mut windows_core::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acphalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacpnext), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
            }
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn GetEndACP<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacp: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::GetEndACP(this) {
                    Ok(ok__) => {
                        pacp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut TsViewCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::GetActiveView(this) {
                    Ok(ok__) => {
                        pvcview.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, ptscreen: *const super::windef::POINT, dwflags: u32, pacp: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        pacp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, acpstart: i32, acpend: i32, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
            }
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACP2_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview)) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
            RequestLock: RequestLock::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            QueryInsert: QueryInsert::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, OFFSET>,
            GetEndACP: GetEndACP::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACP2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_windef", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextStoreACP2 {}
windows_core::imp::define_interface!(ITextStoreACPSink, ITextStoreACPSink_Vtbl, 0x22d44c94_a419_4542_a272_ae26093ececf);
windows_core::imp::interface_hierarchy!(ITextStoreACPSink, windows_core::IUnknown);
impl ITextStoreACPSink {
    pub unsafe fn OnTextChange(&self, dwflags: u32, pchange: *const TS_TEXTCHANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTextChange)(windows_core::Interface::as_raw(self), dwflags, pchange) }
    }
    pub unsafe fn OnSelectionChange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSelectionChange)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: TsViewCookie) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLayoutChange)(windows_core::Interface::as_raw(self), lcode, vcview) }
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn OnAttrsChange(&self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const TS_ATTRID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnAttrsChange)(windows_core::Interface::as_raw(self), acpstart, acpend, cattrs, paattrs) }
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLockGranted)(windows_core::Interface::as_raw(self), dwlockflags) }
    }
    pub unsafe fn OnStartEditTransaction(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStartEditTransaction)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnEndEditTransaction(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEndEditTransaction)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTextChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const TS_TEXTCHANGE) -> windows_core::HRESULT,
    pub OnSelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void, TsLayoutCode, TsViewCookie) -> windows_core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnAttrsChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, *const TS_ATTRID) -> windows_core::HRESULT,
    pub OnLockGranted: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnStartEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITextStoreACPSink_Impl: windows_core::IUnknownImpl {
    fn OnTextChange(&self, dwflags: u32, pchange: *const TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn OnSelectionChange(&self) -> windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: TsViewCookie) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OnAttrsChange(&self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: u32) -> windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> windows_core::Result<()>;
}
impl ITextStoreACPSink_Vtbl {
    pub const fn new<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTextChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchange: *const TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnTextChange(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnSelectionChange(this).into()
            }
        }
        unsafe extern "system" fn OnLayoutChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcode: TsLayoutCode, vcview: TsViewCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnLayoutChange(this, core::mem::transmute_copy(&lcode), core::mem::transmute_copy(&vcview)).into()
            }
        }
        unsafe extern "system" fn OnStatusChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnAttrsChange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&cattrs), core::mem::transmute_copy(&paattrs)).into()
            }
        }
        unsafe extern "system" fn OnLockGranted<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnLockGranted(this, core::mem::transmute_copy(&dwlockflags)).into()
            }
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnStartEditTransaction(this).into()
            }
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnEndEditTransaction(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTextChange: OnTextChange::<Identity, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextStoreACPSink {}
windows_core::imp::define_interface!(ITextStoreAnchor, ITextStoreAnchor_Vtbl, 0x9b2077b0_5f18_4dec_bee9_3cc722f5dfe0);
windows_core::imp::interface_hierarchy!(ITextStoreAnchor, windows_core::IUnknown);
impl ITextStoreAnchor {
    pub unsafe fn AdviseSink<P1>(&self, riid: *const windows_core::GUID, punk: P1, dwmask: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AdviseSink)(windows_core::Interface::as_raw(self), riid, punk.param().abi(), dwmask) }
    }
    pub unsafe fn UnadviseSink<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnadviseSink)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestLock)(windows_core::Interface::as_raw(self), dwlockflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TS_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueryInsert<P0, P1>(&self, pateststart: P0, patestend: P1, cch: u32, pparesultstart: *mut Option<IAnchor>, pparesultend: *mut Option<IAnchor>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryInsert)(windows_core::Interface::as_raw(self), pateststart.param().abi(), patestend.param().abi(), cch, core::mem::transmute(pparesultstart), core::mem::transmute(pparesultend)) }
    }
    pub unsafe fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ulindex, ulcount, core::mem::transmute(pselection), pcfetched as _) }
    }
    pub unsafe fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(pselection)) }
    }
    pub unsafe fn GetText<P1, P2>(&self, dwflags: u32, pastart: P1, paend: P2, pchtext: *mut u16, cchreq: u32, pcch: *mut u32, fupdateanchor: bool) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi(), pchtext as _, cchreq, pcch as _, fupdateanchor.into()) }
    }
    pub unsafe fn SetText<P1, P2>(&self, dwflags: u32, pastart: P1, paend: P2, pchtext: *const u16, cch: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi(), pchtext, cch) }
    }
    #[cfg(feature = "Win32_objidl")]
    pub unsafe fn GetFormattedText<P0, P1>(&self, pastart: P0, paend: P1) -> windows_core::Result<super::objidl::IDataObject>
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), pastart.param().abi(), paend.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEmbedded<P1, T>(&self, dwflags: u32, papos: P1, rguidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        P1: windows_core::Param<IAnchor>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), dwflags, papos.param().abi(), rguidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_objidl")]
    pub unsafe fn InsertEmbedded<P1, P2, P3>(&self, dwflags: u32, pastart: P1, paend: P2, pdataobject: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
        P3: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi(), pdataobject.param().abi()) }
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestSupportedAttrs)(windows_core::Interface::as_raw(self), dwflags, cfilterattrs, pafilterattrs) }
    }
    pub unsafe fn RequestAttrsAtPosition<P0>(&self, papos: P0, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsAtPosition)(windows_core::Interface::as_raw(self), papos.param().abi(), cfilterattrs, pafilterattrs, dwflags) }
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition<P0>(&self, papos: P0, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(windows_core::Interface::as_raw(self), papos.param().abi(), cfilterattrs, pafilterattrs, dwflags) }
    }
    pub unsafe fn FindNextAttrTransition<P0, P1>(&self, pastart: P0, pahalt: P1, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32, pffound: *mut windows_core::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindNextAttrTransition)(windows_core::Interface::as_raw(self), pastart.param().abi(), pahalt.param().abi(), cfilterattrs, pafilterattrs, dwflags, pffound as _, plfoundoffset as _) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RetrieveRequestedAttrs)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(paattrvals), pcfetched as _) }
    }
    pub unsafe fn GetStart(&self) -> windows_core::Result<IAnchor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStart)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEnd(&self) -> windows_core::Result<IAnchor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnd)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetActiveView(&self) -> windows_core::Result<TsViewCookie> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetAnchorFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<IAnchor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAnchorFromPoint)(windows_core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetTextExt<P1, P2>(&self, vcview: TsViewCookie, pastart: P1, paend: P2, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), vcview, pastart.param().abi(), paend.param().abi(), prc as _, pfclipped as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetScreenExt(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetWnd(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), vcview, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_objidl", feature = "Win32_wtypes"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertTextAtSelection(&self, dwflags: u32, pchtext: *const u16, cch: u32, ppastart: *mut Option<IAnchor>, ppaend: *mut Option<IAnchor>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), dwflags, pchtext, cch, core::mem::transmute(ppastart), core::mem::transmute(ppaend)) }
    }
    #[cfg(feature = "Win32_objidl")]
    pub unsafe fn InsertEmbeddedAtSelection<P1>(&self, dwflags: u32, pdataobject: P1, ppastart: *mut Option<IAnchor>, ppaend: *mut Option<IAnchor>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), dwflags, pdataobject.param().abi(), core::mem::transmute(ppastart), core::mem::transmute(ppaend)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut TS_SELECTION_ANCHOR, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const TS_SELECTION_ANCHOR) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u16, u32, *mut u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *const u16, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidl")]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidl"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidl")]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidl"))]
    InsertEmbedded: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const TS_ATTRID) -> windows_core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const TS_ATTRID, u32) -> windows_core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const TS_ATTRID, u32) -> windows_core::HRESULT,
    pub FindNextAttrTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const TS_ATTRID, u32, *mut windows_core::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TS_ATTRVAL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    RetrieveRequestedAttrs: usize,
    pub GetStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TsViewCookie) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetAnchorFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *const super::windef::POINT, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetAnchorFromPoint: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::windef::RECT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetScreenExt: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetWnd: usize,
    #[cfg(all(feature = "Win32_objidl", feature = "Win32_wtypes"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::objidl::FORMATETC, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_objidl", feature = "Win32_wtypes")))]
    QueryInsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidl")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidl"))]
    InsertEmbeddedAtSelection: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_windef", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextStoreAnchor_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<windows_core::IUnknown>, dwmask: u32) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn GetStatus(&self) -> windows_core::Result<TS_STATUS>;
    fn QueryInsert(&self, pateststart: windows_core::Ref<IAnchor>, patestend: windows_core::Ref<IAnchor>, cch: u32, pparesultstart: windows_core::OutRef<IAnchor>, pparesultend: windows_core::OutRef<IAnchor>) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> windows_core::Result<()>;
    fn GetText(&self, dwflags: u32, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, pchtext: *mut u16, cchreq: u32, pcch: *mut u32, fupdateanchor: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: u32, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, pchtext: *const u16, cch: u32) -> windows_core::Result<()>;
    fn GetFormattedText(&self, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>) -> windows_core::Result<super::objidl::IDataObject>;
    fn GetEmbedded(&self, dwflags: u32, papos: windows_core::Ref<IAnchor>, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn InsertEmbedded(&self, dwflags: u32, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, papos: windows_core::Ref<IAnchor>, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, papos: windows_core::Ref<IAnchor>, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, pastart: windows_core::Ref<IAnchor>, pahalt: windows_core::Ref<IAnchor>, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32, pffound: *mut windows_core::BOOL, plfoundoffset: *mut i32) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn GetStart(&self) -> windows_core::Result<IAnchor>;
    fn GetEnd(&self) -> windows_core::Result<IAnchor>;
    fn GetActiveView(&self) -> windows_core::Result<TsViewCookie>;
    fn GetAnchorFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<IAnchor>;
    fn GetTextExt(&self, vcview: TsViewCookie, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::RECT>;
    fn GetWnd(&self, vcview: TsViewCookie) -> windows_core::Result<super::windef::HWND>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC) -> windows_core::Result<windows_core::BOOL>;
    fn InsertTextAtSelection(&self, dwflags: u32, pchtext: *const u16, cch: u32, ppastart: windows_core::OutRef<IAnchor>, ppaend: windows_core::OutRef<IAnchor>) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: u32, pdataobject: windows_core::Ref<super::objidl::IDataObject>, ppastart: windows_core::OutRef<IAnchor>, ppaend: windows_core::OutRef<IAnchor>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_windef", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextStoreAnchor_Vtbl {
    pub const fn new<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, dwmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&dwmask)).into()
            }
        }
        unsafe extern "system" fn UnadviseSink<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::UnadviseSink(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn RequestLock<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags)) {
                    Ok(ok__) => {
                        phrsession.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdcs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pateststart: *mut core::ffi::c_void, patestend: *mut core::ffi::c_void, cch: u32, pparesultstart: *mut *mut core::ffi::c_void, pparesultend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::QueryInsert(this, core::mem::transmute_copy(&pateststart), core::mem::transmute_copy(&patestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pparesultstart), core::mem::transmute_copy(&pparesultend)).into()
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pchtext: *mut u16, cchreq: u32, pcch: *mut u32, fupdateanchor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&fupdateanchor)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pchtext: *const u16, cch: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::SetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch)).into()
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::GetFormattedText(this, core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend)) {
                    Ok(ok__) => {
                        ppdataobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, papos: *mut core::ffi::c_void, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&papos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend), core::mem::transmute_copy(&pdataobject)).into()
            }
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papos: *mut core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&papos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papos: *mut core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&papos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, pahalt: *mut core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const TS_ATTRID, dwflags: u32, pffound: *mut windows_core::BOOL, plfoundoffset: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&pahalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
            }
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn GetStart<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppastart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::GetStart(this) {
                    Ok(ok__) => {
                        ppastart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnd<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::GetEnd(this) {
                    Ok(ok__) => {
                        ppaend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut TsViewCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::GetActiveView(this) {
                    Ok(ok__) => {
                        pvcview.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAnchorFromPoint<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, ptscreen: *const super::windef::POINT, dwflags: u32, ppasite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::GetAnchorFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppasite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
            }
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview)) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::GetWnd(this, core::mem::transmute_copy(&vcview)) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                    Ok(ok__) => {
                        pfinsertable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pchtext: *const u16, cch: u32, ppastart: *mut *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&ppastart), core::mem::transmute_copy(&ppaend)).into()
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdataobject: *mut core::ffi::c_void, ppastart: *mut *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&ppastart), core::mem::transmute_copy(&ppaend)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
            RequestLock: RequestLock::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            QueryInsert: QueryInsert::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, OFFSET>,
            GetStart: GetStart::<Identity, OFFSET>,
            GetEnd: GetEnd::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            GetAnchorFromPoint: GetAnchorFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_windef", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextStoreAnchor {}
windows_core::imp::define_interface!(ITextStoreAnchorSink, ITextStoreAnchorSink_Vtbl, 0xaa80e905_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITextStoreAnchorSink, windows_core::IUnknown);
impl ITextStoreAnchorSink {
    pub unsafe fn OnTextChange<P1, P2>(&self, dwflags: u32, pastart: P1, paend: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnTextChange)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi()) }
    }
    pub unsafe fn OnSelectionChange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSelectionChange)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: TsViewCookie) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLayoutChange)(windows_core::Interface::as_raw(self), lcode, vcview) }
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn OnAttrsChange<P0, P1>(&self, pastart: P0, paend: P1, cattrs: u32, paattrs: *const TS_ATTRID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnAttrsChange)(windows_core::Interface::as_raw(self), pastart.param().abi(), paend.param().abi(), cattrs, paattrs) }
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLockGranted)(windows_core::Interface::as_raw(self), dwlockflags) }
    }
    pub unsafe fn OnStartEditTransaction(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStartEditTransaction)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnEndEditTransaction(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEndEditTransaction)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchorSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTextChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void, TsLayoutCode, TsViewCookie) -> windows_core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnAttrsChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const TS_ATTRID) -> windows_core::HRESULT,
    pub OnLockGranted: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnStartEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITextStoreAnchorSink_Impl: windows_core::IUnknownImpl {
    fn OnTextChange(&self, dwflags: u32, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>) -> windows_core::Result<()>;
    fn OnSelectionChange(&self) -> windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: TsViewCookie) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OnAttrsChange(&self, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, cattrs: u32, paattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: u32) -> windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> windows_core::Result<()>;
}
impl ITextStoreAnchorSink_Vtbl {
    pub const fn new<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTextChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnTextChange(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend)).into()
            }
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnSelectionChange(this).into()
            }
        }
        unsafe extern "system" fn OnLayoutChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcode: TsLayoutCode, vcview: TsViewCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnLayoutChange(this, core::mem::transmute_copy(&lcode), core::mem::transmute_copy(&vcview)).into()
            }
        }
        unsafe extern "system" fn OnStatusChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, cattrs: u32, paattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnAttrsChange(this, core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend), core::mem::transmute_copy(&cattrs), core::mem::transmute_copy(&paattrs)).into()
            }
        }
        unsafe extern "system" fn OnLockGranted<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnLockGranted(this, core::mem::transmute_copy(&dwlockflags)).into()
            }
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnStartEditTransaction(this).into()
            }
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnEndEditTransaction(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTextChange: OnTextChange::<Identity, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreAnchorSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextStoreAnchorSink {}
pub const TS_AE_END: TsActiveSelEnd = 2;
pub const TS_AE_NONE: TsActiveSelEnd = 0;
pub const TS_AE_START: TsActiveSelEnd = 1;
pub const TS_AS_ALL_SINKS: u32 = 31;
pub const TS_AS_ATTR_CHANGE: u32 = 8;
pub const TS_AS_LAYOUT_CHANGE: u32 = 4;
pub const TS_AS_SEL_CHANGE: u32 = 2;
pub const TS_AS_STATUS_CHANGE: u32 = 16;
pub const TS_AS_TEXT_CHANGE: u32 = 1;
pub type TS_ATTRID = windows_core::GUID;
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub struct TS_ATTRVAL {
    pub idAttr: TS_ATTRID,
    pub dwOverlapId: u32,
    pub varValue: super::oaidl::VARIANT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Clone for TS_ATTRVAL {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_ATTR_FIND_BACKWARDS: u32 = 1;
pub const TS_ATTR_FIND_HIDDEN: u32 = 32;
pub const TS_ATTR_FIND_UPDATESTART: u32 = 4;
pub const TS_ATTR_FIND_WANT_END: u32 = 16;
pub const TS_ATTR_FIND_WANT_OFFSET: u32 = 2;
pub const TS_ATTR_FIND_WANT_VALUE: u32 = 8;
pub const TS_CHAR_EMBEDDED: u32 = 65532;
pub const TS_CHAR_REGION: u32 = 0;
pub const TS_CHAR_REPLACEMENT: u32 = 65533;
pub const TS_CH_FOLLOWING_DEL: u32 = 2;
pub const TS_CH_PRECEDING_DEL: u32 = 1;
pub const TS_DEFAULT_SELECTION: u32 = 4294967295;
pub const TS_E_FORMAT: i32 = -2147220982;
pub const TS_E_INVALIDPOINT: i32 = -2147220985;
pub const TS_E_INVALIDPOS: i32 = -2147220992;
pub const TS_E_NOINTERFACE: i32 = -2147220988;
pub const TS_E_NOLAYOUT: i32 = -2147220986;
pub const TS_E_NOLOCK: i32 = -2147220991;
pub const TS_E_NOOBJECT: i32 = -2147220990;
pub const TS_E_NOSELECTION: i32 = -2147220987;
pub const TS_E_NOSERVICE: i32 = -2147220989;
pub const TS_E_READONLY: i32 = -2147220983;
pub const TS_E_SYNCHRONOUS: i32 = -2147220984;
pub const TS_GEA_HIDDEN: u32 = 1;
pub const TS_GR_BACKWARD: TsGravity = 0;
pub const TS_GR_FORWARD: TsGravity = 1;
pub const TS_GTA_HIDDEN: u32 = 1;
pub const TS_IAS_NOQUERY: u32 = 1;
pub const TS_IAS_QUERYONLY: u32 = 2;
pub const TS_IE_COMPOSITION: u32 = 2;
pub const TS_IE_CORRECTION: u32 = 1;
pub const TS_LC_CHANGE: TsLayoutCode = 1;
pub const TS_LC_CREATE: TsLayoutCode = 0;
pub const TS_LC_DESTROY: TsLayoutCode = 2;
pub const TS_LF_READ: u32 = 2;
pub const TS_LF_READWRITE: u32 = 6;
pub const TS_LF_SYNC: u32 = 1;
pub const TS_RT_HIDDEN: TsRunType = 1;
pub const TS_RT_OPAQUE: TsRunType = 2;
pub const TS_RT_PLAIN: TsRunType = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TS_RUNINFO {
    pub uCount: u32,
    pub r#type: TsRunType,
}
pub const TS_SD_BACKWARD: TsShiftDir = 0;
pub const TS_SD_DISABLEWRITINGSUGGESTIONS: u32 = 512;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED: u32 = 128;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE: u32 = 256;
pub const TS_SD_FORWARD: TsShiftDir = 1;
pub const TS_SD_INPUTPANEMANUALDISPLAYENABLE: u32 = 64;
pub const TS_SD_LOADING: u32 = 2;
pub const TS_SD_MASKALL: u32 = 3;
pub const TS_SD_READONLY: u32 = 1;
pub const TS_SD_RESERVED: u32 = 4;
pub const TS_SD_TKBAUTOCORRECTENABLE: u32 = 8;
pub const TS_SD_TKBPREDICTIONENABLE: u32 = 16;
pub const TS_SD_UIINTEGRATIONENABLE: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TS_SELECTIONSTYLE {
    pub ase: TsActiveSelEnd,
    pub fInterimChar: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TS_SELECTION_ACP {
    pub acpStart: i32,
    pub acpEnd: i32,
    pub style: TS_SELECTIONSTYLE,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TS_SELECTION_ANCHOR {
    pub paStart: core::mem::ManuallyDrop<Option<IAnchor>>,
    pub paEnd: core::mem::ManuallyDrop<Option<IAnchor>>,
    pub style: TS_SELECTIONSTYLE,
}
pub const TS_SHIFT_COUNT_HIDDEN: u32 = 1;
pub const TS_SHIFT_COUNT_ONLY: u32 = 8;
pub const TS_SHIFT_HALT_HIDDEN: u32 = 2;
pub const TS_SHIFT_HALT_VISIBLE: u32 = 4;
pub const TS_SS_DISJOINTSEL: u32 = 1;
pub const TS_SS_MULTILINE: u32 = 128;
pub const TS_SS_NOHIDDENTEXT: u32 = 8;
pub const TS_SS_REGIONS: u32 = 2;
pub const TS_SS_TKBAUTOCORRECTENABLE: u32 = 16;
pub const TS_SS_TKBPREDICTIONENABLE: u32 = 32;
pub const TS_SS_TRANSITORY: u32 = 4;
pub const TS_SS_UWPCONTROL: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TS_STATUS {
    pub dwDynamicFlags: u32,
    pub dwStaticFlags: u32,
}
pub const TS_ST_CORRECTION: u32 = 1;
pub const TS_S_ASYNC: u32 = 262912;
pub const TS_TC_CORRECTION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TS_TEXTCHANGE {
    pub acpStart: i32,
    pub acpOldEnd: i32,
    pub acpNewEnd: i32,
}
pub const TS_VCOOKIE_NUL: u32 = 4294967295;
pub type TsActiveSelEnd = i32;
pub type TsGravity = i32;
pub type TsLayoutCode = i32;
pub type TsRunType = i32;
pub type TsShiftDir = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TsViewCookie(pub u32);
