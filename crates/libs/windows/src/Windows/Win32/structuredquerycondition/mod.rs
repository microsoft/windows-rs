pub type CONDITION_OPERATION = i32;
pub type CONDITION_TYPE = i32;
pub const COP_APPLICATION_SPECIFIC: CONDITION_OPERATION = 14;
pub const COP_DOSWILDCARDS: CONDITION_OPERATION = 11;
pub const COP_EQUAL: CONDITION_OPERATION = 1;
pub const COP_GREATERTHAN: CONDITION_OPERATION = 4;
pub const COP_GREATERTHANOREQUAL: CONDITION_OPERATION = 6;
pub const COP_IMPLICIT: CONDITION_OPERATION = 0;
pub const COP_LESSTHAN: CONDITION_OPERATION = 3;
pub const COP_LESSTHANOREQUAL: CONDITION_OPERATION = 5;
pub const COP_NOTEQUAL: CONDITION_OPERATION = 2;
pub const COP_VALUE_CONTAINS: CONDITION_OPERATION = 9;
pub const COP_VALUE_ENDSWITH: CONDITION_OPERATION = 8;
pub const COP_VALUE_NOTCONTAINS: CONDITION_OPERATION = 10;
pub const COP_VALUE_STARTSWITH: CONDITION_OPERATION = 7;
pub const COP_WORD_EQUAL: CONDITION_OPERATION = 12;
pub const COP_WORD_STARTSWITH: CONDITION_OPERATION = 13;
pub const CT_AND_CONDITION: CONDITION_TYPE = 0;
pub const CT_LEAF_CONDITION: CONDITION_TYPE = 3;
pub const CT_NOT_CONDITION: CONDITION_TYPE = 2;
pub const CT_OR_CONDITION: CONDITION_TYPE = 1;
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(ICondition, ICondition_Vtbl, 0x0fc988d4_c935_4b97_a973_46282ea175c8);
#[cfg(feature = "objidl")]
impl core::ops::Deref for ICondition {
    type Target = super::IPersistStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(ICondition, windows_core::IUnknown, super::IPersist, super::IPersistStream);
#[cfg(feature = "objidl")]
impl ICondition {
    pub unsafe fn GetConditionType(&self) -> windows_core::Result<CONDITION_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConditionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSubConditions<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSubConditions)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetComparisonInfo(&self, ppszpropertyname: *mut windows_core::PWSTR, pcop: Option<*mut CONDITION_OPERATION>, ppropvar: Option<*mut super::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetComparisonInfo)(windows_core::Interface::as_raw(self), ppszpropertyname as _, pcop.unwrap_or(core::mem::zeroed()) as _, ppropvar.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetValueType(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetValueNormalization(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValueNormalization)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInputTerms(&self, pppropertyterm: Option<*mut Option<IRichChunk>>, ppoperationterm: Option<*mut Option<IRichChunk>>, ppvalueterm: Option<*mut Option<IRichChunk>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputTerms)(windows_core::Interface::as_raw(self), pppropertyterm.unwrap_or(core::mem::zeroed()) as _, ppoperationterm.unwrap_or(core::mem::zeroed()) as _, ppvalueterm.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICondition_Vtbl {
    pub base__: super::IPersistStream_Vtbl,
    pub GetConditionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CONDITION_TYPE) -> windows_core::HRESULT,
    pub GetSubConditions: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetComparisonInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut CONDITION_OPERATION, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetComparisonInfo: usize,
    pub GetValueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetValueNormalization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetInputTerms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICondition_Impl: super::IPersistStream_Impl {
    fn GetConditionType(&self) -> windows_core::Result<CONDITION_TYPE>;
    fn GetSubConditions(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetComparisonInfo(&self, ppszpropertyname: *mut windows_core::PWSTR, pcop: *mut CONDITION_OPERATION, ppropvar: *mut super::PROPVARIANT) -> windows_core::Result<()>;
    fn GetValueType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetValueNormalization(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetInputTerms(&self, pppropertyterm: windows_core::OutRef<IRichChunk>, ppoperationterm: windows_core::OutRef<IRichChunk>, ppvalueterm: windows_core::OutRef<IRichChunk>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICondition>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl ICondition_Vtbl {
    pub const fn new<Identity: ICondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConditionType<Identity: ICondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnodetype: *mut CONDITION_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICondition_Impl::GetConditionType(this) {
                    Ok(ok__) => {
                        pnodetype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSubConditions<Identity: ICondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICondition_Impl::GetSubConditions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetComparisonInfo<Identity: ICondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpropertyname: *mut windows_core::PWSTR, pcop: *mut CONDITION_OPERATION, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICondition_Impl::GetComparisonInfo(this, core::mem::transmute_copy(&ppszpropertyname), core::mem::transmute_copy(&pcop), core::mem::transmute_copy(&ppropvar)).into()
            }
        }
        unsafe extern "system" fn GetValueType<Identity: ICondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszvaluetypename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICondition_Impl::GetValueType(this) {
                    Ok(ok__) => {
                        ppszvaluetypename.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValueNormalization<Identity: ICondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsznormalization: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICondition_Impl::GetValueNormalization(this) {
                    Ok(ok__) => {
                        ppsznormalization.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputTerms<Identity: ICondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyterm: *mut *mut core::ffi::c_void, ppoperationterm: *mut *mut core::ffi::c_void, ppvalueterm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICondition_Impl::GetInputTerms(this, core::mem::transmute_copy(&pppropertyterm), core::mem::transmute_copy(&ppoperationterm), core::mem::transmute_copy(&ppvalueterm)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ICondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICondition_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IPersistStream_Vtbl::new::<Identity, OFFSET>(),
            GetConditionType: GetConditionType::<Identity, OFFSET>,
            GetSubConditions: GetSubConditions::<Identity, OFFSET>,
            GetComparisonInfo: GetComparisonInfo::<Identity, OFFSET>,
            GetValueType: GetValueType::<Identity, OFFSET>,
            GetValueNormalization: GetValueNormalization::<Identity, OFFSET>,
            GetInputTerms: GetInputTerms::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICondition as windows_core::Interface>::IID || iid == &<super::IPersist as windows_core::Interface>::IID || iid == &<super::IPersistStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICondition {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(ICondition2, ICondition2_Vtbl, 0x0db8851d_2e5b_47eb_9208_d28c325a01d7);
#[cfg(feature = "objidl")]
impl core::ops::Deref for ICondition2 {
    type Target = ICondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(ICondition2, windows_core::IUnknown, super::IPersist, super::IPersistStream, ICondition);
#[cfg(feature = "objidl")]
impl ICondition2 {
    pub unsafe fn GetLocale(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetLeafConditionInfo(&self, ppropkey: Option<*mut super::PROPERTYKEY>, pcop: Option<*mut CONDITION_OPERATION>, ppropvar: Option<*mut super::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLeafConditionInfo)(windows_core::Interface::as_raw(self), ppropkey.unwrap_or(core::mem::zeroed()) as _, pcop.unwrap_or(core::mem::zeroed()) as _, ppropvar.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICondition2_Vtbl {
    pub base__: ICondition_Vtbl,
    pub GetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetLeafConditionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PROPERTYKEY, *mut CONDITION_OPERATION, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetLeafConditionInfo: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICondition2_Impl: ICondition_Impl {
    fn GetLocale(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetLeafConditionInfo(&self, ppropkey: *mut super::PROPERTYKEY, pcop: *mut CONDITION_OPERATION, ppropvar: *mut super::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl ICondition2_Vtbl {
    pub const fn new<Identity: ICondition2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLocale<Identity: ICondition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszlocalename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICondition2_Impl::GetLocale(this) {
                    Ok(ok__) => {
                        ppszlocalename.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLeafConditionInfo<Identity: ICondition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropkey: *mut super::PROPERTYKEY, pcop: *mut CONDITION_OPERATION, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICondition2_Impl::GetLeafConditionInfo(this, core::mem::transmute_copy(&ppropkey), core::mem::transmute_copy(&pcop), core::mem::transmute_copy(&ppropvar)).into()
            }
        }
        Self {
            base__: ICondition_Vtbl::new::<Identity, OFFSET>(),
            GetLocale: GetLocale::<Identity, OFFSET>,
            GetLeafConditionInfo: GetLeafConditionInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICondition2 as windows_core::Interface>::IID || iid == &<super::IPersist as windows_core::Interface>::IID || iid == &<super::IPersistStream as windows_core::Interface>::IID || iid == &<ICondition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICondition2 {}
windows_core::imp::define_interface!(IRichChunk, IRichChunk_Vtbl, 0x4fdef69c_dbc9_454e_9910_b34f3c64b510);
windows_core::imp::interface_hierarchy!(IRichChunk, windows_core::IUnknown);
impl IRichChunk {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetData(&self, pfirstpos: Option<*mut u32>, plength: Option<*mut u32>, ppsz: *mut windows_core::PWSTR, pvalue: Option<*mut super::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pfirstpos.unwrap_or(core::mem::zeroed()) as _, plength.unwrap_or(core::mem::zeroed()) as _, ppsz as _, pvalue.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichChunk_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut windows_core::PWSTR, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetData: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRichChunk_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut windows_core::PWSTR, pvalue: *mut super::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IRichChunk_Vtbl {
    pub const fn new<Identity: IRichChunk_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: IRichChunk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut windows_core::PWSTR, pvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichChunk_Impl::GetData(this, core::mem::transmute_copy(&pfirstpos), core::mem::transmute_copy(&plength), core::mem::transmute_copy(&ppsz), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichChunk as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRichChunk {}
