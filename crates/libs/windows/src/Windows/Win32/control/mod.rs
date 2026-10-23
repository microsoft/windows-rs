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
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Item(&self, litem: i32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), litem, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAMCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAMCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn Item(&self, litem: i32) -> windows_core::Result<windows_core::IUnknown>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAMCollection_Vtbl {
    pub const fn new<Identity: IAMCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IAMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAMCollection_Impl::get_Count(this) {
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
        unsafe extern "system" fn get__NewEnum<Identity: IAMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAMCollection_Impl::get__NewEnum(this) {
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
            get_Count: get_Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAMCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    pub GetValueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut i32, *mut f64, *mut f64, *mut f64, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub GetValueByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut i32, *mut f64, *mut f64, *mut f64, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub AddValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, f64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAMStats_Impl: super::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn GetValueByIndex(&self, lindex: i32, szname: *mut windows_core::BSTR, lcount: *mut i32, dlast: *mut f64, daverage: *mut f64, dstddev: *mut f64, dmin: *mut f64, dmax: *mut f64) -> windows_core::Result<()>;
    fn GetValueByName(&self, szname: &windows_core::BSTR, lindex: *mut i32, lcount: *mut i32, dlast: *mut f64, daverage: *mut f64, dstddev: *mut f64, dmin: *mut f64, dmax: *mut f64) -> windows_core::Result<()>;
    fn GetIndex(&self, szname: &windows_core::BSTR, lcreate: i32) -> windows_core::Result<i32>;
    fn AddValue(&self, lindex: i32, dvalue: f64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAMStats_Vtbl {
    pub const fn new<Identity: IAMStats_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IAMStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAMStats_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn get_Count<Identity: IAMStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAMStats_Impl::get_Count(this) {
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
            get_Count: get_Count::<Identity, OFFSET>,
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn put_Volume(&self, lvolume: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Volume)(windows_core::Interface::as_raw(self), lvolume) }
    }
    pub unsafe fn get_Volume(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Volume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Balance(&self, lbalance: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Balance)(windows_core::Interface::as_raw(self), lbalance) }
    }
    pub unsafe fn get_Balance(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Balance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBasicAudio_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub put_Volume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Balance: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Balance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBasicAudio_Impl: super::IDispatch_Impl {
    fn put_Volume(&self, lvolume: i32) -> windows_core::Result<()>;
    fn get_Volume(&self) -> windows_core::Result<i32>;
    fn put_Balance(&self, lbalance: i32) -> windows_core::Result<()>;
    fn get_Balance(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBasicAudio_Vtbl {
    pub const fn new<Identity: IBasicAudio_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_Volume<Identity: IBasicAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicAudio_Impl::put_Volume(this, core::mem::transmute_copy(&lvolume)).into()
            }
        }
        unsafe extern "system" fn get_Volume<Identity: IBasicAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicAudio_Impl::get_Volume(this) {
                    Ok(ok__) => {
                        plvolume.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Balance<Identity: IBasicAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbalance: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicAudio_Impl::put_Balance(this, core::mem::transmute_copy(&lbalance)).into()
            }
        }
        unsafe extern "system" fn get_Balance<Identity: IBasicAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbalance: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicAudio_Impl::get_Balance(this) {
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
            put_Volume: put_Volume::<Identity, OFFSET>,
            get_Volume: get_Volume::<Identity, OFFSET>,
            put_Balance: put_Balance::<Identity, OFFSET>,
            get_Balance: get_Balance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBasicAudio as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn get_AvgTimePerFrame(&self) -> windows_core::Result<super::REFTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AvgTimePerFrame)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_BitRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BitRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_BitErrorRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BitErrorRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_VideoWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VideoWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_VideoHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VideoHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SourceLeft(&self, sourceleft: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SourceLeft)(windows_core::Interface::as_raw(self), sourceleft) }
    }
    pub unsafe fn get_SourceLeft(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SourceLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SourceWidth(&self, sourcewidth: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SourceWidth)(windows_core::Interface::as_raw(self), sourcewidth) }
    }
    pub unsafe fn get_SourceWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SourceWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SourceTop(&self, sourcetop: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SourceTop)(windows_core::Interface::as_raw(self), sourcetop) }
    }
    pub unsafe fn get_SourceTop(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SourceTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SourceHeight(&self, sourceheight: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SourceHeight)(windows_core::Interface::as_raw(self), sourceheight) }
    }
    pub unsafe fn get_SourceHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SourceHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DestinationLeft(&self, destinationleft: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DestinationLeft)(windows_core::Interface::as_raw(self), destinationleft) }
    }
    pub unsafe fn get_DestinationLeft(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DestinationLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DestinationWidth(&self, destinationwidth: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DestinationWidth)(windows_core::Interface::as_raw(self), destinationwidth) }
    }
    pub unsafe fn get_DestinationWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DestinationWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DestinationTop(&self, destinationtop: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DestinationTop)(windows_core::Interface::as_raw(self), destinationtop) }
    }
    pub unsafe fn get_DestinationTop(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DestinationTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DestinationHeight(&self, destinationheight: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DestinationHeight)(windows_core::Interface::as_raw(self), destinationheight) }
    }
    pub unsafe fn get_DestinationHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DestinationHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "shared")]
    pub get_AvgTimePerFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::REFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_AvgTimePerFrame: usize,
    pub get_BitRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_BitErrorRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_VideoWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_VideoHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SourceLeft: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_SourceLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SourceWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_SourceWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SourceTop: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_SourceTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SourceHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_SourceHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_DestinationLeft: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_DestinationLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_DestinationWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_DestinationWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_DestinationTop: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_DestinationTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_DestinationHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_DestinationHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBasicVideo_Impl: super::IDispatch_Impl {
    fn get_AvgTimePerFrame(&self) -> windows_core::Result<super::REFTIME>;
    fn get_BitRate(&self) -> windows_core::Result<i32>;
    fn get_BitErrorRate(&self) -> windows_core::Result<i32>;
    fn get_VideoWidth(&self) -> windows_core::Result<i32>;
    fn get_VideoHeight(&self) -> windows_core::Result<i32>;
    fn put_SourceLeft(&self, sourceleft: i32) -> windows_core::Result<()>;
    fn get_SourceLeft(&self) -> windows_core::Result<i32>;
    fn put_SourceWidth(&self, sourcewidth: i32) -> windows_core::Result<()>;
    fn get_SourceWidth(&self) -> windows_core::Result<i32>;
    fn put_SourceTop(&self, sourcetop: i32) -> windows_core::Result<()>;
    fn get_SourceTop(&self) -> windows_core::Result<i32>;
    fn put_SourceHeight(&self, sourceheight: i32) -> windows_core::Result<()>;
    fn get_SourceHeight(&self) -> windows_core::Result<i32>;
    fn put_DestinationLeft(&self, destinationleft: i32) -> windows_core::Result<()>;
    fn get_DestinationLeft(&self) -> windows_core::Result<i32>;
    fn put_DestinationWidth(&self, destinationwidth: i32) -> windows_core::Result<()>;
    fn get_DestinationWidth(&self) -> windows_core::Result<i32>;
    fn put_DestinationTop(&self, destinationtop: i32) -> windows_core::Result<()>;
    fn get_DestinationTop(&self) -> windows_core::Result<i32>;
    fn put_DestinationHeight(&self, destinationheight: i32) -> windows_core::Result<()>;
    fn get_DestinationHeight(&self) -> windows_core::Result<i32>;
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBasicVideo_Vtbl {
    pub const fn new<Identity: IBasicVideo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AvgTimePerFrame<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pavgtimeperframe: *mut super::REFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_AvgTimePerFrame(this) {
                    Ok(ok__) => {
                        pavgtimeperframe.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BitRate<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitrate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_BitRate(this) {
                    Ok(ok__) => {
                        pbitrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BitErrorRate<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbiterrorrate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_BitErrorRate(this) {
                    Ok(ok__) => {
                        pbiterrorrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VideoWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideowidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_VideoWidth(this) {
                    Ok(ok__) => {
                        pvideowidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VideoHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_VideoHeight(this) {
                    Ok(ok__) => {
                        pvideoheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SourceLeft<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceleft: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::put_SourceLeft(this, core::mem::transmute_copy(&sourceleft)).into()
            }
        }
        unsafe extern "system" fn get_SourceLeft<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourceleft: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_SourceLeft(this) {
                    Ok(ok__) => {
                        psourceleft.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SourceWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcewidth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::put_SourceWidth(this, core::mem::transmute_copy(&sourcewidth)).into()
            }
        }
        unsafe extern "system" fn get_SourceWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcewidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_SourceWidth(this) {
                    Ok(ok__) => {
                        psourcewidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SourceTop<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcetop: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::put_SourceTop(this, core::mem::transmute_copy(&sourcetop)).into()
            }
        }
        unsafe extern "system" fn get_SourceTop<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_SourceTop(this) {
                    Ok(ok__) => {
                        psourcetop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SourceHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceheight: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::put_SourceHeight(this, core::mem::transmute_copy(&sourceheight)).into()
            }
        }
        unsafe extern "system" fn get_SourceHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourceheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_SourceHeight(this) {
                    Ok(ok__) => {
                        psourceheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DestinationLeft<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationleft: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::put_DestinationLeft(this, core::mem::transmute_copy(&destinationleft)).into()
            }
        }
        unsafe extern "system" fn get_DestinationLeft<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationleft: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_DestinationLeft(this) {
                    Ok(ok__) => {
                        pdestinationleft.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DestinationWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationwidth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::put_DestinationWidth(this, core::mem::transmute_copy(&destinationwidth)).into()
            }
        }
        unsafe extern "system" fn get_DestinationWidth<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_DestinationWidth(this) {
                    Ok(ok__) => {
                        pdestinationwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DestinationTop<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationtop: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::put_DestinationTop(this, core::mem::transmute_copy(&destinationtop)).into()
            }
        }
        unsafe extern "system" fn get_DestinationTop<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationtop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_DestinationTop(this) {
                    Ok(ok__) => {
                        pdestinationtop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DestinationHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationheight: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBasicVideo_Impl::put_DestinationHeight(this, core::mem::transmute_copy(&destinationheight)).into()
            }
        }
        unsafe extern "system" fn get_DestinationHeight<Identity: IBasicVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicVideo_Impl::get_DestinationHeight(this) {
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
            get_AvgTimePerFrame: get_AvgTimePerFrame::<Identity, OFFSET>,
            get_BitRate: get_BitRate::<Identity, OFFSET>,
            get_BitErrorRate: get_BitErrorRate::<Identity, OFFSET>,
            get_VideoWidth: get_VideoWidth::<Identity, OFFSET>,
            get_VideoHeight: get_VideoHeight::<Identity, OFFSET>,
            put_SourceLeft: put_SourceLeft::<Identity, OFFSET>,
            get_SourceLeft: get_SourceLeft::<Identity, OFFSET>,
            put_SourceWidth: put_SourceWidth::<Identity, OFFSET>,
            get_SourceWidth: get_SourceWidth::<Identity, OFFSET>,
            put_SourceTop: put_SourceTop::<Identity, OFFSET>,
            get_SourceTop: get_SourceTop::<Identity, OFFSET>,
            put_SourceHeight: put_SourceHeight::<Identity, OFFSET>,
            get_SourceHeight: get_SourceHeight::<Identity, OFFSET>,
            put_DestinationLeft: put_DestinationLeft::<Identity, OFFSET>,
            get_DestinationLeft: get_DestinationLeft::<Identity, OFFSET>,
            put_DestinationWidth: put_DestinationWidth::<Identity, OFFSET>,
            get_DestinationWidth: get_DestinationWidth::<Identity, OFFSET>,
            put_DestinationTop: put_DestinationTop::<Identity, OFFSET>,
            get_DestinationTop: get_DestinationTop::<Identity, OFFSET>,
            put_DestinationHeight: put_DestinationHeight::<Identity, OFFSET>,
            get_DestinationHeight: get_DestinationHeight::<Identity, OFFSET>,
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBasicVideo2_Impl: IBasicVideo_Impl {
    fn GetPreferredAspectRatio(&self, plaspectx: *mut i32, plaspecty: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBasicVideo2 {}
windows_core::imp::define_interface!(IDeferredCommand, IDeferredCommand_Vtbl, 0x56a868b8_0ad4_11ce_b03a_0020af0ba770);
windows_core::imp::interface_hierarchy!(IDeferredCommand, windows_core::IUnknown);
impl IDeferredCommand {
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Confidence(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Confidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Postpone(&self, newtime: super::REFTIME) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub Confidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Confidence: usize,
    #[cfg(feature = "shared")]
    pub Postpone: unsafe extern "system" fn(*mut core::ffi::c_void, super::REFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Postpone: usize,
    pub GetHResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IDeferredCommand_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Confidence(&self) -> windows_core::Result<super::LONG>;
    fn Postpone(&self, newtime: super::REFTIME) -> windows_core::Result<()>;
    fn GetHResult(&self) -> windows_core::Result<windows_core::HRESULT>;
}
#[cfg(feature = "shared")]
impl IDeferredCommand_Vtbl {
    pub const fn new<Identity: IDeferredCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IDeferredCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeferredCommand_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Confidence<Identity: IDeferredCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfidence: *mut super::LONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Postpone<Identity: IDeferredCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newtime: super::REFTIME) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
            (windows_core::Interface::vtable(self).FindPin)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpinid), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_VendorInfo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VendorInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Filter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Filter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Pins(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Pins)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_IsFileSource(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsFileSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Filename(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Filename)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Filename(&self, strfilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Filename)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfilename)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFilterInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub FindPin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_VendorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Pins: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_IsFileSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_IsFileSource: usize,
    pub get_Filename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Filename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFilterInfo_Impl: super::IDispatch_Impl {
    fn FindPin(&self, strpinid: &windows_core::BSTR) -> windows_core::Result<super::IDispatch>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_VendorInfo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Filter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Pins(&self) -> windows_core::Result<super::IDispatch>;
    fn get_IsFileSource(&self) -> windows_core::Result<super::LONG>;
    fn get_Filename(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Filename(&self, strfilename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn get_Name<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::get_Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VendorInfo<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvendorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::get_VendorInfo(this) {
                    Ok(ok__) => {
                        strvendorinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Filter<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::get_Filter(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Pins<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::get_Pins(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsFileSource<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbissource: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::get_IsFileSource(this) {
                    Ok(ok__) => {
                        pbissource.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Filename<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFilterInfo_Impl::get_Filename(this) {
                    Ok(ok__) => {
                        pstrfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Filename<Identity: IFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilterInfo_Impl::put_Filename(this, core::mem::transmute(&strfilename)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FindPin: FindPin::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            get_VendorInfo: get_VendorInfo::<Identity, OFFSET>,
            get_Filter: get_Filter::<Identity, OFFSET>,
            get_Pins: get_Pins::<Identity, OFFSET>,
            get_IsFileSource: get_IsFileSource::<Identity, OFFSET>,
            get_Filename: get_Filename::<Identity, OFFSET>,
            put_Filename: put_Filename::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFilterInfo {}
pub const IID_IAMCollection: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b9_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IAMStats: windows_core::GUID = windows_core::GUID::from_u128(0xbc9bcf80_dcd2_11d2_abf6_00a0c905f375);
pub const IID_IBasicAudio: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b3_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IBasicVideo: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b5_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IBasicVideo2: windows_core::GUID = windows_core::GUID::from_u128(0x329bb360_f6ea_11d1_9038_00a0c9697298);
pub const IID_IDeferredCommand: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b8_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IFilterInfo: windows_core::GUID = windows_core::GUID::from_u128(0x56a868ba_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaControl: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b1_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaEvent: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b6_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaEventEx: windows_core::GUID = windows_core::GUID::from_u128(0x56a868c0_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaPosition: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b2_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IMediaTypeInfo: windows_core::GUID = windows_core::GUID::from_u128(0x56a868bc_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IPinInfo: windows_core::GUID = windows_core::GUID::from_u128(0x56a868bd_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IQueueCommand: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b7_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IRegFilterInfo: windows_core::GUID = windows_core::GUID::from_u128(0x56a868bb_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IVideoWindow: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b4_0ad4_11ce_b03a_0020af0ba770);
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetState(&self, mstimeout: super::LONG) -> windows_core::Result<OAFilterState> {
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
            (windows_core::Interface::vtable(self).AddSourceFilter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfilename), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_FilterCollection(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FilterCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_RegFilterCollection(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RegFilterCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    #[cfg(feature = "shared")]
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut OAFilterState) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetState: usize,
    pub RenderFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSourceFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FilterCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RegFilterCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopWhenReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaControl_Impl: super::IDispatch_Impl {
    fn Run(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn GetState(&self, mstimeout: super::LONG) -> windows_core::Result<OAFilterState>;
    fn RenderFile(&self, strfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddSourceFilter(&self, strfilename: &windows_core::BSTR) -> windows_core::Result<super::IDispatch>;
    fn get_FilterCollection(&self) -> windows_core::Result<super::IDispatch>;
    fn get_RegFilterCollection(&self) -> windows_core::Result<super::IDispatch>;
    fn StopWhenReady(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn GetState<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mstimeout: super::LONG, pfs: *mut OAFilterState) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_FilterCollection<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaControl_Impl::get_FilterCollection(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RegFilterCollection<Identity: IMediaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaControl_Impl::get_RegFilterCollection(this) {
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
            get_FilterCollection: get_FilterCollection::<Identity, OFFSET>,
            get_RegFilterCollection: get_RegFilterCollection::<Identity, OFFSET>,
            StopWhenReady: StopWhenReady::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaControl as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetEvent(&self, leventcode: *mut i32, lparam1: *mut super::LONG_PTR, lparam2: *mut super::LONG_PTR, mstimeout: i32) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub unsafe fn FreeEventParams(&self, levcode: i32, lparam1: super::LONG_PTR, lparam2: super::LONG_PTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeEventParams)(windows_core::Interface::as_raw(self), levcode, lparam1, lparam2) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEvent_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub GetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OAEVENT) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut super::LONG_PTR, *mut super::LONG_PTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEvent: usize,
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub CancelDefaultHandling: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RestoreDefaultHandling: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub FreeEventParams: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::LONG_PTR, super::LONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FreeEventParams: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaEvent_Impl: super::IDispatch_Impl {
    fn GetEventHandle(&self) -> windows_core::Result<OAEVENT>;
    fn GetEvent(&self, leventcode: *mut i32, lparam1: *mut super::LONG_PTR, lparam2: *mut super::LONG_PTR, mstimeout: i32) -> windows_core::Result<()>;
    fn WaitForCompletion(&self, mstimeout: i32) -> windows_core::Result<i32>;
    fn CancelDefaultHandling(&self, levcode: i32) -> windows_core::Result<()>;
    fn RestoreDefaultHandling(&self, levcode: i32) -> windows_core::Result<()>;
    fn FreeEventParams(&self, levcode: i32, lparam1: super::LONG_PTR, lparam2: super::LONG_PTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn GetEvent<Identity: IMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, leventcode: *mut i32, lparam1: *mut super::LONG_PTR, lparam2: *mut super::LONG_PTR, mstimeout: i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn FreeEventParams<Identity: IMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, levcode: i32, lparam1: super::LONG_PTR, lparam2: super::LONG_PTR) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn SetNotifyWindow(&self, hwnd: OAHWND, lmsg: i32, linstancedata: super::LONG_PTR) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub SetNotifyWindow: unsafe extern "system" fn(*mut core::ffi::c_void, OAHWND, i32, super::LONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetNotifyWindow: usize,
    pub SetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaEventEx_Impl: IMediaEvent_Impl {
    fn SetNotifyWindow(&self, hwnd: OAHWND, lmsg: i32, linstancedata: super::LONG_PTR) -> windows_core::Result<()>;
    fn SetNotifyFlags(&self, lnonotifyflags: i32) -> windows_core::Result<()>;
    fn GetNotifyFlags(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMediaEventEx_Vtbl {
    pub const fn new<Identity: IMediaEventEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNotifyWindow<Identity: IMediaEventEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: OAHWND, lmsg: i32, linstancedata: super::LONG_PTR) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn get_Duration(&self) -> windows_core::Result<super::REFTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_CurrentPosition(&self, lltime: super::REFTIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CurrentPosition)(windows_core::Interface::as_raw(self), lltime) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentPosition(&self) -> windows_core::Result<super::REFTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_StopTime(&self) -> windows_core::Result<super::REFTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StopTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_StopTime(&self, lltime: super::REFTIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StopTime)(windows_core::Interface::as_raw(self), lltime) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_PrerollTime(&self) -> windows_core::Result<super::REFTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PrerollTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_PrerollTime(&self, lltime: super::REFTIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PrerollTime)(windows_core::Interface::as_raw(self), lltime) }
    }
    pub unsafe fn put_Rate(&self, drate: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Rate)(windows_core::Interface::as_raw(self), drate) }
    }
    pub unsafe fn get_Rate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Rate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CanSeekForward(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanSeekForward)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CanSeekBackward(&self) -> windows_core::Result<super::LONG> {
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
    #[cfg(feature = "shared")]
    pub get_Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::REFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Duration: usize,
    #[cfg(feature = "shared")]
    pub put_CurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::REFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_CurrentPosition: usize,
    #[cfg(feature = "shared")]
    pub get_CurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::REFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentPosition: usize,
    #[cfg(feature = "shared")]
    pub get_StopTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::REFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StopTime: usize,
    #[cfg(feature = "shared")]
    pub put_StopTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::REFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_StopTime: usize,
    #[cfg(feature = "shared")]
    pub get_PrerollTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::REFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_PrerollTime: usize,
    #[cfg(feature = "shared")]
    pub put_PrerollTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::REFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_PrerollTime: usize,
    pub put_Rate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub get_Rate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub CanSeekForward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CanSeekForward: usize,
    #[cfg(feature = "shared")]
    pub CanSeekBackward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CanSeekBackward: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaPosition_Impl: super::IDispatch_Impl {
    fn get_Duration(&self) -> windows_core::Result<super::REFTIME>;
    fn put_CurrentPosition(&self, lltime: super::REFTIME) -> windows_core::Result<()>;
    fn get_CurrentPosition(&self) -> windows_core::Result<super::REFTIME>;
    fn get_StopTime(&self) -> windows_core::Result<super::REFTIME>;
    fn put_StopTime(&self, lltime: super::REFTIME) -> windows_core::Result<()>;
    fn get_PrerollTime(&self) -> windows_core::Result<super::REFTIME>;
    fn put_PrerollTime(&self, lltime: super::REFTIME) -> windows_core::Result<()>;
    fn put_Rate(&self, drate: f64) -> windows_core::Result<()>;
    fn get_Rate(&self) -> windows_core::Result<f64>;
    fn CanSeekForward(&self) -> windows_core::Result<super::LONG>;
    fn CanSeekBackward(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMediaPosition_Vtbl {
    pub const fn new<Identity: IMediaPosition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Duration<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plength: *mut super::REFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::get_Duration(this) {
                    Ok(ok__) => {
                        plength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CurrentPosition<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lltime: super::REFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaPosition_Impl::put_CurrentPosition(this, core::mem::transmute_copy(&lltime)).into()
            }
        }
        unsafe extern "system" fn get_CurrentPosition<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plltime: *mut super::REFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::get_CurrentPosition(this) {
                    Ok(ok__) => {
                        plltime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StopTime<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plltime: *mut super::REFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::get_StopTime(this) {
                    Ok(ok__) => {
                        plltime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StopTime<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lltime: super::REFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaPosition_Impl::put_StopTime(this, core::mem::transmute_copy(&lltime)).into()
            }
        }
        unsafe extern "system" fn get_PrerollTime<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plltime: *mut super::REFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::get_PrerollTime(this) {
                    Ok(ok__) => {
                        plltime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PrerollTime<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lltime: super::REFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaPosition_Impl::put_PrerollTime(this, core::mem::transmute_copy(&lltime)).into()
            }
        }
        unsafe extern "system" fn put_Rate<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drate: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaPosition_Impl::put_Rate(this, core::mem::transmute_copy(&drate)).into()
            }
        }
        unsafe extern "system" fn get_Rate<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdrate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPosition_Impl::get_Rate(this) {
                    Ok(ok__) => {
                        pdrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanSeekForward<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcanseekforward: *mut super::LONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CanSeekBackward<Identity: IMediaPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcanseekbackward: *mut super::LONG) -> windows_core::HRESULT {
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
            get_Duration: get_Duration::<Identity, OFFSET>,
            put_CurrentPosition: put_CurrentPosition::<Identity, OFFSET>,
            get_CurrentPosition: get_CurrentPosition::<Identity, OFFSET>,
            get_StopTime: get_StopTime::<Identity, OFFSET>,
            put_StopTime: put_StopTime::<Identity, OFFSET>,
            get_PrerollTime: get_PrerollTime::<Identity, OFFSET>,
            put_PrerollTime: put_PrerollTime::<Identity, OFFSET>,
            put_Rate: put_Rate::<Identity, OFFSET>,
            get_Rate: get_Rate::<Identity, OFFSET>,
            CanSeekForward: CanSeekForward::<Identity, OFFSET>,
            CanSeekBackward: CanSeekBackward::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaPosition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Type(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Subtype(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Subtype)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTypeInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Subtype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMediaTypeInfo_Impl: super::IDispatch_Impl {
    fn get_Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Subtype(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMediaTypeInfo_Vtbl {
    pub const fn new<Identity: IMediaTypeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Type<Identity: IMediaTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strtype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaTypeInfo_Impl::get_Type(this) {
                    Ok(ok__) => {
                        strtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Subtype<Identity: IMediaTypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strtype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaTypeInfo_Impl::get_Subtype(this) {
                    Ok(ok__) => {
                        strtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_Type: get_Type::<Identity, OFFSET>, get_Subtype: get_Subtype::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaTypeInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Pin(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Pin)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ConnectedTo(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ConnectionMediaType(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ConnectionMediaType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_FilterInfo(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FilterInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Direction(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_PinID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PinID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_MediaTypes(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MediaTypes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub get_Pin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ConnectionMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FilterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Direction: usize,
    pub get_PinID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectDirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectWithType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPinInfo_Impl: super::IDispatch_Impl {
    fn get_Pin(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_ConnectedTo(&self) -> windows_core::Result<super::IDispatch>;
    fn get_ConnectionMediaType(&self) -> windows_core::Result<super::IDispatch>;
    fn get_FilterInfo(&self) -> windows_core::Result<super::IDispatch>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Direction(&self) -> windows_core::Result<super::LONG>;
    fn get_PinID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_MediaTypes(&self) -> windows_core::Result<super::IDispatch>;
    fn Connect(&self, ppin: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ConnectDirect(&self, ppin: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ConnectWithType(&self, ppin: windows_core::Ref<windows_core::IUnknown>, pmediatype: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Render(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPinInfo_Vtbl {
    pub const fn new<Identity: IPinInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Pin<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::get_Pin(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ConnectedTo<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::get_ConnectedTo(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ConnectionMediaType<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::get_ConnectionMediaType(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FilterInfo<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::get_FilterInfo(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Name<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::get_Name(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Direction<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdirection: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::get_Direction(this) {
                    Ok(ok__) => {
                        ppdirection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PinID<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpinid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::get_PinID(this) {
                    Ok(ok__) => {
                        strpinid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MediaTypes<Identity: IPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPinInfo_Impl::get_MediaTypes(this) {
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
            get_Pin: get_Pin::<Identity, OFFSET>,
            get_ConnectedTo: get_ConnectedTo::<Identity, OFFSET>,
            get_ConnectionMediaType: get_ConnectionMediaType::<Identity, OFFSET>,
            get_FilterInfo: get_FilterInfo::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            get_Direction: get_Direction::<Identity, OFFSET>,
            get_PinID: get_PinID::<Identity, OFFSET>,
            get_MediaTypes: get_MediaTypes::<Identity, OFFSET>,
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPinInfo {}
windows_core::imp::define_interface!(IQueueCommand, IQueueCommand_Vtbl, 0x56a868b7_0ad4_11ce_b03a_0020af0ba770);
windows_core::imp::interface_hierarchy!(IQueueCommand, windows_core::IUnknown);
impl IQueueCommand {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InvokeAtStreamTime(&self, pcmd: *mut Option<IDeferredCommand>, time: super::REFTIME, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvokeAtStreamTime)(windows_core::Interface::as_raw(self), core::mem::transmute(pcmd), time, iid, dispidmethod, wflags, cargs, pdispparams, pvarresult, puargerr as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InvokeAtPresentationTime(&self, pcmd: *mut Option<IDeferredCommand>, time: super::REFTIME, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvokeAtPresentationTime)(windows_core::Interface::as_raw(self), core::mem::transmute(pcmd), time, iid, dispidmethod, wflags, cargs, pdispparams, pvarresult, puargerr as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueueCommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub InvokeAtStreamTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::REFTIME, *const windows_core::GUID, i32, i16, i32, *const super::VARIANT, *mut super::VARIANT, *mut i16) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    InvokeAtStreamTime: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub InvokeAtPresentationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::REFTIME, *const windows_core::GUID, i32, i16, i32, *const super::VARIANT, *mut super::VARIANT, *mut i16) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    InvokeAtPresentationTime: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IQueueCommand_Impl: windows_core::IUnknownImpl {
    fn InvokeAtStreamTime(&self, pcmd: windows_core::OutRef<IDeferredCommand>, time: super::REFTIME, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::Result<()>;
    fn InvokeAtPresentationTime(&self, pcmd: windows_core::OutRef<IDeferredCommand>, time: super::REFTIME, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IQueueCommand_Vtbl {
    pub const fn new<Identity: IQueueCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvokeAtStreamTime<Identity: IQueueCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmd: *mut *mut core::ffi::c_void, time: super::REFTIME, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueueCommand_Impl::InvokeAtStreamTime(this, core::mem::transmute_copy(&pcmd), core::mem::transmute_copy(&time), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&dispidmethod), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&cargs), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        unsafe extern "system" fn InvokeAtPresentationTime<Identity: IQueueCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmd: *mut *mut core::ffi::c_void, time: super::REFTIME, iid: *const windows_core::GUID, dispidmethod: i32, wflags: i16, cargs: i32, pdispparams: *const super::VARIANT, pvarresult: *mut super::VARIANT, puargerr: *mut i16) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Filter(&self) -> windows_core::Result<super::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Filter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegFilterInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRegFilterInfo_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Filter(&self) -> windows_core::Result<super::IDispatch>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRegFilterInfo_Vtbl {
    pub const fn new<Identity: IRegFilterInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IRegFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegFilterInfo_Impl::get_Name(this) {
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
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_Name: get_Name::<Identity, OFFSET>, Filter: Filter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegFilterInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn put_Caption(&self, strcaption: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Caption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strcaption)) }
    }
    pub unsafe fn get_Caption(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Caption)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_WindowStyle(&self, windowstyle: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WindowStyle)(windows_core::Interface::as_raw(self), windowstyle) }
    }
    pub unsafe fn get_WindowStyle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_WindowStyleEx(&self, windowstyleex: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WindowStyleEx)(windows_core::Interface::as_raw(self), windowstyleex) }
    }
    pub unsafe fn get_WindowStyleEx(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowStyleEx)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AutoShow(&self, autoshow: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AutoShow)(windows_core::Interface::as_raw(self), autoshow) }
    }
    pub unsafe fn get_AutoShow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoShow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_WindowState(&self, windowstate: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WindowState)(windows_core::Interface::as_raw(self), windowstate) }
    }
    pub unsafe fn get_WindowState(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BackgroundPalette(&self, backgroundpalette: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BackgroundPalette)(windows_core::Interface::as_raw(self), backgroundpalette) }
    }
    pub unsafe fn get_BackgroundPalette(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BackgroundPalette)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Visible(&self, visible: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Visible)(windows_core::Interface::as_raw(self), visible) }
    }
    pub unsafe fn get_Visible(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Left(&self, left: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Left)(windows_core::Interface::as_raw(self), left) }
    }
    pub unsafe fn get_Left(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Left)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Width(&self, width: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Width)(windows_core::Interface::as_raw(self), width) }
    }
    pub unsafe fn get_Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Top(&self, top: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Top)(windows_core::Interface::as_raw(self), top) }
    }
    pub unsafe fn get_Top(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Top)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Height(&self, height: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Height)(windows_core::Interface::as_raw(self), height) }
    }
    pub unsafe fn get_Height(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Owner(&self, owner: OAHWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Owner)(windows_core::Interface::as_raw(self), owner) }
    }
    pub unsafe fn get_Owner(&self) -> windows_core::Result<OAHWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Owner)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MessageDrain(&self, drain: OAHWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MessageDrain)(windows_core::Interface::as_raw(self), drain) }
    }
    pub unsafe fn get_MessageDrain(&self) -> windows_core::Result<OAHWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MessageDrain)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_BorderColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BorderColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BorderColor(&self, color: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BorderColor)(windows_core::Interface::as_raw(self), color) }
    }
    pub unsafe fn get_FullScreenMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FullScreenMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_FullScreenMode(&self, fullscreenmode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FullScreenMode)(windows_core::Interface::as_raw(self), fullscreenmode) }
    }
    pub unsafe fn SetWindowForeground(&self, focus: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWindowForeground)(windows_core::Interface::as_raw(self), focus) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn NotifyOwnerMessage(&self, hwnd: OAHWND, umsg: i32, wparam: super::LONG_PTR, lparam: super::LONG_PTR) -> windows_core::HRESULT {
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
    pub put_Caption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Caption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_WindowStyle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_WindowStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_WindowStyleEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_WindowStyleEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_AutoShow: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_AutoShow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_WindowState: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_WindowState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_BackgroundPalette: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_BackgroundPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Visible: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Left: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Left: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Width: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Top: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Top: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Height: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Owner: unsafe extern "system" fn(*mut core::ffi::c_void, OAHWND) -> windows_core::HRESULT,
    pub get_Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OAHWND) -> windows_core::HRESULT,
    pub put_MessageDrain: unsafe extern "system" fn(*mut core::ffi::c_void, OAHWND) -> windows_core::HRESULT,
    pub get_MessageDrain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OAHWND) -> windows_core::HRESULT,
    pub get_BorderColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_BorderColor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_FullScreenMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_FullScreenMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetWindowForeground: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub NotifyOwnerMessage: unsafe extern "system" fn(*mut core::ffi::c_void, OAHWND, i32, super::LONG_PTR, super::LONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    NotifyOwnerMessage: usize,
    pub SetWindowPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub GetWindowPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetMinIdealImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetMaxIdealImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetRestorePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub HideCursor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsCursorHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IVideoWindow_Impl: super::IDispatch_Impl {
    fn put_Caption(&self, strcaption: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Caption(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_WindowStyle(&self, windowstyle: i32) -> windows_core::Result<()>;
    fn get_WindowStyle(&self) -> windows_core::Result<i32>;
    fn put_WindowStyleEx(&self, windowstyleex: i32) -> windows_core::Result<()>;
    fn get_WindowStyleEx(&self) -> windows_core::Result<i32>;
    fn put_AutoShow(&self, autoshow: i32) -> windows_core::Result<()>;
    fn get_AutoShow(&self) -> windows_core::Result<i32>;
    fn put_WindowState(&self, windowstate: i32) -> windows_core::Result<()>;
    fn get_WindowState(&self) -> windows_core::Result<i32>;
    fn put_BackgroundPalette(&self, backgroundpalette: i32) -> windows_core::Result<()>;
    fn get_BackgroundPalette(&self) -> windows_core::Result<i32>;
    fn put_Visible(&self, visible: i32) -> windows_core::Result<()>;
    fn get_Visible(&self) -> windows_core::Result<i32>;
    fn put_Left(&self, left: i32) -> windows_core::Result<()>;
    fn get_Left(&self) -> windows_core::Result<i32>;
    fn put_Width(&self, width: i32) -> windows_core::Result<()>;
    fn get_Width(&self) -> windows_core::Result<i32>;
    fn put_Top(&self, top: i32) -> windows_core::Result<()>;
    fn get_Top(&self) -> windows_core::Result<i32>;
    fn put_Height(&self, height: i32) -> windows_core::Result<()>;
    fn get_Height(&self) -> windows_core::Result<i32>;
    fn put_Owner(&self, owner: OAHWND) -> windows_core::Result<()>;
    fn get_Owner(&self) -> windows_core::Result<OAHWND>;
    fn put_MessageDrain(&self, drain: OAHWND) -> windows_core::Result<()>;
    fn get_MessageDrain(&self) -> windows_core::Result<OAHWND>;
    fn get_BorderColor(&self) -> windows_core::Result<i32>;
    fn put_BorderColor(&self, color: i32) -> windows_core::Result<()>;
    fn get_FullScreenMode(&self) -> windows_core::Result<i32>;
    fn put_FullScreenMode(&self, fullscreenmode: i32) -> windows_core::Result<()>;
    fn SetWindowForeground(&self, focus: i32) -> windows_core::Result<()>;
    fn NotifyOwnerMessage(&self, hwnd: OAHWND, umsg: i32, wparam: super::LONG_PTR, lparam: super::LONG_PTR) -> windows_core::Result<()>;
    fn SetWindowPosition(&self, left: i32, top: i32, width: i32, height: i32) -> windows_core::Result<()>;
    fn GetWindowPosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn GetMinIdealImageSize(&self, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn GetMaxIdealImageSize(&self, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn GetRestorePosition(&self, pleft: *mut i32, ptop: *mut i32, pwidth: *mut i32, pheight: *mut i32) -> windows_core::Result<()>;
    fn HideCursor(&self, hidecursor: i32) -> windows_core::Result<()>;
    fn IsCursorHidden(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IVideoWindow_Vtbl {
    pub const fn new<Identity: IVideoWindow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_Caption<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strcaption: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_Caption(this, core::mem::transmute(&strcaption)).into()
            }
        }
        unsafe extern "system" fn get_Caption<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strcaption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_Caption(this) {
                    Ok(ok__) => {
                        strcaption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WindowStyle<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstyle: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_WindowStyle(this, core::mem::transmute_copy(&windowstyle)).into()
            }
        }
        unsafe extern "system" fn get_WindowStyle<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstyle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_WindowStyle(this) {
                    Ok(ok__) => {
                        windowstyle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WindowStyleEx<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstyleex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_WindowStyleEx(this, core::mem::transmute_copy(&windowstyleex)).into()
            }
        }
        unsafe extern "system" fn get_WindowStyleEx<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstyleex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_WindowStyleEx(this) {
                    Ok(ok__) => {
                        windowstyleex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AutoShow<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoshow: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_AutoShow(this, core::mem::transmute_copy(&autoshow)).into()
            }
        }
        unsafe extern "system" fn get_AutoShow<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoshow: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_AutoShow(this) {
                    Ok(ok__) => {
                        autoshow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WindowState<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_WindowState(this, core::mem::transmute_copy(&windowstate)).into()
            }
        }
        unsafe extern "system" fn get_WindowState<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowstate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_WindowState(this) {
                    Ok(ok__) => {
                        windowstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BackgroundPalette<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, backgroundpalette: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_BackgroundPalette(this, core::mem::transmute_copy(&backgroundpalette)).into()
            }
        }
        unsafe extern "system" fn get_BackgroundPalette<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbackgroundpalette: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_BackgroundPalette(this) {
                    Ok(ok__) => {
                        pbackgroundpalette.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Visible<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visible: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_Visible(this, core::mem::transmute_copy(&visible)).into()
            }
        }
        unsafe extern "system" fn get_Visible<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvisible: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_Visible(this) {
                    Ok(ok__) => {
                        pvisible.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Left<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_Left(this, core::mem::transmute_copy(&left)).into()
            }
        }
        unsafe extern "system" fn get_Left<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pleft: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_Left(this) {
                    Ok(ok__) => {
                        pleft.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Width<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_Width(this, core::mem::transmute_copy(&width)).into()
            }
        }
        unsafe extern "system" fn get_Width<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_Width(this) {
                    Ok(ok__) => {
                        pwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Top<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_Top(this, core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn get_Top<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptop: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_Top(this) {
                    Ok(ok__) => {
                        ptop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Height<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_Height(this, core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn get_Height<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_Height(this) {
                    Ok(ok__) => {
                        pheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Owner<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: OAHWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_Owner(this, core::mem::transmute_copy(&owner)).into()
            }
        }
        unsafe extern "system" fn get_Owner<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, owner: *mut OAHWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_Owner(this) {
                    Ok(ok__) => {
                        owner.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MessageDrain<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drain: OAHWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_MessageDrain(this, core::mem::transmute_copy(&drain)).into()
            }
        }
        unsafe extern "system" fn get_MessageDrain<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drain: *mut OAHWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_MessageDrain(this) {
                    Ok(ok__) => {
                        drain.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BorderColor<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_BorderColor(this) {
                    Ok(ok__) => {
                        color.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BorderColor<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_BorderColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn get_FullScreenMode<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullscreenmode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVideoWindow_Impl::get_FullScreenMode(this) {
                    Ok(ok__) => {
                        fullscreenmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FullScreenMode<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullscreenmode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::put_FullScreenMode(this, core::mem::transmute_copy(&fullscreenmode)).into()
            }
        }
        unsafe extern "system" fn SetWindowForeground<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, focus: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVideoWindow_Impl::SetWindowForeground(this, core::mem::transmute_copy(&focus)).into()
            }
        }
        unsafe extern "system" fn NotifyOwnerMessage<Identity: IVideoWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: OAHWND, umsg: i32, wparam: super::LONG_PTR, lparam: super::LONG_PTR) -> windows_core::HRESULT {
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
            put_Caption: put_Caption::<Identity, OFFSET>,
            get_Caption: get_Caption::<Identity, OFFSET>,
            put_WindowStyle: put_WindowStyle::<Identity, OFFSET>,
            get_WindowStyle: get_WindowStyle::<Identity, OFFSET>,
            put_WindowStyleEx: put_WindowStyleEx::<Identity, OFFSET>,
            get_WindowStyleEx: get_WindowStyleEx::<Identity, OFFSET>,
            put_AutoShow: put_AutoShow::<Identity, OFFSET>,
            get_AutoShow: get_AutoShow::<Identity, OFFSET>,
            put_WindowState: put_WindowState::<Identity, OFFSET>,
            get_WindowState: get_WindowState::<Identity, OFFSET>,
            put_BackgroundPalette: put_BackgroundPalette::<Identity, OFFSET>,
            get_BackgroundPalette: get_BackgroundPalette::<Identity, OFFSET>,
            put_Visible: put_Visible::<Identity, OFFSET>,
            get_Visible: get_Visible::<Identity, OFFSET>,
            put_Left: put_Left::<Identity, OFFSET>,
            get_Left: get_Left::<Identity, OFFSET>,
            put_Width: put_Width::<Identity, OFFSET>,
            get_Width: get_Width::<Identity, OFFSET>,
            put_Top: put_Top::<Identity, OFFSET>,
            get_Top: get_Top::<Identity, OFFSET>,
            put_Height: put_Height::<Identity, OFFSET>,
            get_Height: get_Height::<Identity, OFFSET>,
            put_Owner: put_Owner::<Identity, OFFSET>,
            get_Owner: get_Owner::<Identity, OFFSET>,
            put_MessageDrain: put_MessageDrain::<Identity, OFFSET>,
            get_MessageDrain: get_MessageDrain::<Identity, OFFSET>,
            get_BorderColor: get_BorderColor::<Identity, OFFSET>,
            put_BorderColor: put_BorderColor::<Identity, OFFSET>,
            get_FullScreenMode: get_FullScreenMode::<Identity, OFFSET>,
            put_FullScreenMode: put_FullScreenMode::<Identity, OFFSET>,
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IVideoWindow {}
pub const LIBID_QuartzTypeLib: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b0_0ad4_11ce_b03a_0020af0ba770);
#[cfg(target_arch = "x86")]
pub type OAEVENT = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type OAEVENT = i64;
pub type OAFilterState = i32;
#[cfg(target_arch = "x86")]
pub type OAHWND = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type OAHWND = i64;
