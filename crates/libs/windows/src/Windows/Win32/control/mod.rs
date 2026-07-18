pub const CLSID_FilgraphManager: windows_core::GUID = windows_core::GUID::from_u128(0xe436ebb3_524f_11ce_9f53_0020af0ba770);
pub const FilgraphManager: windows_core::GUID = windows_core::GUID::from_u128(0xe436ebb3_524f_11ce_9f53_0020af0ba770);
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAMCollection, IAMCollection_Vtbl, 0x56a868b9_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAMCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAMCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IAMCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Item(&self, litem: i32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), litem, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAMCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAMCollection_Impl: super::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, litem: i32) -> windows_core::Result<windows_core::IUnknown>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAMCollection_Vtbl {
    pub const fn new<Identity: IAMCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IAMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAMCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IAMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, litem: i32, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAMCollection_Impl::Item(this, core::mem::transmute_copy(&litem)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAMCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAMCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAMCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAMStats, IAMStats_Vtbl, 0xbc9bcf80_dcd2_11d2_abf6_00a0c905f375);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAMStats {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAMStats, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IAMStats {
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetValueByIndex(&self, lindex: i32, szname: *mut windows_core::BSTR, lcount: *mut i32, dlast: *mut f64, daverage: *mut f64, dstddev: *mut f64, dmin: *mut f64, dmax: *mut f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValueByIndex)(windows_core::Interface::as_raw(self), lindex, core::mem::transmute(szname), lcount as _, dlast as _, daverage as _, dstddev as _, dmin as _, dmax as _) }
    }
    pub unsafe fn GetValueByName(&self, szname: &windows_core::BSTR, lindex: *mut i32, lcount: *mut i32, dlast: *mut f64, daverage: *mut f64, dstddev: *mut f64, dmin: *mut f64, dmax: *mut f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValueByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(szname), lindex as _, lcount as _, dlast as _, daverage as _, dstddev as _, dmin as _, dmax as _) }
    }
    pub unsafe fn GetIndex(&self, szname: &windows_core::BSTR, lcreate: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(szname), lcreate, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddValue(&self, lindex: i32, dvalue: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddValue)(windows_core::Interface::as_raw(self), lindex, dvalue) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAMStats_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetValueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut i32, *mut f64, *mut f64, *mut f64, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub GetValueByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut i32, *mut f64, *mut f64, *mut f64, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub AddValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, f64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAMStats_Impl: super::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn GetValueByIndex(&self, lindex: i32, szname: *mut windows_core::BSTR, lcount: *mut i32, dlast: *mut f64, daverage: *mut f64, dstddev: *mut f64, dmin: *mut f64, dmax: *mut f64) -> windows_core::Result<()>;
    fn GetValueByName(&self, szname: &windows_core::BSTR, lindex: *mut i32, lcount: *mut i32, dlast: *mut f64, daverage: *mut f64, dstddev: *mut f64, dmin: *mut f64, dmax: *mut f64) -> windows_core::Result<()>;
    fn GetIndex(&self, szname: &windows_core::BSTR, lcreate: i32) -> windows_core::Result<i32>;
    fn AddValue(&self, lindex: i32, dvalue: f64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAMStats_Vtbl {
    pub const fn new<Identity: IAMStats_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IAMStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAMStats_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Count<Identity: IAMStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAMStats_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValueByIndex<Identity: IAMStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, szname: *mut *mut core::ffi::c_void, lcount: *mut i32, dlast: *mut f64, daverage: *mut f64, dstddev: *mut f64, dmin: *mut f64, dmax: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAMStats_Impl::GetValueByIndex(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&lcount), core::mem::transmute_copy(&dlast), core::mem::transmute_copy(&daverage), core::mem::transmute_copy(&dstddev), core::mem::transmute_copy(&dmin), core::mem::transmute_copy(&dmax)).into()
            }
        }
        unsafe extern "system" fn GetValueByName<Identity: IAMStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: *mut core::ffi::c_void, lindex: *mut i32, lcount: *mut i32, dlast: *mut f64, daverage: *mut f64, dstddev: *mut f64, dmin: *mut f64, dmax: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAMStats_Impl::GetValueByName(this, core::mem::transmute(&szname), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&lcount), core::mem::transmute_copy(&dlast), core::mem::transmute_copy(&daverage), core::mem::transmute_copy(&dstddev), core::mem::transmute_copy(&dmin), core::mem::transmute_copy(&dmax)).into()
            }
        }
        unsafe extern "system" fn GetIndex<Identity: IAMStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: *mut core::ffi::c_void, lcreate: i32, plindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAMStats_Impl::GetIndex(this, core::mem::transmute(&szname), core::mem::transmute_copy(&lcreate)) {
                    Ok(ok__) => {
                        plindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddValue<Identity: IAMStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, dvalue: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAMStats_Impl::AddValue(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&dvalue)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            GetValueByIndex: GetValueByIndex::<Identity, OFFSET>,
            GetValueByName: GetValueByName::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            AddValue: AddValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAMStats as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAMStats {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IBasicAudio, IBasicAudio_Vtbl, 0x56a868b3_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IBasicAudio {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IBasicAudio, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IBasicAudio {
    pub unsafe fn SetVolume(&self, lvolume: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), lvolume) }
    }
    pub unsafe fn Volume(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Volume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBalance(&self, lbalance: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBalance)(windows_core::Interface::as_raw(self), lbalance) }
    }
    pub unsafe fn Balance(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Balance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBasicAudio_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBalance: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Balance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBasicAudio_Impl: super::IDispatch_Impl {
    fn SetVolume(&self, lvolume: i32) -> windows_core::Result<()>;
    fn Volume(&self) -> windows_core::Result<i32>;
    fn SetBalance(&self, lbalance: i32) -> windows_core::Result<()>;
    fn Balance(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBasicAudio_Vtbl {
    pub const fn new<Identity: IBasicAudio_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetVolume<Identity: IBasicAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicAudio_Impl::SetVolume(this, core::mem::transmute_copy(&lvolume)).into()
            }
        }
        unsafe extern "system" fn Volume<Identity: IBasicAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicAudio_Impl::Volume(this) {
                    Ok(ok__) => {
                        plvolume.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBalance<Identity: IBasicAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbalance: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicAudio_Impl::SetBalance(this, core::mem::transmute_copy(&lbalance)).into()
            }
        }
        unsafe extern "system" fn Balance<Identity: IBasicAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbalance: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicAudio_Impl::Balance(this) {
                    Ok(ok__) => {
                        plbalance.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetVolume: SetVolume::<Identity, OFFSET>,
            Volume: Volume::<Identity, OFFSET>,
            SetBalance: SetBalance::<Identity, OFFSET>,
            Balance: Balance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBasicAudio as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBasicAudio {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IBasicVideo, IBasicVideo_Vtbl, 0x56a868b5_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IBasicVideo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IBasicVideo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IBasicVideo {
    pub unsafe fn AvgTimePerFrame(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AvgTimePerFrame)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BitRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BitRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BitErrorRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BitErrorRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VideoWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VideoHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSourceLeft(&self, sourceleft: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceLeft)(windows_core::Interface::as_raw(self), sourceleft) }
    }
    pub unsafe fn SourceLeft(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSourceWidth(&self, sourcewidth: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceWidth)(windows_core::Interface::as_raw(self), sourcewidth) }
    }
    pub unsafe fn SourceWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSourceTop(&self, sourcetop: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceTop)(windows_core::Interface::as_raw(self), sourcetop) }
    }
    pub unsafe fn SourceTop(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSourceHeight(&self, sourceheight: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceHeight)(windows_core::Interface::as_raw(self), sourceheight) }
    }
    pub unsafe fn SourceHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDestinationLeft(&self, destinationleft: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationLeft)(windows_core::Interface::as_raw(self), destinationleft) }
    }
    pub unsafe fn DestinationLeft(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDestinationWidth(&self, destinationwidth: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationWidth)(windows_core::Interface::as_raw(self), destinationwidth) }
    }
    pub unsafe fn DestinationWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDestinationTop(&self, destinationtop: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationTop)(windows_core::Interface::as_raw(self), destinationtop) }
    }
    pub unsafe fn DestinationTop(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDestinationHeight(&self, destinationheight: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationHeight)(windows_core::Interface::as_raw(self), destinationheight) }
    }
    pub unsafe fn DestinationHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSourcePosition(&self, left: i32, top: i32, width: i32, height: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourcePosition)(windows_core::Interface::as_raw(self), left, top, width, height) }
    }
    pub unsafe fn GetSourcePosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSourcePosition)(windows_core::Interface::as_raw(self), pleft as _, ptop as _, pwidth as _, pheight as _) }
    }
    pub unsafe fn SetDefaultSourcePosition(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultSourcePosition)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetDestinationPosition(&self, left: i32, top: i32, width: i32, height: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationPosition)(windows_core::Interface::as_raw(self), left, top, width, height) }
    }
    pub unsafe fn GetDestinationPosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDestinationPosition)(windows_core::Interface::as_raw(self), pleft as _, ptop as _, pwidth as _, pheight as _) }
    }
    pub unsafe fn SetDefaultDestinationPosition(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultDestinationPosition)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetVideoSize(&self, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _) }
    }
    pub unsafe fn GetVideoPaletteEntries(&self, startindex: i32, entries: i32, pretrieved: *mut i32, ppalette: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoPaletteEntries)(windows_core::Interface::as_raw(self), startindex, entries, pretrieved as _, ppalette as _) }
    }
    pub unsafe fn GetCurrentImage(&self, pbuffersize: *mut i32, pdibimage: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentImage)(windows_core::Interface::as_raw(self), pbuffersize as _, pdibimage as _) }
    }
    pub unsafe fn IsUsingDefaultSource(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsUsingDefaultSource)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsUsingDefaultDestination(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsUsingDefaultDestination)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBasicVideo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub AvgTimePerFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub BitRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BitErrorRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VideoWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSourceLeft: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSourceWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSourceTop: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSourceHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDestinationLeft: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DestinationLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDestinationWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DestinationWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDestinationTop: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DestinationTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDestinationHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DestinationHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSourcePosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub GetSourcePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetDefaultSourcePosition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDestinationPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub GetDestinationPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetDefaultDestinationPosition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVideoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetVideoPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetCurrentImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub IsUsingDefaultSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsUsingDefaultDestination: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBasicVideo_Impl: super::IDispatch_Impl {
    fn AvgTimePerFrame(&self) -> windows_core::Result<f64>;
    fn BitRate(&self) -> windows_core::Result<i32>;
    fn BitErrorRate(&self) -> windows_core::Result<i32>;
    fn VideoWidth(&self) -> windows_core::Result<i32>;
    fn VideoHeight(&self) -> windows_core::Result<i32>;
    fn SetSourceLeft(&self, sourceleft: i32) -> windows_core::Result<()>;
    fn SourceLeft(&self) -> windows_core::Result<i32>;
    fn SetSourceWidth(&self, sourcewidth: i32) -> windows_core::Result<()>;
    fn SourceWidth(&self) -> windows_core::Result<i32>;
    fn SetSourceTop(&self, sourcetop: i32) -> windows_core::Result<()>;
    fn SourceTop(&self) -> windows_core::Result<i32>;
    fn SetSourceHeight(&self, sourceheight: i32) -> windows_core::Result<()>;
    fn SourceHeight(&self) -> windows_core::Result<i32>;
    fn SetDestinationLeft(&self, destinationleft: i32) -> windows_core::Result<()>;
    fn DestinationLeft(&self) -> windows_core::Result<i32>;
    fn SetDestinationWidth(&self, destinationwidth: i32) -> windows_core::Result<()>;
    fn DestinationWidth(&self) -> windows_core::Result<i32>;
    fn SetDestinationTop(&self, destinationtop: i32) -> windows_core::Result<()>;
    fn DestinationTop(&self) -> windows_core::Result<i32>;
    fn SetDestinationHeight(&self, destinationheight: i32) -> windows_core::Result<()>;
    fn DestinationHeight(&self) -> windows_core::Result<i32>;
    fn SetSourcePosition(&self, left: i32, top: i32, width: i32, height: i32) -> windows_core::Result<()>;
    fn GetSourcePosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn SetDefaultSourcePosition(&self) -> windows_core::Result<()>;
    fn SetDestinationPosition(&self, left: i32, top: i32, width: i32, height: i32) -> windows_core::Result<()>;
    fn GetDestinationPosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn SetDefaultDestinationPosition(&self) -> windows_core::Result<()>;
    fn GetVideoSize(&self, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn GetVideoPaletteEntries(&self, startindex: i32, entries: i32, pretrieved: *mut i32, ppalette: *mut i32) -> windows_core::Result<()>;
    fn GetCurrentImage(&self, pbuffersize: *mut i32, pdibimage: *mut i32) -> windows_core::Result<()>;
    fn IsUsingDefaultSource(&self) -> windows_core::Result<()>;
    fn IsUsingDefaultDestination(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBasicVideo_Vtbl {
    pub const fn new<Identity: IBasicVideo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AvgTimePerFrame<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pavgtimeperframe: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::AvgTimePerFrame(this) {
                    Ok(ok__) => {
                        pavgtimeperframe.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BitRate<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitrate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::BitRate(this) {
                    Ok(ok__) => {
                        pbitrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BitErrorRate<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbiterrorrate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::BitErrorRate(this) {
                    Ok(ok__) => {
                        pbiterrorrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VideoWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideowidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::VideoWidth(this) {
                    Ok(ok__) => {
                        pvideowidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VideoHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::VideoHeight(this) {
                    Ok(ok__) => {
                        pvideoheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSourceLeft<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceleft: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetSourceLeft(this, core::mem::transmute_copy(&sourceleft)).into()
            }
        }
        unsafe extern "system" fn SourceLeft<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourceleft: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::SourceLeft(this) {
                    Ok(ok__) => {
                        psourceleft.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSourceWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcewidth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetSourceWidth(this, core::mem::transmute_copy(&sourcewidth)).into()
            }
        }
        unsafe extern "system" fn SourceWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcewidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::SourceWidth(this) {
                    Ok(ok__) => {
                        psourcewidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSourceTop<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcetop: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetSourceTop(this, core::mem::transmute_copy(&sourcetop)).into()
            }
        }
        unsafe extern "system" fn SourceTop<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::SourceTop(this) {
                    Ok(ok__) => {
                        psourcetop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSourceHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceheight: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetSourceHeight(this, core::mem::transmute_copy(&sourceheight)).into()
            }
        }
        unsafe extern "system" fn SourceHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourceheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::SourceHeight(this) {
                    Ok(ok__) => {
                        psourceheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDestinationLeft<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationleft: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetDestinationLeft(this, core::mem::transmute_copy(&destinationleft)).into()
            }
        }
        unsafe extern "system" fn DestinationLeft<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationleft: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::DestinationLeft(this) {
                    Ok(ok__) => {
                        pdestinationleft.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDestinationWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationwidth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetDestinationWidth(this, core::mem::transmute_copy(&destinationwidth)).into()
            }
        }
        unsafe extern "system" fn DestinationWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::DestinationWidth(this) {
                    Ok(ok__) => {
                        pdestinationwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDestinationTop<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationtop: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetDestinationTop(this, core::mem::transmute_copy(&destinationtop)).into()
            }
        }
        unsafe extern "system" fn DestinationTop<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationtop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::DestinationTop(this) {
                    Ok(ok__) => {
                        pdestinationtop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDestinationHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationheight: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetDestinationHeight(this, core::mem::transmute_copy(&destinationheight)).into()
            }
        }
        unsafe extern "system" fn DestinationHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::DestinationHeight(this) {
                    Ok(ok__) => {
                        pdestinationheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSourcePosition<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32, top: i32, width: i32, height: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetSourcePosition(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn GetSourcePosition<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::GetSourcePosition(this, core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn SetDefaultSourcePosition<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetDefaultSourcePosition(this).into()
            }
        }
        unsafe extern "system" fn SetDestinationPosition<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32, top: i32, width: i32, height: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetDestinationPosition(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn GetDestinationPosition<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::GetDestinationPosition(this, core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn SetDefaultDestinationPosition<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::SetDefaultDestinationPosition(this).into()
            }
        }
        unsafe extern "system" fn GetVideoSize<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::GetVideoSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn GetVideoPaletteEntries<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: i32, entries: i32, pretrieved: *mut i32, ppalette: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::GetVideoPaletteEntries(this, core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&entries), core::mem::transmute_copy(&pretrieved), core::mem::transmute_copy(&ppalette)).into()
            }
        }
        unsafe extern "system" fn GetCurrentImage<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffersize: *mut i32, pdibimage: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::GetCurrentImage(this, core::mem::transmute_copy(&pbuffersize), core::mem::transmute_copy(&pdibimage)).into()
            }
        }
        unsafe extern "system" fn IsUsingDefaultSource<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::IsUsingDefaultSource(this).into()
            }
        }
        unsafe extern "system" fn IsUsingDefaultDestination<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::IsUsingDefaultDestination(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AvgTimePerFrame: AvgTimePerFrame::<Identity, OFFSET>,
            BitRate: BitRate::<Identity, OFFSET>,
            BitErrorRate: BitErrorRate::<Identity, OFFSET>,
            VideoWidth: VideoWidth::<Identity, OFFSET>,
            VideoHeight: VideoHeight::<Identity, OFFSET>,
            SetSourceLeft: SetSourceLeft::<Identity, OFFSET>,
            SourceLeft: SourceLeft::<Identity, OFFSET>,
            SetSourceWidth: SetSourceWidth::<Identity, OFFSET>,
            SourceWidth: SourceWidth::<Identity, OFFSET>,
            SetSourceTop: SetSourceTop::<Identity, OFFSET>,
            SourceTop: SourceTop::<Identity, OFFSET>,
            SetSourceHeight: SetSourceHeight::<Identity, OFFSET>,
            SourceHeight: SourceHeight::<Identity, OFFSET>,
            SetDestinationLeft: SetDestinationLeft::<Identity, OFFSET>,
            DestinationLeft: DestinationLeft::<Identity, OFFSET>,
            SetDestinationWidth: SetDestinationWidth::<Identity, OFFSET>,
            DestinationWidth: DestinationWidth::<Identity, OFFSET>,
            SetDestinationTop: SetDestinationTop::<Identity, OFFSET>,
            DestinationTop: DestinationTop::<Identity, OFFSET>,
            SetDestinationHeight: SetDestinationHeight::<Identity, OFFSET>,
            DestinationHeight: DestinationHeight::<Identity, OFFSET>,
            SetSourcePosition: SetSourcePosition::<Identity, OFFSET>,
            GetSourcePosition: GetSourcePosition::<Identity, OFFSET>,
            SetDefaultSourcePosition: SetDefaultSourcePosition::<Identity, OFFSET>,
            SetDestinationPosition: SetDestinationPosition::<Identity, OFFSET>,
            GetDestinationPosition: GetDestinationPosition::<Identity, OFFSET>,
            SetDefaultDestinationPosition: SetDefaultDestinationPosition::<Identity, OFFSET>,
            GetVideoSize: GetVideoSize::<Identity, OFFSET>,
            GetVideoPaletteEntries: GetVideoPaletteEntries::<Identity, OFFSET>,
            GetCurrentImage: GetCurrentImage::<Identity, OFFSET>,
            IsUsingDefaultSource: IsUsingDefaultSource::<Identity, OFFSET>,
            IsUsingDefaultDestination: IsUsingDefaultDestination::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBasicVideo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBasicVideo {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IBasicVideo2, IBasicVideo2_Vtbl, 0x329bb360_f6ea_11d1_9038_00a0c9697298);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IBasicVideo2 {
    type Target = IBasicVideo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IBasicVideo2, windows_core::IUnknown, super::IDispatch, IBasicVideo);
#[cfg(feature = "oaidl")]
impl IBasicVideo2 {
    pub unsafe fn GetPreferredAspectRatio(&self, plaspectx: *mut i32, plaspecty: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredAspectRatio)(windows_core::Interface::as_raw(self), plaspectx as _, plaspecty as _) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBasicVideo2_Vtbl {
    pub base__: IBasicVideo_Vtbl,
    pub GetPreferredAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBasicVideo2_Impl: IBasicVideo_Impl {
    fn GetPreferredAspectRatio(&self, plaspectx: *mut i32, plaspecty: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBasicVideo2_Vtbl {
    pub const fn new<Identity: IBasicVideo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPreferredAspectRatio<Identity: IBasicVideo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plaspectx: *mut i32, plaspecty: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo2_Impl::GetPreferredAspectRatio(this, core::mem::transmute_copy(&plaspectx), core::mem::transmute_copy(&plaspecty)).into()
            }
        }
        Self { base__: IBasicVideo_Vtbl::new::<Identity, OFFSET>(), GetPreferredAspectRatio: GetPreferredAspectRatio::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBasicVideo2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IBasicVideo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBasicVideo2 {}
windows_core::imp::define_interface!(IDeferredCommand, IDeferredCommand_Vtbl, 0x56a868b8_0ad4_11ce_b03a_0020af0ba770);
windows_core::imp::interface_hierarchy!(IDeferredCommand, windows_core::IUnknown);
impl IDeferredCommand {
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Confidence(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Confidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Postpone(&self, newtime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Postpone)(windows_core::Interface::as_raw(self), newtime) }
    }
    pub unsafe fn GetHResult(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferredCommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Confidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Postpone: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetHResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IDeferredCommand_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Confidence(&self) -> windows_core::Result<i32>;
    fn Postpone(&self, newtime: f64) -> windows_core::Result<()>;
    fn GetHResult(&self) -> windows_core::Result<windows_core::HRESULT>;
}
impl IDeferredCommand_Vtbl {
    pub const fn new<Identity: IDeferredCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IDeferredCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeferredCommand_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Confidence<Identity: IDeferredCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfidence: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeferredCommand_Impl::Confidence(this) {
                    Ok(ok__) => {
                        pconfidence.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Postpone<Identity: IDeferredCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newtime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeferredCommand_Impl::Postpone(this, core::mem::transmute_copy(&newtime)).into()
            }
        }
        unsafe extern "system" fn GetHResult<Identity: IDeferredCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeferredCommand_Impl::GetHResult(this) {
                    Ok(ok__) => {
                        phrresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Confidence: Confidence::<Identity, OFFSET>,
            Postpone: Postpone::<Identity, OFFSET>,
            GetHResult: GetHResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeferredCommand as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDeferredCommand {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFilterInfo, IFilterInfo_Vtbl, 0x56a868ba_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFilterInfo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFilterInfo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFilterInfo {
    pub unsafe fn FindPin(&self, strpinid: &windows_core::BSTR) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindPin)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpinid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VendorInfo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VendorInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Filter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Filter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Pins(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pins)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsFileSource(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFileSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Filename(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Filename)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFilename(&self, strfilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFilename)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfilename)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFilterInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub FindPin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VendorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pins: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFileSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Filename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFilename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFilterInfo_Impl: super::IDispatch_Impl {
    fn FindPin(&self, strpinid: &windows_core::BSTR) -> windows_core::Result<super::IDispatch>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VendorInfo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Filter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Pins(&self) -> windows_core::Result<super::IDispatch>;
    fn IsFileSource(&self) -> windows_core::Result<i32>;
    fn Filename(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFilename(&self, strfilename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFilterInfo_Vtbl {
    pub const fn new<Identity: IFilterInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindPin<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpinid: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::FindPin(this, core::mem::transmute(&strpinid)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VendorInfo<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvendorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::VendorInfo(this) {
                    Ok(ok__) => {
                        strvendorinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Filter<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::Filter(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pins<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::Pins(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFileSource<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbissource: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::IsFileSource(this) {
                    Ok(ok__) => {
                        pbissource.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Filename<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::Filename(this) {
                    Ok(ok__) => {
                        pstrfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilename<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilterInfo_Impl::SetFilename(this, core::mem::transmute(&strfilename)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FindPin: FindPin::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            VendorInfo: VendorInfo::<Identity, OFFSET>,
            Filter: Filter::<Identity, OFFSET>,
            Pins: Pins::<Identity, OFFSET>,
            IsFileSource: IsFileSource::<Identity, OFFSET>,
            Filename: Filename::<Identity, OFFSET>,
            SetFilename: SetFilename::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFilterInfo {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMediaControl, IMediaControl_Vtbl, 0x56a868b1_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMediaControl {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMediaControl, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IMediaControl {
    pub unsafe fn Run(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetState(&self, mstimeout: i32) -> windows_core::Result<OAFilterState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), mstimeout, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RenderFile(&self, strfilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RenderFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfilename)) }
    }
    pub unsafe fn AddSourceFilter(&self, strfilename: &windows_core::BSTR) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddSourceFilter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfilename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FilterCollection(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilterCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegFilterCollection(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegFilterCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn StopWhenReady(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopWhenReady)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut OAFilterState) -> windows_core::HRESULT,
    pub RenderFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSourceFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FilterCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegFilterCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopWhenReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaControl_Impl: super::IDispatch_Impl {
    fn Run(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn GetState(&self, mstimeout: i32) -> windows_core::Result<OAFilterState>;
    fn RenderFile(&self, strfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddSourceFilter(&self, strfilename: &windows_core::BSTR) -> windows_core::Result<super::IDispatch>;
    fn FilterCollection(&self) -> windows_core::Result<super::IDispatch>;
    fn RegFilterCollection(&self) -> windows_core::Result<super::IDispatch>;
    fn StopWhenReady(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMediaControl_Vtbl {
    pub const fn new<Identity: IMediaControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Run<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaControl_Impl::Run(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaControl_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaControl_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mstimeout: i32, pfs: *mut OAFilterState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaControl_Impl::GetState(this, core::mem::transmute_copy(&mstimeout)) {
                    Ok(ok__) => {
                        pfs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RenderFile<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaControl_Impl::RenderFile(this, core::mem::transmute(&strfilename)).into()
            }
        }
        unsafe extern "system" fn AddSourceFilter<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilename: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaControl_Impl::AddSourceFilter(this, core::mem::transmute(&strfilename)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FilterCollection<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaControl_Impl::FilterCollection(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegFilterCollection<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaControl_Impl::RegFilterCollection(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StopWhenReady<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaControl_Impl::StopWhenReady(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Run: Run::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            RenderFile: RenderFile::<Identity, OFFSET>,
            AddSourceFilter: AddSourceFilter::<Identity, OFFSET>,
            FilterCollection: FilterCollection::<Identity, OFFSET>,
            RegFilterCollection: RegFilterCollection::<Identity, OFFSET>,
            StopWhenReady: StopWhenReady::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaControl as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMediaControl {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMediaEvent, IMediaEvent_Vtbl, 0x56a868b6_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMediaEvent {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMediaEvent, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IMediaEvent {
    pub unsafe fn GetEventHandle(&self) -> windows_core::Result<OAEVENT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEvent(&self, leventcode: *mut i32, lparam1: *mut isize, lparam2: *mut isize, mstimeout: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEvent)(windows_core::Interface::as_raw(self), leventcode as _, lparam1 as _, lparam2 as _, mstimeout) }
    }
    pub unsafe fn WaitForCompletion(&self, mstimeout: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), mstimeout, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CancelDefaultHandling(&self, levcode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelDefaultHandling)(windows_core::Interface::as_raw(self), levcode) }
    }
    pub unsafe fn RestoreDefaultHandling(&self, levcode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreDefaultHandling)(windows_core::Interface::as_raw(self), levcode) }
    }
    pub unsafe fn FreeEventParams(&self, levcode: i32, lparam1: isize, lparam2: isize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeEventParams)(windows_core::Interface::as_raw(self), levcode, lparam1, lparam2) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEvent_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub GetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OAEVENT) -> windows_core::HRESULT,
    pub GetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut isize, *mut isize, i32) -> windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub CancelDefaultHandling: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RestoreDefaultHandling: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FreeEventParams: unsafe extern "system" fn(*mut core::ffi::c_void, i32, isize, isize) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaEvent_Impl: super::IDispatch_Impl {
    fn GetEventHandle(&self) -> windows_core::Result<OAEVENT>;
    fn GetEvent(&self, leventcode: *mut i32, lparam1: *mut isize, lparam2: *mut isize, mstimeout: i32) -> windows_core::Result<()>;
    fn WaitForCompletion(&self, mstimeout: i32) -> windows_core::Result<i32>;
    fn CancelDefaultHandling(&self, levcode: i32) -> windows_core::Result<()>;
    fn RestoreDefaultHandling(&self, levcode: i32) -> windows_core::Result<()>;
    fn FreeEventParams(&self, levcode: i32, lparam1: isize, lparam2: isize) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMediaEvent_Vtbl {
    pub const fn new<Identity: IMediaEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEventHandle<Identity: IMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: *mut OAEVENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaEvent_Impl::GetEventHandle(this) {
                    Ok(ok__) => {
                        hevent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEvent<Identity: IMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, leventcode: *mut i32, lparam1: *mut isize, lparam2: *mut isize, mstimeout: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaEvent_Impl::GetEvent(this, core::mem::transmute_copy(&leventcode), core::mem::transmute_copy(&lparam1), core::mem::transmute_copy(&lparam2), core::mem::transmute_copy(&mstimeout)).into()
            }
        }
        unsafe extern "system" fn WaitForCompletion<Identity: IMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mstimeout: i32, pevcode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaEvent_Impl::WaitForCompletion(this, core::mem::transmute_copy(&mstimeout)) {
                    Ok(ok__) => {
                        pevcode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CancelDefaultHandling<Identity: IMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, levcode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaEvent_Impl::CancelDefaultHandling(this, core::mem::transmute_copy(&levcode)).into()
            }
        }
        unsafe extern "system" fn RestoreDefaultHandling<Identity: IMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, levcode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaEvent_Impl::RestoreDefaultHandling(this, core::mem::transmute_copy(&levcode)).into()
            }
        }
        unsafe extern "system" fn FreeEventParams<Identity: IMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, levcode: i32, lparam1: isize, lparam2: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaEvent_Impl::FreeEventParams(this, core::mem::transmute_copy(&levcode), core::mem::transmute_copy(&lparam1), core::mem::transmute_copy(&lparam2)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetEventHandle: GetEventHandle::<Identity, OFFSET>,
            GetEvent: GetEvent::<Identity, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, OFFSET>,
            CancelDefaultHandling: CancelDefaultHandling::<Identity, OFFSET>,
            RestoreDefaultHandling: RestoreDefaultHandling::<Identity, OFFSET>,
            FreeEventParams: FreeEventParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaEvent as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMediaEvent {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMediaEventEx, IMediaEventEx_Vtbl, 0x56a868c0_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMediaEventEx {
    type Target = IMediaEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMediaEventEx, windows_core::IUnknown, super::IDispatch, IMediaEvent);
#[cfg(feature = "oaidl")]
impl IMediaEventEx {
    pub unsafe fn SetNotifyWindow(&self, hwnd: OAHWND, lmsg: i32, linstancedata: isize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotifyWindow)(windows_core::Interface::as_raw(self), hwnd, lmsg, linstancedata) }
    }
    pub unsafe fn SetNotifyFlags(&self, lnonotifyflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotifyFlags)(windows_core::Interface::as_raw(self), lnonotifyflags) }
    }
    pub unsafe fn GetNotifyFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNotifyFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEventEx_Vtbl {
    pub base__: IMediaEvent_Vtbl,
    pub SetNotifyWindow: unsafe extern "system" fn(*mut core::ffi::c_void, OAHWND, i32, isize) -> windows_core::HRESULT,
    pub SetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaEventEx_Impl: IMediaEvent_Impl {
    fn SetNotifyWindow(&self, hwnd: OAHWND, lmsg: i32, linstancedata: isize) -> windows_core::Result<()>;
    fn SetNotifyFlags(&self, lnonotifyflags: i32) -> windows_core::Result<()>;
    fn GetNotifyFlags(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMediaEventEx_Vtbl {
    pub const fn new<Identity: IMediaEventEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNotifyWindow<Identity: IMediaEventEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: OAHWND, lmsg: i32, linstancedata: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaEventEx_Impl::SetNotifyWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&lmsg), core::mem::transmute_copy(&linstancedata)).into()
            }
        }
        unsafe extern "system" fn SetNotifyFlags<Identity: IMediaEventEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnonotifyflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaEventEx_Impl::SetNotifyFlags(this, core::mem::transmute_copy(&lnonotifyflags)).into()
            }
        }
        unsafe extern "system" fn GetNotifyFlags<Identity: IMediaEventEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lplnonotifyflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaEventEx_Impl::GetNotifyFlags(this) {
                    Ok(ok__) => {
                        lplnonotifyflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMediaEvent_Vtbl::new::<Identity, OFFSET>(),
            SetNotifyWindow: SetNotifyWindow::<Identity, OFFSET>,
            SetNotifyFlags: SetNotifyFlags::<Identity, OFFSET>,
            GetNotifyFlags: GetNotifyFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaEventEx as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IMediaEvent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMediaEventEx {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMediaPosition, IMediaPosition_Vtbl, 0x56a868b2_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMediaPosition {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMediaPosition, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IMediaPosition {
    pub unsafe fn Duration(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCurrentPosition(&self, lltime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPosition)(windows_core::Interface::as_raw(self), lltime) }
    }
    pub unsafe fn CurrentPosition(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StopTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StopTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStopTime(&self, lltime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStopTime)(windows_core::Interface::as_raw(self), lltime) }
    }
    pub unsafe fn PrerollTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrerollTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrerollTime(&self, lltime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrerollTime)(windows_core::Interface::as_raw(self), lltime) }
    }
    pub unsafe fn SetRate(&self, drate: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRate)(windows_core::Interface::as_raw(self), drate) }
    }
    pub unsafe fn Rate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanSeekForward(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanSeekForward)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanSeekBackward(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanSeekBackward)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPosition_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub StopTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub PrerollTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetPrerollTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Rate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CanSeekForward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CanSeekBackward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaPosition_Impl: super::IDispatch_Impl {
    fn Duration(&self) -> windows_core::Result<f64>;
    fn SetCurrentPosition(&self, lltime: f64) -> windows_core::Result<()>;
    fn CurrentPosition(&self) -> windows_core::Result<f64>;
    fn StopTime(&self) -> windows_core::Result<f64>;
    fn SetStopTime(&self, lltime: f64) -> windows_core::Result<()>;
    fn PrerollTime(&self) -> windows_core::Result<f64>;
    fn SetPrerollTime(&self, lltime: f64) -> windows_core::Result<()>;
    fn SetRate(&self, drate: f64) -> windows_core::Result<()>;
    fn Rate(&self) -> windows_core::Result<f64>;
    fn CanSeekForward(&self) -> windows_core::Result<i32>;
    fn CanSeekBackward(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMediaPosition_Vtbl {
    pub const fn new<Identity: IMediaPosition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Duration<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plength: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::Duration(this) {
                    Ok(ok__) => {
                        plength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lltime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaPosition_Impl::SetCurrentPosition(this, core::mem::transmute_copy(&lltime)).into()
            }
        }
        unsafe extern "system" fn CurrentPosition<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plltime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::CurrentPosition(this) {
                    Ok(ok__) => {
                        plltime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StopTime<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plltime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::StopTime(this) {
                    Ok(ok__) => {
                        plltime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStopTime<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lltime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaPosition_Impl::SetStopTime(this, core::mem::transmute_copy(&lltime)).into()
            }
        }
        unsafe extern "system" fn PrerollTime<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plltime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::PrerollTime(this) {
                    Ok(ok__) => {
                        plltime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrerollTime<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lltime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaPosition_Impl::SetPrerollTime(this, core::mem::transmute_copy(&lltime)).into()
            }
        }
        unsafe extern "system" fn SetRate<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drate: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaPosition_Impl::SetRate(this, core::mem::transmute_copy(&drate)).into()
            }
        }
        unsafe extern "system" fn Rate<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdrate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::Rate(this) {
                    Ok(ok__) => {
                        pdrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanSeekForward<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcanseekforward: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::CanSeekForward(this) {
                    Ok(ok__) => {
                        pcanseekforward.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanSeekBackward<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcanseekbackward: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::CanSeekBackward(this) {
                    Ok(ok__) => {
                        pcanseekbackward.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Duration: Duration::<Identity, OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Identity, OFFSET>,
            CurrentPosition: CurrentPosition::<Identity, OFFSET>,
            StopTime: StopTime::<Identity, OFFSET>,
            SetStopTime: SetStopTime::<Identity, OFFSET>,
            PrerollTime: PrerollTime::<Identity, OFFSET>,
            SetPrerollTime: SetPrerollTime::<Identity, OFFSET>,
            SetRate: SetRate::<Identity, OFFSET>,
            Rate: Rate::<Identity, OFFSET>,
            CanSeekForward: CanSeekForward::<Identity, OFFSET>,
            CanSeekBackward: CanSeekBackward::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaPosition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMediaPosition {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMediaTypeInfo, IMediaTypeInfo_Vtbl, 0x56a868bc_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMediaTypeInfo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMediaTypeInfo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IMediaTypeInfo {
    pub unsafe fn Type(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Subtype(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subtype)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTypeInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Subtype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaTypeInfo_Impl: super::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Subtype(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMediaTypeInfo_Vtbl {
    pub const fn new<Identity: IMediaTypeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IMediaTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strtype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaTypeInfo_Impl::Type(this) {
                    Ok(ok__) => {
                        strtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Subtype<Identity: IMediaTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strtype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaTypeInfo_Impl::Subtype(this) {
                    Ok(ok__) => {
                        strtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Type: Type::<Identity, OFFSET>, Subtype: Subtype::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaTypeInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMediaTypeInfo {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IPinInfo, IPinInfo_Vtbl, 0x56a868bd_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IPinInfo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IPinInfo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IPinInfo {
    pub unsafe fn Pin(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pin)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ConnectedTo(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ConnectionMediaType(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectionMediaType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FilterInfo(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilterInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Direction(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PinID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PinID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MediaTypes(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaTypes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Connect<P0>(&self, ppin: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), ppin.param().abi()) }
    }
    pub unsafe fn ConnectDirect<P0>(&self, ppin: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectDirect)(windows_core::Interface::as_raw(self), ppin.param().abi()) }
    }
    pub unsafe fn ConnectWithType<P0, P1>(&self, ppin: P0, pmediatype: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectWithType)(windows_core::Interface::as_raw(self), ppin.param().abi(), pmediatype.param().abi()) }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Render(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPinInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Pin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FilterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PinID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectDirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectWithType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPinInfo_Impl: super::IDispatch_Impl {
    fn Pin(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ConnectedTo(&self) -> windows_core::Result<super::IDispatch>;
    fn ConnectionMediaType(&self) -> windows_core::Result<super::IDispatch>;
    fn FilterInfo(&self) -> windows_core::Result<super::IDispatch>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Direction(&self) -> windows_core::Result<i32>;
    fn PinID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MediaTypes(&self) -> windows_core::Result<super::IDispatch>;
    fn Connect(&self, ppin: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ConnectDirect(&self, ppin: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ConnectWithType(&self, ppin: windows_core::Ref<windows_core::IUnknown>, pmediatype: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Render(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPinInfo_Vtbl {
    pub const fn new<Identity: IPinInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pin<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::Pin(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectedTo<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::ConnectedTo(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectionMediaType<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::ConnectionMediaType(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FilterInfo<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::FilterInfo(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::Name(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Direction<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdirection: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::Direction(this) {
                    Ok(ok__) => {
                        ppdirection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PinID<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpinid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::PinID(this) {
                    Ok(ok__) => {
                        strpinid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MediaTypes<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::MediaTypes(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Connect<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPinInfo_Impl::Connect(this, core::mem::transmute_copy(&ppin)).into()
            }
        }
        unsafe extern "system" fn ConnectDirect<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPinInfo_Impl::ConnectDirect(this, core::mem::transmute_copy(&ppin)).into()
            }
        }
        unsafe extern "system" fn ConnectWithType<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppin: *mut core::ffi::c_void, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPinInfo_Impl::ConnectWithType(this, core::mem::transmute_copy(&ppin), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPinInfo_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn Render<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPinInfo_Impl::Render(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Pin: Pin::<Identity, OFFSET>,
            ConnectedTo: ConnectedTo::<Identity, OFFSET>,
            ConnectionMediaType: ConnectionMediaType::<Identity, OFFSET>,
            FilterInfo: FilterInfo::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Direction: Direction::<Identity, OFFSET>,
            PinID: PinID::<Identity, OFFSET>,
            MediaTypes: MediaTypes::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            ConnectDirect: ConnectDirect::<Identity, OFFSET>,
            ConnectWithType: ConnectWithType::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Render: Render::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPinInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPinInfo {}
windows_core::imp::define_interface!(IQueueCommand, IQueueCommand_Vtbl, 0x56a868b7_0ad4_11ce_b03a_0020af0ba770);
windows_core::imp::interface_hierarchy!(IQueueCommand, windows_core::IUnknown);
impl IQueueCommand {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InvokeAtStreamTime(&self, pcmd: *mut Option<IDeferredCommand>, time: f64, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvokeAtStreamTime)(windows_core::Interface::as_raw(self), core::mem::transmute(pcmd), time, iid, dispidmethod, wflags, cargs, pdispparams, pvarresult, puargerr as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InvokeAtPresentationTime(&self, pcmd: *mut Option<IDeferredCommand>, time: f64, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvokeAtPresentationTime)(windows_core::Interface::as_raw(self), core::mem::transmute(pcmd), time, iid, dispidmethod, wflags, cargs, pdispparams, pvarresult, puargerr as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueueCommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub InvokeAtStreamTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, f64, *const windows_core::GUID, i32, i16, i32, *const super::VARIANT, *mut super::VARIANT, *mut i16) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    InvokeAtStreamTime: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub InvokeAtPresentationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, f64, *const windows_core::GUID, i32, i16, i32, *const super::VARIANT, *mut super::VARIANT, *mut i16) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    InvokeAtPresentationTime: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IQueueCommand_Impl: windows_core::IUnknownImpl {
    fn InvokeAtStreamTime(&self, pcmd: windows_core::OutRef<IDeferredCommand>, time: f64, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::Result<()>;
    fn InvokeAtPresentationTime(&self, pcmd: windows_core::OutRef<IDeferredCommand>, time: f64, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IQueueCommand_Vtbl {
    pub const fn new<Identity: IQueueCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvokeAtStreamTime<Identity: IQueueCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmd: *mut *mut core::ffi::c_void, time: f64, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueueCommand_Impl::InvokeAtStreamTime(this, core::mem::transmute_copy(&pcmd), core::mem::transmute_copy(&time), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&dispidmethod), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&cargs), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        unsafe extern "system" fn InvokeAtPresentationTime<Identity: IQueueCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmd: *mut *mut core::ffi::c_void, time: f64, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueueCommand_Impl::InvokeAtPresentationTime(this, core::mem::transmute_copy(&pcmd), core::mem::transmute_copy(&time), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&dispidmethod), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&cargs), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InvokeAtStreamTime: InvokeAtStreamTime::<Identity, OFFSET>,
            InvokeAtPresentationTime: InvokeAtPresentationTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueueCommand as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IQueueCommand {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRegFilterInfo, IRegFilterInfo_Vtbl, 0x56a868bb_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRegFilterInfo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRegFilterInfo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IRegFilterInfo {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Filter(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Filter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegFilterInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRegFilterInfo_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Filter(&self) -> windows_core::Result<super::IDispatch>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRegFilterInfo_Vtbl {
    pub const fn new<Identity: IRegFilterInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IRegFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegFilterInfo_Impl::Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Filter<Identity: IRegFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegFilterInfo_Impl::Filter(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Name: Name::<Identity, OFFSET>, Filter: Filter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegFilterInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRegFilterInfo {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IVideoWindow, IVideoWindow_Vtbl, 0x56a868b4_0ad4_11ce_b03a_0020af0ba770);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IVideoWindow {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IVideoWindow, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IVideoWindow {
    pub unsafe fn SetCaption(&self, strcaption: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCaption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strcaption)) }
    }
    pub unsafe fn Caption(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Caption)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetWindowStyle(&self, windowstyle: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWindowStyle)(windows_core::Interface::as_raw(self), windowstyle) }
    }
    pub unsafe fn WindowStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWindowStyleEx(&self, windowstyleex: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWindowStyleEx)(windows_core::Interface::as_raw(self), windowstyleex) }
    }
    pub unsafe fn WindowStyleEx(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowStyleEx)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutoShow(&self, autoshow: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoShow)(windows_core::Interface::as_raw(self), autoshow) }
    }
    pub unsafe fn AutoShow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoShow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWindowState(&self, windowstate: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWindowState)(windows_core::Interface::as_raw(self), windowstate) }
    }
    pub unsafe fn WindowState(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBackgroundPalette(&self, backgroundpalette: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBackgroundPalette)(windows_core::Interface::as_raw(self), backgroundpalette) }
    }
    pub unsafe fn BackgroundPalette(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackgroundPalette)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetVisible(&self, visible: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVisible)(windows_core::Interface::as_raw(self), visible) }
    }
    pub unsafe fn Visible(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLeft(&self, left: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLeft)(windows_core::Interface::as_raw(self), left) }
    }
    pub unsafe fn Left(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Left)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWidth(&self, width: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWidth)(windows_core::Interface::as_raw(self), width) }
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTop(&self, top: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTop)(windows_core::Interface::as_raw(self), top) }
    }
    pub unsafe fn Top(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Top)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHeight(&self, height: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHeight)(windows_core::Interface::as_raw(self), height) }
    }
    pub unsafe fn Height(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOwner(&self, owner: OAHWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOwner)(windows_core::Interface::as_raw(self), owner) }
    }
    pub unsafe fn Owner(&self) -> windows_core::Result<OAHWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Owner)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMessageDrain(&self, drain: OAHWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageDrain)(windows_core::Interface::as_raw(self), drain) }
    }
    pub unsafe fn MessageDrain(&self) -> windows_core::Result<OAHWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageDrain)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BorderColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BorderColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBorderColor(&self, color: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBorderColor)(windows_core::Interface::as_raw(self), color) }
    }
    pub unsafe fn FullScreenMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FullScreenMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFullScreenMode(&self, fullscreenmode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFullScreenMode)(windows_core::Interface::as_raw(self), fullscreenmode) }
    }
    pub unsafe fn SetWindowForeground(&self, focus: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWindowForeground)(windows_core::Interface::as_raw(self), focus) }
    }
    pub unsafe fn NotifyOwnerMessage(&self, hwnd: OAHWND, umsg: i32, wparam: isize, lparam: isize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NotifyOwnerMessage)(windows_core::Interface::as_raw(self), hwnd, umsg, wparam, lparam) }
    }
    pub unsafe fn SetWindowPosition(&self, left: i32, top: i32, width: i32, height: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWindowPosition)(windows_core::Interface::as_raw(self), left, top, width, height) }
    }
    pub unsafe fn GetWindowPosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWindowPosition)(windows_core::Interface::as_raw(self), pleft as _, ptop as _, pwidth as _, pheight as _) }
    }
    pub unsafe fn GetMinIdealImageSize(&self, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMinIdealImageSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _) }
    }
    pub unsafe fn GetMaxIdealImageSize(&self, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMaxIdealImageSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _) }
    }
    pub unsafe fn GetRestorePosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRestorePosition)(windows_core::Interface::as_raw(self), pleft as _, ptop as _, pwidth as _, pheight as _) }
    }
    pub unsafe fn HideCursor(&self, hidecursor: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HideCursor)(windows_core::Interface::as_raw(self), hidecursor) }
    }
    pub unsafe fn IsCursorHidden(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCursorHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVideoWindow_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetCaption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Caption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWindowStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub WindowStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWindowStyleEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub WindowStyleEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutoShow: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoShow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWindowState: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub WindowState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBackgroundPalette: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BackgroundPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Left: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTop: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Top: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, OAHWND) -> windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OAHWND) -> windows_core::HRESULT,
    pub SetMessageDrain: unsafe extern "system" fn(*mut core::ffi::c_void, OAHWND) -> windows_core::HRESULT,
    pub MessageDrain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OAHWND) -> windows_core::HRESULT,
    pub BorderColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBorderColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FullScreenMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFullScreenMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetWindowForeground: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NotifyOwnerMessage: unsafe extern "system" fn(*mut core::ffi::c_void, OAHWND, i32, isize, isize) -> windows_core::HRESULT,
    pub SetWindowPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub GetWindowPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetMinIdealImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetMaxIdealImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetRestorePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub HideCursor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsCursorHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IVideoWindow_Impl: super::IDispatch_Impl {
    fn SetCaption(&self, strcaption: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Caption(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetWindowStyle(&self, windowstyle: i32) -> windows_core::Result<()>;
    fn WindowStyle(&self) -> windows_core::Result<i32>;
    fn SetWindowStyleEx(&self, windowstyleex: i32) -> windows_core::Result<()>;
    fn WindowStyleEx(&self) -> windows_core::Result<i32>;
    fn SetAutoShow(&self, autoshow: i32) -> windows_core::Result<()>;
    fn AutoShow(&self) -> windows_core::Result<i32>;
    fn SetWindowState(&self, windowstate: i32) -> windows_core::Result<()>;
    fn WindowState(&self) -> windows_core::Result<i32>;
    fn SetBackgroundPalette(&self, backgroundpalette: i32) -> windows_core::Result<()>;
    fn BackgroundPalette(&self) -> windows_core::Result<i32>;
    fn SetVisible(&self, visible: i32) -> windows_core::Result<()>;
    fn Visible(&self) -> windows_core::Result<i32>;
    fn SetLeft(&self, left: i32) -> windows_core::Result<()>;
    fn Left(&self) -> windows_core::Result<i32>;
    fn SetWidth(&self, width: i32) -> windows_core::Result<()>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn SetTop(&self, top: i32) -> windows_core::Result<()>;
    fn Top(&self) -> windows_core::Result<i32>;
    fn SetHeight(&self, height: i32) -> windows_core::Result<()>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn SetOwner(&self, owner: OAHWND) -> windows_core::Result<()>;
    fn Owner(&self) -> windows_core::Result<OAHWND>;
    fn SetMessageDrain(&self, drain: OAHWND) -> windows_core::Result<()>;
    fn MessageDrain(&self) -> windows_core::Result<OAHWND>;
    fn BorderColor(&self) -> windows_core::Result<i32>;
    fn SetBorderColor(&self, color: i32) -> windows_core::Result<()>;
    fn FullScreenMode(&self) -> windows_core::Result<i32>;
    fn SetFullScreenMode(&self, fullscreenmode: i32) -> windows_core::Result<()>;
    fn SetWindowForeground(&self, focus: i32) -> windows_core::Result<()>;
    fn NotifyOwnerMessage(&self, hwnd: OAHWND, umsg: i32, wparam: isize, lparam: isize) -> windows_core::Result<()>;
    fn SetWindowPosition(&self, left: i32, top: i32, width: i32, height: i32) -> windows_core::Result<()>;
    fn GetWindowPosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn GetMinIdealImageSize(&self, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn GetMaxIdealImageSize(&self, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn GetRestorePosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn HideCursor(&self, hidecursor: i32) -> windows_core::Result<()>;
    fn IsCursorHidden(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IVideoWindow_Vtbl {
    pub const fn new<Identity: IVideoWindow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCaption<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strcaption: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetCaption(this, core::mem::transmute(&strcaption)).into()
            }
        }
        unsafe extern "system" fn Caption<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strcaption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::Caption(this) {
                    Ok(ok__) => {
                        strcaption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWindowStyle<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstyle: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetWindowStyle(this, core::mem::transmute_copy(&windowstyle)).into()
            }
        }
        unsafe extern "system" fn WindowStyle<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstyle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::WindowStyle(this) {
                    Ok(ok__) => {
                        windowstyle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWindowStyleEx<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstyleex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetWindowStyleEx(this, core::mem::transmute_copy(&windowstyleex)).into()
            }
        }
        unsafe extern "system" fn WindowStyleEx<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstyleex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::WindowStyleEx(this) {
                    Ok(ok__) => {
                        windowstyleex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoShow<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoshow: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetAutoShow(this, core::mem::transmute_copy(&autoshow)).into()
            }
        }
        unsafe extern "system" fn AutoShow<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoshow: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::AutoShow(this) {
                    Ok(ok__) => {
                        autoshow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWindowState<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetWindowState(this, core::mem::transmute_copy(&windowstate)).into()
            }
        }
        unsafe extern "system" fn WindowState<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::WindowState(this) {
                    Ok(ok__) => {
                        windowstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackgroundPalette<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, backgroundpalette: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetBackgroundPalette(this, core::mem::transmute_copy(&backgroundpalette)).into()
            }
        }
        unsafe extern "system" fn BackgroundPalette<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbackgroundpalette: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::BackgroundPalette(this) {
                    Ok(ok__) => {
                        pbackgroundpalette.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVisible<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetVisible(this, core::mem::transmute_copy(&visible)).into()
            }
        }
        unsafe extern "system" fn Visible<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvisible: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::Visible(this) {
                    Ok(ok__) => {
                        pvisible.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLeft<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetLeft(this, core::mem::transmute_copy(&left)).into()
            }
        }
        unsafe extern "system" fn Left<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleft: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::Left(this) {
                    Ok(ok__) => {
                        pleft.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidth<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetWidth(this, core::mem::transmute_copy(&width)).into()
            }
        }
        unsafe extern "system" fn Width<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::Width(this) {
                    Ok(ok__) => {
                        pwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTop<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetTop(this, core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn Top<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::Top(this) {
                    Ok(ok__) => {
                        ptop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHeight<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetHeight(this, core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn Height<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::Height(this) {
                    Ok(ok__) => {
                        pheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOwner<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: OAHWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetOwner(this, core::mem::transmute_copy(&owner)).into()
            }
        }
        unsafe extern "system" fn Owner<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut OAHWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::Owner(this) {
                    Ok(ok__) => {
                        owner.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessageDrain<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drain: OAHWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetMessageDrain(this, core::mem::transmute_copy(&drain)).into()
            }
        }
        unsafe extern "system" fn MessageDrain<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drain: *mut OAHWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::MessageDrain(this) {
                    Ok(ok__) => {
                        drain.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BorderColor<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::BorderColor(this) {
                    Ok(ok__) => {
                        color.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBorderColor<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetBorderColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn FullScreenMode<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullscreenmode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::FullScreenMode(this) {
                    Ok(ok__) => {
                        fullscreenmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFullScreenMode<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullscreenmode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetFullScreenMode(this, core::mem::transmute_copy(&fullscreenmode)).into()
            }
        }
        unsafe extern "system" fn SetWindowForeground<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, focus: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetWindowForeground(this, core::mem::transmute_copy(&focus)).into()
            }
        }
        unsafe extern "system" fn NotifyOwnerMessage<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: OAHWND, umsg: i32, wparam: isize, lparam: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::NotifyOwnerMessage(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn SetWindowPosition<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32, top: i32, width: i32, height: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetWindowPosition(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn GetWindowPosition<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::GetWindowPosition(this, core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn GetMinIdealImageSize<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::GetMinIdealImageSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn GetMaxIdealImageSize<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::GetMaxIdealImageSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn GetRestorePosition<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::GetRestorePosition(this, core::mem::transmute_copy(&pleft), core::mem::transmute_copy(&ptop), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn HideCursor<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hidecursor: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::HideCursor(this, core::mem::transmute_copy(&hidecursor)).into()
            }
        }
        unsafe extern "system" fn IsCursorHidden<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cursorhidden: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::IsCursorHidden(this) {
                    Ok(ok__) => {
                        cursorhidden.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetCaption: SetCaption::<Identity, OFFSET>,
            Caption: Caption::<Identity, OFFSET>,
            SetWindowStyle: SetWindowStyle::<Identity, OFFSET>,
            WindowStyle: WindowStyle::<Identity, OFFSET>,
            SetWindowStyleEx: SetWindowStyleEx::<Identity, OFFSET>,
            WindowStyleEx: WindowStyleEx::<Identity, OFFSET>,
            SetAutoShow: SetAutoShow::<Identity, OFFSET>,
            AutoShow: AutoShow::<Identity, OFFSET>,
            SetWindowState: SetWindowState::<Identity, OFFSET>,
            WindowState: WindowState::<Identity, OFFSET>,
            SetBackgroundPalette: SetBackgroundPalette::<Identity, OFFSET>,
            BackgroundPalette: BackgroundPalette::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            SetLeft: SetLeft::<Identity, OFFSET>,
            Left: Left::<Identity, OFFSET>,
            SetWidth: SetWidth::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            SetTop: SetTop::<Identity, OFFSET>,
            Top: Top::<Identity, OFFSET>,
            SetHeight: SetHeight::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            SetOwner: SetOwner::<Identity, OFFSET>,
            Owner: Owner::<Identity, OFFSET>,
            SetMessageDrain: SetMessageDrain::<Identity, OFFSET>,
            MessageDrain: MessageDrain::<Identity, OFFSET>,
            BorderColor: BorderColor::<Identity, OFFSET>,
            SetBorderColor: SetBorderColor::<Identity, OFFSET>,
            FullScreenMode: FullScreenMode::<Identity, OFFSET>,
            SetFullScreenMode: SetFullScreenMode::<Identity, OFFSET>,
            SetWindowForeground: SetWindowForeground::<Identity, OFFSET>,
            NotifyOwnerMessage: NotifyOwnerMessage::<Identity, OFFSET>,
            SetWindowPosition: SetWindowPosition::<Identity, OFFSET>,
            GetWindowPosition: GetWindowPosition::<Identity, OFFSET>,
            GetMinIdealImageSize: GetMinIdealImageSize::<Identity, OFFSET>,
            GetMaxIdealImageSize: GetMaxIdealImageSize::<Identity, OFFSET>,
            GetRestorePosition: GetRestorePosition::<Identity, OFFSET>,
            HideCursor: HideCursor::<Identity, OFFSET>,
            IsCursorHidden: IsCursorHidden::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVideoWindow as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IVideoWindow {}
pub const LIBID_QuartzTypeLib: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b0_0ad4_11ce_b03a_0020af0ba770);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OAEVENT(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OAFilterState(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OAHWND(pub isize);
