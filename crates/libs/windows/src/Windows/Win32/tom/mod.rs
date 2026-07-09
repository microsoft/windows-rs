#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextDisplays, ITextDisplays_Vtbl, 0xc241f5f2_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextDisplays {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextDisplays, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDisplays_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextDisplays_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextDisplays_Vtbl {
    pub const fn new<Identity: ITextDisplays_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDisplays as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextDisplays {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextDocument, ITextDocument_Vtbl, 0x8cc497c0_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextDocument {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextDocument, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ITextDocument {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSelection(&self) -> windows_core::Result<ITextSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStoryCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStoryRanges(&self) -> windows_core::Result<ITextStoryRanges> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryRanges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSaved(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSaved)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSaved(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSaved)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetDefaultTabStop(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultTabStop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultTabStop)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn New(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).New)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Open(&self, pvar: *const super::oaidl::VARIANT, flags: i32, codepage: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), flags, codepage) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Save(&self, pvar: *const super::oaidl::VARIANT, flags: i32, codepage: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), flags, codepage) }
    }
    pub unsafe fn Freeze(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Freeze)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unfreeze(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Unfreeze)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginEditCollection(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginEditCollection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndEditCollection(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndEditCollection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Undo(&self, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Undo)(windows_core::Interface::as_raw(self), count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Redo(&self, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Redo)(windows_core::Interface::as_raw(self), count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Range)(windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromPoint)(windows_core::Interface::as_raw(self), x, y, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetStoryRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSaved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSaved: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDefaultTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDefaultTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub New: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Open: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Save: usize,
    pub Freeze: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Unfreeze: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BeginEditCollection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndEditCollection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Undo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Redo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Range: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextDocument_Impl: super::oaidl::IDispatch_Impl {
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSelection(&self) -> windows_core::Result<ITextSelection>;
    fn GetStoryCount(&self) -> windows_core::Result<i32>;
    fn GetStoryRanges(&self) -> windows_core::Result<ITextStoryRanges>;
    fn GetSaved(&self) -> windows_core::Result<i32>;
    fn SetSaved(&self, value: i32) -> windows_core::Result<()>;
    fn GetDefaultTabStop(&self) -> windows_core::Result<f32>;
    fn SetDefaultTabStop(&self, value: f32) -> windows_core::Result<()>;
    fn New(&self) -> windows_core::Result<()>;
    fn Open(&self, pvar: *const super::oaidl::VARIANT, flags: i32, codepage: i32) -> windows_core::Result<()>;
    fn Save(&self, pvar: *const super::oaidl::VARIANT, flags: i32, codepage: i32) -> windows_core::Result<()>;
    fn Freeze(&self) -> windows_core::Result<i32>;
    fn Unfreeze(&self) -> windows_core::Result<i32>;
    fn BeginEditCollection(&self) -> windows_core::Result<()>;
    fn EndEditCollection(&self) -> windows_core::Result<()>;
    fn Undo(&self, count: i32) -> windows_core::Result<i32>;
    fn Redo(&self, count: i32) -> windows_core::Result<i32>;
    fn Range(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange>;
    fn RangeFromPoint(&self, x: i32, y: i32) -> windows_core::Result<ITextRange>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextDocument_Vtbl {
    pub const fn new<Identity: ITextDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetName(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        ppsel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoryCount<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetStoryCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoryRanges<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstories: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetStoryRanges(this) {
                    Ok(ok__) => {
                        ppstories.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSaved<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetSaved(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSaved<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetSaved(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDefaultTabStop<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::GetDefaultTabStop(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::SetDefaultTabStop(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn New<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::New(this).into()
            }
        }
        unsafe extern "system" fn Open<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::oaidl::VARIANT, flags: i32, codepage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::Open(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&codepage)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::oaidl::VARIANT, flags: i32, codepage: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::Save(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&codepage)).into()
            }
        }
        unsafe extern "system" fn Freeze<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Freeze(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unfreeze<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Unfreeze(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginEditCollection<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::BeginEditCollection(this).into()
            }
        }
        unsafe extern "system" fn EndEditCollection<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument_Impl::EndEditCollection(this).into()
            }
        }
        unsafe extern "system" fn Undo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Undo(this, core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Redo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Redo(this, core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Range<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::Range(this, core::mem::transmute_copy(&cpactive), core::mem::transmute_copy(&cpanchor)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RangeFromPoint<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument_Impl::RangeFromPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            GetStoryCount: GetStoryCount::<Identity, OFFSET>,
            GetStoryRanges: GetStoryRanges::<Identity, OFFSET>,
            GetSaved: GetSaved::<Identity, OFFSET>,
            SetSaved: SetSaved::<Identity, OFFSET>,
            GetDefaultTabStop: GetDefaultTabStop::<Identity, OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Identity, OFFSET>,
            New: New::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Freeze: Freeze::<Identity, OFFSET>,
            Unfreeze: Unfreeze::<Identity, OFFSET>,
            BeginEditCollection: BeginEditCollection::<Identity, OFFSET>,
            EndEditCollection: EndEditCollection::<Identity, OFFSET>,
            Undo: Undo::<Identity, OFFSET>,
            Redo: Redo::<Identity, OFFSET>,
            Range: Range::<Identity, OFFSET>,
            RangeFromPoint: RangeFromPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextDocument {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextDocument2, ITextDocument2_Vtbl, 0xc241f5e0_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextDocument2 {
    type Target = ITextDocument;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextDocument2, windows_core::IUnknown, super::oaidl::IDispatch, ITextDocument);
#[cfg(feature = "Win32_oaidl")]
impl ITextDocument2 {
    pub unsafe fn GetCaretType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaretType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCaretType(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCaretType)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetDisplays(&self) -> windows_core::Result<ITextDisplays> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplays)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocumentFont(&self) -> windows_core::Result<ITextFont2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentFont)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDocumentFont<P0>(&self, pfont: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextFont2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentFont)(windows_core::Interface::as_raw(self), pfont.param().abi()) }
    }
    pub unsafe fn GetDocumentPara(&self) -> windows_core::Result<ITextPara2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentPara)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDocumentPara<P0>(&self, ppara: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextPara2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentPara)(windows_core::Interface::as_raw(self), ppara.param().abi()) }
    }
    pub unsafe fn GetEastAsianFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEastAsianFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGenerator(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGenerator)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetIMEInProgress(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIMEInProgress)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetNotificationMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNotificationMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotificationMode(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationMode)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetSelection2(&self) -> windows_core::Result<ITextSelection2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStoryRanges2(&self) -> windows_core::Result<ITextStoryRanges2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryRanges2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTypographyOptions(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypographyOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetWindow(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AttachMsgFilter<P0>(&self, pfilter: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AttachMsgFilter)(windows_core::Interface::as_raw(self), pfilter.param().abi()) }
    }
    pub unsafe fn CheckTextLimit(&self, cch: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckTextLimit)(windows_core::Interface::as_raw(self), cch, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCallManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientRect)(windows_core::Interface::as_raw(self), r#type, pleft as _, ptop as _, pright as _, pbottom as _) }
    }
    pub unsafe fn GetEffectColor(&self, index: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEffectColor)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetImmContext(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredFont)(windows_core::Interface::as_raw(self), cp, charrep, options, curcharrep, curfontsize, core::mem::transmute(pbstr), ppitchandfamily as _, pnewfontsize as _) }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStrings(&self) -> windows_core::Result<ITextStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Notify(&self, notify: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), notify) }
    }
    pub unsafe fn Range2(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Range2)(windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RangeFromPoint2(&self, x: i32, y: i32, r#type: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromPoint2)(windows_core::Interface::as_raw(self), x, y, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReleaseCallManager<P0>(&self, pvoid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseCallManager)(windows_core::Interface::as_raw(self), pvoid.param().abi()) }
    }
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseImmContext)(windows_core::Interface::as_raw(self), context) }
    }
    pub unsafe fn SetEffectColor(&self, index: i32, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEffectColor)(windows_core::Interface::as_raw(self), index, value) }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value) }
    }
    pub unsafe fn SetTypographyOptions(&self, options: i32, mask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTypographyOptions)(windows_core::Interface::as_raw(self), options, mask) }
    }
    pub unsafe fn SysBeep(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SysBeep)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Update(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn UpdateWindow(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateWindow)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMathProperties(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMathProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMathProperties(&self, options: i32, mask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMathProperties)(windows_core::Interface::as_raw(self), options, mask) }
    }
    pub unsafe fn GetActiveStory(&self) -> windows_core::Result<ITextStory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveStory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetActiveStory<P0>(&self, pstory: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextStory>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActiveStory)(windows_core::Interface::as_raw(self), pstory.param().abi()) }
    }
    pub unsafe fn GetMainStory(&self) -> windows_core::Result<ITextStory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMainStory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNewStory(&self) -> windows_core::Result<ITextStory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNewStory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStory(&self, index: i32) -> windows_core::Result<ITextStory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStory)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2_Vtbl {
    pub base__: ITextDocument_Vtbl,
    pub GetCaretType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCaretType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDisplays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDocumentFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDocumentPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEastAsianFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetGenerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIMEInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSelection2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoryRanges2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTypographyOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub AttachMsgFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckTextLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetCallManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClientRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetEffectColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetImmContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetPreferredFont: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, *mut *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetStrings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Range2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RangeFromPoint2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseCallManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseImmContext: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetEffectColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetTypographyOptions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SysBeep: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UpdateWindow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMathProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMathProperties: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetActiveStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActiveStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMainStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNewStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStory: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextDocument2_Impl: ITextDocument_Impl {
    fn GetCaretType(&self) -> windows_core::Result<i32>;
    fn SetCaretType(&self, value: i32) -> windows_core::Result<()>;
    fn GetDisplays(&self) -> windows_core::Result<ITextDisplays>;
    fn GetDocumentFont(&self) -> windows_core::Result<ITextFont2>;
    fn SetDocumentFont(&self, pfont: windows_core::Ref<ITextFont2>) -> windows_core::Result<()>;
    fn GetDocumentPara(&self) -> windows_core::Result<ITextPara2>;
    fn SetDocumentPara(&self, ppara: windows_core::Ref<ITextPara2>) -> windows_core::Result<()>;
    fn GetEastAsianFlags(&self) -> windows_core::Result<i32>;
    fn GetGenerator(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetIMEInProgress(&self, value: i32) -> windows_core::Result<()>;
    fn GetNotificationMode(&self) -> windows_core::Result<i32>;
    fn SetNotificationMode(&self, value: i32) -> windows_core::Result<()>;
    fn GetSelection2(&self) -> windows_core::Result<ITextSelection2>;
    fn GetStoryRanges2(&self) -> windows_core::Result<ITextStoryRanges2>;
    fn GetTypographyOptions(&self) -> windows_core::Result<i32>;
    fn GetVersion(&self) -> windows_core::Result<i32>;
    fn GetWindow(&self) -> windows_core::Result<i64>;
    fn AttachMsgFilter(&self, pfilter: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CheckTextLimit(&self, cch: i32) -> windows_core::Result<i32>;
    fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()>;
    fn GetEffectColor(&self, index: i32) -> windows_core::Result<i32>;
    fn GetImmContext(&self) -> windows_core::Result<i64>;
    fn GetPreferredFont(&self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetStrings(&self) -> windows_core::Result<ITextStrings>;
    fn Notify(&self, notify: i32) -> windows_core::Result<()>;
    fn Range2(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2>;
    fn RangeFromPoint2(&self, x: i32, y: i32, r#type: i32) -> windows_core::Result<ITextRange2>;
    fn ReleaseCallManager(&self, pvoid: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ReleaseImmContext(&self, context: i64) -> windows_core::Result<()>;
    fn SetEffectColor(&self, index: i32, value: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
    fn SetTypographyOptions(&self, options: i32, mask: i32) -> windows_core::Result<()>;
    fn SysBeep(&self) -> windows_core::Result<()>;
    fn Update(&self, value: i32) -> windows_core::Result<()>;
    fn UpdateWindow(&self) -> windows_core::Result<()>;
    fn GetMathProperties(&self) -> windows_core::Result<i32>;
    fn SetMathProperties(&self, options: i32, mask: i32) -> windows_core::Result<()>;
    fn GetActiveStory(&self) -> windows_core::Result<ITextStory>;
    fn SetActiveStory(&self, pstory: windows_core::Ref<ITextStory>) -> windows_core::Result<()>;
    fn GetMainStory(&self) -> windows_core::Result<ITextStory>;
    fn GetNewStory(&self) -> windows_core::Result<ITextStory>;
    fn GetStory(&self, index: i32) -> windows_core::Result<ITextStory>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextDocument2_Vtbl {
    pub const fn new<Identity: ITextDocument2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCaretType<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetCaretType(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetCaretType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDisplays<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisplays: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetDisplays(this) {
                    Ok(ok__) => {
                        ppdisplays.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentFont<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetDocumentFont(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDocumentFont<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetDocumentFont(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetDocumentPara<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetDocumentPara(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDocumentPara<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetDocumentPara(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn GetEastAsianFlags<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetEastAsianFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGenerator<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetGenerator(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIMEInProgress<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetIMEInProgress(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetNotificationMode<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetNotificationMode(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationMode<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetNotificationMode(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSelection2<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetSelection2(this) {
                    Ok(ok__) => {
                        ppsel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoryRanges2<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstories: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetStoryRanges2(this) {
                    Ok(ok__) => {
                        ppstories.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypographyOptions<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetTypographyOptions(this) {
                    Ok(ok__) => {
                        poptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWindow<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AttachMsgFilter<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::AttachMsgFilter(this, core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn CheckTextLimit<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: i32, pcch: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::CheckTextLimit(this, core::mem::transmute_copy(&cch)) {
                    Ok(ok__) => {
                        pcch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCallManager<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvoid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetCallManager(this) {
                    Ok(ok__) => {
                        ppvoid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClientRect<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::GetClientRect(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom)).into()
            }
        }
        unsafe extern "system" fn GetEffectColor<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetEffectColor(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImmContext<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetImmContext(this) {
                    Ok(ok__) => {
                        pcontext.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreferredFont<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut *mut core::ffi::c_void, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::GetPreferredFont(this, core::mem::transmute_copy(&cp), core::mem::transmute_copy(&charrep), core::mem::transmute_copy(&options), core::mem::transmute_copy(&curcharrep), core::mem::transmute_copy(&curfontsize), core::mem::transmute_copy(&pbstr), core::mem::transmute_copy(&ppitchandfamily), core::mem::transmute_copy(&pnewfontsize)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStrings<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetStrings(this) {
                    Ok(ok__) => {
                        ppstrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Notify<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notify: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::Notify(this, core::mem::transmute_copy(&notify)).into()
            }
        }
        unsafe extern "system" fn Range2<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::Range2(this, core::mem::transmute_copy(&cpactive), core::mem::transmute_copy(&cpanchor)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RangeFromPoint2<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, r#type: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::RangeFromPoint2(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvoid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::ReleaseCallManager(this, core::mem::transmute_copy(&pvoid)).into()
            }
        }
        unsafe extern "system" fn ReleaseImmContext<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::ReleaseImmContext(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn SetEffectColor<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetEffectColor(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetTypographyOptions<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetTypographyOptions(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn SysBeep<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SysBeep(this).into()
            }
        }
        unsafe extern "system" fn Update<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::Update(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn UpdateWindow<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::UpdateWindow(this).into()
            }
        }
        unsafe extern "system" fn GetMathProperties<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetMathProperties(this) {
                    Ok(ok__) => {
                        poptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMathProperties<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetMathProperties(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn GetActiveStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetActiveStory(this) {
                    Ok(ok__) => {
                        ppstory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActiveStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2_Impl::SetActiveStory(this, core::mem::transmute_copy(&pstory)).into()
            }
        }
        unsafe extern "system" fn GetMainStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetMainStory(this) {
                    Ok(ok__) => {
                        ppstory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNewStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetNewStory(this) {
                    Ok(ok__) => {
                        ppstory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStory<Identity: ITextDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ppstory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2_Impl::GetStory(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppstory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITextDocument_Vtbl::new::<Identity, OFFSET>(),
            GetCaretType: GetCaretType::<Identity, OFFSET>,
            SetCaretType: SetCaretType::<Identity, OFFSET>,
            GetDisplays: GetDisplays::<Identity, OFFSET>,
            GetDocumentFont: GetDocumentFont::<Identity, OFFSET>,
            SetDocumentFont: SetDocumentFont::<Identity, OFFSET>,
            GetDocumentPara: GetDocumentPara::<Identity, OFFSET>,
            SetDocumentPara: SetDocumentPara::<Identity, OFFSET>,
            GetEastAsianFlags: GetEastAsianFlags::<Identity, OFFSET>,
            GetGenerator: GetGenerator::<Identity, OFFSET>,
            SetIMEInProgress: SetIMEInProgress::<Identity, OFFSET>,
            GetNotificationMode: GetNotificationMode::<Identity, OFFSET>,
            SetNotificationMode: SetNotificationMode::<Identity, OFFSET>,
            GetSelection2: GetSelection2::<Identity, OFFSET>,
            GetStoryRanges2: GetStoryRanges2::<Identity, OFFSET>,
            GetTypographyOptions: GetTypographyOptions::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetWindow: GetWindow::<Identity, OFFSET>,
            AttachMsgFilter: AttachMsgFilter::<Identity, OFFSET>,
            CheckTextLimit: CheckTextLimit::<Identity, OFFSET>,
            GetCallManager: GetCallManager::<Identity, OFFSET>,
            GetClientRect: GetClientRect::<Identity, OFFSET>,
            GetEffectColor: GetEffectColor::<Identity, OFFSET>,
            GetImmContext: GetImmContext::<Identity, OFFSET>,
            GetPreferredFont: GetPreferredFont::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetStrings: GetStrings::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            Range2: Range2::<Identity, OFFSET>,
            RangeFromPoint2: RangeFromPoint2::<Identity, OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Identity, OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Identity, OFFSET>,
            SetEffectColor: SetEffectColor::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            SetTypographyOptions: SetTypographyOptions::<Identity, OFFSET>,
            SysBeep: SysBeep::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            UpdateWindow: UpdateWindow::<Identity, OFFSET>,
            GetMathProperties: GetMathProperties::<Identity, OFFSET>,
            SetMathProperties: SetMathProperties::<Identity, OFFSET>,
            GetActiveStory: GetActiveStory::<Identity, OFFSET>,
            SetActiveStory: SetActiveStory::<Identity, OFFSET>,
            GetMainStory: GetMainStory::<Identity, OFFSET>,
            GetNewStory: GetNewStory::<Identity, OFFSET>,
            GetStory: GetStory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ITextDocument as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextDocument2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextDocument2Old, ITextDocument2Old_Vtbl, 0x01c25500_4268_11d1_883a_3c8b00c10000);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextDocument2Old {
    type Target = ITextDocument;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextDocument2Old, windows_core::IUnknown, super::oaidl::IDispatch, ITextDocument);
#[cfg(feature = "Win32_oaidl")]
impl ITextDocument2Old {
    pub unsafe fn AttachMsgFilter<P0>(&self, pfilter: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AttachMsgFilter)(windows_core::Interface::as_raw(self), pfilter.param().abi()) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SetEffectColor(&self, index: i32, cr: super::windef::COLORREF) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEffectColor)(windows_core::Interface::as_raw(self), index, cr) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetEffectColor(&self, index: i32) -> windows_core::Result<super::windef::COLORREF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEffectColor)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCaretType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaretType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCaretType(&self, carettype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCaretType)(windows_core::Interface::as_raw(self), carettype) }
    }
    pub unsafe fn GetImmContext(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseImmContext)(windows_core::Interface::as_raw(self), context) }
    }
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredFont)(windows_core::Interface::as_raw(self), cp, charrep, option, charrepcur, curfontsize, core::mem::transmute(pbstr), ppitchandfamily as _, pnewfontsize as _) }
    }
    pub unsafe fn GetNotificationMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNotificationMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotificationMode(&self, mode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationMode)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientRect)(windows_core::Interface::as_raw(self), r#type, pleft as _, ptop as _, pright as _, pbottom as _) }
    }
    pub unsafe fn GetSelection2(&self) -> windows_core::Result<ITextSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWindow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFEFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFEFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateWindow(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateWindow)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CheckTextLimit(&self, cch: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckTextLimit)(windows_core::Interface::as_raw(self), cch, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IMEInProgress(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IMEInProgress)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn SysBeep(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SysBeep)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Update(&self, mode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn Notify(&self, notify: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), notify) }
    }
    pub unsafe fn GetDocumentFont(&self) -> windows_core::Result<ITextFont> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentFont)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocumentPara(&self) -> windows_core::Result<ITextPara> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentPara)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCallManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReleaseCallManager<P0>(&self, pvoid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseCallManager)(windows_core::Interface::as_raw(self), pvoid.param().abi()) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2Old_Vtbl {
    pub base__: ITextDocument_Vtbl,
    pub AttachMsgFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub SetEffectColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::windef::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SetEffectColor: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetEffectColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::windef::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetEffectColor: usize,
    pub GetCaretType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCaretType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetImmContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub ReleaseImmContext: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetPreferredFont: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, *mut *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetClientRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetSelection2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFEFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UpdateWindow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckTextLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub IMEInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SysBeep: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDocumentFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCallManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseCallManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextDocument2Old_Impl: ITextDocument_Impl {
    fn AttachMsgFilter(&self, pfilter: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetEffectColor(&self, index: i32, cr: super::windef::COLORREF) -> windows_core::Result<()>;
    fn GetEffectColor(&self, index: i32) -> windows_core::Result<super::windef::COLORREF>;
    fn GetCaretType(&self) -> windows_core::Result<i32>;
    fn SetCaretType(&self, carettype: i32) -> windows_core::Result<()>;
    fn GetImmContext(&self) -> windows_core::Result<i64>;
    fn ReleaseImmContext(&self, context: i64) -> windows_core::Result<()>;
    fn GetPreferredFont(&self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::Result<()>;
    fn GetNotificationMode(&self) -> windows_core::Result<i32>;
    fn SetNotificationMode(&self, mode: i32) -> windows_core::Result<()>;
    fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()>;
    fn GetSelection2(&self) -> windows_core::Result<ITextSelection>;
    fn GetWindow(&self) -> windows_core::Result<i32>;
    fn GetFEFlags(&self) -> windows_core::Result<i32>;
    fn UpdateWindow(&self) -> windows_core::Result<()>;
    fn CheckTextLimit(&self, cch: i32) -> windows_core::Result<i32>;
    fn IMEInProgress(&self, value: i32) -> windows_core::Result<()>;
    fn SysBeep(&self) -> windows_core::Result<()>;
    fn Update(&self, mode: i32) -> windows_core::Result<()>;
    fn Notify(&self, notify: i32) -> windows_core::Result<()>;
    fn GetDocumentFont(&self) -> windows_core::Result<ITextFont>;
    fn GetDocumentPara(&self) -> windows_core::Result<ITextPara>;
    fn GetCallManager(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ReleaseCallManager(&self, pvoid: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextDocument2Old_Vtbl {
    pub const fn new<Identity: ITextDocument2Old_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachMsgFilter<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::AttachMsgFilter(this, core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn SetEffectColor<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, cr: super::windef::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::SetEffectColor(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&cr)).into()
            }
        }
        unsafe extern "system" fn GetEffectColor<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pcr: *mut super::windef::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetEffectColor(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pcr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCaretType<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcarettype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetCaretType(this) {
                    Ok(ok__) => {
                        pcarettype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, carettype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::SetCaretType(this, core::mem::transmute_copy(&carettype)).into()
            }
        }
        unsafe extern "system" fn GetImmContext<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetImmContext(this) {
                    Ok(ok__) => {
                        pcontext.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseImmContext<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::ReleaseImmContext(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn GetPreferredFont<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut *mut core::ffi::c_void, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::GetPreferredFont(this, core::mem::transmute_copy(&cp), core::mem::transmute_copy(&charrep), core::mem::transmute_copy(&option), core::mem::transmute_copy(&charrepcur), core::mem::transmute_copy(&curfontsize), core::mem::transmute_copy(&pbstr), core::mem::transmute_copy(&ppitchandfamily), core::mem::transmute_copy(&pnewfontsize)).into()
            }
        }
        unsafe extern "system" fn GetNotificationMode<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetNotificationMode(this) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationMode<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::SetNotificationMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn GetClientRect<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::GetClientRect(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom)).into()
            }
        }
        unsafe extern "system" fn GetSelection2<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetSelection2(this) {
                    Ok(ok__) => {
                        ppsel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWindow<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFEFlags<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetFEFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateWindow<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::UpdateWindow(this).into()
            }
        }
        unsafe extern "system" fn CheckTextLimit<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: i32, pcch: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::CheckTextLimit(this, core::mem::transmute_copy(&cch)) {
                    Ok(ok__) => {
                        pcch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IMEInProgress<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::IMEInProgress(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SysBeep<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::SysBeep(this).into()
            }
        }
        unsafe extern "system" fn Update<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::Update(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn Notify<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notify: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::Notify(this, core::mem::transmute_copy(&notify)).into()
            }
        }
        unsafe extern "system" fn GetDocumentFont<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitextfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetDocumentFont(this) {
                    Ok(ok__) => {
                        ppitextfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentPara<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitextpara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetDocumentPara(this) {
                    Ok(ok__) => {
                        ppitextpara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCallManager<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvoid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextDocument2Old_Impl::GetCallManager(this) {
                    Ok(ok__) => {
                        ppvoid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Identity: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvoid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextDocument2Old_Impl::ReleaseCallManager(this, core::mem::transmute_copy(&pvoid)).into()
            }
        }
        Self {
            base__: ITextDocument_Vtbl::new::<Identity, OFFSET>(),
            AttachMsgFilter: AttachMsgFilter::<Identity, OFFSET>,
            SetEffectColor: SetEffectColor::<Identity, OFFSET>,
            GetEffectColor: GetEffectColor::<Identity, OFFSET>,
            GetCaretType: GetCaretType::<Identity, OFFSET>,
            SetCaretType: SetCaretType::<Identity, OFFSET>,
            GetImmContext: GetImmContext::<Identity, OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Identity, OFFSET>,
            GetPreferredFont: GetPreferredFont::<Identity, OFFSET>,
            GetNotificationMode: GetNotificationMode::<Identity, OFFSET>,
            SetNotificationMode: SetNotificationMode::<Identity, OFFSET>,
            GetClientRect: GetClientRect::<Identity, OFFSET>,
            GetSelection2: GetSelection2::<Identity, OFFSET>,
            GetWindow: GetWindow::<Identity, OFFSET>,
            GetFEFlags: GetFEFlags::<Identity, OFFSET>,
            UpdateWindow: UpdateWindow::<Identity, OFFSET>,
            CheckTextLimit: CheckTextLimit::<Identity, OFFSET>,
            IMEInProgress: IMEInProgress::<Identity, OFFSET>,
            SysBeep: SysBeep::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            GetDocumentFont: GetDocumentFont::<Identity, OFFSET>,
            GetDocumentPara: GetDocumentPara::<Identity, OFFSET>,
            GetCallManager: GetCallManager::<Identity, OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument2Old as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ITextDocument as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextDocument2Old {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextFont, ITextFont_Vtbl, 0x8cc497c3_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextFont {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextFont, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ITextFont {
    pub unsafe fn GetDuplicate(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDuplicate<P0>(&self, pfont: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDuplicate)(windows_core::Interface::as_raw(self), pfont.param().abi()) }
    }
    pub unsafe fn CanChange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual<P0>(&self, pfont: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pfont.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStyle(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStyle)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetAllCaps(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllCaps(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllCaps)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetAnimation(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAnimation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAnimation(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAnimation)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetBackColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBackColor(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBackColor)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetBold(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBold)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBold(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBold)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetEmboss(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEmboss)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEmboss(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEmboss)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetForeColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetForeColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetForeColor(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetForeColor)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetHidden(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHidden(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHidden)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetEngrave(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEngrave)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEngrave(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEngrave)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetItalic(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItalic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetItalic(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetItalic)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetKerning(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKerning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKerning(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKerning)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetLanguageID(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguageID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLanguageID(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLanguageID)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstr: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)) }
    }
    pub unsafe fn GetOutline(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutline(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOutline)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetPosition(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPosition(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetProtected(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProtected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProtected(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProtected)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetShadow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetShadow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShadow(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetShadow)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSize(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetSmallCaps(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSmallCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSmallCaps(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSmallCaps)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetSpacing(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpacing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpacing(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSpacing)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetStrikeThrough(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStrikeThrough)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStrikeThrough(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStrikeThrough)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetSubscript(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubscript)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSubscript(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSubscript)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetSuperscript(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSuperscript)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSuperscript(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSuperscript)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetUnderline(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnderline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUnderline(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUnderline)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetWeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWeight(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWeight)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextFont_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAllCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAllCaps: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetBackColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetBold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBold: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEmboss: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEmboss: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetForeColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetForeColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEngrave: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEngrave: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetItalic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetKerning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOutline: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProtected: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetShadow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetShadow: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetSmallCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSmallCaps: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetStrikeThrough: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStrikeThrough: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSubscript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSuperscript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetWeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextFont_Impl: super::oaidl::IDispatch_Impl {
    fn GetDuplicate(&self) -> windows_core::Result<ITextFont>;
    fn SetDuplicate(&self, pfont: windows_core::Ref<ITextFont>) -> windows_core::Result<()>;
    fn CanChange(&self) -> windows_core::Result<i32>;
    fn IsEqual(&self, pfont: windows_core::Ref<ITextFont>) -> windows_core::Result<i32>;
    fn Reset(&self, value: i32) -> windows_core::Result<()>;
    fn GetStyle(&self) -> windows_core::Result<i32>;
    fn SetStyle(&self, value: i32) -> windows_core::Result<()>;
    fn GetAllCaps(&self) -> windows_core::Result<i32>;
    fn SetAllCaps(&self, value: i32) -> windows_core::Result<()>;
    fn GetAnimation(&self) -> windows_core::Result<i32>;
    fn SetAnimation(&self, value: i32) -> windows_core::Result<()>;
    fn GetBackColor(&self) -> windows_core::Result<i32>;
    fn SetBackColor(&self, value: i32) -> windows_core::Result<()>;
    fn GetBold(&self) -> windows_core::Result<i32>;
    fn SetBold(&self, value: i32) -> windows_core::Result<()>;
    fn GetEmboss(&self) -> windows_core::Result<i32>;
    fn SetEmboss(&self, value: i32) -> windows_core::Result<()>;
    fn GetForeColor(&self) -> windows_core::Result<i32>;
    fn SetForeColor(&self, value: i32) -> windows_core::Result<()>;
    fn GetHidden(&self) -> windows_core::Result<i32>;
    fn SetHidden(&self, value: i32) -> windows_core::Result<()>;
    fn GetEngrave(&self) -> windows_core::Result<i32>;
    fn SetEngrave(&self, value: i32) -> windows_core::Result<()>;
    fn GetItalic(&self) -> windows_core::Result<i32>;
    fn SetItalic(&self, value: i32) -> windows_core::Result<()>;
    fn GetKerning(&self) -> windows_core::Result<f32>;
    fn SetKerning(&self, value: f32) -> windows_core::Result<()>;
    fn GetLanguageID(&self) -> windows_core::Result<i32>;
    fn SetLanguageID(&self, value: i32) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetOutline(&self) -> windows_core::Result<i32>;
    fn SetOutline(&self, value: i32) -> windows_core::Result<()>;
    fn GetPosition(&self) -> windows_core::Result<f32>;
    fn SetPosition(&self, value: f32) -> windows_core::Result<()>;
    fn GetProtected(&self) -> windows_core::Result<i32>;
    fn SetProtected(&self, value: i32) -> windows_core::Result<()>;
    fn GetShadow(&self) -> windows_core::Result<i32>;
    fn SetShadow(&self, value: i32) -> windows_core::Result<()>;
    fn GetSize(&self) -> windows_core::Result<f32>;
    fn SetSize(&self, value: f32) -> windows_core::Result<()>;
    fn GetSmallCaps(&self) -> windows_core::Result<i32>;
    fn SetSmallCaps(&self, value: i32) -> windows_core::Result<()>;
    fn GetSpacing(&self) -> windows_core::Result<f32>;
    fn SetSpacing(&self, value: f32) -> windows_core::Result<()>;
    fn GetStrikeThrough(&self) -> windows_core::Result<i32>;
    fn SetStrikeThrough(&self, value: i32) -> windows_core::Result<()>;
    fn GetSubscript(&self) -> windows_core::Result<i32>;
    fn SetSubscript(&self, value: i32) -> windows_core::Result<()>;
    fn GetSuperscript(&self) -> windows_core::Result<i32>;
    fn SetSuperscript(&self, value: i32) -> windows_core::Result<()>;
    fn GetUnderline(&self) -> windows_core::Result<i32>;
    fn SetUnderline(&self, value: i32) -> windows_core::Result<()>;
    fn GetWeight(&self) -> windows_core::Result<i32>;
    fn SetWeight(&self, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextFont_Vtbl {
    pub const fn new<Identity: ITextFont_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDuplicate<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetDuplicate(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuplicate<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetDuplicate(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn CanChange<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::CanChange(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::IsEqual(this, core::mem::transmute_copy(&pfont)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::Reset(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetStyle<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetStyle(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetStyle(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAllCaps<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetAllCaps(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllCaps<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetAllCaps(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAnimation<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetAnimation(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAnimation<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetAnimation(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetBackColor<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetBackColor(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetBackColor(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetBold<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetBold(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBold<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetBold(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEmboss<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetEmboss(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEmboss<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetEmboss(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetForeColor<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetForeColor(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetForeColor(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetHidden<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetHidden(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHidden<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetHidden(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEngrave<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetEngrave(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEngrave<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetEngrave(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetItalic<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetItalic(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetItalic<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetItalic(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKerning<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetKerning(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKerning<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetKerning(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetLanguageID<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetLanguageID(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLanguageID<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetLanguageID(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetName(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn GetOutline<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetOutline(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutline<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetOutline(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetPosition<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetPosition(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPosition<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetPosition(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetProtected<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetProtected(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtected<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetProtected(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetShadow<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetShadow(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShadow<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetShadow(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSize<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSize(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSize<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSize(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSmallCaps<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSmallCaps(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSmallCaps<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSmallCaps(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSpacing<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSpacing(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpacing<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSpacing(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetStrikeThrough<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetStrikeThrough(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrikeThrough<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetStrikeThrough(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSubscript<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSubscript(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubscript<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSubscript(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSuperscript<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetSuperscript(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSuperscript<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetSuperscript(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetUnderline<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetUnderline(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetUnderline(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetWeight<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont_Impl::GetWeight(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWeight<Identity: ITextFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont_Impl::SetWeight(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDuplicate: GetDuplicate::<Identity, OFFSET>,
            SetDuplicate: SetDuplicate::<Identity, OFFSET>,
            CanChange: CanChange::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            GetStyle: GetStyle::<Identity, OFFSET>,
            SetStyle: SetStyle::<Identity, OFFSET>,
            GetAllCaps: GetAllCaps::<Identity, OFFSET>,
            SetAllCaps: SetAllCaps::<Identity, OFFSET>,
            GetAnimation: GetAnimation::<Identity, OFFSET>,
            SetAnimation: SetAnimation::<Identity, OFFSET>,
            GetBackColor: GetBackColor::<Identity, OFFSET>,
            SetBackColor: SetBackColor::<Identity, OFFSET>,
            GetBold: GetBold::<Identity, OFFSET>,
            SetBold: SetBold::<Identity, OFFSET>,
            GetEmboss: GetEmboss::<Identity, OFFSET>,
            SetEmboss: SetEmboss::<Identity, OFFSET>,
            GetForeColor: GetForeColor::<Identity, OFFSET>,
            SetForeColor: SetForeColor::<Identity, OFFSET>,
            GetHidden: GetHidden::<Identity, OFFSET>,
            SetHidden: SetHidden::<Identity, OFFSET>,
            GetEngrave: GetEngrave::<Identity, OFFSET>,
            SetEngrave: SetEngrave::<Identity, OFFSET>,
            GetItalic: GetItalic::<Identity, OFFSET>,
            SetItalic: SetItalic::<Identity, OFFSET>,
            GetKerning: GetKerning::<Identity, OFFSET>,
            SetKerning: SetKerning::<Identity, OFFSET>,
            GetLanguageID: GetLanguageID::<Identity, OFFSET>,
            SetLanguageID: SetLanguageID::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetOutline: GetOutline::<Identity, OFFSET>,
            SetOutline: SetOutline::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            GetProtected: GetProtected::<Identity, OFFSET>,
            SetProtected: SetProtected::<Identity, OFFSET>,
            GetShadow: GetShadow::<Identity, OFFSET>,
            SetShadow: SetShadow::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            GetSmallCaps: GetSmallCaps::<Identity, OFFSET>,
            SetSmallCaps: SetSmallCaps::<Identity, OFFSET>,
            GetSpacing: GetSpacing::<Identity, OFFSET>,
            SetSpacing: SetSpacing::<Identity, OFFSET>,
            GetStrikeThrough: GetStrikeThrough::<Identity, OFFSET>,
            SetStrikeThrough: SetStrikeThrough::<Identity, OFFSET>,
            GetSubscript: GetSubscript::<Identity, OFFSET>,
            SetSubscript: SetSubscript::<Identity, OFFSET>,
            GetSuperscript: GetSuperscript::<Identity, OFFSET>,
            SetSuperscript: SetSuperscript::<Identity, OFFSET>,
            GetUnderline: GetUnderline::<Identity, OFFSET>,
            SetUnderline: SetUnderline::<Identity, OFFSET>,
            GetWeight: GetWeight::<Identity, OFFSET>,
            SetWeight: SetWeight::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextFont as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextFont {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextFont2, ITextFont2_Vtbl, 0xc241f5e3_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextFont2 {
    type Target = ITextFont;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextFont2, windows_core::IUnknown, super::oaidl::IDispatch, ITextFont);
#[cfg(feature = "Win32_oaidl")]
impl ITextFont2 {
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAutoLigatures(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutoLigatures)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutoLigatures(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoLigatures)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetAutospaceAlpha(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutospaceAlpha)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutospaceAlpha(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutospaceAlpha)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetAutospaceNumeric(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutospaceNumeric)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutospaceNumeric(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutospaceNumeric)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetAutospaceParens(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutospaceParens)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutospaceParens(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutospaceParens)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCharRep(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCharRep)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCharRep(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCharRep)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCompressionMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCompressionMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCompressionMode(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCompressionMode)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCookie(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCookie(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCookie)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetDoubleStrike(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDoubleStrike)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDoubleStrike(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDoubleStrike)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetDuplicate2(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDuplicate2<P0>(&self, pfont: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDuplicate2)(windows_core::Interface::as_raw(self), pfont.param().abi()) }
    }
    pub unsafe fn GetLinkType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLinkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMathZone(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMathZone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMathZone(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMathZone)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetModWidthPairs(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModWidthPairs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetModWidthPairs(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModWidthPairs)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetModWidthSpace(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModWidthSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetModWidthSpace(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModWidthSpace)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetOldNumbers(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOldNumbers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOldNumbers(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOldNumbers)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetOverlapping(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverlapping)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverlapping(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOverlapping)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetPositionSubSuper(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPositionSubSuper)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPositionSubSuper(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPositionSubSuper)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetScaling(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScaling)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScaling(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScaling)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetSpaceExtension(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpaceExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpaceExtension(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSpaceExtension)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetUnderlinePositionMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnderlinePositionMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUnderlinePositionMode(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUnderlinePositionMode)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEffects)(windows_core::Interface::as_raw(self), pvalue as _, pmask as _) }
    }
    pub unsafe fn GetEffects2(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEffects2)(windows_core::Interface::as_raw(self), pvalue as _, pmask as _) }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPropertyInfo(&self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), index, ptype as _, pvalue as _) }
    }
    pub unsafe fn IsEqual2<P0>(&self, pfont: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual2)(windows_core::Interface::as_raw(self), pfont.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEffects)(windows_core::Interface::as_raw(self), value, mask) }
    }
    pub unsafe fn SetEffects2(&self, value: i32, mask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEffects2)(windows_core::Interface::as_raw(self), value, mask) }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextFont2_Vtbl {
    pub base__: ITextFont_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetAutoLigatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutoLigatures: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAutospaceAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutospaceAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAutospaceNumeric: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutospaceNumeric: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAutospaceParens: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutospaceParens: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCharRep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCharRep: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCompressionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCompressionMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCookie: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDoubleStrike: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDoubleStrike: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLinkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetMathZone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMathZone: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetModWidthPairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetModWidthPairs: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetModWidthSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetModWidthSpace: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetOldNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOldNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetOverlapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOverlapping: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPositionSubSuper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPositionSubSuper: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetScaling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetScaling: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSpaceExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceExtension: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetUnderlinePositionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetUnderlinePositionMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetEffects2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub IsEqual2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetEffects2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextFont2_Impl: ITextFont_Impl {
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetAutoLigatures(&self) -> windows_core::Result<i32>;
    fn SetAutoLigatures(&self, value: i32) -> windows_core::Result<()>;
    fn GetAutospaceAlpha(&self) -> windows_core::Result<i32>;
    fn SetAutospaceAlpha(&self, value: i32) -> windows_core::Result<()>;
    fn GetAutospaceNumeric(&self) -> windows_core::Result<i32>;
    fn SetAutospaceNumeric(&self, value: i32) -> windows_core::Result<()>;
    fn GetAutospaceParens(&self) -> windows_core::Result<i32>;
    fn SetAutospaceParens(&self, value: i32) -> windows_core::Result<()>;
    fn GetCharRep(&self) -> windows_core::Result<i32>;
    fn SetCharRep(&self, value: i32) -> windows_core::Result<()>;
    fn GetCompressionMode(&self) -> windows_core::Result<i32>;
    fn SetCompressionMode(&self, value: i32) -> windows_core::Result<()>;
    fn GetCookie(&self) -> windows_core::Result<i32>;
    fn SetCookie(&self, value: i32) -> windows_core::Result<()>;
    fn GetDoubleStrike(&self) -> windows_core::Result<i32>;
    fn SetDoubleStrike(&self, value: i32) -> windows_core::Result<()>;
    fn GetDuplicate2(&self) -> windows_core::Result<ITextFont2>;
    fn SetDuplicate2(&self, pfont: windows_core::Ref<ITextFont2>) -> windows_core::Result<()>;
    fn GetLinkType(&self) -> windows_core::Result<i32>;
    fn GetMathZone(&self) -> windows_core::Result<i32>;
    fn SetMathZone(&self, value: i32) -> windows_core::Result<()>;
    fn GetModWidthPairs(&self) -> windows_core::Result<i32>;
    fn SetModWidthPairs(&self, value: i32) -> windows_core::Result<()>;
    fn GetModWidthSpace(&self) -> windows_core::Result<i32>;
    fn SetModWidthSpace(&self, value: i32) -> windows_core::Result<()>;
    fn GetOldNumbers(&self) -> windows_core::Result<i32>;
    fn SetOldNumbers(&self, value: i32) -> windows_core::Result<()>;
    fn GetOverlapping(&self) -> windows_core::Result<i32>;
    fn SetOverlapping(&self, value: i32) -> windows_core::Result<()>;
    fn GetPositionSubSuper(&self) -> windows_core::Result<i32>;
    fn SetPositionSubSuper(&self, value: i32) -> windows_core::Result<()>;
    fn GetScaling(&self) -> windows_core::Result<i32>;
    fn SetScaling(&self, value: i32) -> windows_core::Result<()>;
    fn GetSpaceExtension(&self) -> windows_core::Result<f32>;
    fn SetSpaceExtension(&self, value: f32) -> windows_core::Result<()>;
    fn GetUnderlinePositionMode(&self) -> windows_core::Result<i32>;
    fn SetUnderlinePositionMode(&self, value: i32) -> windows_core::Result<()>;
    fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()>;
    fn GetEffects2(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetPropertyInfo(&self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> windows_core::Result<()>;
    fn IsEqual2(&self, pfont: windows_core::Ref<ITextFont2>) -> windows_core::Result<i32>;
    fn SetEffects(&self, value: i32, mask: i32) -> windows_core::Result<()>;
    fn SetEffects2(&self, value: i32, mask: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextFont2_Vtbl {
    pub const fn new<Identity: ITextFont2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAutoLigatures<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetAutoLigatures(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoLigatures<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetAutoLigatures(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAutospaceAlpha<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetAutospaceAlpha(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutospaceAlpha<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetAutospaceAlpha(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAutospaceNumeric<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetAutospaceNumeric(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutospaceNumeric<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetAutospaceNumeric(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAutospaceParens<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetAutospaceParens(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutospaceParens<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetAutospaceParens(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCharRep<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetCharRep(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCharRep<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetCharRep(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCompressionMode<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetCompressionMode(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompressionMode<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetCompressionMode(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCookie<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetCookie(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCookie<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetCookie(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDoubleStrike<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetDoubleStrike(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDoubleStrike<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetDoubleStrike(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetDuplicate2(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuplicate2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetDuplicate2(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetLinkType<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetLinkType(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMathZone<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetMathZone(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMathZone<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetMathZone(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetModWidthPairs<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetModWidthPairs(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModWidthPairs<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetModWidthPairs(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetModWidthSpace<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetModWidthSpace(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModWidthSpace<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetModWidthSpace(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetOldNumbers<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetOldNumbers(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOldNumbers<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetOldNumbers(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetOverlapping<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetOverlapping(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverlapping<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetOverlapping(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetPositionSubSuper<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetPositionSubSuper(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPositionSubSuper<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetPositionSubSuper(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetScaling<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetScaling(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScaling<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetScaling(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSpaceExtension<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetSpaceExtension(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceExtension<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetSpaceExtension(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetUnderlinePositionMode<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetUnderlinePositionMode(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnderlinePositionMode<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetUnderlinePositionMode(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEffects<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::GetEffects(this, core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pmask)).into()
            }
        }
        unsafe extern "system" fn GetEffects2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::GetEffects2(this, core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pmask)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ptype: *mut i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn IsEqual2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void, pb: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextFont2_Impl::IsEqual2(this, core::mem::transmute_copy(&pfont)) {
                    Ok(ok__) => {
                        pb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEffects<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetEffects(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn SetEffects2<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetEffects2(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextFont2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextFont2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: ITextFont_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAutoLigatures: GetAutoLigatures::<Identity, OFFSET>,
            SetAutoLigatures: SetAutoLigatures::<Identity, OFFSET>,
            GetAutospaceAlpha: GetAutospaceAlpha::<Identity, OFFSET>,
            SetAutospaceAlpha: SetAutospaceAlpha::<Identity, OFFSET>,
            GetAutospaceNumeric: GetAutospaceNumeric::<Identity, OFFSET>,
            SetAutospaceNumeric: SetAutospaceNumeric::<Identity, OFFSET>,
            GetAutospaceParens: GetAutospaceParens::<Identity, OFFSET>,
            SetAutospaceParens: SetAutospaceParens::<Identity, OFFSET>,
            GetCharRep: GetCharRep::<Identity, OFFSET>,
            SetCharRep: SetCharRep::<Identity, OFFSET>,
            GetCompressionMode: GetCompressionMode::<Identity, OFFSET>,
            SetCompressionMode: SetCompressionMode::<Identity, OFFSET>,
            GetCookie: GetCookie::<Identity, OFFSET>,
            SetCookie: SetCookie::<Identity, OFFSET>,
            GetDoubleStrike: GetDoubleStrike::<Identity, OFFSET>,
            SetDoubleStrike: SetDoubleStrike::<Identity, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, OFFSET>,
            SetDuplicate2: SetDuplicate2::<Identity, OFFSET>,
            GetLinkType: GetLinkType::<Identity, OFFSET>,
            GetMathZone: GetMathZone::<Identity, OFFSET>,
            SetMathZone: SetMathZone::<Identity, OFFSET>,
            GetModWidthPairs: GetModWidthPairs::<Identity, OFFSET>,
            SetModWidthPairs: SetModWidthPairs::<Identity, OFFSET>,
            GetModWidthSpace: GetModWidthSpace::<Identity, OFFSET>,
            SetModWidthSpace: SetModWidthSpace::<Identity, OFFSET>,
            GetOldNumbers: GetOldNumbers::<Identity, OFFSET>,
            SetOldNumbers: SetOldNumbers::<Identity, OFFSET>,
            GetOverlapping: GetOverlapping::<Identity, OFFSET>,
            SetOverlapping: SetOverlapping::<Identity, OFFSET>,
            GetPositionSubSuper: GetPositionSubSuper::<Identity, OFFSET>,
            SetPositionSubSuper: SetPositionSubSuper::<Identity, OFFSET>,
            GetScaling: GetScaling::<Identity, OFFSET>,
            SetScaling: SetScaling::<Identity, OFFSET>,
            GetSpaceExtension: GetSpaceExtension::<Identity, OFFSET>,
            SetSpaceExtension: SetSpaceExtension::<Identity, OFFSET>,
            GetUnderlinePositionMode: GetUnderlinePositionMode::<Identity, OFFSET>,
            SetUnderlinePositionMode: SetUnderlinePositionMode::<Identity, OFFSET>,
            GetEffects: GetEffects::<Identity, OFFSET>,
            GetEffects2: GetEffects2::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            IsEqual2: IsEqual2::<Identity, OFFSET>,
            SetEffects: SetEffects::<Identity, OFFSET>,
            SetEffects2: SetEffects2::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextFont2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ITextFont as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextFont2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextPara, ITextPara_Vtbl, 0x8cc497c4_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextPara {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextPara, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ITextPara {
    pub unsafe fn GetDuplicate(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDuplicate<P0>(&self, ppara: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDuplicate)(windows_core::Interface::as_raw(self), ppara.param().abi()) }
    }
    pub unsafe fn CanChange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual<P0>(&self, ppara: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), ppara.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStyle(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStyle)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAlignment(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAlignment)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetHyphenation(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHyphenation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHyphenation(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHyphenation)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetFirstLineIndent(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstLineIndent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetKeepTogether(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeepTogether)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeepTogether(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKeepTogether)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetKeepWithNext(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeepWithNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKeepWithNext)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetLeftIndent(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLeftIndent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLineSpacing(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLineSpacing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLineSpacingRule(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLineSpacingRule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetListAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListAlignment(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetListAlignment)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetListLevelIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListLevelIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListLevelIndex(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetListLevelIndex)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetListStart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListStart(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetListStart)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetListTab(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListTab)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListTab(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetListTab)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetListType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetListType(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetListType)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetNoLineNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNoLineNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNoLineNumber(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNoLineNumber)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetPageBreakBefore(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPageBreakBefore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPageBreakBefore(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPageBreakBefore)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetRightIndent(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRightIndent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRightIndent(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRightIndent)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn SetIndents(&self, first: f32, left: f32, right: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIndents)(windows_core::Interface::as_raw(self), first, left, right) }
    }
    pub unsafe fn SetLineSpacing(&self, rule: i32, spacing: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLineSpacing)(windows_core::Interface::as_raw(self), rule, spacing) }
    }
    pub unsafe fn GetSpaceAfter(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpaceAfter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpaceAfter(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSpaceAfter)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetSpaceBefore(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpaceBefore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpaceBefore(&self, value: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSpaceBefore)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetWidowControl(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWidowControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWidowControl(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWidowControl)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetTabCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTabCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddTab)(windows_core::Interface::as_raw(self), tbpos, tbalign, tbleader) }
    }
    pub unsafe fn ClearAllTabs(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearAllTabs)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DeleteTab(&self, tbpos: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteTab)(windows_core::Interface::as_raw(self), tbpos) }
    }
    pub unsafe fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTab)(windows_core::Interface::as_raw(self), itab, ptbpos as _, ptbalign as _, ptbleader as _) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextPara_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetHyphenation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHyphenation: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetFirstLineIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetKeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetKeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetLeftIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetLineSpacingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetListAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetListLevelIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListLevelIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetListStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetListTab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetListTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetListType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetListType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetNoLineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetNoLineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPageBreakBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPageBreakBefore: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetRightIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRightIndent: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetIndents: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, i32, f32) -> windows_core::HRESULT,
    pub GetSpaceAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceAfter: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetSpaceBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpaceBefore: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetWidowControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWidowControl: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetTabCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AddTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32, i32, i32) -> windows_core::HRESULT,
    pub ClearAllTabs: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteTab: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetTab: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut f32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextPara_Impl: super::oaidl::IDispatch_Impl {
    fn GetDuplicate(&self) -> windows_core::Result<ITextPara>;
    fn SetDuplicate(&self, ppara: windows_core::Ref<ITextPara>) -> windows_core::Result<()>;
    fn CanChange(&self) -> windows_core::Result<i32>;
    fn IsEqual(&self, ppara: windows_core::Ref<ITextPara>) -> windows_core::Result<i32>;
    fn Reset(&self, value: i32) -> windows_core::Result<()>;
    fn GetStyle(&self) -> windows_core::Result<i32>;
    fn SetStyle(&self, value: i32) -> windows_core::Result<()>;
    fn GetAlignment(&self) -> windows_core::Result<i32>;
    fn SetAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetHyphenation(&self) -> windows_core::Result<i32>;
    fn SetHyphenation(&self, value: i32) -> windows_core::Result<()>;
    fn GetFirstLineIndent(&self) -> windows_core::Result<f32>;
    fn GetKeepTogether(&self) -> windows_core::Result<i32>;
    fn SetKeepTogether(&self, value: i32) -> windows_core::Result<()>;
    fn GetKeepWithNext(&self) -> windows_core::Result<i32>;
    fn SetKeepWithNext(&self, value: i32) -> windows_core::Result<()>;
    fn GetLeftIndent(&self) -> windows_core::Result<f32>;
    fn GetLineSpacing(&self) -> windows_core::Result<f32>;
    fn GetLineSpacingRule(&self) -> windows_core::Result<i32>;
    fn GetListAlignment(&self) -> windows_core::Result<i32>;
    fn SetListAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetListLevelIndex(&self) -> windows_core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> windows_core::Result<()>;
    fn GetListStart(&self) -> windows_core::Result<i32>;
    fn SetListStart(&self, value: i32) -> windows_core::Result<()>;
    fn GetListTab(&self) -> windows_core::Result<f32>;
    fn SetListTab(&self, value: f32) -> windows_core::Result<()>;
    fn GetListType(&self) -> windows_core::Result<i32>;
    fn SetListType(&self, value: i32) -> windows_core::Result<()>;
    fn GetNoLineNumber(&self) -> windows_core::Result<i32>;
    fn SetNoLineNumber(&self, value: i32) -> windows_core::Result<()>;
    fn GetPageBreakBefore(&self) -> windows_core::Result<i32>;
    fn SetPageBreakBefore(&self, value: i32) -> windows_core::Result<()>;
    fn GetRightIndent(&self) -> windows_core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> windows_core::Result<()>;
    fn SetIndents(&self, first: f32, left: f32, right: f32) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, rule: i32, spacing: f32) -> windows_core::Result<()>;
    fn GetSpaceAfter(&self) -> windows_core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> windows_core::Result<()>;
    fn GetSpaceBefore(&self) -> windows_core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> windows_core::Result<()>;
    fn GetWidowControl(&self) -> windows_core::Result<i32>;
    fn SetWidowControl(&self, value: i32) -> windows_core::Result<()>;
    fn GetTabCount(&self) -> windows_core::Result<i32>;
    fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> windows_core::Result<()>;
    fn ClearAllTabs(&self) -> windows_core::Result<()>;
    fn DeleteTab(&self, tbpos: f32) -> windows_core::Result<()>;
    fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextPara_Vtbl {
    pub const fn new<Identity: ITextPara_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDuplicate<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetDuplicate(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuplicate<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetDuplicate(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn CanChange<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::CanChange(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::IsEqual(this, core::mem::transmute_copy(&ppara)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::Reset(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetStyle<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetStyle(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetStyle(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAlignment<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetHyphenation<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetHyphenation(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHyphenation<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetHyphenation(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetFirstLineIndent<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetFirstLineIndent(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeepTogether<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetKeepTogether(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetKeepTogether(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKeepWithNext<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetKeepWithNext(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetKeepWithNext(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetLeftIndent<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetLeftIndent(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLineSpacing<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetLineSpacing(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLineSpacingRule<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetLineSpacingRule(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetListAlignment<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListAlignment<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetListLevelIndex<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListLevelIndex(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListLevelIndex(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetListStart<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListStart(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListStart<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListStart(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetListTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListTab(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListTab(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetListType<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetListType(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListType<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetListType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetNoLineNumber<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetNoLineNumber(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetNoLineNumber(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetPageBreakBefore<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetPageBreakBefore(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetPageBreakBefore(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetRightIndent<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetRightIndent(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRightIndent<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetRightIndent(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetIndents<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, first: f32, left: f32, right: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetIndents(this, core::mem::transmute_copy(&first), core::mem::transmute_copy(&left), core::mem::transmute_copy(&right)).into()
            }
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: i32, spacing: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetLineSpacing(this, core::mem::transmute_copy(&rule), core::mem::transmute_copy(&spacing)).into()
            }
        }
        unsafe extern "system" fn GetSpaceAfter<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetSpaceAfter(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetSpaceAfter(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSpaceBefore<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetSpaceBefore(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetSpaceBefore(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetWidowControl<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetWidowControl(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidowControl<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::SetWidowControl(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetTabCount<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara_Impl::GetTabCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::AddTab(this, core::mem::transmute_copy(&tbpos), core::mem::transmute_copy(&tbalign), core::mem::transmute_copy(&tbleader)).into()
            }
        }
        unsafe extern "system" fn ClearAllTabs<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::ClearAllTabs(this).into()
            }
        }
        unsafe extern "system" fn DeleteTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tbpos: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::DeleteTab(this, core::mem::transmute_copy(&tbpos)).into()
            }
        }
        unsafe extern "system" fn GetTab<Identity: ITextPara_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara_Impl::GetTab(this, core::mem::transmute_copy(&itab), core::mem::transmute_copy(&ptbpos), core::mem::transmute_copy(&ptbalign), core::mem::transmute_copy(&ptbleader)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDuplicate: GetDuplicate::<Identity, OFFSET>,
            SetDuplicate: SetDuplicate::<Identity, OFFSET>,
            CanChange: CanChange::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            GetStyle: GetStyle::<Identity, OFFSET>,
            SetStyle: SetStyle::<Identity, OFFSET>,
            GetAlignment: GetAlignment::<Identity, OFFSET>,
            SetAlignment: SetAlignment::<Identity, OFFSET>,
            GetHyphenation: GetHyphenation::<Identity, OFFSET>,
            SetHyphenation: SetHyphenation::<Identity, OFFSET>,
            GetFirstLineIndent: GetFirstLineIndent::<Identity, OFFSET>,
            GetKeepTogether: GetKeepTogether::<Identity, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Identity, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, OFFSET>,
            GetLeftIndent: GetLeftIndent::<Identity, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, OFFSET>,
            GetLineSpacingRule: GetLineSpacingRule::<Identity, OFFSET>,
            GetListAlignment: GetListAlignment::<Identity, OFFSET>,
            SetListAlignment: SetListAlignment::<Identity, OFFSET>,
            GetListLevelIndex: GetListLevelIndex::<Identity, OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Identity, OFFSET>,
            GetListStart: GetListStart::<Identity, OFFSET>,
            SetListStart: SetListStart::<Identity, OFFSET>,
            GetListTab: GetListTab::<Identity, OFFSET>,
            SetListTab: SetListTab::<Identity, OFFSET>,
            GetListType: GetListType::<Identity, OFFSET>,
            SetListType: SetListType::<Identity, OFFSET>,
            GetNoLineNumber: GetNoLineNumber::<Identity, OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Identity, OFFSET>,
            GetPageBreakBefore: GetPageBreakBefore::<Identity, OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Identity, OFFSET>,
            GetRightIndent: GetRightIndent::<Identity, OFFSET>,
            SetRightIndent: SetRightIndent::<Identity, OFFSET>,
            SetIndents: SetIndents::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            GetSpaceAfter: GetSpaceAfter::<Identity, OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Identity, OFFSET>,
            GetSpaceBefore: GetSpaceBefore::<Identity, OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Identity, OFFSET>,
            GetWidowControl: GetWidowControl::<Identity, OFFSET>,
            SetWidowControl: SetWidowControl::<Identity, OFFSET>,
            GetTabCount: GetTabCount::<Identity, OFFSET>,
            AddTab: AddTab::<Identity, OFFSET>,
            ClearAllTabs: ClearAllTabs::<Identity, OFFSET>,
            DeleteTab: DeleteTab::<Identity, OFFSET>,
            GetTab: GetTab::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextPara as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextPara {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextPara2, ITextPara2_Vtbl, 0xc241f5e4_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextPara2 {
    type Target = ITextPara;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextPara2, windows_core::IUnknown, super::oaidl::IDispatch, ITextPara);
#[cfg(feature = "Win32_oaidl")]
impl ITextPara2 {
    pub unsafe fn GetBorders(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBorders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDuplicate2(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDuplicate2<P0>(&self, ppara: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDuplicate2)(windows_core::Interface::as_raw(self), ppara.param().abi()) }
    }
    pub unsafe fn GetFontAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFontAlignment(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFontAlignment)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetHangingPunctuation(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHangingPunctuation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHangingPunctuation(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHangingPunctuation)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetSnapToGrid(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSnapToGrid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSnapToGrid(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSnapToGrid)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetTrimPunctuationAtStart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTrimPunctuationAtStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTrimPunctuationAtStart(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTrimPunctuationAtStart)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEffects)(windows_core::Interface::as_raw(self), pvalue as _, pmask as _) }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual2<P0>(&self, ppara: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual2)(windows_core::Interface::as_raw(self), ppara.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEffects)(windows_core::Interface::as_raw(self), value, mask) }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextPara2_Vtbl {
    pub base__: ITextPara_Vtbl,
    pub GetBorders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFontAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetHangingPunctuation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHangingPunctuation: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSnapToGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSnapToGrid: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetTrimPunctuationAtStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTrimPunctuationAtStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub IsEqual2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextPara2_Impl: ITextPara_Impl {
    fn GetBorders(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetDuplicate2(&self) -> windows_core::Result<ITextPara2>;
    fn SetDuplicate2(&self, ppara: windows_core::Ref<ITextPara2>) -> windows_core::Result<()>;
    fn GetFontAlignment(&self) -> windows_core::Result<i32>;
    fn SetFontAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetHangingPunctuation(&self) -> windows_core::Result<i32>;
    fn SetHangingPunctuation(&self, value: i32) -> windows_core::Result<()>;
    fn GetSnapToGrid(&self) -> windows_core::Result<i32>;
    fn SetSnapToGrid(&self, value: i32) -> windows_core::Result<()>;
    fn GetTrimPunctuationAtStart(&self) -> windows_core::Result<i32>;
    fn SetTrimPunctuationAtStart(&self, value: i32) -> windows_core::Result<()>;
    fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn IsEqual2(&self, ppara: windows_core::Ref<ITextPara2>) -> windows_core::Result<i32>;
    fn SetEffects(&self, value: i32, mask: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextPara2_Vtbl {
    pub const fn new<Identity: ITextPara2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBorders<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppborders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetBorders(this) {
                    Ok(ok__) => {
                        ppborders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetDuplicate2(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuplicate2<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetDuplicate2(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn GetFontAlignment<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetFontAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFontAlignment<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetFontAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetHangingPunctuation<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetHangingPunctuation(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHangingPunctuation<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetHangingPunctuation(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSnapToGrid<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetSnapToGrid(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSnapToGrid<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetSnapToGrid(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetTrimPunctuationAtStart<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetTrimPunctuationAtStart(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTrimPunctuationAtStart<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetTrimPunctuationAtStart(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEffects<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::GetEffects(this, core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pmask)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual2<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void, pb: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextPara2_Impl::IsEqual2(this, core::mem::transmute_copy(&ppara)) {
                    Ok(ok__) => {
                        pb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEffects<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, mask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetEffects(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&mask)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextPara2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextPara2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: ITextPara_Vtbl::new::<Identity, OFFSET>(),
            GetBorders: GetBorders::<Identity, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, OFFSET>,
            SetDuplicate2: SetDuplicate2::<Identity, OFFSET>,
            GetFontAlignment: GetFontAlignment::<Identity, OFFSET>,
            SetFontAlignment: SetFontAlignment::<Identity, OFFSET>,
            GetHangingPunctuation: GetHangingPunctuation::<Identity, OFFSET>,
            SetHangingPunctuation: SetHangingPunctuation::<Identity, OFFSET>,
            GetSnapToGrid: GetSnapToGrid::<Identity, OFFSET>,
            SetSnapToGrid: SetSnapToGrid::<Identity, OFFSET>,
            GetTrimPunctuationAtStart: GetTrimPunctuationAtStart::<Identity, OFFSET>,
            SetTrimPunctuationAtStart: SetTrimPunctuationAtStart::<Identity, OFFSET>,
            GetEffects: GetEffects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            IsEqual2: IsEqual2::<Identity, OFFSET>,
            SetEffects: SetEffects::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextPara2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ITextPara as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextPara2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextRange, ITextRange_Vtbl, 0x8cc497c2_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextRange {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextRange, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ITextRange {
    pub unsafe fn GetText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetText(&self, bstr: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)) }
    }
    pub unsafe fn GetChar(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChar(&self, char: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChar)(windows_core::Interface::as_raw(self), char) }
    }
    pub unsafe fn GetDuplicate(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFormattedText(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFormattedText<P0>(&self, prange: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFormattedText)(windows_core::Interface::as_raw(self), prange.param().abi()) }
    }
    pub unsafe fn GetStart(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStart(&self, cpfirst: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStart)(windows_core::Interface::as_raw(self), cpfirst) }
    }
    pub unsafe fn GetEnd(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnd(&self, cplim: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnd)(windows_core::Interface::as_raw(self), cplim) }
    }
    pub unsafe fn GetFont(&self) -> windows_core::Result<ITextFont> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFont<P0>(&self, pfont: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextFont>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFont)(windows_core::Interface::as_raw(self), pfont.param().abi()) }
    }
    pub unsafe fn GetPara(&self) -> windows_core::Result<ITextPara> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPara)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPara<P0>(&self, ppara: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextPara>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPara)(windows_core::Interface::as_raw(self), ppara.param().abi()) }
    }
    pub unsafe fn GetStoryLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStoryType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoryType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Collapse(&self, bstart: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Collapse)(windows_core::Interface::as_raw(self), bstart) }
    }
    pub unsafe fn Expand(&self, unit: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), unit, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIndex(&self, unit: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), unit, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIndex)(windows_core::Interface::as_raw(self), unit, index, extend) }
    }
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRange)(windows_core::Interface::as_raw(self), cpanchor, cpactive) }
    }
    pub unsafe fn InRange<P0>(&self, prange: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InRange)(windows_core::Interface::as_raw(self), prange.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InStory<P0>(&self, prange: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InStory)(windows_core::Interface::as_raw(self), prange.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual<P0>(&self, prange: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), prange.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Select(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartOf)(windows_core::Interface::as_raw(self), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndOf)(windows_core::Interface::as_raw(self), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Move(&self, unit: i32, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveStart)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEnd)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveWhile)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveStartWhile)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEndWhile)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveUntil)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveStartUntil)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEndUntil)(windows_core::Interface::as_raw(self), core::mem::transmute(cset), count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FindText(&self, bstr: &windows_core::BSTR, count: i32, flags: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), count, flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FindTextStart(&self, bstr: &windows_core::BSTR, count: i32, flags: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindTextStart)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), count, flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FindTextEnd(&self, bstr: &windows_core::BSTR, count: i32, flags: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindTextEnd)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), count, flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Cut(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cut)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Copy(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Paste(&self, pvar: *const super::oaidl::VARIANT, format: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Paste)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), format) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::oaidl::VARIANT, format: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPaste)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), format, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanEdit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanEdit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ChangeCase(&self, r#type: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangeCase)(windows_core::Interface::as_raw(self), r#type) }
    }
    pub unsafe fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPoint)(windows_core::Interface::as_raw(self), r#type, px as _, py as _) }
    }
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPoint)(windows_core::Interface::as_raw(self), x, y, r#type, extend) }
    }
    pub unsafe fn ScrollIntoView(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ScrollIntoView)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetEmbeddedObject(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEmbeddedObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetChar: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDuplicate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetStoryType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub SetRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub InRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InStory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartOf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub EndOf: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveStart: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveEnd: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub MoveWhile: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    MoveWhile: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub MoveStartWhile: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    MoveStartWhile: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub MoveEndWhile: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    MoveEndWhile: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub MoveUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    MoveUntil: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub MoveStartUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    MoveStartUntil: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub MoveEndUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    MoveEndUntil: usize,
    pub FindText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub FindTextStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub FindTextEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Cut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Cut: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Copy: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Paste: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Paste: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub CanPaste: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    CanPaste: usize,
    pub CanEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ChangeCase: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetEmbeddedObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextRange_Impl: super::oaidl::IDispatch_Impl {
    fn GetText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetText(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetChar(&self) -> windows_core::Result<i32>;
    fn SetChar(&self, char: i32) -> windows_core::Result<()>;
    fn GetDuplicate(&self) -> windows_core::Result<ITextRange>;
    fn GetFormattedText(&self) -> windows_core::Result<ITextRange>;
    fn SetFormattedText(&self, prange: windows_core::Ref<ITextRange>) -> windows_core::Result<()>;
    fn GetStart(&self) -> windows_core::Result<i32>;
    fn SetStart(&self, cpfirst: i32) -> windows_core::Result<()>;
    fn GetEnd(&self) -> windows_core::Result<i32>;
    fn SetEnd(&self, cplim: i32) -> windows_core::Result<()>;
    fn GetFont(&self) -> windows_core::Result<ITextFont>;
    fn SetFont(&self, pfont: windows_core::Ref<ITextFont>) -> windows_core::Result<()>;
    fn GetPara(&self) -> windows_core::Result<ITextPara>;
    fn SetPara(&self, ppara: windows_core::Ref<ITextPara>) -> windows_core::Result<()>;
    fn GetStoryLength(&self) -> windows_core::Result<i32>;
    fn GetStoryType(&self) -> windows_core::Result<i32>;
    fn Collapse(&self, bstart: i32) -> windows_core::Result<()>;
    fn Expand(&self, unit: i32) -> windows_core::Result<i32>;
    fn GetIndex(&self, unit: i32) -> windows_core::Result<i32>;
    fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> windows_core::Result<()>;
    fn SetRange(&self, cpanchor: i32, cpactive: i32) -> windows_core::Result<()>;
    fn InRange(&self, prange: windows_core::Ref<ITextRange>) -> windows_core::Result<i32>;
    fn InStory(&self, prange: windows_core::Ref<ITextRange>) -> windows_core::Result<i32>;
    fn IsEqual(&self, prange: windows_core::Ref<ITextRange>) -> windows_core::Result<i32>;
    fn Select(&self) -> windows_core::Result<()>;
    fn StartOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn EndOf(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn Move(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn MoveStart(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn MoveEnd(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn MoveWhile(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveStartWhile(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveEndWhile(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveUntil(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveStartUntil(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn MoveEndUntil(&self, cset: *const super::oaidl::VARIANT, count: i32) -> windows_core::Result<i32>;
    fn FindText(&self, bstr: &windows_core::BSTR, count: i32, flags: i32) -> windows_core::Result<i32>;
    fn FindTextStart(&self, bstr: &windows_core::BSTR, count: i32, flags: i32) -> windows_core::Result<i32>;
    fn FindTextEnd(&self, bstr: &windows_core::BSTR, count: i32, flags: i32) -> windows_core::Result<i32>;
    fn Delete(&self, unit: i32, count: i32) -> windows_core::Result<i32>;
    fn Cut(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Copy(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Paste(&self, pvar: *const super::oaidl::VARIANT, format: i32) -> windows_core::Result<()>;
    fn CanPaste(&self, pvar: *const super::oaidl::VARIANT, format: i32) -> windows_core::Result<i32>;
    fn CanEdit(&self) -> windows_core::Result<i32>;
    fn ChangeCase(&self, r#type: i32) -> windows_core::Result<()>;
    fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> windows_core::Result<()>;
    fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> windows_core::Result<()>;
    fn ScrollIntoView(&self, value: i32) -> windows_core::Result<()>;
    fn GetEmbeddedObject(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextRange_Vtbl {
    pub const fn new<Identity: ITextRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetText(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetText(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn GetChar<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchar: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetChar(this) {
                    Ok(ok__) => {
                        pchar.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChar<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, char: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetChar(this, core::mem::transmute_copy(&char)).into()
            }
        }
        unsafe extern "system" fn GetDuplicate<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetDuplicate(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetFormattedText(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetFormattedText(this, core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn GetStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpfirst: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetStart(this) {
                    Ok(ok__) => {
                        pcpfirst.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpfirst: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetStart(this, core::mem::transmute_copy(&cpfirst)).into()
            }
        }
        unsafe extern "system" fn GetEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcplim: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetEnd(this) {
                    Ok(ok__) => {
                        pcplim.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cplim: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetEnd(this, core::mem::transmute_copy(&cplim)).into()
            }
        }
        unsafe extern "system" fn GetFont<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetFont(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFont<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetFont(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetPara<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetPara(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPara<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetPara(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn GetStoryLength<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetStoryLength(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoryType<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetStoryType(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Collapse<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstart: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Collapse(this, core::mem::transmute_copy(&bstart)).into()
            }
        }
        unsafe extern "system" fn Expand<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Expand(this, core::mem::transmute_copy(&unit)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, pindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetIndex(this, core::mem::transmute_copy(&unit)) {
                    Ok(ok__) => {
                        pindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndex<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, index: i32, extend: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetIndex(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&index), core::mem::transmute_copy(&extend)).into()
            }
        }
        unsafe extern "system" fn SetRange<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpanchor: i32, cpactive: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetRange(this, core::mem::transmute_copy(&cpanchor), core::mem::transmute_copy(&cpactive)).into()
            }
        }
        unsafe extern "system" fn InRange<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::InRange(this, core::mem::transmute_copy(&prange)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InStory<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::InStory(this, core::mem::transmute_copy(&prange)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::IsEqual(this, core::mem::transmute_copy(&prange)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Select<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Select(this).into()
            }
        }
        unsafe extern "system" fn StartOf<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::StartOf(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndOf<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::EndOf(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Move<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Move(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveStart(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveEnd(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveWhile<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::oaidl::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveWhile(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveStartWhile<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::oaidl::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveStartWhile(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEndWhile<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::oaidl::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveEndWhile(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveUntil<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::oaidl::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveUntil(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveStartUntil<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::oaidl::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveStartUntil(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEndUntil<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cset: *const super::oaidl::VARIANT, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::MoveEndUntil(this, core::mem::transmute_copy(&cset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, count: i32, flags: i32, plength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::FindText(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        plength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindTextStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, count: i32, flags: i32, plength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::FindTextStart(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        plength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindTextEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, count: i32, flags: i32, plength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::FindTextEnd(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        plength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Delete(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cut<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Cut(this) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Copy<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::Copy(this) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Paste<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::oaidl::VARIANT, format: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::Paste(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn CanPaste<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::oaidl::VARIANT, format: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::CanPaste(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanEdit<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::CanEdit(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeCase<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::ChangeCase(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn GetPoint<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, px: *mut i32, py: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::GetPoint(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&px), core::mem::transmute_copy(&py)).into()
            }
        }
        unsafe extern "system" fn SetPoint<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, r#type: i32, extend: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::SetPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&extend)).into()
            }
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange_Impl::ScrollIntoView(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetEmbeddedObject<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange_Impl::GetEmbeddedObject(this) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetChar: GetChar::<Identity, OFFSET>,
            SetChar: SetChar::<Identity, OFFSET>,
            GetDuplicate: GetDuplicate::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, OFFSET>,
            GetStart: GetStart::<Identity, OFFSET>,
            SetStart: SetStart::<Identity, OFFSET>,
            GetEnd: GetEnd::<Identity, OFFSET>,
            SetEnd: SetEnd::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
            SetFont: SetFont::<Identity, OFFSET>,
            GetPara: GetPara::<Identity, OFFSET>,
            SetPara: SetPara::<Identity, OFFSET>,
            GetStoryLength: GetStoryLength::<Identity, OFFSET>,
            GetStoryType: GetStoryType::<Identity, OFFSET>,
            Collapse: Collapse::<Identity, OFFSET>,
            Expand: Expand::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            SetIndex: SetIndex::<Identity, OFFSET>,
            SetRange: SetRange::<Identity, OFFSET>,
            InRange: InRange::<Identity, OFFSET>,
            InStory: InStory::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Select: Select::<Identity, OFFSET>,
            StartOf: StartOf::<Identity, OFFSET>,
            EndOf: EndOf::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            MoveStart: MoveStart::<Identity, OFFSET>,
            MoveEnd: MoveEnd::<Identity, OFFSET>,
            MoveWhile: MoveWhile::<Identity, OFFSET>,
            MoveStartWhile: MoveStartWhile::<Identity, OFFSET>,
            MoveEndWhile: MoveEndWhile::<Identity, OFFSET>,
            MoveUntil: MoveUntil::<Identity, OFFSET>,
            MoveStartUntil: MoveStartUntil::<Identity, OFFSET>,
            MoveEndUntil: MoveEndUntil::<Identity, OFFSET>,
            FindText: FindText::<Identity, OFFSET>,
            FindTextStart: FindTextStart::<Identity, OFFSET>,
            FindTextEnd: FindTextEnd::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cut: Cut::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Paste: Paste::<Identity, OFFSET>,
            CanPaste: CanPaste::<Identity, OFFSET>,
            CanEdit: CanEdit::<Identity, OFFSET>,
            ChangeCase: ChangeCase::<Identity, OFFSET>,
            GetPoint: GetPoint::<Identity, OFFSET>,
            SetPoint: SetPoint::<Identity, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, OFFSET>,
            GetEmbeddedObject: GetEmbeddedObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRange as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextRange {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextRange2, ITextRange2_Vtbl, 0xc241f5e2_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextRange2 {
    type Target = ITextSelection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextRange2, windows_core::IUnknown, super::oaidl::IDispatch, ITextRange, ITextSelection);
#[cfg(feature = "Win32_oaidl")]
impl ITextRange2 {
    pub unsafe fn GetCch(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCch)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCells(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCells)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColumn(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDuplicate2(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuplicate2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFont2(&self) -> windows_core::Result<ITextFont2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFont2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFont2<P0>(&self, pfont: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextFont2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFont2)(windows_core::Interface::as_raw(self), pfont.param().abi()) }
    }
    pub unsafe fn GetFormattedText2(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFormattedText2<P0>(&self, prange: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFormattedText2)(windows_core::Interface::as_raw(self), prange.param().abi()) }
    }
    pub unsafe fn GetGravity(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGravity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGravity(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGravity)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetPara2(&self) -> windows_core::Result<ITextPara2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPara2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPara2<P0>(&self, ppara: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextPara2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPara2)(windows_core::Interface::as_raw(self), ppara.param().abi()) }
    }
    pub unsafe fn GetRow(&self) -> windows_core::Result<ITextRow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRow)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStartPara(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStartPara)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTable(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetURL(&self, bstr: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)) }
    }
    pub unsafe fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddSubrange)(windows_core::Interface::as_raw(self), cp1, cp2, activate) }
    }
    pub unsafe fn BuildUpMath(&self, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BuildUpMath)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteSubrange)(windows_core::Interface::as_raw(self), cpfirst, cplim) }
    }
    pub unsafe fn Find<P0>(&self, prange: P0, count: i32, flags: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Find)(windows_core::Interface::as_raw(self), prange.param().abi(), count, flags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChar2(&self, pchar: *mut i32, offset: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChar2)(windows_core::Interface::as_raw(self), pchar as _, offset) }
    }
    pub unsafe fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDropCap)(windows_core::Interface::as_raw(self), pcline as _, pposition as _) }
    }
    pub unsafe fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInlineObject)(windows_core::Interface::as_raw(self), ptype as _, palign as _, pchar as _, pchar1 as _, pchar2 as _, pcount as _, ptexstyle as _, pccol as _, plevel as _) }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRect)(windows_core::Interface::as_raw(self), r#type, pleft as _, ptop as _, pright as _, pbottom as _, phit as _) }
    }
    pub unsafe fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSubrange)(windows_core::Interface::as_raw(self), isubrange, pcpfirst as _, pcplim as _) }
    }
    pub unsafe fn GetText2(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText2)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HexToUnicode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HexToUnicode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertTable)(windows_core::Interface::as_raw(self), ccol, crow, autofit) }
    }
    pub unsafe fn Linearize(&self, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Linearize)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetActiveSubrange)(windows_core::Interface::as_raw(self), cpanchor, cpactive) }
    }
    pub unsafe fn SetDropCap(&self, cline: i32, position: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDropCap)(windows_core::Interface::as_raw(self), cline, position) }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value) }
    }
    pub unsafe fn SetText2(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetText2)(windows_core::Interface::as_raw(self), flags, core::mem::transmute_copy(bstr)) }
    }
    pub unsafe fn UnicodeToHex(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnicodeToHex)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInlineObject)(windows_core::Interface::as_raw(self), r#type, align, char, char1, char2, count, texstyle, ccol) }
    }
    pub unsafe fn GetMathFunctionType(&self, bstr: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMathFunctionType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn InsertImage<P5>(&self, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: &windows_core::BSTR, pstream: P5) -> windows_core::HRESULT
    where
        P5: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertImage)(windows_core::Interface::as_raw(self), width, height, ascent, r#type, core::mem::transmute_copy(bstralttext), pstream.param().abi()) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange2_Vtbl {
    pub base__: ITextSelection_Vtbl,
    pub GetCch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCells: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetDuplicate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFont2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFont2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFormattedText2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormattedText2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPara2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPara2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStartPara: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSubrange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub BuildUpMath: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeleteSubrange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Find: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub GetChar2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, i32) -> windows_core::HRESULT,
    pub GetDropCap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetSubrange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetText2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HexToUnicode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertTable: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub Linearize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetActiveSubrange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetDropCap: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetText2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnicodeToHex: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub GetMathFunctionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub InsertImage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    InsertImage: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextRange2_Impl: ITextSelection_Impl {
    fn GetCch(&self) -> windows_core::Result<i32>;
    fn GetCells(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetColumn(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetDuplicate2(&self) -> windows_core::Result<ITextRange2>;
    fn GetFont2(&self) -> windows_core::Result<ITextFont2>;
    fn SetFont2(&self, pfont: windows_core::Ref<ITextFont2>) -> windows_core::Result<()>;
    fn GetFormattedText2(&self) -> windows_core::Result<ITextRange2>;
    fn SetFormattedText2(&self, prange: windows_core::Ref<ITextRange2>) -> windows_core::Result<()>;
    fn GetGravity(&self) -> windows_core::Result<i32>;
    fn SetGravity(&self, value: i32) -> windows_core::Result<()>;
    fn GetPara2(&self) -> windows_core::Result<ITextPara2>;
    fn SetPara2(&self, ppara: windows_core::Ref<ITextPara2>) -> windows_core::Result<()>;
    fn GetRow(&self) -> windows_core::Result<ITextRow>;
    fn GetStartPara(&self) -> windows_core::Result<i32>;
    fn GetTable(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetURL(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> windows_core::Result<()>;
    fn BuildUpMath(&self, flags: i32) -> windows_core::Result<()>;
    fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> windows_core::Result<()>;
    fn Find(&self, prange: windows_core::Ref<ITextRange2>, count: i32, flags: i32) -> windows_core::Result<i32>;
    fn GetChar2(&self, pchar: *mut i32, offset: i32) -> windows_core::Result<()>;
    fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> windows_core::Result<()>;
    fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> windows_core::Result<()>;
    fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> windows_core::Result<()>;
    fn GetText2(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn HexToUnicode(&self) -> windows_core::Result<()>;
    fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> windows_core::Result<()>;
    fn Linearize(&self, flags: i32) -> windows_core::Result<()>;
    fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> windows_core::Result<()>;
    fn SetDropCap(&self, cline: i32, position: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
    fn SetText2(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UnicodeToHex(&self) -> windows_core::Result<()>;
    fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> windows_core::Result<()>;
    fn GetMathFunctionType(&self, bstr: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn InsertImage(&self, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: &windows_core::BSTR, pstream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextRange2_Vtbl {
    pub const fn new<Identity: ITextRange2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCch<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcch: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetCch(this) {
                    Ok(ok__) => {
                        pcch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCells<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcells: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetCells(this) {
                    Ok(ok__) => {
                        ppcells.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumn<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolumn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetColumn(this) {
                    Ok(ok__) => {
                        ppcolumn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetDuplicate2(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFont2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetFont2(this) {
                    Ok(ok__) => {
                        ppfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFont2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfont: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetFont2(this, core::mem::transmute_copy(&pfont)).into()
            }
        }
        unsafe extern "system" fn GetFormattedText2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetFormattedText2(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormattedText2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetFormattedText2(this, core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn GetGravity<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetGravity(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGravity<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetGravity(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetPara2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppara: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetPara2(this) {
                    Ok(ok__) => {
                        pppara.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPara2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppara: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetPara2(this, core::mem::transmute_copy(&ppara)).into()
            }
        }
        unsafe extern "system" fn GetRow<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetRow(this) {
                    Ok(ok__) => {
                        pprow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStartPara<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetStartPara(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTable<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetTable(this) {
                    Ok(ok__) => {
                        pptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetURL<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetURL(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetURL<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetURL(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn AddSubrange<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::AddSubrange(this, core::mem::transmute_copy(&cp1), core::mem::transmute_copy(&cp2), core::mem::transmute_copy(&activate)).into()
            }
        }
        unsafe extern "system" fn BuildUpMath<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::BuildUpMath(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn DeleteSubrange<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpfirst: i32, cplim: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::DeleteSubrange(this, core::mem::transmute_copy(&cpfirst), core::mem::transmute_copy(&cplim)).into()
            }
        }
        unsafe extern "system" fn Find<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, count: i32, flags: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::Find(this, core::mem::transmute_copy(&prange), core::mem::transmute_copy(&count), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChar2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchar: *mut i32, offset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetChar2(this, core::mem::transmute_copy(&pchar), core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn GetDropCap<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetDropCap(this, core::mem::transmute_copy(&pcline), core::mem::transmute_copy(&pposition)).into()
            }
        }
        unsafe extern "system" fn GetInlineObject<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetInlineObject(this, core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&palign), core::mem::transmute_copy(&pchar), core::mem::transmute_copy(&pchar1), core::mem::transmute_copy(&pchar2), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&ptexstyle), core::mem::transmute_copy(&pccol), core::mem::transmute_copy(&plevel)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRect<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetRect(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pright), core::mem::transmute_copy(&pbottom), core::mem::transmute_copy(&phit)).into()
            }
        }
        unsafe extern "system" fn GetSubrange<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::GetSubrange(this, core::mem::transmute_copy(&isubrange), core::mem::transmute_copy(&pcpfirst), core::mem::transmute_copy(&pcplim)).into()
            }
        }
        unsafe extern "system" fn GetText2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetText2(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HexToUnicode<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::HexToUnicode(this).into()
            }
        }
        unsafe extern "system" fn InsertTable<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::InsertTable(this, core::mem::transmute_copy(&ccol), core::mem::transmute_copy(&crow), core::mem::transmute_copy(&autofit)).into()
            }
        }
        unsafe extern "system" fn Linearize<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::Linearize(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn SetActiveSubrange<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpanchor: i32, cpactive: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetActiveSubrange(this, core::mem::transmute_copy(&cpanchor), core::mem::transmute_copy(&cpactive)).into()
            }
        }
        unsafe extern "system" fn SetDropCap<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cline: i32, position: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetDropCap(this, core::mem::transmute_copy(&cline), core::mem::transmute_copy(&position)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetText2<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetText2(this, core::mem::transmute_copy(&flags), core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn UnicodeToHex<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::UnicodeToHex(this).into()
            }
        }
        unsafe extern "system" fn SetInlineObject<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::SetInlineObject(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&align), core::mem::transmute_copy(&char), core::mem::transmute_copy(&char1), core::mem::transmute_copy(&char2), core::mem::transmute_copy(&count), core::mem::transmute_copy(&texstyle), core::mem::transmute_copy(&ccol)).into()
            }
        }
        unsafe extern "system" fn GetMathFunctionType<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRange2_Impl::GetMathFunctionType(this, core::mem::transmute(&bstr)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertImage<Identity: ITextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRange2_Impl::InsertImage(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&ascent), core::mem::transmute_copy(&r#type), core::mem::transmute(&bstralttext), core::mem::transmute_copy(&pstream)).into()
            }
        }
        Self {
            base__: ITextSelection_Vtbl::new::<Identity, OFFSET>(),
            GetCch: GetCch::<Identity, OFFSET>,
            GetCells: GetCells::<Identity, OFFSET>,
            GetColumn: GetColumn::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, OFFSET>,
            GetFont2: GetFont2::<Identity, OFFSET>,
            SetFont2: SetFont2::<Identity, OFFSET>,
            GetFormattedText2: GetFormattedText2::<Identity, OFFSET>,
            SetFormattedText2: SetFormattedText2::<Identity, OFFSET>,
            GetGravity: GetGravity::<Identity, OFFSET>,
            SetGravity: SetGravity::<Identity, OFFSET>,
            GetPara2: GetPara2::<Identity, OFFSET>,
            SetPara2: SetPara2::<Identity, OFFSET>,
            GetRow: GetRow::<Identity, OFFSET>,
            GetStartPara: GetStartPara::<Identity, OFFSET>,
            GetTable: GetTable::<Identity, OFFSET>,
            GetURL: GetURL::<Identity, OFFSET>,
            SetURL: SetURL::<Identity, OFFSET>,
            AddSubrange: AddSubrange::<Identity, OFFSET>,
            BuildUpMath: BuildUpMath::<Identity, OFFSET>,
            DeleteSubrange: DeleteSubrange::<Identity, OFFSET>,
            Find: Find::<Identity, OFFSET>,
            GetChar2: GetChar2::<Identity, OFFSET>,
            GetDropCap: GetDropCap::<Identity, OFFSET>,
            GetInlineObject: GetInlineObject::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetRect: GetRect::<Identity, OFFSET>,
            GetSubrange: GetSubrange::<Identity, OFFSET>,
            GetText2: GetText2::<Identity, OFFSET>,
            HexToUnicode: HexToUnicode::<Identity, OFFSET>,
            InsertTable: InsertTable::<Identity, OFFSET>,
            Linearize: Linearize::<Identity, OFFSET>,
            SetActiveSubrange: SetActiveSubrange::<Identity, OFFSET>,
            SetDropCap: SetDropCap::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            SetText2: SetText2::<Identity, OFFSET>,
            UnicodeToHex: UnicodeToHex::<Identity, OFFSET>,
            SetInlineObject: SetInlineObject::<Identity, OFFSET>,
            GetMathFunctionType: GetMathFunctionType::<Identity, OFFSET>,
            InsertImage: InsertImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRange2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ITextRange as windows_core::Interface>::IID || iid == &<ITextSelection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextRange2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextRow, ITextRow_Vtbl, 0xc241f5ef_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextRow {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextRow, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ITextRow {
    pub unsafe fn GetAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAlignment(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAlignment)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellCount(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellCount)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellCountCache(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellCountCache)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellCountCache(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellCountCache)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellIndex(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellIndex)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellMargin(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellMargin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellMargin(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellMargin)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHeight(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHeight)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetIndent(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndent(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIndent)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetKeepTogether(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeepTogether)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeepTogether(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKeepTogether)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetKeepWithNext(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeepWithNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKeepWithNext)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetNestLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNestLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRTL(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRTL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRTL(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRTL)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellAlignment(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellAlignment(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellAlignment)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellColorBack(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellColorBack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellColorBack(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellColorBack)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellColorFore(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellColorFore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellColorFore(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellColorFore)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellMergeFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellMergeFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellMergeFlags(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellMergeFlags)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellShading(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellShading)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellShading(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellShading)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellVerticalText(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellVerticalText)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellVerticalText(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellVerticalText)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCellWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCellWidth(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellWidth)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetCellBorderColors(&self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCellBorderColors)(windows_core::Interface::as_raw(self), pcrleft as _, pcrtop as _, pcrright as _, pcrbottom as _) }
    }
    pub unsafe fn GetCellBorderWidths(&self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCellBorderWidths)(windows_core::Interface::as_raw(self), pduleft as _, pdutop as _, pduright as _, pdubottom as _) }
    }
    pub unsafe fn SetCellBorderColors(&self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellBorderColors)(windows_core::Interface::as_raw(self), crleft, crtop, crright, crbottom) }
    }
    pub unsafe fn SetCellBorderWidths(&self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCellBorderWidths)(windows_core::Interface::as_raw(self), duleft, dutop, duright, dubottom) }
    }
    pub unsafe fn Apply(&self, crow: i32, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self), crow, flags) }
    }
    pub unsafe fn CanChange(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Insert(&self, crow: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), crow) }
    }
    pub unsafe fn IsEqual<P0>(&self, prow: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), prow.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextRow_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellCountCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellCountCache: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellMargin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellMargin: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetIndent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIndent: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetKeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKeepTogether: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetKeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKeepWithNext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetNestLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetRTL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRTL: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellColorBack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellColorBack: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellColorFore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellColorFore: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellMergeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellMergeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellShading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellShading: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellVerticalText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellVerticalText: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCellWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCellBorderColors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetCellBorderWidths: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetCellBorderColors: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub SetCellBorderWidths: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub CanChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextRow_Impl: super::oaidl::IDispatch_Impl {
    fn GetAlignment(&self) -> windows_core::Result<i32>;
    fn SetAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellCount(&self) -> windows_core::Result<i32>;
    fn SetCellCount(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellCountCache(&self) -> windows_core::Result<i32>;
    fn SetCellCountCache(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellIndex(&self) -> windows_core::Result<i32>;
    fn SetCellIndex(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellMargin(&self) -> windows_core::Result<i32>;
    fn SetCellMargin(&self, value: i32) -> windows_core::Result<()>;
    fn GetHeight(&self) -> windows_core::Result<i32>;
    fn SetHeight(&self, value: i32) -> windows_core::Result<()>;
    fn GetIndent(&self) -> windows_core::Result<i32>;
    fn SetIndent(&self, value: i32) -> windows_core::Result<()>;
    fn GetKeepTogether(&self) -> windows_core::Result<i32>;
    fn SetKeepTogether(&self, value: i32) -> windows_core::Result<()>;
    fn GetKeepWithNext(&self) -> windows_core::Result<i32>;
    fn SetKeepWithNext(&self, value: i32) -> windows_core::Result<()>;
    fn GetNestLevel(&self) -> windows_core::Result<i32>;
    fn GetRTL(&self) -> windows_core::Result<i32>;
    fn SetRTL(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellAlignment(&self) -> windows_core::Result<i32>;
    fn SetCellAlignment(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellColorBack(&self) -> windows_core::Result<i32>;
    fn SetCellColorBack(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellColorFore(&self) -> windows_core::Result<i32>;
    fn SetCellColorFore(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellMergeFlags(&self) -> windows_core::Result<i32>;
    fn SetCellMergeFlags(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellShading(&self) -> windows_core::Result<i32>;
    fn SetCellShading(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellVerticalText(&self) -> windows_core::Result<i32>;
    fn SetCellVerticalText(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellWidth(&self) -> windows_core::Result<i32>;
    fn SetCellWidth(&self, value: i32) -> windows_core::Result<()>;
    fn GetCellBorderColors(&self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> windows_core::Result<()>;
    fn GetCellBorderWidths(&self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> windows_core::Result<()>;
    fn SetCellBorderColors(&self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> windows_core::Result<()>;
    fn SetCellBorderWidths(&self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> windows_core::Result<()>;
    fn Apply(&self, crow: i32, flags: i32) -> windows_core::Result<()>;
    fn CanChange(&self) -> windows_core::Result<i32>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn Insert(&self, crow: i32) -> windows_core::Result<()>;
    fn IsEqual(&self, prow: windows_core::Ref<ITextRow>) -> windows_core::Result<i32>;
    fn Reset(&self, value: i32) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextRow_Vtbl {
    pub const fn new<Identity: ITextRow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAlignment<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellCount<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellCount(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellCount<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellCount(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellCountCache<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellCountCache(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellCountCache<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellCountCache(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellIndex<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellIndex(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellIndex<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellIndex(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellMargin<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellMargin(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellMargin<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellMargin(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetHeight<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetHeight(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHeight<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetHeight(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetIndent<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetIndent(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndent<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetIndent(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKeepTogether<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetKeepTogether(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetKeepTogether(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetKeepWithNext<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetKeepWithNext(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetKeepWithNext(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetNestLevel<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetNestLevel(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRTL<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetRTL(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRTL<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetRTL(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellAlignment<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellAlignment(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellAlignment<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellAlignment(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellColorBack<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellColorBack(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellColorBack<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellColorBack(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellColorFore<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellColorFore(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellColorFore<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellColorFore(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellMergeFlags<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellMergeFlags(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellMergeFlags<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellMergeFlags(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellShading<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellShading(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellShading<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellShading(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellVerticalText<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellVerticalText(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellVerticalText<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellVerticalText(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellWidth<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetCellWidth(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCellWidth<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellWidth(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCellBorderColors<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::GetCellBorderColors(this, core::mem::transmute_copy(&pcrleft), core::mem::transmute_copy(&pcrtop), core::mem::transmute_copy(&pcrright), core::mem::transmute_copy(&pcrbottom)).into()
            }
        }
        unsafe extern "system" fn GetCellBorderWidths<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::GetCellBorderWidths(this, core::mem::transmute_copy(&pduleft), core::mem::transmute_copy(&pdutop), core::mem::transmute_copy(&pduright), core::mem::transmute_copy(&pdubottom)).into()
            }
        }
        unsafe extern "system" fn SetCellBorderColors<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellBorderColors(this, core::mem::transmute_copy(&crleft), core::mem::transmute_copy(&crtop), core::mem::transmute_copy(&crright), core::mem::transmute_copy(&crbottom)).into()
            }
        }
        unsafe extern "system" fn SetCellBorderWidths<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetCellBorderWidths(this, core::mem::transmute_copy(&duleft), core::mem::transmute_copy(&dutop), core::mem::transmute_copy(&duright), core::mem::transmute_copy(&dubottom)).into()
            }
        }
        unsafe extern "system" fn Apply<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crow: i32, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::Apply(this, core::mem::transmute_copy(&crow), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn CanChange<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::CanChange(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Insert<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crow: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::Insert(this, core::mem::transmute_copy(&crow)).into()
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prow: *mut core::ffi::c_void, pb: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRow_Impl::IsEqual(this, core::mem::transmute_copy(&prow)) {
                    Ok(ok__) => {
                        pb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::Reset(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextRow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRow_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetAlignment: GetAlignment::<Identity, OFFSET>,
            SetAlignment: SetAlignment::<Identity, OFFSET>,
            GetCellCount: GetCellCount::<Identity, OFFSET>,
            SetCellCount: SetCellCount::<Identity, OFFSET>,
            GetCellCountCache: GetCellCountCache::<Identity, OFFSET>,
            SetCellCountCache: SetCellCountCache::<Identity, OFFSET>,
            GetCellIndex: GetCellIndex::<Identity, OFFSET>,
            SetCellIndex: SetCellIndex::<Identity, OFFSET>,
            GetCellMargin: GetCellMargin::<Identity, OFFSET>,
            SetCellMargin: SetCellMargin::<Identity, OFFSET>,
            GetHeight: GetHeight::<Identity, OFFSET>,
            SetHeight: SetHeight::<Identity, OFFSET>,
            GetIndent: GetIndent::<Identity, OFFSET>,
            SetIndent: SetIndent::<Identity, OFFSET>,
            GetKeepTogether: GetKeepTogether::<Identity, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Identity, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, OFFSET>,
            GetNestLevel: GetNestLevel::<Identity, OFFSET>,
            GetRTL: GetRTL::<Identity, OFFSET>,
            SetRTL: SetRTL::<Identity, OFFSET>,
            GetCellAlignment: GetCellAlignment::<Identity, OFFSET>,
            SetCellAlignment: SetCellAlignment::<Identity, OFFSET>,
            GetCellColorBack: GetCellColorBack::<Identity, OFFSET>,
            SetCellColorBack: SetCellColorBack::<Identity, OFFSET>,
            GetCellColorFore: GetCellColorFore::<Identity, OFFSET>,
            SetCellColorFore: SetCellColorFore::<Identity, OFFSET>,
            GetCellMergeFlags: GetCellMergeFlags::<Identity, OFFSET>,
            SetCellMergeFlags: SetCellMergeFlags::<Identity, OFFSET>,
            GetCellShading: GetCellShading::<Identity, OFFSET>,
            SetCellShading: SetCellShading::<Identity, OFFSET>,
            GetCellVerticalText: GetCellVerticalText::<Identity, OFFSET>,
            SetCellVerticalText: SetCellVerticalText::<Identity, OFFSET>,
            GetCellWidth: GetCellWidth::<Identity, OFFSET>,
            SetCellWidth: SetCellWidth::<Identity, OFFSET>,
            GetCellBorderColors: GetCellBorderColors::<Identity, OFFSET>,
            GetCellBorderWidths: GetCellBorderWidths::<Identity, OFFSET>,
            SetCellBorderColors: SetCellBorderColors::<Identity, OFFSET>,
            SetCellBorderWidths: SetCellBorderWidths::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            CanChange: CanChange::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRow as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextRow {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextSelection, ITextSelection_Vtbl, 0x8cc497c1_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextSelection {
    type Target = ITextRange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextSelection, windows_core::IUnknown, super::oaidl::IDispatch, ITextRange);
#[cfg(feature = "Win32_oaidl")]
impl ITextSelection {
    pub unsafe fn GetFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlags(&self, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveLeft)(windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveRight)(windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveUp)(windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveDown)(windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HomeKey(&self, unit: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HomeKey)(windows_core::Interface::as_raw(self), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndKey)(windows_core::Interface::as_raw(self), unit, extend, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TypeText(&self, bstr: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TypeText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection_Vtbl {
    pub base__: ITextRange_Vtbl,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MoveLeft: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveRight: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveUp: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveDown: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub HomeKey: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub EndKey: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    pub TypeText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextSelection_Impl: ITextRange_Impl {
    fn GetFlags(&self) -> windows_core::Result<i32>;
    fn SetFlags(&self, flags: i32) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<i32>;
    fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> windows_core::Result<i32>;
    fn HomeKey(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn EndKey(&self, unit: i32, extend: i32) -> windows_core::Result<i32>;
    fn TypeText(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextSelection_Vtbl {
    pub const fn new<Identity: ITextSelection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFlags<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextSelection_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::GetType(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveLeft<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveLeft(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveRight<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveRight(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveUp<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveUp(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveDown<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::MoveDown(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HomeKey<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::HomeKey(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndKey<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextSelection_Impl::EndKey(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&extend)) {
                    Ok(ok__) => {
                        pdelta.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TypeText<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextSelection_Impl::TypeText(this, core::mem::transmute(&bstr)).into()
            }
        }
        Self {
            base__: ITextRange_Vtbl::new::<Identity, OFFSET>(),
            GetFlags: GetFlags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            MoveLeft: MoveLeft::<Identity, OFFSET>,
            MoveRight: MoveRight::<Identity, OFFSET>,
            MoveUp: MoveUp::<Identity, OFFSET>,
            MoveDown: MoveDown::<Identity, OFFSET>,
            HomeKey: HomeKey::<Identity, OFFSET>,
            EndKey: EndKey::<Identity, OFFSET>,
            TypeText: TypeText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextSelection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ITextRange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextSelection {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextSelection2, ITextSelection2_Vtbl, 0xc241f5e1_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextSelection2 {
    type Target = ITextRange2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextSelection2, windows_core::IUnknown, super::oaidl::IDispatch, ITextRange, ITextSelection, ITextRange2);
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection2_Vtbl {
    pub base__: ITextRange2_Vtbl,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextSelection2_Impl: ITextRange2_Impl {}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextSelection2_Vtbl {
    pub const fn new<Identity: ITextSelection2_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ITextRange2_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextSelection2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ITextRange as windows_core::Interface>::IID || iid == &<ITextSelection as windows_core::Interface>::IID || iid == &<ITextRange2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextSelection2 {}
windows_core::imp::define_interface!(ITextStory, ITextStory_Vtbl, 0xc241f5f3_7206_11d8_a2c7_00a0d1d6c6b3);
windows_core::imp::interface_hierarchy!(ITextStory, windows_core::IUnknown);
impl ITextStory {
    pub unsafe fn GetActive(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetActive(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetActive)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetDisplay(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplay)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_oaidl")]
    pub unsafe fn GetRange(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRange)(windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetText(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFormattedText<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFormattedText)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), r#type, value) }
    }
    pub unsafe fn SetText(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), flags, core::mem::transmute_copy(bstr)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetActive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_oaidl")]
    pub GetRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_oaidl"))]
    GetRange: usize,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_oaidl")]
pub trait ITextStory_Impl: windows_core::IUnknownImpl {
    fn GetActive(&self) -> windows_core::Result<i32>;
    fn SetActive(&self, value: i32) -> windows_core::Result<()>;
    fn GetDisplay(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetIndex(&self) -> windows_core::Result<i32>;
    fn GetType(&self) -> windows_core::Result<i32>;
    fn SetType(&self, value: i32) -> windows_core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> windows_core::Result<i32>;
    fn GetRange(&self, cpactive: i32, cpanchor: i32) -> windows_core::Result<ITextRange2>;
    fn GetText(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormattedText(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> windows_core::Result<()>;
    fn SetText(&self, flags: i32, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_oaidl")]
impl ITextStory_Vtbl {
    pub const fn new<Identity: ITextStory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActive<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetActive(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActive<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetActive(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetDisplay<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisplay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetDisplay(this) {
                    Ok(ok__) => {
                        ppdisplay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetIndex(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetType(this) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetProperty(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRange<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetRange(this, core::mem::transmute_copy(&cpactive), core::mem::transmute_copy(&cpanchor)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStory_Impl::GetText(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetFormattedText(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetProperty(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextStory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStory_Impl::SetText(this, core::mem::transmute_copy(&flags), core::mem::transmute(&bstr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActive: GetActive::<Identity, OFFSET>,
            SetActive: SetActive::<Identity, OFFSET>,
            GetDisplay: GetDisplay::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetRange: GetRange::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_oaidl")]
impl windows_core::RuntimeName for ITextStory {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextStoryRanges, ITextStoryRanges_Vtbl, 0x8cc497c5_a1df_11ce_8098_00aa0047be5d);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextStoryRanges {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextStoryRanges, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ITextStoryRanges {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ITextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRanges_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextStoryRanges_Impl: super::oaidl::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<ITextRange>;
    fn GetCount(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextStoryRanges_Vtbl {
    pub const fn new<Identity: ITextStoryRanges_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoryRanges_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunkenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoryRanges_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoryRanges_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoryRanges as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextStoryRanges {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextStoryRanges2, ITextStoryRanges2_Vtbl, 0xc241f5e5_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextStoryRanges2 {
    type Target = ITextStoryRanges;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextStoryRanges2, windows_core::IUnknown, super::oaidl::IDispatch, ITextStoryRanges);
#[cfg(feature = "Win32_oaidl")]
impl ITextStoryRanges2 {
    pub unsafe fn Item2(&self, index: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item2)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRanges2_Vtbl {
    pub base__: ITextStoryRanges_Vtbl,
    pub Item2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextStoryRanges2_Impl: ITextStoryRanges_Impl {
    fn Item2(&self, index: i32) -> windows_core::Result<ITextRange2>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextStoryRanges2_Vtbl {
    pub const fn new<Identity: ITextStoryRanges2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item2<Identity: ITextStoryRanges2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoryRanges2_Impl::Item2(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ITextStoryRanges_Vtbl::new::<Identity, OFFSET>(), Item2: Item2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoryRanges2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ITextStoryRanges as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextStoryRanges2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ITextStrings, ITextStrings_Vtbl, 0xc241f5e7_7206_11d8_a2c7_00a0d1d6c6b3);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ITextStrings {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ITextStrings, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ITextStrings {
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ITextRange2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, bstr: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)) }
    }
    pub unsafe fn Append<P0>(&self, prange: P0, istring: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), prange.param().abi(), istring) }
    }
    pub unsafe fn Cat2(&self, istring: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cat2)(windows_core::Interface::as_raw(self), istring) }
    }
    pub unsafe fn CatTop2(&self, bstr: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CatTop2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)) }
    }
    pub unsafe fn DeleteRange<P0>(&self, prange: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteRange)(windows_core::Interface::as_raw(self), prange.param().abi()) }
    }
    pub unsafe fn EncodeFunction<P8>(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: P8) -> windows_core::HRESULT
    where
        P8: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).EncodeFunction)(windows_core::Interface::as_raw(self), r#type, align, char, char1, char2, count, texstyle, ccol, prange.param().abi()) }
    }
    pub unsafe fn GetCch(&self, istring: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCch)(windows_core::Interface::as_raw(self), istring, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertNullStr(&self, istring: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertNullStr)(windows_core::Interface::as_raw(self), istring) }
    }
    pub unsafe fn MoveBoundary(&self, istring: i32, cch: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveBoundary)(windows_core::Interface::as_raw(self), istring, cch) }
    }
    pub unsafe fn PrefixTop(&self, bstr: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrefixTop)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)) }
    }
    pub unsafe fn Remove(&self, istring: i32, cstring: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), istring, cstring) }
    }
    pub unsafe fn SetFormattedText<P0, P1>(&self, pranged: P0, pranges: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITextRange2>,
        P1: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFormattedText)(windows_core::Interface::as_raw(self), pranged.param().abi(), pranges.param().abi()) }
    }
    pub unsafe fn SetOpCp(&self, istring: i32, cp: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOpCp)(windows_core::Interface::as_raw(self), istring, cp) }
    }
    pub unsafe fn SuffixTop<P1>(&self, bstr: &windows_core::BSTR, prange: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITextRange2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SuffixTop)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr), prange.param().abi()) }
    }
    pub unsafe fn Swap(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Swap)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextStrings_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Cat2: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CatTop2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EncodeFunction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, i32, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCch: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub InsertNullStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MoveBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub PrefixTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOpCp: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SuffixTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Swap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ITextStrings_Impl: super::oaidl::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<ITextRange2>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn Add(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Append(&self, prange: windows_core::Ref<ITextRange2>, istring: i32) -> windows_core::Result<()>;
    fn Cat2(&self, istring: i32) -> windows_core::Result<()>;
    fn CatTop2(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteRange(&self, prange: windows_core::Ref<ITextRange2>) -> windows_core::Result<()>;
    fn EncodeFunction(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: windows_core::Ref<ITextRange2>) -> windows_core::Result<()>;
    fn GetCch(&self, istring: i32) -> windows_core::Result<i32>;
    fn InsertNullStr(&self, istring: i32) -> windows_core::Result<()>;
    fn MoveBoundary(&self, istring: i32, cch: i32) -> windows_core::Result<()>;
    fn PrefixTop(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Remove(&self, istring: i32, cstring: i32) -> windows_core::Result<()>;
    fn SetFormattedText(&self, pranged: windows_core::Ref<ITextRange2>, pranges: windows_core::Ref<ITextRange2>) -> windows_core::Result<()>;
    fn SetOpCp(&self, istring: i32, cp: i32) -> windows_core::Result<()>;
    fn SuffixTop(&self, bstr: &windows_core::BSTR, prange: windows_core::Ref<ITextRange2>) -> windows_core::Result<()>;
    fn Swap(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ITextStrings_Vtbl {
    pub const fn new<Identity: ITextStrings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStrings_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStrings_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Add(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, istring: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Append(this, core::mem::transmute_copy(&prange), core::mem::transmute_copy(&istring)).into()
            }
        }
        unsafe extern "system" fn Cat2<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Cat2(this, core::mem::transmute_copy(&istring)).into()
            }
        }
        unsafe extern "system" fn CatTop2<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::CatTop2(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn DeleteRange<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::DeleteRange(this, core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn EncodeFunction<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::EncodeFunction(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&align), core::mem::transmute_copy(&char), core::mem::transmute_copy(&char1), core::mem::transmute_copy(&char2), core::mem::transmute_copy(&count), core::mem::transmute_copy(&texstyle), core::mem::transmute_copy(&ccol), core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn GetCch<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, pcch: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStrings_Impl::GetCch(this, core::mem::transmute_copy(&istring)) {
                    Ok(ok__) => {
                        pcch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertNullStr<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::InsertNullStr(this, core::mem::transmute_copy(&istring)).into()
            }
        }
        unsafe extern "system" fn MoveBoundary<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, cch: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::MoveBoundary(this, core::mem::transmute_copy(&istring), core::mem::transmute_copy(&cch)).into()
            }
        }
        unsafe extern "system" fn PrefixTop<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::PrefixTop(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, cstring: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Remove(this, core::mem::transmute_copy(&istring), core::mem::transmute_copy(&cstring)).into()
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pranged: *mut core::ffi::c_void, pranges: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::SetFormattedText(this, core::mem::transmute_copy(&pranged), core::mem::transmute_copy(&pranges)).into()
            }
        }
        unsafe extern "system" fn SetOpCp<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istring: i32, cp: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::SetOpCp(this, core::mem::transmute_copy(&istring), core::mem::transmute_copy(&cp)).into()
            }
        }
        unsafe extern "system" fn SuffixTop<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::SuffixTop(this, core::mem::transmute(&bstr), core::mem::transmute_copy(&prange)).into()
            }
        }
        unsafe extern "system" fn Swap<Identity: ITextStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStrings_Impl::Swap(this).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            Cat2: Cat2::<Identity, OFFSET>,
            CatTop2: CatTop2::<Identity, OFFSET>,
            DeleteRange: DeleteRange::<Identity, OFFSET>,
            EncodeFunction: EncodeFunction::<Identity, OFFSET>,
            GetCch: GetCch::<Identity, OFFSET>,
            InsertNullStr: InsertNullStr::<Identity, OFFSET>,
            MoveBoundary: MoveBoundary::<Identity, OFFSET>,
            PrefixTop: PrefixTop::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, OFFSET>,
            SetOpCp: SetOpCp::<Identity, OFFSET>,
            SuffixTop: SuffixTop::<Identity, OFFSET>,
            Swap: Swap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStrings as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ITextStrings {}
pub type MANCODE = i32;
pub const MBOLD: MANCODE = 16;
pub const MFRAK: MANCODE = 2;
pub const MGREEK: MANCODE = 64;
pub const MINIT: MANCODE = 8;
pub const MISOL: MANCODE = 7;
pub const MITAL: MANCODE = 32;
pub const MLOOP: MANCODE = 11;
pub const MMATH: MANCODE = 6;
pub const MMONO: MANCODE = 5;
pub const MOPEN: MANCODE = 3;
pub const MOPENA: MANCODE = 12;
pub const MROMN: MANCODE = 0;
pub const MSANS: MANCODE = 4;
pub const MSCRP: MANCODE = 1;
pub const MSTRCH: MANCODE = 10;
pub const MTAIL: MANCODE = 9;
pub type OBJECTTYPE = i32;
pub const tomAboriginal: tomConstants = 39;
pub const tomAccent: OBJECTTYPE = 10;
pub const tomAdjustCRLF: tomConstants = 1;
pub const tomAlignBar: tomConstants = 4;
pub const tomAlignCenter: tomConstants = 1;
pub const tomAlignDecimal: tomConstants = 3;
pub const tomAlignDefault: tomConstants = 0;
pub const tomAlignInterLetter: tomConstants = 5;
pub const tomAlignInterWord: tomConstants = 3;
pub const tomAlignJustify: tomConstants = 3;
pub const tomAlignLeft: tomConstants = 0;
pub const tomAlignMatchAscentDescent: tomConstants = 2;
pub const tomAlignNewspaper: tomConstants = 4;
pub const tomAlignRight: tomConstants = 2;
pub const tomAlignScaled: tomConstants = 6;
pub const tomAllCaps: tomConstants = -2147483520;
pub const tomAllowFinalEOP: tomConstants = 8;
pub const tomAllowMathBold: tomConstants = 2048;
pub const tomAllowOffClient: tomConstants = 512;
pub const tomAnimationMax: tomConstants = 8;
pub const tomAnsi: tomConstants = 0;
pub const tomApplyLater: tomConstants = 1;
pub const tomApplyNow: tomConstants = 0;
pub const tomApplyRtfDocProps: tomConstants = 16384;
pub const tomApplyTmp: tomConstants = 4;
pub const tomArabic: tomConstants = 6;
pub const tomArmenian: tomConstants = 19;
pub const tomAtEnd: tomConstants = 4096;
pub const tomAutoBackColor: tomConstants = -2080374784;
pub const tomAutoColor: tomConstants = -9999997;
pub const tomAutoLinkEmail: tomConstants = 5;
pub const tomAutoLinkPath: tomConstants = 7;
pub const tomAutoLinkPhone: tomConstants = 6;
pub const tomAutoLinkURL: tomConstants = 4;
pub const tomAutoSpaceAlpha: tomConstants = 4;
pub const tomAutoSpaceNumeric: tomConstants = 8;
pub const tomAutoSpaceParens: tomConstants = 16;
pub const tomAutoTextColor: tomConstants = -1073741824;
pub const tomBIG5: tomConstants = 15;
pub const tomBackward: tomConstants = -1073741823;
pub const tomBaltic: tomConstants = 7;
pub const tomBengali: tomConstants = 23;
pub const tomBlinkingBackground: tomConstants = 2;
pub const tomBold: tomConstants = -2147483647;
pub const tomBox: OBJECTTYPE = 11;
pub const tomBoxAlignCenter: tomConstants = 1;
pub const tomBoxHideBottom: tomConstants = 2;
pub const tomBoxHideLeft: tomConstants = 4;
pub const tomBoxHideRight: tomConstants = 8;
pub const tomBoxHideTop: tomConstants = 1;
pub const tomBoxStrikeBLTR: tomConstants = 128;
pub const tomBoxStrikeH: tomConstants = 16;
pub const tomBoxStrikeTLBR: tomConstants = 64;
pub const tomBoxStrikeV: tomConstants = 32;
pub const tomBoxedFormula: OBJECTTYPE = 12;
pub const tomBrackets: OBJECTTYPE = 13;
pub const tomBracketsWithSeps: OBJECTTYPE = 14;
pub const tomBraille: tomConstants = 44;
pub const tomCacheParms: tomConstants = 3;
pub const tomCanCopy: tomConstants = 137;
pub const tomCanRedo: tomConstants = 138;
pub const tomCanUndo: tomConstants = 139;
pub const tomCell: tomConstants = 12;
pub const tomCellStructureChangeOnly: tomConstants = 1;
pub const tomCharFormat: tomConstants = 13;
pub const tomCharRepFromLcid: tomConstants = 1073741824;
pub const tomCharRepMax: tomConstants = 63;
pub const tomCharacter: tomConstants = 1;
pub const tomCharset: tomConstants = -2147483648;
pub const tomCheckTextLimit: tomConstants = 32;
pub const tomCherokee: tomConstants = 38;
pub const tomClientCoord: tomConstants = 256;
pub const tomClientLink: tomConstants = 1;
pub const tomCluster: tomConstants = 19;
pub const tomCollapseEnd: tomConstants = 0;
pub const tomCollapseStart: tomConstants = 1;
pub const tomColumn: tomConstants = 9;
pub const tomCommentsStory: tomConstants = 4;
pub const tomCompressMax: tomConstants = 2;
pub const tomCompressNone: tomConstants = 0;
pub const tomCompressPunctuation: tomConstants = 1;
pub const tomCompressPunctuationAndKana: tomConstants = 2;
pub type tomConstants = i32;
pub const tomConvertMathChar: tomConstants = 512;
pub const tomConvertRTF: tomConstants = 8192;
pub const tomCreateAlways: tomConstants = 32;
pub const tomCreateNew: tomConstants = 16;
pub const tomCyrillic: tomConstants = 2;
pub const tomDash: tomConstants = 5;
pub const tomDashDot: tomConstants = 6;
pub const tomDashDotDot: tomConstants = 7;
pub const tomDashes: tomConstants = 2;
pub const tomDecDecSize: tomConstants = 254;
pub const tomDecSize: tomConstants = 255;
pub const tomDefault: tomConstants = -9999996;
pub const tomDefaultCharRep: tomConstants = 9;
pub const tomDefaultTab: tomConstants = 5;
pub const tomDeseret: tomConstants = 61;
pub const tomDevanagari: tomConstants = 22;
pub const tomDisableSmartFont: tomConstants = 8;
pub const tomDisabled: tomConstants = -2147475456;
pub const tomDocAutoLink: tomConstants = 141;
pub const tomDocMathBuild: tomConstants = 128;
pub const tomDontGrowWithContent: tomConstants = 64;
pub const tomDots: tomConstants = 1;
pub const tomDotted: tomConstants = 4;
pub const tomDouble: tomConstants = 3;
pub const tomDoubleWave: tomConstants = 11;
pub const tomDoublestrike: tomConstants = 64;
pub const tomEastEurope: tomConstants = 1;
pub const tomEllipsisEnd: tomConstants = 1;
pub const tomEllipsisMode: tomConstants = 142;
pub const tomEllipsisNone: tomConstants = 0;
pub const tomEllipsisPresent: tomConstants = 1;
pub const tomEllipsisState: tomConstants = 143;
pub const tomEllipsisWord: tomConstants = 3;
pub const tomEmbeddedFont: tomConstants = 32;
pub const tomEmboss: tomConstants = -2147481600;
pub const tomEmoji: tomConstants = 53;
pub const tomEnableSmartFont: tomConstants = 9;
pub const tomEnd: tomConstants = 0;
pub const tomEndnotesStory: tomConstants = 3;
pub const tomEq: OBJECTTYPE = 9;
pub const tomEqArrayAlignBottomRow: tomConstants = 12;
pub const tomEqArrayAlignCenter: tomConstants = 0;
pub const tomEqArrayAlignMask: tomConstants = 12;
pub const tomEqArrayAlignTopRow: tomConstants = 4;
pub const tomEqArrayLayoutWidth: tomConstants = 1;
pub const tomEquals: tomConstants = 5;
pub const tomEquationArray: OBJECTTYPE = 15;
pub const tomEthiopic: tomConstants = 37;
pub const tomEvenPagesFooterStory: tomConstants = 8;
pub const tomEvenPagesHeaderStory: tomConstants = 6;
pub const tomExtend: tomConstants = 1;
pub const tomExtendedChar: tomConstants = -2113929216;
pub const tomFalse: tomConstants = 0;
pub const tomFindStory: tomConstants = 128;
pub const tomFirstPageFooterStory: tomConstants = 11;
pub const tomFirstPageHeaderStory: tomConstants = 10;
pub const tomFoldMathAlpha: tomConstants = 16;
pub const tomFontAlignmentAuto: tomConstants = 0;
pub const tomFontAlignmentBaseline: tomConstants = 2;
pub const tomFontAlignmentBottom: tomConstants = 3;
pub const tomFontAlignmentCenter: tomConstants = 4;
pub const tomFontAlignmentMax: tomConstants = 4;
pub const tomFontAlignmentTop: tomConstants = 1;
pub const tomFontBound: tomConstants = -2146435072;
pub const tomFontPropAlign: tomConstants = 829;
pub const tomFontPropTeXStyle: tomConstants = 828;
pub const tomFontStretch: tomConstants = 830;
pub const tomFontStretchCondensed: tomConstants = 3;
pub const tomFontStretchDefault: tomConstants = 0;
pub const tomFontStretchExpanded: tomConstants = 7;
pub const tomFontStretchExtraCondensed: tomConstants = 2;
pub const tomFontStretchExtraExpanded: tomConstants = 8;
pub const tomFontStretchNormal: tomConstants = 5;
pub const tomFontStretchSemiCondensed: tomConstants = 4;
pub const tomFontStretchSemiExpanded: tomConstants = 6;
pub const tomFontStretchUltraCondensed: tomConstants = 1;
pub const tomFontStretchUltraExpanded: tomConstants = 9;
pub const tomFontStyle: tomConstants = 831;
pub const tomFontStyleItalic: tomConstants = 2;
pub const tomFontStyleOblique: tomConstants = 1;
pub const tomFontStyleUpright: tomConstants = 0;
pub const tomFontWeightBlack: tomConstants = 900;
pub const tomFontWeightBold: tomConstants = 700;
pub const tomFontWeightDefault: tomConstants = 0;
pub const tomFontWeightExtraBlack: tomConstants = 950;
pub const tomFontWeightExtraBold: tomConstants = 800;
pub const tomFontWeightExtraLight: tomConstants = 200;
pub const tomFontWeightHeavy: tomConstants = 900;
pub const tomFontWeightLight: tomConstants = 300;
pub const tomFontWeightMedium: tomConstants = 500;
pub const tomFontWeightNormal: tomConstants = 400;
pub const tomFontWeightRegular: tomConstants = 400;
pub const tomFontWeightSemiBold: tomConstants = 600;
pub const tomFontWeightThin: tomConstants = 100;
pub const tomFootnotesStory: tomConstants = 2;
pub const tomForward: tomConstants = 1073741823;
pub const tomFraction: OBJECTTYPE = 16;
pub const tomFriendlyLinkAddress: tomConstants = 3;
pub const tomFriendlyLinkName: tomConstants = 2;
pub const tomFunctionApply: OBJECTTYPE = 17;
pub const tomFunctionTypeIsLim: tomConstants = 4;
pub const tomFunctionTypeNone: tomConstants = 0;
pub const tomFunctionTypeTakesArg: tomConstants = 1;
pub const tomFunctionTypeTakesLim: tomConstants = 2;
pub const tomFunctionTypeTakesLim2: tomConstants = 3;
pub const tomGB2312: tomConstants = 13;
pub const tomGeorgian: tomConstants = 35;
pub const tomGetHeightOnly: tomConstants = 8;
pub const tomGlagolitic: tomConstants = 54;
pub const tomGothic: tomConstants = 60;
pub const tomGravityBack: tomConstants = 1;
pub const tomGravityBackward: tomConstants = 536870912;
pub const tomGravityFore: tomConstants = 2;
pub const tomGravityForward: tomConstants = 1073741824;
pub const tomGravityIn: tomConstants = 3;
pub const tomGravityOut: tomConstants = 4;
pub const tomGravityUI: tomConstants = 0;
pub const tomGreek: tomConstants = 3;
pub const tomGrowWithContent: tomConstants = 128;
pub const tomGujarati: tomConstants = 25;
pub const tomGurmukhi: tomConstants = 24;
pub const tomHContCell: tomConstants = 8;
pub const tomHStartCell: tomConstants = 4;
pub const tomHTML: tomConstants = 3;
pub const tomHair: tomConstants = 10;
pub const tomHangul: tomConstants = 14;
pub const tomHardParagraph: tomConstants = 18;
pub const tomHeavyWave: tomConstants = 12;
pub const tomHebrew: tomConstants = 5;
pub const tomHidden: tomConstants = -2147483392;
pub const tomHorzVert: OBJECTTYPE = 2;
pub const tomHstring: tomConstants = 596;
pub const tomIgnoreCurrentFont: tomConstants = 0;
pub const tomIgnoreNumberStyle: tomConstants = 16777216;
pub const tomImprint: tomConstants = -2147479552;
pub const tomIncIncSize: tomConstants = 66;
pub const tomIncSize: tomConstants = 65;
pub const tomIncludeInset: tomConstants = 1;
pub const tomIncludeNumbering: tomConstants = 64;
pub const tomInlineObject: tomConstants = 20;
pub const tomInlineObjectArg: tomConstants = 21;
pub const tomInlineObjectStart: tomConstants = -2130706432;
pub const tomItalic: tomConstants = -2147483646;
pub const tomJamo: tomConstants = 36;
pub const tomKannada: tomConstants = 29;
pub const tomKayahli: tomConstants = 51;
pub const tomKharoshthi: tomConstants = 50;
pub const tomKhmer: tomConstants = 42;
pub const tomKoreanBlockCaret: tomConstants = 1;
pub const tomLanguageTag: tomConstants = 4096;
pub const tomLao: tomConstants = 32;
pub const tomLasVegasLights: tomConstants = 1;
pub const tomLayoutColumn: tomConstants = 23;
pub const tomLeafLine: tomConstants = 22;
pub const tomLeftSubSup: OBJECTTYPE = 18;
pub const tomLimbu: tomConstants = 46;
pub const tomLimitAlignCenter: tomConstants = 0;
pub const tomLimitAlignLeft: tomConstants = 1;
pub const tomLimitAlignMask: tomConstants = 3;
pub const tomLimitAlignRight: tomConstants = 2;
pub const tomLimitsDefault: tomConstants = 0;
pub const tomLimitsOpposite: tomConstants = 4;
pub const tomLimitsSubSup: tomConstants = 2;
pub const tomLimitsUnderOver: tomConstants = 1;
pub const tomLine: tomConstants = 5;
pub const tomLineSpace1pt5: tomConstants = 1;
pub const tomLineSpaceAtLeast: tomConstants = 3;
pub const tomLineSpaceDouble: tomConstants = 2;
pub const tomLineSpaceExactly: tomConstants = 4;
pub const tomLineSpaceMultiple: tomConstants = 5;
pub const tomLineSpacePercent: tomConstants = 6;
pub const tomLineSpaceSingle: tomConstants = 0;
pub const tomLines: tomConstants = 3;
pub const tomLink: tomConstants = -2147483616;
pub const tomLinkProtected: tomConstants = -2139095040;
pub const tomListBullet: tomConstants = 1;
pub const tomListMinus: tomConstants = 524288;
pub const tomListNoNumber: tomConstants = 262144;
pub const tomListNone: tomConstants = 0;
pub const tomListNumberAsArabic: tomConstants = 2;
pub const tomListNumberAsLCLetter: tomConstants = 3;
pub const tomListNumberAsLCRoman: tomConstants = 5;
pub const tomListNumberAsSequence: tomConstants = 7;
pub const tomListNumberAsUCLetter: tomConstants = 4;
pub const tomListNumberAsUCRoman: tomConstants = 6;
pub const tomListNumberedArabic1: tomConstants = 16;
pub const tomListNumberedArabic2: tomConstants = 17;
pub const tomListNumberedArabicWide: tomConstants = 11;
pub const tomListNumberedBlackCircleWingding: tomConstants = 9;
pub const tomListNumberedChS: tomConstants = 12;
pub const tomListNumberedChT: tomConstants = 13;
pub const tomListNumberedCircle: tomConstants = 8;
pub const tomListNumberedHebrew: tomConstants = 18;
pub const tomListNumberedHindiAlpha: tomConstants = 21;
pub const tomListNumberedHindiAlpha1: tomConstants = 22;
pub const tomListNumberedHindiNum: tomConstants = 23;
pub const tomListNumberedJpnChS: tomConstants = 14;
pub const tomListNumberedJpnKor: tomConstants = 15;
pub const tomListNumberedThaiAlpha: tomConstants = 19;
pub const tomListNumberedThaiNum: tomConstants = 20;
pub const tomListNumberedWhiteCircleWingding: tomConstants = 10;
pub const tomListParentheses: tomConstants = 65536;
pub const tomListPeriod: tomConstants = 131072;
pub const tomListPlain: tomConstants = 196608;
pub const tomLisu: tomConstants = 55;
pub const tomLongDash: tomConstants = 13;
pub const tomLowerCase: tomConstants = 0;
pub const tomLowerLimit: OBJECTTYPE = 19;
pub const tomMac: tomConstants = 18;
pub const tomMainTextStory: tomConstants = 1;
pub const tomMalayalam: tomConstants = 30;
pub const tomMarchingBlackAnts: tomConstants = 4;
pub const tomMarchingRedAnts: tomConstants = 5;
pub const tomMatchAscii: tomConstants = 4;
pub const tomMatchCase: tomConstants = 4;
pub const tomMatchCharRep: tomConstants = 1;
pub const tomMatchFontSignature: tomConstants = 2;
pub const tomMatchMathFont: tomConstants = 16;
pub const tomMatchPattern: tomConstants = 8;
pub const tomMatchWord: tomConstants = 2;
pub const tomMath: OBJECTTYPE = 10;
pub const tomMathArgShadingEnd: tomConstants = 594;
pub const tomMathArgShadingStart: tomConstants = 593;
pub const tomMathBreakCenter: tomConstants = 126;
pub const tomMathBreakLeft: tomConstants = 125;
pub const tomMathBreakRight: tomConstants = 127;
pub const tomMathBrkBinAfter: tomConstants = 65536;
pub const tomMathBrkBinBefore: tomConstants = 0;
pub const tomMathBrkBinDup: tomConstants = 131072;
pub const tomMathBrkBinMask: tomConstants = 196608;
pub const tomMathBrkBinSubMM: tomConstants = 0;
pub const tomMathBrkBinSubMP: tomConstants = 524288;
pub const tomMathBrkBinSubMask: tomConstants = 786432;
pub const tomMathBrkBinSubPM: tomConstants = 262144;
pub const tomMathCFCheck: tomConstants = 4;
pub const tomMathDispAlignCenter: tomConstants = 1;
pub const tomMathDispAlignCenterGroup: tomConstants = 0;
pub const tomMathDispAlignLeft: tomConstants = 2;
pub const tomMathDispAlignMask: tomConstants = 3;
pub const tomMathDispAlignRight: tomConstants = 3;
pub const tomMathDispDef: tomConstants = 2048;
pub const tomMathDispFracTeX: tomConstants = 8;
pub const tomMathDispIntUnderOver: tomConstants = 4;
pub const tomMathDispNaryGrow: tomConstants = 16;
pub const tomMathDispNarySubSup: tomConstants = 1024;
pub const tomMathDocDiffDefault: tomConstants = 0;
pub const tomMathDocDiffItalic: tomConstants = 512;
pub const tomMathDocDiffMask: tomConstants = 768;
pub const tomMathDocDiffOpenItalic: tomConstants = 768;
pub const tomMathDocDiffUpright: tomConstants = 256;
pub const tomMathDocEmptyArgAlways: tomConstants = 32;
pub const tomMathDocEmptyArgAuto: tomConstants = 0;
pub const tomMathDocEmptyArgMask: tomConstants = 96;
pub const tomMathDocEmptyArgNever: tomConstants = 64;
pub const tomMathDocSbSpOpUnchanged: tomConstants = 128;
pub const tomMathEnableRtl: tomConstants = 4096;
pub const tomMathEqAlign: tomConstants = 128;
pub const tomMathInterSpace: tomConstants = 135;
pub const tomMathIntraSpace: tomConstants = 136;
pub const tomMathLMargin: tomConstants = 129;
pub const tomMathManualBreakMask: tomConstants = 127;
pub const tomMathObjShadingEnd: tomConstants = 596;
pub const tomMathObjShadingStart: tomConstants = 595;
pub const tomMathParaAlignCenter: tomConstants = 2;
pub const tomMathParaAlignCenterGroup: tomConstants = 1;
pub const tomMathParaAlignDefault: tomConstants = 0;
pub const tomMathParaAlignLeft: tomConstants = 3;
pub const tomMathParaAlignRight: tomConstants = 4;
pub const tomMathPostSpace: tomConstants = 134;
pub const tomMathPreSpace: tomConstants = 133;
pub const tomMathRMargin: tomConstants = 130;
pub const tomMathRelSize: tomConstants = 64;
pub const tomMathVariant: tomConstants = 32;
pub const tomMathWrapIndent: tomConstants = 131;
pub const tomMathWrapRight: tomConstants = 132;
pub const tomMathZone: tomConstants = -1879048192;
pub const tomMathZoneDisplay: tomConstants = 262144;
pub const tomMathZoneNoBuildUp: tomConstants = -2013265920;
pub const tomMathZoneOrdinary: tomConstants = -1610612736;
pub const tomMatrix: OBJECTTYPE = 20;
pub const tomMatrixAlignBottomRow: tomConstants = 3;
pub const tomMatrixAlignCenter: tomConstants = 0;
pub const tomMatrixAlignMask: tomConstants = 3;
pub const tomMatrixAlignTopRow: tomConstants = 1;
pub const tomModWidthPairs: tomConstants = 1;
pub const tomModWidthSpace: tomConstants = 2;
pub const tomMongolian: tomConstants = 43;
pub const tomMove: tomConstants = 0;
pub const tomMyanmar: tomConstants = 34;
pub const tomNKo: tomConstants = 57;
pub const tomNary: OBJECTTYPE = 21;
pub const tomNewTaiLue: tomConstants = 48;
pub const tomNoAnimation: tomConstants = 0;
pub const tomNoBreak: tomConstants = 128;
pub const tomNoHidden: tomConstants = 32;
pub const tomNoIME: tomConstants = 524288;
pub const tomNoLink: tomConstants = 0;
pub const tomNoMathZoneBrackets: tomConstants = 256;
pub const tomNoSelection: tomConstants = 0;
pub const tomNoUCGreekItalic: tomConstants = 1024;
pub const tomNoUpScroll: tomConstants = 65536;
pub const tomNoVpScroll: tomConstants = 262144;
pub const tomNone: tomConstants = 0;
pub const tomNormalCaret: tomConstants = 0;
pub const tomNullCaret: tomConstants = 2;
pub const tomOEM: tomConstants = 17;
pub const tomObject: tomConstants = 16;
pub const tomObjectArg: tomConstants = 2048;
pub const tomObjectMax: OBJECTTYPE = 33;
pub const tomOgham: tomConstants = 40;
pub const tomOpChar: OBJECTTYPE = 22;
pub const tomOpenAlways: tomConstants = 64;
pub const tomOpenExisting: tomConstants = 48;
pub const tomOriya: tomConstants = 26;
pub const tomOsmanya: tomConstants = 58;
pub const tomOutline: tomConstants = -2147483136;
pub const tomOverbar: OBJECTTYPE = 23;
pub const tomOverlapping: tomConstants = 128;
pub const tomPC437: tomConstants = 16;
pub const tomPage: tomConstants = 17;
pub const tomParaEffectBox: tomConstants = 1024;
pub const tomParaEffectCollapsed: tomConstants = 256;
pub const tomParaEffectDoNotHyphen: tomConstants = 64;
pub const tomParaEffectKeep: tomConstants = 2;
pub const tomParaEffectKeepNext: tomConstants = 4;
pub const tomParaEffectNoLineNumber: tomConstants = 16;
pub const tomParaEffectNoWidowControl: tomConstants = 32;
pub const tomParaEffectOutlineLevel: tomConstants = 512;
pub const tomParaEffectPageBreakBefore: tomConstants = 8;
pub const tomParaEffectRTL: tomConstants = 1;
pub const tomParaEffectSideBySide: tomConstants = 128;
pub const tomParaEffectTable: tomConstants = 16384;
pub const tomParaEffectTableRowDelimiter: tomConstants = 4096;
pub const tomParaFormat: tomConstants = 14;
pub const tomParaPropMathAlign: tomConstants = 1079;
pub const tomParaStyleHeading1: tomConstants = -2;
pub const tomParaStyleHeading2: tomConstants = -3;
pub const tomParaStyleHeading3: tomConstants = -4;
pub const tomParaStyleHeading4: tomConstants = -5;
pub const tomParaStyleHeading5: tomConstants = -6;
pub const tomParaStyleHeading6: tomConstants = -7;
pub const tomParaStyleHeading7: tomConstants = -8;
pub const tomParaStyleHeading8: tomConstants = -9;
pub const tomParaStyleHeading9: tomConstants = -10;
pub const tomParaStyleNormal: tomConstants = -1;
pub const tomParagraph: tomConstants = 4;
pub const tomPasteFile: tomConstants = 4096;
pub const tomPhagsPa: tomConstants = 59;
pub const tomPhantom: OBJECTTYPE = 24;
pub const tomPhantomASmash: tomConstants = 5;
pub const tomPhantomDSmash: tomConstants = 9;
pub const tomPhantomHSmash: tomConstants = 3;
pub const tomPhantomHorz: tomConstants = 12;
pub const tomPhantomShow: tomConstants = 1;
pub const tomPhantomSmash: tomConstants = 13;
pub const tomPhantomTransparent: tomConstants = 16;
pub const tomPhantomVert: tomConstants = 2;
pub const tomPhantomZeroAscent: tomConstants = 4;
pub const tomPhantomZeroDescent: tomConstants = 8;
pub const tomPhantomZeroWidth: tomConstants = 2;
pub const tomPrimaryFooterStory: tomConstants = 9;
pub const tomPrimaryHeaderStory: tomConstants = 7;
pub const tomProcessId: tomConstants = 1073741825;
pub const tomProtected: tomConstants = -2147483632;
pub const tomRE10Mode: tomConstants = 1;
pub const tomRTF: tomConstants = 1;
pub const tomRadical: OBJECTTYPE = 25;
pub const tomReadOnly: tomConstants = 256;
pub const tomReplaceStory: tomConstants = 129;
pub const tomResume: tomConstants = -9999994;
pub const tomRevised: tomConstants = -2147467264;
pub const tomRow: tomConstants = 10;
pub const tomRowApplyDefault: tomConstants = 0;
pub const tomRowHeightActual: tomConstants = 2059;
pub const tomRowUpdate: tomConstants = 1;
pub const tomRuby: OBJECTTYPE = 1;
pub const tomRubyAlign010: tomConstants = 1;
pub const tomRubyAlign121: tomConstants = 2;
pub const tomRubyAlignCenter: tomConstants = 0;
pub const tomRubyAlignLeft: tomConstants = 3;
pub const tomRubyAlignRight: tomConstants = 4;
pub const tomRubyBelow: tomConstants = 128;
pub const tomRunic: tomConstants = 41;
pub const tomScratchStory: tomConstants = 127;
pub const tomScreen: tomConstants = 7;
pub const tomSection: tomConstants = 8;
pub const tomSelActive: tomConstants = 8;
pub const tomSelAtEOL: tomConstants = 2;
pub const tomSelOvertype: tomConstants = 4;
pub const tomSelRange: tomConstants = 597;
pub const tomSelReplace: tomConstants = 16;
pub const tomSelStartActive: tomConstants = 1;
pub const tomSelectionBlock: tomConstants = 6;
pub const tomSelectionColumn: tomConstants = 4;
pub const tomSelectionFrame: tomConstants = 3;
pub const tomSelectionIP: tomConstants = 1;
pub const tomSelectionInlineShape: tomConstants = 7;
pub const tomSelectionNormal: tomConstants = 2;
pub const tomSelectionRow: tomConstants = 5;
pub const tomSelectionShape: tomConstants = 8;
pub const tomSelfIME: tomConstants = 262144;
pub const tomSentence: tomConstants = 3;
pub const tomSentenceCase: tomConstants = 4;
pub const tomShadow: tomConstants = -2147482624;
pub const tomShareDenyRead: tomConstants = 512;
pub const tomShareDenyWrite: tomConstants = 1024;
pub const tomShiftJIS: tomConstants = 12;
pub const tomShimmer: tomConstants = 6;
pub const tomShowDegPlaceHldr: tomConstants = 8;
pub const tomShowLLimPlaceHldr: tomConstants = 8;
pub const tomShowMatPlaceHldr: tomConstants = 8;
pub const tomShowULimPlaceHldr: tomConstants = 16;
pub const tomSimpleText: OBJECTTYPE = 0;
pub const tomSingle: tomConstants = 1;
pub const tomSinhala: tomConstants = 31;
pub const tomSizeScript: tomConstants = 64;
pub const tomSizeScriptScript: tomConstants = 96;
pub const tomSizeText: tomConstants = 32;
pub const tomSlashedFraction: OBJECTTYPE = 26;
pub const tomSmallCaps: tomConstants = -2147483584;
pub const tomSpaceBinary: tomConstants = 8;
pub const tomSpaceDefault: tomConstants = 0;
pub const tomSpaceDifferential: tomConstants = 24;
pub const tomSpaceMask: tomConstants = 28;
pub const tomSpaceOrd: tomConstants = 20;
pub const tomSpaceRelational: tomConstants = 12;
pub const tomSpaceSkip: tomConstants = 16;
pub const tomSpaceUnary: tomConstants = 4;
pub const tomSpaces: tomConstants = 0;
pub const tomSparkleText: tomConstants = 3;
pub const tomStack: OBJECTTYPE = 27;
pub const tomStart: tomConstants = 32;
pub const tomStory: tomConstants = 6;
pub const tomStoryActiveDisplay: tomConstants = 1;
pub const tomStoryActiveDisplayUI: tomConstants = 3;
pub const tomStoryActiveUI: tomConstants = 2;
pub const tomStoryInactive: tomConstants = 0;
pub const tomStretchBaseAbove: tomConstants = 3;
pub const tomStretchBaseBelow: tomConstants = 2;
pub const tomStretchCharAbove: tomConstants = 1;
pub const tomStretchCharBelow: tomConstants = 0;
pub const tomStretchStack: OBJECTTYPE = 28;
pub const tomStrikeout: tomConstants = -2147483640;
pub const tomStyleDefault: tomConstants = 0;
pub const tomStyleDisplay: tomConstants = 8;
pub const tomStyleDisplayCramped: tomConstants = 7;
pub const tomStyleScript: tomConstants = 4;
pub const tomStyleScriptCramped: tomConstants = 3;
pub const tomStyleScriptScript: tomConstants = 2;
pub const tomStyleScriptScriptCramped: tomConstants = 1;
pub const tomStyleText: tomConstants = 6;
pub const tomStyleTextCramped: tomConstants = 5;
pub const tomSubSup: OBJECTTYPE = 30;
pub const tomSubSupAlign: tomConstants = 1;
pub const tomSubscript: OBJECTTYPE = 29;
pub const tomSubscriptCF: tomConstants = -2147418112;
pub const tomSuperscript: OBJECTTYPE = 31;
pub const tomSuperscriptCF: tomConstants = -2147352576;
pub const tomSuspend: tomConstants = -9999995;
pub const tomSylotiNagri: tomConstants = 49;
pub const tomSymbol: tomConstants = 10;
pub const tomSyriac: tomConstants = 20;
pub const tomTabBack: tomConstants = -3;
pub const tomTabHere: tomConstants = -1;
pub const tomTabNext: tomConstants = -2;
pub const tomTable: tomConstants = 15;
pub const tomTableColumn: tomConstants = 9;
pub const tomTaiLe: tomConstants = 47;
pub const tomTamil: tomConstants = 27;
pub const tomTelugu: tomConstants = 28;
pub const tomText: tomConstants = 2;
pub const tomTextFlowES: tomConstants = 0;
pub const tomTextFlowMask: tomConstants = 12;
pub const tomTextFlowNE: tomConstants = 12;
pub const tomTextFlowSW: tomConstants = 4;
pub const tomTextFlowWN: tomConstants = 8;
pub const tomTextFrameStory: tomConstants = 5;
pub const tomTextize: tomConstants = 4;
pub const tomThaana: tomConstants = 21;
pub const tomThai: tomConstants = 11;
pub const tomThick: tomConstants = 9;
pub const tomThickDash: tomConstants = 14;
pub const tomThickDashDot: tomConstants = 15;
pub const tomThickDashDotDot: tomConstants = 16;
pub const tomThickDotted: tomConstants = 17;
pub const tomThickLines: tomConstants = 4;
pub const tomThickLongDash: tomConstants = 18;
pub const tomTibetan: tomConstants = 33;
pub const tomTifinagh: tomConstants = 62;
pub const tomTitleCase: tomConstants = 2;
pub const tomToggle: tomConstants = -9999998;
pub const tomToggleCase: tomConstants = 5;
pub const tomTrackParms: tomConstants = 2;
pub const tomTransform: tomConstants = 1024;
pub const tomTranslateTableCell: tomConstants = 128;
pub const tomTransparentForPositioning: tomConstants = 256;
pub const tomTransparentForSpacing: tomConstants = 512;
pub const tomTrue: tomConstants = -1;
pub const tomTruncateExisting: tomConstants = 80;
pub const tomTurkish: tomConstants = 4;
pub const tomUndefined: tomConstants = -9999999;
pub const tomUnderbar: OBJECTTYPE = 32;
pub const tomUnderline: tomConstants = -2147483644;
pub const tomUnderlinePositionAbove: tomConstants = 2;
pub const tomUnderlinePositionAuto: tomConstants = 0;
pub const tomUnderlinePositionBelow: tomConstants = 1;
pub const tomUnderlinePositionMax: tomConstants = 2;
pub const tomUndoLimit: tomConstants = 140;
pub const tomUnhide: tomConstants = 16;
pub const tomUnicodeBiDi: tomConstants = 1;
pub const tomUnknownStory: tomConstants = 0;
pub const tomUnlink: tomConstants = 8;
pub const tomUpperCase: tomConstants = 1;
pub const tomUpperLimit: OBJECTTYPE = 33;
pub const tomUpperLimitAsSuperScript: tomConstants = 3;
pub const tomUseAtFont: tomConstants = 2;
pub const tomUseCRLF: tomConstants = 2;
pub const tomUsePoints: tomConstants = 10;
pub const tomUseTwips: tomConstants = 11;
pub const tomUsymbol: tomConstants = 52;
pub const tomVLowCell: tomConstants = 2;
pub const tomVTopCell: tomConstants = 1;
pub const tomVai: tomConstants = 56;
pub const tomVietnamese: tomConstants = 8;
pub const tomWarichu: OBJECTTYPE = 3;
pub const tomWave: tomConstants = 8;
pub const tomWindow: tomConstants = 11;
pub const tomWipeDown: tomConstants = 7;
pub const tomWipeRight: tomConstants = 8;
pub const tomWord: tomConstants = 2;
pub const tomWordDocument: tomConstants = 4;
pub const tomWords: tomConstants = 2;
pub const tomYi: tomConstants = 45;
