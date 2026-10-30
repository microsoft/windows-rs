pub const GUID_TS_SERVICE_ACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0xf9786200_a5bf_4a0f_8c24_fb16f5d1aabb);
pub const GUID_TS_SERVICE_ACTIVEX: windows_core::GUID = windows_core::GUID::from_u128(0xea937a50_c9a6_4b7d_894a_49d99b784834);
pub const GUID_TS_SERVICE_DATAOBJECT: windows_core::GUID = windows_core::GUID::from_u128(0x6086fbb5_e225_46ce_a770_c1bbd3e05d7b);
pub const GXFPF_NEAREST: i32 = 2;
pub const GXFPF_ROUND_NEAREST: i32 = 1;
windows_core::imp::define_interface!(IAnchor, IAnchor_Vtbl, 0x0feb7e34_5a60_4356_8ef7_abdec2ff7cf8);
windows_core::imp::interface_hierarchy!(IAnchor, windows_core::IUnknown);
impl IAnchor {
    pub unsafe fn SetGravity(&self, gravity: TsGravity) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGravity)(windows_core::Interface::as_raw(self), gravity) }
    }
    pub unsafe fn GetGravity(&self, pgravity: *mut TsGravity) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGravity)(windows_core::Interface::as_raw(self), pgravity as _) }
    }
    pub unsafe fn IsEqual<P0>(&self, pawith: P0, pfequal: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pawith.param().abi(), pfequal as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Compare<P0>(&self, pawith: P0, plresult: *mut super::LONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), pawith.param().abi(), plresult as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Shift<P3>(&self, dwflags: super::DWORD, cchreq: super::LONG, pcch: *mut super::LONG, pahaltanchor: P3) -> windows_core::HRESULT
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
    #[cfg(feature = "shared")]
    pub unsafe fn ShiftRegion(&self, dwflags: super::DWORD, dir: TsShiftDir, pfnoregion: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShiftRegion)(windows_core::Interface::as_raw(self), dwflags, dir, pfnoregion as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetChangeHistoryMask(&self, dwmask: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChangeHistoryMask)(windows_core::Interface::as_raw(self), dwmask) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetChangeHistory(&self, pdwhistory: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChangeHistory)(windows_core::Interface::as_raw(self), pdwhistory as _) }
    }
    pub unsafe fn ClearChangeHistory(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearChangeHistory)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    #[cfg(feature = "shared")]
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Compare: usize,
    #[cfg(feature = "shared")]
    pub Shift: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LONG, *mut super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Shift: usize,
    pub ShiftTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub ShiftRegion: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, TsShiftDir, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ShiftRegion: usize,
    #[cfg(feature = "shared")]
    pub SetChangeHistoryMask: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetChangeHistoryMask: usize,
    #[cfg(feature = "shared")]
    pub GetChangeHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetChangeHistory: usize,
    pub ClearChangeHistory: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IAnchor_Impl: windows_core::IUnknownImpl {
    fn SetGravity(&self, gravity: TsGravity) -> windows_core::Result<()>;
    fn GetGravity(&self, pgravity: *mut TsGravity) -> windows_core::Result<()>;
    fn IsEqual(&self, pawith: windows_core::Ref<IAnchor>, pfequal: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn Compare(&self, pawith: windows_core::Ref<IAnchor>, plresult: *mut super::LONG) -> windows_core::Result<()>;
    fn Shift(&self, dwflags: super::DWORD, cchreq: super::LONG, pcch: *mut super::LONG, pahaltanchor: windows_core::Ref<IAnchor>) -> windows_core::Result<()>;
    fn ShiftTo(&self, pasite: windows_core::Ref<IAnchor>) -> windows_core::Result<()>;
    fn ShiftRegion(&self, dwflags: super::DWORD, dir: TsShiftDir, pfnoregion: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetChangeHistoryMask(&self, dwmask: super::DWORD) -> windows_core::Result<()>;
    fn GetChangeHistory(&self, pdwhistory: *mut super::DWORD) -> windows_core::Result<()>;
    fn ClearChangeHistory(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IAnchor>;
}
#[cfg(feature = "shared")]
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
                IAnchor_Impl::GetGravity(this, core::mem::transmute_copy(&pgravity)).into()
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pawith: *mut core::ffi::c_void, pfequal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::IsEqual(this, core::mem::transmute_copy(&pawith), core::mem::transmute_copy(&pfequal)).into()
            }
        }
        unsafe extern "system" fn Compare<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pawith: *mut core::ffi::c_void, plresult: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::Compare(this, core::mem::transmute_copy(&pawith), core::mem::transmute_copy(&plresult)).into()
            }
        }
        unsafe extern "system" fn Shift<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, cchreq: super::LONG, pcch: *mut super::LONG, pahaltanchor: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ShiftRegion<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, dir: TsShiftDir, pfnoregion: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::ShiftRegion(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dir), core::mem::transmute_copy(&pfnoregion)).into()
            }
        }
        unsafe extern "system" fn SetChangeHistoryMask<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmask: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::SetChangeHistoryMask(this, core::mem::transmute_copy(&dwmask)).into()
            }
        }
        unsafe extern "system" fn GetChangeHistory<Identity: IAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhistory: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchor_Impl::GetChangeHistory(this, core::mem::transmute_copy(&pdwhistory)).into()
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IAnchor {}
windows_core::imp::define_interface!(ITextStoreACP, ITextStoreACP_Vtbl, 0x28888fe3_c2a0_483a_a3ea_8cb1ce51ff3d);
windows_core::imp::interface_hierarchy!(ITextStoreACP, windows_core::IUnknown);
impl ITextStoreACP {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn AdviseSink<P1>(&self, riid: *const super::IID, punk: P1, dwmask: super::DWORD) -> windows_core::HRESULT
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
    #[cfg(feature = "shared")]
    pub unsafe fn RequestLock(&self, dwlockflags: super::DWORD, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestLock)(windows_core::Interface::as_raw(self), dwlockflags, phrsession as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatus(&self, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdcs as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn QueryInsert(&self, acpteststart: super::LONG, acptestend: super::LONG, cch: super::ULONG, pacpresultstart: *mut super::LONG, pacpresultend: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInsert)(windows_core::Interface::as_raw(self), acpteststart, acptestend, cch, pacpresultstart as _, pacpresultend as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSelection(&self, ulindex: super::ULONG, ulcount: super::ULONG, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ulindex, ulcount, pselection as _, pcfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSelection(&self, pselection: &[TS_SELECTION_ACP]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), pselection.len().try_into().unwrap(), pselection.as_ptr()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetText(&self, acpstart: super::LONG, acpend: super::LONG, pchplain: *mut super::WCHAR, cchplainreq: super::ULONG, pcchplainret: *mut super::ULONG, prgruninfo: *mut TS_RUNINFO, cruninforeq: super::ULONG, pcruninforet: *mut super::ULONG, pacpnext: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), acpstart, acpend, pchplain as _, cchplainreq, pcchplainret as _, prgruninfo as _, cruninforeq, pcruninforet as _, pacpnext as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetText(&self, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pchtext: &[super::WCHAR], pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pchtext.as_ptr(), pchtext.len().try_into().unwrap(), pchange as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn GetFormattedText(&self, acpstart: super::LONG, acpend: super::LONG) -> windows_core::Result<super::IDataObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetEmbedded(&self, acppos: super::LONG, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), acppos, rguidservice, riid, core::mem::transmute(ppunk)) }
    }
    #[cfg(all(feature = "objidl", feature = "shared", feature = "wtypes"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, pfinsertable as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn InsertEmbedded<P3>(&self, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pdataobject: P3, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
    where
        P3: windows_core::Param<super::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pdataobject.param().abi(), pchange as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn InsertTextAtSelection(&self, dwflags: super::DWORD, pchtext: &[super::WCHAR], pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), dwflags, pchtext.as_ptr(), pchtext.len().try_into().unwrap(), pacpstart as _, pacpend as _, pchange as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn InsertEmbeddedAtSelection<P1>(&self, dwflags: super::DWORD, pdataobject: P1, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), dwflags, pdataobject.param().abi(), pacpstart as _, pacpend as _, pchange as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: super::DWORD, pafilterattrs: Option<&[TS_ATTRID]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestSupportedAttrs)(windows_core::Interface::as_raw(self), dwflags, pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: super::LONG, pafilterattrs: Option<&[TS_ATTRID]>, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsAtPosition)(windows_core::Interface::as_raw(self), acppos, pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: super::LONG, pafilterattrs: Option<&[TS_ATTRID]>, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(windows_core::Interface::as_raw(self), acppos, pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn FindNextAttrTransition(&self, acpstart: super::LONG, acphalt: super::LONG, pafilterattrs: Option<&[TS_ATTRID]>, dwflags: super::DWORD, pacpnext: *mut super::LONG, pffound: *mut windows_core::BOOL, plfoundoffset: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindNextAttrTransition)(windows_core::Interface::as_raw(self), acpstart, acphalt, pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwflags, pacpnext as _, pffound as _, plfoundoffset as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: super::ULONG, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RetrieveRequestedAttrs)(windows_core::Interface::as_raw(self), ulcount, paattrvals, pcfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEndACP(&self, pacp: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEndACP)(windows_core::Interface::as_raw(self), pacp as _) }
    }
    pub unsafe fn GetActiveView(&self, pvcview: *mut TsViewCookie) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), pvcview as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetACPFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::POINT, dwflags: super::DWORD, pacp: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetACPFromPoint)(windows_core::Interface::as_raw(self), vcview, ptscreen, dwflags, pacp as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetTextExt(&self, vcview: TsViewCookie, acpstart: super::LONG, acpend: super::LONG, prc: *mut super::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), vcview, acpstart, acpend, prc as _, pfclipped as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetScreenExt(&self, vcview: TsViewCookie, prc: *mut super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), vcview, prc as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetWnd(&self, vcview: TsViewCookie, phwnd: *mut super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), vcview, phwnd as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    AdviseSink: usize,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RequestLock: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestLock: usize,
    #[cfg(feature = "shared")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatus: usize,
    #[cfg(feature = "shared")]
    pub QueryInsert: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::ULONG, *mut super::LONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    QueryInsert: usize,
    #[cfg(feature = "shared")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::ULONG, *mut TS_SELECTION_ACP, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSelection: usize,
    #[cfg(feature = "shared")]
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const TS_SELECTION_ACP) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSelection: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, *mut super::WCHAR, super::ULONG, *mut super::ULONG, *mut TS_RUNINFO, super::ULONG, *mut super::ULONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetText: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LONG, super::LONG, *const super::WCHAR, super::ULONG, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetText: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    GetFormattedText: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *const windows_core::GUID, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetEmbedded: usize,
    #[cfg(all(feature = "objidl", feature = "shared", feature = "wtypes"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::FORMATETC, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared", feature = "wtypes")))]
    QueryInsertEmbedded: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LONG, super::LONG, *mut core::ffi::c_void, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    InsertEmbedded: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::WCHAR, super::ULONG, *mut super::LONG, *mut super::LONG, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    InsertTextAtSelection: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut super::LONG, *mut super::LONG, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    InsertEmbeddedAtSelection: usize,
    #[cfg(feature = "shared")]
    pub RequestSupportedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::ULONG, *const TS_ATTRID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestSupportedAttrs: usize,
    #[cfg(feature = "shared")]
    pub RequestAttrsAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::ULONG, *const TS_ATTRID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestAttrsAtPosition: usize,
    #[cfg(feature = "shared")]
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::ULONG, *const TS_ATTRID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestAttrsTransitioningAtPosition: usize,
    #[cfg(feature = "shared")]
    pub FindNextAttrTransition: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::ULONG, *const TS_ATTRID, super::DWORD, *mut super::LONG, *mut windows_core::BOOL, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FindNextAttrTransition: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut TS_ATTRVAL, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    RetrieveRequestedAttrs: usize,
    #[cfg(feature = "shared")]
    pub GetEndACP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEndACP: usize,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TsViewCookie) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetACPFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *const super::POINT, super::DWORD, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetACPFromPoint: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, super::LONG, super::LONG, *mut super::RECT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetTextExt: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetScreenExt: usize,
    #[cfg(feature = "windef")]
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWnd: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITextStoreACP_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const super::IID, punk: windows_core::Ref<windows_core::IUnknown>, dwmask: super::DWORD) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: super::DWORD, phrsession: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetStatus(&self, pdcs: *mut TS_STATUS) -> windows_core::Result<()>;
    fn QueryInsert(&self, acpteststart: super::LONG, acptestend: super::LONG, cch: super::ULONG, pacpresultstart: *mut super::LONG, pacpresultend: *mut super::LONG) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: super::ULONG, ulcount: super::ULONG, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: super::ULONG, pselection: *const TS_SELECTION_ACP) -> windows_core::Result<()>;
    fn GetText(&self, acpstart: super::LONG, acpend: super::LONG, pchplain: *mut super::WCHAR, cchplainreq: super::ULONG, pcchplainret: *mut super::ULONG, prgruninfo: *mut TS_RUNINFO, cruninforeq: super::ULONG, pcruninforet: *mut super::ULONG, pacpnext: *mut super::LONG) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pchtext: *const super::WCHAR, cch: super::ULONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn GetFormattedText(&self, acpstart: super::LONG, acpend: super::LONG) -> windows_core::Result<super::IDataObject>;
    fn GetEmbedded(&self, acppos: super::LONG, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn InsertEmbedded(&self, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pdataobject: windows_core::Ref<super::IDataObject>, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertTextAtSelection(&self, dwflags: super::DWORD, pchtext: *const super::WCHAR, cch: super::ULONG, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: super::DWORD, pdataobject: windows_core::Ref<super::IDataObject>, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: super::DWORD, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: super::LONG, acphalt: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD, pacpnext: *mut super::LONG, pffound: *mut windows_core::BOOL, plfoundoffset: *mut super::LONG) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: super::ULONG, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetEndACP(&self, pacp: *mut super::LONG) -> windows_core::Result<()>;
    fn GetActiveView(&self, pvcview: *mut TsViewCookie) -> windows_core::Result<()>;
    fn GetACPFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::POINT, dwflags: super::DWORD, pacp: *mut super::LONG) -> windows_core::Result<()>;
    fn GetTextExt(&self, vcview: TsViewCookie, acpstart: super::LONG, acpend: super::LONG, prc: *mut super::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: TsViewCookie, prc: *mut super::RECT) -> windows_core::Result<()>;
    fn GetWnd(&self, vcview: TsViewCookie, phwnd: *mut super::HWND) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITextStoreACP_Vtbl {
    pub const fn new<Identity: ITextStoreACP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, punk: *mut core::ffi::c_void, dwmask: super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RequestLock<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: super::DWORD, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags), core::mem::transmute_copy(&phrsession)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetStatus(this, core::mem::transmute_copy(&pdcs)).into()
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpteststart: super::LONG, acptestend: super::LONG, cch: super::ULONG, pacpresultstart: *mut super::LONG, pacpresultend: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::QueryInsert(this, core::mem::transmute_copy(&acpteststart), core::mem::transmute_copy(&acptestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpresultstart), core::mem::transmute_copy(&pacpresultend)).into()
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: super::ULONG, ulcount: super::ULONG, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: super::LONG, acpend: super::LONG, pchplain: *mut super::WCHAR, cchplainreq: super::ULONG, pcchplainret: *mut super::ULONG, prgruninfo: *mut TS_RUNINFO, cruninforeq: super::ULONG, pcruninforet: *mut super::ULONG, pacpnext: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchplain), core::mem::transmute_copy(&cchplainreq), core::mem::transmute_copy(&pcchplainret), core::mem::transmute_copy(&prgruninfo), core::mem::transmute_copy(&cruninforeq), core::mem::transmute_copy(&pcruninforet), core::mem::transmute_copy(&pacpnext)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pchtext: *const super::WCHAR, cch: super::ULONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::SetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: super::LONG, acpend: super::LONG, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetEmbedded<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: super::LONG, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetEmbedded(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pfinsertable)).into()
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pdataobject: *mut core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pchtext: *const super::WCHAR, cch: super::ULONG, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pdataobject: *mut core::ffi::c_void, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: super::LONG, acphalt: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD, pacpnext: *mut super::LONG, pffound: *mut windows_core::BOOL, plfoundoffset: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acphalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacpnext), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
            }
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn GetEndACP<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacp: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetEndACP(this, core::mem::transmute_copy(&pacp)).into()
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut TsViewCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetActiveView(this, core::mem::transmute_copy(&pvcview)).into()
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, ptscreen: *const super::POINT, dwflags: super::DWORD, pacp: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacp)).into()
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, acpstart: super::LONG, acpend: super::LONG, prc: *mut super::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
            }
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, prc: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITextStoreACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, phwnd: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP_Impl::GetWnd(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&phwnd)).into()
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITextStoreACP {}
windows_core::imp::define_interface!(ITextStoreACP2, ITextStoreACP2_Vtbl, 0xf86ad89f_5fe4_4b8d_bb9f_ef3797a84f1f);
windows_core::imp::interface_hierarchy!(ITextStoreACP2, windows_core::IUnknown);
impl ITextStoreACP2 {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn AdviseSink<P1>(&self, riid: *const super::IID, punk: P1, dwmask: super::DWORD) -> windows_core::HRESULT
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
    #[cfg(feature = "shared")]
    pub unsafe fn RequestLock(&self, dwlockflags: super::DWORD, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestLock)(windows_core::Interface::as_raw(self), dwlockflags, phrsession as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatus(&self, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdcs as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn QueryInsert(&self, acpteststart: super::LONG, acptestend: super::LONG, cch: super::ULONG, pacpresultstart: *mut super::LONG, pacpresultend: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInsert)(windows_core::Interface::as_raw(self), acpteststart, acptestend, cch, pacpresultstart as _, pacpresultend as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSelection(&self, ulindex: super::ULONG, ulcount: super::ULONG, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ulindex, ulcount, pselection as _, pcfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSelection(&self, pselection: &[TS_SELECTION_ACP]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), pselection.len().try_into().unwrap(), pselection.as_ptr()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetText(&self, acpstart: super::LONG, acpend: super::LONG, pchplain: *mut super::WCHAR, cchplainreq: super::ULONG, pcchplainret: *mut super::ULONG, prgruninfo: *mut TS_RUNINFO, cruninforeq: super::ULONG, pcruninforet: *mut super::ULONG, pacpnext: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), acpstart, acpend, pchplain as _, cchplainreq, pcchplainret as _, prgruninfo as _, cruninforeq, pcruninforet as _, pacpnext as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetText(&self, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pchtext: &[super::WCHAR], pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pchtext.as_ptr(), pchtext.len().try_into().unwrap(), pchange as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn GetFormattedText(&self, acpstart: super::LONG, acpend: super::LONG) -> windows_core::Result<super::IDataObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetEmbedded(&self, acppos: super::LONG, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), acppos, rguidservice, riid, core::mem::transmute(ppunk)) }
    }
    #[cfg(all(feature = "objidl", feature = "shared", feature = "wtypes"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, pfinsertable as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn InsertEmbedded<P3>(&self, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pdataobject: P3, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
    where
        P3: windows_core::Param<super::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), dwflags, acpstart, acpend, pdataobject.param().abi(), pchange as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn InsertTextAtSelection(&self, dwflags: super::DWORD, pchtext: &[super::WCHAR], pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), dwflags, pchtext.as_ptr(), pchtext.len().try_into().unwrap(), pacpstart as _, pacpend as _, pchange as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn InsertEmbeddedAtSelection<P1>(&self, dwflags: super::DWORD, pdataobject: P1, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), dwflags, pdataobject.param().abi(), pacpstart as _, pacpend as _, pchange as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: super::DWORD, pafilterattrs: Option<&[TS_ATTRID]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestSupportedAttrs)(windows_core::Interface::as_raw(self), dwflags, pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: super::LONG, pafilterattrs: Option<&[TS_ATTRID]>, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsAtPosition)(windows_core::Interface::as_raw(self), acppos, pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: super::LONG, pafilterattrs: Option<&[TS_ATTRID]>, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(windows_core::Interface::as_raw(self), acppos, pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn FindNextAttrTransition(&self, acpstart: super::LONG, acphalt: super::LONG, pafilterattrs: Option<&[TS_ATTRID]>, dwflags: super::DWORD, pacpnext: *mut super::LONG, pffound: *mut windows_core::BOOL, plfoundoffset: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindNextAttrTransition)(windows_core::Interface::as_raw(self), acpstart, acphalt, pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwflags, pacpnext as _, pffound as _, plfoundoffset as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: super::ULONG, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RetrieveRequestedAttrs)(windows_core::Interface::as_raw(self), ulcount, paattrvals, pcfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEndACP(&self, pacp: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEndACP)(windows_core::Interface::as_raw(self), pacp as _) }
    }
    pub unsafe fn GetActiveView(&self, pvcview: *mut TsViewCookie) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), pvcview as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetACPFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::POINT, dwflags: super::DWORD, pacp: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetACPFromPoint)(windows_core::Interface::as_raw(self), vcview, ptscreen, dwflags, pacp as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetTextExt(&self, vcview: TsViewCookie, acpstart: super::LONG, acpend: super::LONG, prc: *mut super::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), vcview, acpstart, acpend, prc as _, pfclipped as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetScreenExt(&self, vcview: TsViewCookie, prc: *mut super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), vcview, prc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACP2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    AdviseSink: usize,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RequestLock: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestLock: usize,
    #[cfg(feature = "shared")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatus: usize,
    #[cfg(feature = "shared")]
    pub QueryInsert: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::ULONG, *mut super::LONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    QueryInsert: usize,
    #[cfg(feature = "shared")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::ULONG, *mut TS_SELECTION_ACP, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSelection: usize,
    #[cfg(feature = "shared")]
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const TS_SELECTION_ACP) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSelection: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, *mut super::WCHAR, super::ULONG, *mut super::ULONG, *mut TS_RUNINFO, super::ULONG, *mut super::ULONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetText: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LONG, super::LONG, *const super::WCHAR, super::ULONG, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetText: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    GetFormattedText: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *const windows_core::GUID, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetEmbedded: usize,
    #[cfg(all(feature = "objidl", feature = "shared", feature = "wtypes"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::FORMATETC, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared", feature = "wtypes")))]
    QueryInsertEmbedded: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LONG, super::LONG, *mut core::ffi::c_void, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    InsertEmbedded: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::WCHAR, super::ULONG, *mut super::LONG, *mut super::LONG, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    InsertTextAtSelection: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut super::LONG, *mut super::LONG, *mut TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    InsertEmbeddedAtSelection: usize,
    #[cfg(feature = "shared")]
    pub RequestSupportedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::ULONG, *const TS_ATTRID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestSupportedAttrs: usize,
    #[cfg(feature = "shared")]
    pub RequestAttrsAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::ULONG, *const TS_ATTRID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestAttrsAtPosition: usize,
    #[cfg(feature = "shared")]
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::ULONG, *const TS_ATTRID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestAttrsTransitioningAtPosition: usize,
    #[cfg(feature = "shared")]
    pub FindNextAttrTransition: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::ULONG, *const TS_ATTRID, super::DWORD, *mut super::LONG, *mut windows_core::BOOL, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FindNextAttrTransition: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut TS_ATTRVAL, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    RetrieveRequestedAttrs: usize,
    #[cfg(feature = "shared")]
    pub GetEndACP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEndACP: usize,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TsViewCookie) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetACPFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *const super::POINT, super::DWORD, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetACPFromPoint: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, super::LONG, super::LONG, *mut super::RECT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetTextExt: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetScreenExt: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITextStoreACP2_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const super::IID, punk: windows_core::Ref<windows_core::IUnknown>, dwmask: super::DWORD) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: super::DWORD, phrsession: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetStatus(&self, pdcs: *mut TS_STATUS) -> windows_core::Result<()>;
    fn QueryInsert(&self, acpteststart: super::LONG, acptestend: super::LONG, cch: super::ULONG, pacpresultstart: *mut super::LONG, pacpresultend: *mut super::LONG) -> windows_core::Result<()>;
    fn GetSelection(&self, ulindex: super::ULONG, ulcount: super::ULONG, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: super::ULONG, pselection: *const TS_SELECTION_ACP) -> windows_core::Result<()>;
    fn GetText(&self, acpstart: super::LONG, acpend: super::LONG, pchplain: *mut super::WCHAR, cchplainreq: super::ULONG, pcchplainret: *mut super::ULONG, prgruninfo: *mut TS_RUNINFO, cruninforeq: super::ULONG, pcruninforet: *mut super::ULONG, pacpnext: *mut super::LONG) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pchtext: *const super::WCHAR, cch: super::ULONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn GetFormattedText(&self, acpstart: super::LONG, acpend: super::LONG) -> windows_core::Result<super::IDataObject>;
    fn GetEmbedded(&self, acppos: super::LONG, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn InsertEmbedded(&self, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pdataobject: windows_core::Ref<super::IDataObject>, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertTextAtSelection(&self, dwflags: super::DWORD, pchtext: *const super::WCHAR, cch: super::ULONG, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(&self, dwflags: super::DWORD, pdataobject: windows_core::Ref<super::IDataObject>, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: super::DWORD, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, acppos: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, acppos: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, acpstart: super::LONG, acphalt: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD, pacpnext: *mut super::LONG, pffound: *mut windows_core::BOOL, plfoundoffset: *mut super::LONG) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: super::ULONG, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetEndACP(&self, pacp: *mut super::LONG) -> windows_core::Result<()>;
    fn GetActiveView(&self, pvcview: *mut TsViewCookie) -> windows_core::Result<()>;
    fn GetACPFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::POINT, dwflags: super::DWORD, pacp: *mut super::LONG) -> windows_core::Result<()>;
    fn GetTextExt(&self, vcview: TsViewCookie, acpstart: super::LONG, acpend: super::LONG, prc: *mut super::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: TsViewCookie, prc: *mut super::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITextStoreACP2_Vtbl {
    pub const fn new<Identity: ITextStoreACP2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, punk: *mut core::ffi::c_void, dwmask: super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RequestLock<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: super::DWORD, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags), core::mem::transmute_copy(&phrsession)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetStatus(this, core::mem::transmute_copy(&pdcs)).into()
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpteststart: super::LONG, acptestend: super::LONG, cch: super::ULONG, pacpresultstart: *mut super::LONG, pacpresultend: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::QueryInsert(this, core::mem::transmute_copy(&acpteststart), core::mem::transmute_copy(&acptestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpresultstart), core::mem::transmute_copy(&pacpresultend)).into()
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: super::ULONG, ulcount: super::ULONG, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, pselection: *const TS_SELECTION_ACP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: super::LONG, acpend: super::LONG, pchplain: *mut super::WCHAR, cchplainreq: super::ULONG, pcchplainret: *mut super::ULONG, prgruninfo: *mut TS_RUNINFO, cruninforeq: super::ULONG, pcruninforet: *mut super::ULONG, pacpnext: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetText(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchplain), core::mem::transmute_copy(&cchplainreq), core::mem::transmute_copy(&pcchplainret), core::mem::transmute_copy(&prgruninfo), core::mem::transmute_copy(&cruninforeq), core::mem::transmute_copy(&pcruninforet), core::mem::transmute_copy(&pacpnext)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pchtext: *const super::WCHAR, cch: super::ULONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::SetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: super::LONG, acpend: super::LONG, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetEmbedded<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: super::LONG, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetEmbedded(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pfinsertable)).into()
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, acpstart: super::LONG, acpend: super::LONG, pdataobject: *mut core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pchtext: *const super::WCHAR, cch: super::ULONG, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pdataobject: *mut core::ffi::c_void, pacpstart: *mut super::LONG, pacpend: *mut super::LONG, pchange: *mut TS_TEXTCHANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&pacpstart), core::mem::transmute_copy(&pacpend), core::mem::transmute_copy(&pchange)).into()
            }
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acppos: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&acppos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: super::LONG, acphalt: super::LONG, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD, pacpnext: *mut super::LONG, pffound: *mut windows_core::BOOL, plfoundoffset: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acphalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacpnext), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
            }
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::RetrieveRequestedAttrs(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&paattrvals), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn GetEndACP<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacp: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetEndACP(this, core::mem::transmute_copy(&pacp)).into()
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcview: *mut TsViewCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetActiveView(this, core::mem::transmute_copy(&pvcview)).into()
            }
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, ptscreen: *const super::POINT, dwflags: super::DWORD, pacp: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pacp)).into()
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, acpstart: super::LONG, acpend: super::LONG, prc: *mut super::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
            }
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITextStoreACP2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, prc: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACP2_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&prc)).into()
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITextStoreACP2 {}
windows_core::imp::define_interface!(ITextStoreACPSink, ITextStoreACPSink_Vtbl, 0x22d44c94_a419_4542_a272_ae26093ececf);
windows_core::imp::interface_hierarchy!(ITextStoreACPSink, windows_core::IUnknown);
impl ITextStoreACPSink {
    #[cfg(feature = "shared")]
    pub unsafe fn OnTextChange(&self, dwflags: super::DWORD, pchange: *const TS_TEXTCHANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnTextChange)(windows_core::Interface::as_raw(self), dwflags, pchange) }
    }
    pub unsafe fn OnSelectionChange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSelectionChange)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: TsViewCookie) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLayoutChange)(windows_core::Interface::as_raw(self), lcode, vcview) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnStatusChange(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnAttrsChange(&self, acpstart: super::LONG, acpend: super::LONG, paattrs: &[TS_ATTRID]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnAttrsChange)(windows_core::Interface::as_raw(self), acpstart, acpend, paattrs.len().try_into().unwrap(), paattrs.as_ptr()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnLockGranted(&self, dwlockflags: super::DWORD) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub OnTextChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const TS_TEXTCHANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnTextChange: usize,
    pub OnSelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void, TsLayoutCode, TsViewCookie) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnStatusChange: usize,
    #[cfg(feature = "shared")]
    pub OnAttrsChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::ULONG, *const TS_ATTRID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnAttrsChange: usize,
    #[cfg(feature = "shared")]
    pub OnLockGranted: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnLockGranted: usize,
    pub OnStartEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait ITextStoreACPSink_Impl: windows_core::IUnknownImpl {
    fn OnTextChange(&self, dwflags: super::DWORD, pchange: *const TS_TEXTCHANGE) -> windows_core::Result<()>;
    fn OnSelectionChange(&self) -> windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: TsViewCookie) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn OnAttrsChange(&self, acpstart: super::LONG, acpend: super::LONG, cattrs: super::ULONG, paattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: super::DWORD) -> windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITextStoreACPSink_Vtbl {
    pub const fn new<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTextChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pchange: *const TS_TEXTCHANGE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn OnStatusChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: super::LONG, acpend: super::LONG, cattrs: super::ULONG, paattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPSink_Impl::OnAttrsChange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&cattrs), core::mem::transmute_copy(&paattrs)).into()
            }
        }
        unsafe extern "system" fn OnLockGranted<Identity: ITextStoreACPSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITextStoreACPSink {}
windows_core::imp::define_interface!(ITextStoreAnchor, ITextStoreAnchor_Vtbl, 0x9b2077b0_5f18_4dec_bee9_3cc722f5dfe0);
windows_core::imp::interface_hierarchy!(ITextStoreAnchor, windows_core::IUnknown);
impl ITextStoreAnchor {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn AdviseSink<P1>(&self, riid: *const super::IID, punk: P1, dwmask: super::DWORD) -> windows_core::HRESULT
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
    #[cfg(feature = "shared")]
    pub unsafe fn RequestLock(&self, dwlockflags: super::DWORD, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestLock)(windows_core::Interface::as_raw(self), dwlockflags, phrsession as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatus(&self, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdcs as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn QueryInsert<P0, P1>(&self, pateststart: P0, patestend: P1, cch: super::ULONG, pparesultstart: *mut Option<IAnchor>) -> windows_core::Result<IAnchor>
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryInsert)(windows_core::Interface::as_raw(self), pateststart.param().abi(), patestend.param().abi(), cch, core::mem::transmute(pparesultstart), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSelection(&self, ulindex: super::ULONG, ulcount: super::ULONG, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ulindex, ulcount, pselection, pcfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSelection(&self, pselection: &[TS_SELECTION_ANCHOR]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), pselection.len().try_into().unwrap(), pselection.as_ptr()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetText<P1, P2>(&self, dwflags: super::DWORD, pastart: P1, paend: P2, pchtext: *mut super::WCHAR, cchreq: super::ULONG, pcch: *mut super::ULONG, fupdateanchor: bool) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi(), pchtext as _, cchreq, pcch as _, fupdateanchor.into()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetText<P1, P2>(&self, dwflags: super::DWORD, pastart: P1, paend: P2, pchtext: &[super::WCHAR]) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi(), pchtext.as_ptr(), pchtext.len().try_into().unwrap()) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetFormattedText<P0, P1>(&self, pastart: P0, paend: P1) -> windows_core::Result<super::IDataObject>
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), pastart.param().abi(), paend.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetEmbedded<P1>(&self, dwflags: super::DWORD, papos: P1, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), dwflags, papos.param().abi(), rguidservice, riid, core::mem::transmute(ppunk)) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn InsertEmbedded<P1, P2, P3>(&self, dwflags: super::DWORD, pastart: P1, paend: P2, pdataobject: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
        P3: windows_core::Param<super::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), dwflags, pastart.param().abi(), paend.param().abi(), pdataobject.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: super::DWORD, pafilterattrs: Option<&[TS_ATTRID]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestSupportedAttrs)(windows_core::Interface::as_raw(self), dwflags, pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestAttrsAtPosition<P0>(&self, papos: P0, pafilterattrs: Option<&[TS_ATTRID]>, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsAtPosition)(windows_core::Interface::as_raw(self), papos.param().abi(), pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestAttrsTransitioningAtPosition<P0>(&self, papos: P0, pafilterattrs: Option<&[TS_ATTRID]>, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(windows_core::Interface::as_raw(self), papos.param().abi(), pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn FindNextAttrTransition<P0, P1>(&self, pastart: P0, pahalt: P1, pafilterattrs: Option<&[TS_ATTRID]>, dwflags: super::DWORD, pffound: *mut windows_core::BOOL, plfoundoffset: *mut super::LONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindNextAttrTransition)(windows_core::Interface::as_raw(self), pastart.param().abi(), pahalt.param().abi(), pafilterattrs.map_or(0, |slice| slice.len().try_into().unwrap()), pafilterattrs.map_or(core::ptr::null(), |slice| slice.as_ptr()), dwflags, pffound as _, plfoundoffset as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: super::ULONG, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RetrieveRequestedAttrs)(windows_core::Interface::as_raw(self), ulcount, paattrvals, pcfetched as _) }
    }
    pub unsafe fn GetStart(&self) -> windows_core::Result<IAnchor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStart)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEnd(&self) -> windows_core::Result<IAnchor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnd)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetActiveView(&self, pvcview: *mut TsViewCookie) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), pvcview as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetAnchorFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::POINT, dwflags: super::DWORD) -> windows_core::Result<IAnchor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAnchorFromPoint)(windows_core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetTextExt<P1, P2>(&self, vcview: TsViewCookie, pastart: P1, paend: P2, prc: *mut super::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAnchor>,
        P2: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), vcview, pastart.param().abi(), paend.param().abi(), prc as _, pfclipped as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetScreenExt(&self, vcview: TsViewCookie, prc: *mut super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), vcview, prc as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetWnd(&self, vcview: TsViewCookie, phwnd: *mut super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), vcview, phwnd as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared", feature = "wtypes"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, pfinsertable as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn InsertTextAtSelection(&self, dwflags: super::DWORD, pchtext: &[super::WCHAR], ppastart: *mut Option<IAnchor>) -> windows_core::Result<IAnchor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), dwflags, pchtext.as_ptr(), pchtext.len().try_into().unwrap(), core::mem::transmute(ppastart), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn InsertEmbeddedAtSelection<P1>(&self, dwflags: super::DWORD, pdataobject: P1, ppastart: *mut Option<IAnchor>) -> windows_core::Result<IAnchor>
    where
        P1: windows_core::Param<super::IDataObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), dwflags, pdataobject.param().abi(), core::mem::transmute(ppastart), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    AdviseSink: usize,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RequestLock: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestLock: usize,
    #[cfg(feature = "shared")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TS_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatus: usize,
    #[cfg(feature = "shared")]
    pub QueryInsert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    QueryInsert: usize,
    #[cfg(feature = "shared")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::ULONG, *mut TS_SELECTION_ANCHOR, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSelection: usize,
    #[cfg(feature = "shared")]
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const TS_SELECTION_ANCHOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSelection: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::WCHAR, super::ULONG, *mut super::ULONG, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetText: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::WCHAR, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetText: usize,
    #[cfg(feature = "objidl")]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetFormattedText: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *const windows_core::GUID, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetEmbedded: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    InsertEmbedded: usize,
    #[cfg(feature = "shared")]
    pub RequestSupportedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::ULONG, *const TS_ATTRID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestSupportedAttrs: usize,
    #[cfg(feature = "shared")]
    pub RequestAttrsAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG, *const TS_ATTRID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestAttrsAtPosition: usize,
    #[cfg(feature = "shared")]
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG, *const TS_ATTRID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestAttrsTransitioningAtPosition: usize,
    #[cfg(feature = "shared")]
    pub FindNextAttrTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG, *const TS_ATTRID, super::DWORD, *mut windows_core::BOOL, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FindNextAttrTransition: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut TS_ATTRVAL, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    RetrieveRequestedAttrs: usize,
    pub GetStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TsViewCookie) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetAnchorFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *const super::POINT, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetAnchorFromPoint: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::RECT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetTextExt: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetScreenExt: usize,
    #[cfg(feature = "windef")]
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, TsViewCookie, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWnd: usize,
    #[cfg(all(feature = "objidl", feature = "shared", feature = "wtypes"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::FORMATETC, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared", feature = "wtypes")))]
    QueryInsertEmbedded: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::WCHAR, super::ULONG, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    InsertTextAtSelection: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    InsertEmbeddedAtSelection: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITextStoreAnchor_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const super::IID, punk: windows_core::Ref<windows_core::IUnknown>, dwmask: super::DWORD) -> windows_core::Result<()>;
    fn UnadviseSink(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RequestLock(&self, dwlockflags: super::DWORD, phrsession: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetStatus(&self, pdcs: *mut TS_STATUS) -> windows_core::Result<()>;
    fn QueryInsert(&self, pateststart: windows_core::Ref<IAnchor>, patestend: windows_core::Ref<IAnchor>, cch: super::ULONG, pparesultstart: windows_core::OutRef<IAnchor>) -> windows_core::Result<IAnchor>;
    fn GetSelection(&self, ulindex: super::ULONG, ulcount: super::ULONG, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn SetSelection(&self, ulcount: super::ULONG, pselection: *const TS_SELECTION_ANCHOR) -> windows_core::Result<()>;
    fn GetText(&self, dwflags: super::DWORD, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, pchtext: *mut super::WCHAR, cchreq: super::ULONG, pcch: *mut super::ULONG, fupdateanchor: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetText(&self, dwflags: super::DWORD, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, pchtext: *const super::WCHAR, cch: super::ULONG) -> windows_core::Result<()>;
    fn GetFormattedText(&self, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>) -> windows_core::Result<super::IDataObject>;
    fn GetEmbedded(&self, dwflags: super::DWORD, papos: windows_core::Ref<IAnchor>, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn InsertEmbedded(&self, dwflags: super::DWORD, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, pdataobject: windows_core::Ref<super::IDataObject>) -> windows_core::Result<()>;
    fn RequestSupportedAttrs(&self, dwflags: super::DWORD, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn RequestAttrsAtPosition(&self, papos: windows_core::Ref<IAnchor>, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(&self, papos: windows_core::Ref<IAnchor>, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn FindNextAttrTransition(&self, pastart: windows_core::Ref<IAnchor>, pahalt: windows_core::Ref<IAnchor>, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD, pffound: *mut windows_core::BOOL, plfoundoffset: *mut super::LONG) -> windows_core::Result<()>;
    fn RetrieveRequestedAttrs(&self, ulcount: super::ULONG, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetStart(&self) -> windows_core::Result<IAnchor>;
    fn GetEnd(&self) -> windows_core::Result<IAnchor>;
    fn GetActiveView(&self, pvcview: *mut TsViewCookie) -> windows_core::Result<()>;
    fn GetAnchorFromPoint(&self, vcview: TsViewCookie, ptscreen: *const super::POINT, dwflags: super::DWORD) -> windows_core::Result<IAnchor>;
    fn GetTextExt(&self, vcview: TsViewCookie, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, prc: *mut super::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self, vcview: TsViewCookie, prc: *mut super::RECT) -> windows_core::Result<()>;
    fn GetWnd(&self, vcview: TsViewCookie, phwnd: *mut super::HWND) -> windows_core::Result<()>;
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn InsertTextAtSelection(&self, dwflags: super::DWORD, pchtext: *const super::WCHAR, cch: super::ULONG, ppastart: windows_core::OutRef<IAnchor>) -> windows_core::Result<IAnchor>;
    fn InsertEmbeddedAtSelection(&self, dwflags: super::DWORD, pdataobject: windows_core::Ref<super::IDataObject>, ppastart: windows_core::OutRef<IAnchor>) -> windows_core::Result<IAnchor>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITextStoreAnchor_Vtbl {
    pub const fn new<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, punk: *mut core::ffi::c_void, dwmask: super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RequestLock<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: super::DWORD, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::RequestLock(this, core::mem::transmute_copy(&dwlockflags), core::mem::transmute_copy(&phrsession)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TS_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetStatus(this, core::mem::transmute_copy(&pdcs)).into()
            }
        }
        unsafe extern "system" fn QueryInsert<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pateststart: *mut core::ffi::c_void, patestend: *mut core::ffi::c_void, cch: super::ULONG, pparesultstart: *mut *mut core::ffi::c_void, pparesultend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::QueryInsert(this, core::mem::transmute_copy(&pateststart), core::mem::transmute_copy(&patestend), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&pparesultstart)) {
                    Ok(ok__) => {
                        pparesultend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: super::ULONG, ulcount: super::ULONG, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetSelection(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, pselection: *const TS_SELECTION_ANCHOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::SetSelection(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pchtext: *mut super::WCHAR, cchreq: super::ULONG, pcch: *mut super::ULONG, fupdateanchor: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetText(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&fupdateanchor)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pchtext: *const super::WCHAR, cch: super::ULONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetEmbedded<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, papos: *mut core::ffi::c_void, rguidservice: *const windows_core::GUID, riid: *const super::IID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&papos), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::InsertEmbedded(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend), core::mem::transmute_copy(&pdataobject)).into()
            }
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::RequestSupportedAttrs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papos: *mut core::ffi::c_void, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::RequestAttrsAtPosition(this, core::mem::transmute_copy(&papos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papos: *mut core::ffi::c_void, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::RequestAttrsTransitioningAtPosition(this, core::mem::transmute_copy(&papos), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, pahalt: *mut core::ffi::c_void, cfilterattrs: super::ULONG, pafilterattrs: *const TS_ATTRID, dwflags: super::DWORD, pffound: *mut windows_core::BOOL, plfoundoffset: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::FindNextAttrTransition(this, core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&pahalt), core::mem::transmute_copy(&cfilterattrs), core::mem::transmute_copy(&pafilterattrs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pffound), core::mem::transmute_copy(&plfoundoffset)).into()
            }
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
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
                ITextStoreAnchor_Impl::GetActiveView(this, core::mem::transmute_copy(&pvcview)).into()
            }
        }
        unsafe extern "system" fn GetAnchorFromPoint<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, ptscreen: *const super::POINT, dwflags: super::DWORD, ppasite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetTextExt<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, prc: *mut super::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetTextExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
            }
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, prc: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetScreenExt(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcview: TsViewCookie, phwnd: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::GetWnd(this, core::mem::transmute_copy(&vcview), core::mem::transmute_copy(&phwnd)).into()
            }
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchor_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pfinsertable)).into()
            }
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pchtext: *const super::WCHAR, cch: super::ULONG, ppastart: *mut *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&ppastart)) {
                    Ok(ok__) => {
                        ppaend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITextStoreAnchor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pdataobject: *mut core::ffi::c_void, ppastart: *mut *mut core::ffi::c_void, ppaend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreAnchor_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&ppastart)) {
                    Ok(ok__) => {
                        ppaend.write(core::mem::transmute(ok__));
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITextStoreAnchor {}
windows_core::imp::define_interface!(ITextStoreAnchorSink, ITextStoreAnchorSink_Vtbl, 0xaa80e905_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITextStoreAnchorSink, windows_core::IUnknown);
impl ITextStoreAnchorSink {
    #[cfg(feature = "shared")]
    pub unsafe fn OnTextChange<P1, P2>(&self, dwflags: super::DWORD, pastart: P1, paend: P2) -> windows_core::HRESULT
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
    #[cfg(feature = "shared")]
    pub unsafe fn OnStatusChange(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnAttrsChange<P0, P1>(&self, pastart: P0, paend: P1, paattrs: &[TS_ATTRID]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAnchor>,
        P1: windows_core::Param<IAnchor>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnAttrsChange)(windows_core::Interface::as_raw(self), pastart.param().abi(), paend.param().abi(), paattrs.len().try_into().unwrap(), paattrs.as_ptr()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnLockGranted(&self, dwlockflags: super::DWORD) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub OnTextChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnTextChange: usize,
    pub OnSelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void, TsLayoutCode, TsViewCookie) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnStatusChange: usize,
    #[cfg(feature = "shared")]
    pub OnAttrsChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG, *const TS_ATTRID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnAttrsChange: usize,
    #[cfg(feature = "shared")]
    pub OnLockGranted: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnLockGranted: usize,
    pub OnStartEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait ITextStoreAnchorSink_Impl: windows_core::IUnknownImpl {
    fn OnTextChange(&self, dwflags: super::DWORD, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>) -> windows_core::Result<()>;
    fn OnSelectionChange(&self) -> windows_core::Result<()>;
    fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: TsViewCookie) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn OnAttrsChange(&self, pastart: windows_core::Ref<IAnchor>, paend: windows_core::Ref<IAnchor>, cattrs: super::ULONG, paattrs: *const TS_ATTRID) -> windows_core::Result<()>;
    fn OnLockGranted(&self, dwlockflags: super::DWORD) -> windows_core::Result<()>;
    fn OnStartEditTransaction(&self) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITextStoreAnchorSink_Vtbl {
    pub const fn new<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTextChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn OnStatusChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pastart: *mut core::ffi::c_void, paend: *mut core::ffi::c_void, cattrs: super::ULONG, paattrs: *const TS_ATTRID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreAnchorSink_Impl::OnAttrsChange(this, core::mem::transmute_copy(&pastart), core::mem::transmute_copy(&paend), core::mem::transmute_copy(&cattrs), core::mem::transmute_copy(&paattrs)).into()
            }
        }
        unsafe extern "system" fn OnLockGranted<Identity: ITextStoreAnchorSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlockflags: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITextStoreAnchorSink {}
pub const TS_AE_END: TsActiveSelEnd = 2;
pub const TS_AE_NONE: TsActiveSelEnd = 0;
pub const TS_AE_START: TsActiveSelEnd = 1;
pub const TS_AS_ALL_SINKS: i32 = 31;
pub const TS_AS_ATTR_CHANGE: i32 = 8;
pub const TS_AS_LAYOUT_CHANGE: i32 = 4;
pub const TS_AS_SEL_CHANGE: i32 = 2;
pub const TS_AS_STATUS_CHANGE: i32 = 16;
pub const TS_AS_TEXT_CHANGE: i32 = 1;
pub type TS_ATTRID = windows_core::GUID;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct TS_ATTRVAL {
    pub idAttr: TS_ATTRID,
    pub dwOverlapId: super::DWORD,
    pub varValue: super::VARIANT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for TS_ATTRVAL {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TS_ATTR_FIND_BACKWARDS: i32 = 1;
pub const TS_ATTR_FIND_HIDDEN: i32 = 32;
pub const TS_ATTR_FIND_UPDATESTART: i32 = 4;
pub const TS_ATTR_FIND_WANT_END: i32 = 16;
pub const TS_ATTR_FIND_WANT_OFFSET: i32 = 2;
pub const TS_ATTR_FIND_WANT_VALUE: i32 = 8;
pub const TS_CHAR_EMBEDDED: i32 = 65532;
pub const TS_CHAR_REGION: i32 = 0;
pub const TS_CHAR_REPLACEMENT: i32 = 65533;
pub const TS_CH_FOLLOWING_DEL: i32 = 2;
pub const TS_CH_PRECEDING_DEL: i32 = 1;
#[cfg(feature = "shared")]
pub const TS_DEFAULT_SELECTION: super::ULONG = 4294967295;
pub const TS_E_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8004020A_u32 as _);
pub const TS_E_INVALIDPOINT: windows_core::HRESULT = windows_core::HRESULT(0x80040207_u32 as _);
pub const TS_E_INVALIDPOS: windows_core::HRESULT = windows_core::HRESULT(0x80040200_u32 as _);
pub const TS_E_NOINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80040204_u32 as _);
pub const TS_E_NOLAYOUT: windows_core::HRESULT = windows_core::HRESULT(0x80040206_u32 as _);
pub const TS_E_NOLOCK: windows_core::HRESULT = windows_core::HRESULT(0x80040201_u32 as _);
pub const TS_E_NOOBJECT: windows_core::HRESULT = windows_core::HRESULT(0x80040202_u32 as _);
pub const TS_E_NOSELECTION: windows_core::HRESULT = windows_core::HRESULT(0x80040205_u32 as _);
pub const TS_E_NOSERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80040203_u32 as _);
pub const TS_E_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x80040209_u32 as _);
pub const TS_E_SYNCHRONOUS: windows_core::HRESULT = windows_core::HRESULT(0x80040208_u32 as _);
pub const TS_GEA_HIDDEN: i32 = 1;
pub const TS_GR_BACKWARD: TsGravity = 0;
pub const TS_GR_FORWARD: TsGravity = 1;
pub const TS_GTA_HIDDEN: i32 = 1;
pub const TS_IAS_NOQUERY: i32 = 1;
pub const TS_IAS_QUERYONLY: i32 = 2;
pub const TS_IE_COMPOSITION: i32 = 2;
pub const TS_IE_CORRECTION: i32 = 1;
pub const TS_LC_CHANGE: TsLayoutCode = 1;
pub const TS_LC_CREATE: TsLayoutCode = 0;
pub const TS_LC_DESTROY: TsLayoutCode = 2;
pub const TS_LF_READ: i32 = 2;
pub const TS_LF_READWRITE: i32 = 6;
pub const TS_LF_SYNC: i32 = 1;
pub const TS_RT_HIDDEN: TsRunType = 1;
pub const TS_RT_OPAQUE: TsRunType = 2;
pub const TS_RT_PLAIN: TsRunType = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TS_RUNINFO {
    pub uCount: super::ULONG,
    pub r#type: TsRunType,
}
pub const TS_SD_BACKWARD: TsShiftDir = 0;
pub const TS_SD_DISABLEWRITINGSUGGESTIONS: i32 = 512;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED: i32 = 128;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE: i32 = 256;
pub const TS_SD_FORWARD: TsShiftDir = 1;
pub const TS_SD_INPUTPANEMANUALDISPLAYENABLE: i32 = 64;
pub const TS_SD_LOADING: i32 = 2;
pub const TS_SD_MASKALL: i32 = 3;
pub const TS_SD_READONLY: i32 = 1;
pub const TS_SD_RESERVED: i32 = 4;
pub const TS_SD_TKBAUTOCORRECTENABLE: i32 = 8;
pub const TS_SD_TKBPREDICTIONENABLE: i32 = 16;
pub const TS_SD_UIINTEGRATIONENABLE: i32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TS_SELECTIONSTYLE {
    pub ase: TsActiveSelEnd,
    pub fInterimChar: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TS_SELECTION_ACP {
    pub acpStart: super::LONG,
    pub acpEnd: super::LONG,
    pub style: TS_SELECTIONSTYLE,
}
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TS_SELECTION_ANCHOR {
    pub paStart: core::mem::ManuallyDrop<Option<IAnchor>>,
    pub paEnd: core::mem::ManuallyDrop<Option<IAnchor>>,
    pub style: TS_SELECTIONSTYLE,
}
pub const TS_SHIFT_COUNT_HIDDEN: i32 = 1;
pub const TS_SHIFT_COUNT_ONLY: i32 = 8;
pub const TS_SHIFT_HALT_HIDDEN: i32 = 2;
pub const TS_SHIFT_HALT_VISIBLE: i32 = 4;
pub const TS_SS_DISJOINTSEL: i32 = 1;
pub const TS_SS_MULTILINE: i32 = 128;
pub const TS_SS_NOHIDDENTEXT: i32 = 8;
pub const TS_SS_REGIONS: i32 = 2;
pub const TS_SS_TKBAUTOCORRECTENABLE: i32 = 16;
pub const TS_SS_TKBPREDICTIONENABLE: i32 = 32;
pub const TS_SS_TRANSITORY: i32 = 4;
pub const TS_SS_UWPCONTROL: i32 = 64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TS_STATUS {
    pub dwDynamicFlags: super::DWORD,
    pub dwStaticFlags: super::DWORD,
}
pub const TS_ST_CORRECTION: i32 = 1;
pub const TS_S_ASYNC: windows_core::HRESULT = windows_core::HRESULT(0x40300_u32 as _);
pub const TS_TC_CORRECTION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TS_TEXTCHANGE {
    pub acpStart: super::LONG,
    pub acpOldEnd: super::LONG,
    pub acpNewEnd: super::LONG,
}
pub const TS_VCOOKIE_NUL: u32 = 4294967295;
pub type TsActiveSelEnd = i32;
pub type TsGravity = i32;
pub type TsLayoutCode = i32;
pub type TsRunType = i32;
pub type TsShiftDir = i32;
pub type TsViewCookie = u32;
