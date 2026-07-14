pub type CASE_REQUIREMENT = i32;
pub const CASE_REQUIREMENT_ANY: CASE_REQUIREMENT = 0;
pub const CASE_REQUIREMENT_UPPER_IF_AQS: CASE_REQUIREMENT = 1;
pub const CONDITION_CREATION_DEFAULT: CONDITION_CREATION_OPTIONS = 0;
pub const CONDITION_CREATION_NONE: CONDITION_CREATION_OPTIONS = 0;
pub type CONDITION_CREATION_OPTIONS = u32;
pub const CONDITION_CREATION_SIMPLIFY: CONDITION_CREATION_OPTIONS = 1;
pub const CONDITION_CREATION_USE_CONTENT_LOCALE: CONDITION_CREATION_OPTIONS = 16;
pub const CONDITION_CREATION_VECTOR_AND: CONDITION_CREATION_OPTIONS = 2;
pub const CONDITION_CREATION_VECTOR_LEAF: CONDITION_CREATION_OPTIONS = 8;
pub const CONDITION_CREATION_VECTOR_OR: CONDITION_CREATION_OPTIONS = 4;
pub const CompoundCondition: windows_core::GUID = windows_core::GUID::from_u128(0x116f8d13_101e_4fa5_84d4_ff8279381935);
pub const ConditionFactory: windows_core::GUID = windows_core::GUID::from_u128(0xe03e85b0_7be3_4000_ba98_6c13de9fa486);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HITRANGE {
    pub iPosition: u32,
    pub cLength: u32,
}
windows_core::imp::define_interface!(IConditionFactory, IConditionFactory_Vtbl, 0xa5efe073_b16f_474f_9f3e_9f8b497a3e08);
windows_core::imp::interface_hierarchy!(IConditionFactory, windows_core::IUnknown);
impl IConditionFactory {
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub unsafe fn MakeNot<P0>(&self, pcsub: P0, fsimplify: bool) -> windows_core::Result<super::structuredquerycondition::ICondition>
    where
        P0: windows_core::Param<super::structuredquerycondition::ICondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MakeNot)(windows_core::Interface::as_raw(self), pcsub.param().abi(), fsimplify.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidl", feature = "objidlbase", feature = "structuredquerycondition"))]
    pub unsafe fn MakeAndOr<P1>(&self, ct: super::structuredquerycondition::CONDITION_TYPE, peusubs: P1, fsimplify: bool) -> windows_core::Result<super::structuredquerycondition::ICondition>
    where
        P1: windows_core::Param<super::objidlbase::IEnumUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MakeAndOr)(windows_core::Interface::as_raw(self), ct, peusubs.param().abi(), fsimplify.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn MakeLeaf<P0, P2, P4, P5, P6>(&self, pszpropertyname: P0, cop: super::structuredquerycondition::CONDITION_OPERATION, pszvaluetype: P2, ppropvar: *const super::propidlbase::PROPVARIANT, ppropertynameterm: P4, poperationterm: P5, pvalueterm: P6, fexpand: bool) -> windows_core::Result<super::structuredquerycondition::ICondition>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<super::structuredquerycondition::IRichChunk>,
        P5: windows_core::Param<super::structuredquerycondition::IRichChunk>,
        P6: windows_core::Param<super::structuredquerycondition::IRichChunk>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MakeLeaf)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), cop, pszvaluetype.param().abi(), ppropvar, ppropertynameterm.param().abi(), poperationterm.param().abi(), pvalueterm.param().abi(), fexpand.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwinbase", feature = "objidl", feature = "structuredquerycondition"))]
    pub unsafe fn Resolve<P0>(&self, pc: P0, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: Option<*const super::minwinbase::SYSTEMTIME>) -> windows_core::Result<super::structuredquerycondition::ICondition>
    where
        P0: windows_core::Param<super::structuredquerycondition::ICondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), pc.param().abi(), sqro, pstreferencetime.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub MakeNot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "structuredquerycondition")))]
    MakeNot: usize,
    #[cfg(all(feature = "objidl", feature = "objidlbase", feature = "structuredquerycondition"))]
    pub MakeAndOr: unsafe extern "system" fn(*mut core::ffi::c_void, super::structuredquerycondition::CONDITION_TYPE, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "objidlbase", feature = "structuredquerycondition")))]
    MakeAndOr: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
    pub MakeLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::structuredquerycondition::CONDITION_OPERATION, windows_core::PCWSTR, *const super::propidlbase::PROPVARIANT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase")))]
    MakeLeaf: usize,
    #[cfg(all(feature = "minwinbase", feature = "objidl", feature = "structuredquerycondition"))]
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, STRUCTURED_QUERY_RESOLVE_OPTION, *const super::minwinbase::SYSTEMTIME, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "objidl", feature = "structuredquerycondition")))]
    Resolve: usize,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
pub trait IConditionFactory_Impl: windows_core::IUnknownImpl {
    fn MakeNot(&self, pcsub: windows_core::Ref<super::structuredquerycondition::ICondition>, fsimplify: windows_core::BOOL) -> windows_core::Result<super::structuredquerycondition::ICondition>;
    fn MakeAndOr(&self, ct: super::structuredquerycondition::CONDITION_TYPE, peusubs: windows_core::Ref<super::objidlbase::IEnumUnknown>, fsimplify: windows_core::BOOL) -> windows_core::Result<super::structuredquerycondition::ICondition>;
    fn MakeLeaf(&self, pszpropertyname: &windows_core::PCWSTR, cop: super::structuredquerycondition::CONDITION_OPERATION, pszvaluetype: &windows_core::PCWSTR, ppropvar: *const super::propidlbase::PROPVARIANT, ppropertynameterm: windows_core::Ref<super::structuredquerycondition::IRichChunk>, poperationterm: windows_core::Ref<super::structuredquerycondition::IRichChunk>, pvalueterm: windows_core::Ref<super::structuredquerycondition::IRichChunk>, fexpand: windows_core::BOOL) -> windows_core::Result<super::structuredquerycondition::ICondition>;
    fn Resolve(&self, pc: windows_core::Ref<super::structuredquerycondition::ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::minwinbase::SYSTEMTIME) -> windows_core::Result<super::structuredquerycondition::ICondition>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl IConditionFactory_Vtbl {
    pub const fn new<Identity: IConditionFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MakeNot<Identity: IConditionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsub: *mut core::ffi::c_void, fsimplify: windows_core::BOOL, ppcresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConditionFactory_Impl::MakeNot(this, core::mem::transmute_copy(&pcsub), core::mem::transmute_copy(&fsimplify)) {
                    Ok(ok__) => {
                        ppcresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MakeAndOr<Identity: IConditionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: super::structuredquerycondition::CONDITION_TYPE, peusubs: *mut core::ffi::c_void, fsimplify: windows_core::BOOL, ppcresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConditionFactory_Impl::MakeAndOr(this, core::mem::transmute_copy(&ct), core::mem::transmute_copy(&peusubs), core::mem::transmute_copy(&fsimplify)) {
                    Ok(ok__) => {
                        ppcresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MakeLeaf<Identity: IConditionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, cop: super::structuredquerycondition::CONDITION_OPERATION, pszvaluetype: windows_core::PCWSTR, ppropvar: *const super::propidlbase::PROPVARIANT, ppropertynameterm: *mut core::ffi::c_void, poperationterm: *mut core::ffi::c_void, pvalueterm: *mut core::ffi::c_void, fexpand: windows_core::BOOL, ppcresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConditionFactory_Impl::MakeLeaf(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&cop), core::mem::transmute(&pszvaluetype), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&ppropertynameterm), core::mem::transmute_copy(&poperationterm), core::mem::transmute_copy(&pvalueterm), core::mem::transmute_copy(&fexpand)) {
                    Ok(ok__) => {
                        ppcresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resolve<Identity: IConditionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pc: *mut core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::minwinbase::SYSTEMTIME, ppcresolved: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConditionFactory_Impl::Resolve(this, core::mem::transmute_copy(&pc), core::mem::transmute_copy(&sqro), core::mem::transmute_copy(&pstreferencetime)) {
                    Ok(ok__) => {
                        ppcresolved.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MakeNot: MakeNot::<Identity, OFFSET>,
            MakeAndOr: MakeAndOr::<Identity, OFFSET>,
            MakeLeaf: MakeLeaf::<Identity, OFFSET>,
            Resolve: Resolve::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConditionFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IConditionFactory {}
windows_core::imp::define_interface!(IConditionFactory2, IConditionFactory2_Vtbl, 0x71d222e1_432f_429e_8c13_b6dafde5077a);
impl core::ops::Deref for IConditionFactory2 {
    type Target = IConditionFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConditionFactory2, windows_core::IUnknown, IConditionFactory);
impl IConditionFactory2 {
    pub unsafe fn CreateTrueFalse<T>(&self, fval: bool, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateTrueFalse)(windows_core::Interface::as_raw(self), fval.into(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub unsafe fn CreateNegation<P0, T>(&self, pcsub: P0, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::structuredquerycondition::ICondition>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateNegation)(windows_core::Interface::as_raw(self), pcsub.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "objectarray", feature = "structuredquerycondition"))]
    pub unsafe fn CreateCompoundFromObjectArray<P1, T>(&self, ct: super::structuredquerycondition::CONDITION_TYPE, poasubs: P1, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P1: windows_core::Param<super::objectarray::IObjectArray>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateCompoundFromObjectArray)(windows_core::Interface::as_raw(self), ct, poasubs.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub unsafe fn CreateCompoundFromArray<T>(&self, ct: super::structuredquerycondition::CONDITION_TYPE, ppcondsubs: *const Option<super::structuredquerycondition::ICondition>, csubs: u32, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateCompoundFromArray)(windows_core::Interface::as_raw(self), ct, core::mem::transmute(ppcondsubs), csubs, cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "structuredquerycondition", feature = "wtypes"))]
    pub unsafe fn CreateStringLeaf<P2, P3, T>(&self, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, pszvalue: P2, pszlocalename: P3, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateStringLeaf)(windows_core::Interface::as_raw(self), propkey, cop, pszvalue.param().abi(), pszlocalename.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "structuredquerycondition", feature = "wtypes"))]
    pub unsafe fn CreateIntegerLeaf<T>(&self, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateIntegerLeaf)(windows_core::Interface::as_raw(self), propkey, cop, lvalue, cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "structuredquerycondition", feature = "wtypes"))]
    pub unsafe fn CreateBooleanLeaf<T>(&self, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, fvalue: bool, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateBooleanLeaf)(windows_core::Interface::as_raw(self), propkey, cop, fvalue.into(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreateLeaf<P3, P4, P5, P6, P7, T>(&self, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, propvar: *const super::propidlbase::PROPVARIANT, pszsemantictype: P3, pszlocalename: P4, ppropertynameterm: P5, poperationterm: P6, pvalueterm: P7, cco: CONDITION_CREATION_OPTIONS) -> windows_core::Result<T>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<super::structuredquerycondition::IRichChunk>,
        P6: windows_core::Param<super::structuredquerycondition::IRichChunk>,
        P7: windows_core::Param<super::structuredquerycondition::IRichChunk>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateLeaf)(windows_core::Interface::as_raw(self), propkey, cop, propvar, pszsemantictype.param().abi(), pszlocalename.param().abi(), ppropertynameterm.param().abi(), poperationterm.param().abi(), pvalueterm.param().abi(), cco, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "minwinbase", feature = "objidl", feature = "structuredquerycondition"))]
    pub unsafe fn ResolveCondition<P0, T>(&self, pc: P0, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: Option<*const super::minwinbase::SYSTEMTIME>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::structuredquerycondition::ICondition>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ResolveCondition)(windows_core::Interface::as_raw(self), pc.param().abi(), sqro, pstreferencetime.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionFactory2_Vtbl {
    pub base__: IConditionFactory_Vtbl,
    pub CreateTrueFalse: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub CreateNegation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "structuredquerycondition")))]
    CreateNegation: usize,
    #[cfg(all(feature = "objectarray", feature = "structuredquerycondition"))]
    pub CreateCompoundFromObjectArray: unsafe extern "system" fn(*mut core::ffi::c_void, super::structuredquerycondition::CONDITION_TYPE, *mut core::ffi::c_void, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objectarray", feature = "structuredquerycondition")))]
    CreateCompoundFromObjectArray: usize,
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub CreateCompoundFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, super::structuredquerycondition::CONDITION_TYPE, *const *mut core::ffi::c_void, u32, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "structuredquerycondition")))]
    CreateCompoundFromArray: usize,
    #[cfg(all(feature = "structuredquerycondition", feature = "wtypes"))]
    pub CreateStringLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, super::structuredquerycondition::CONDITION_OPERATION, windows_core::PCWSTR, windows_core::PCWSTR, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "structuredquerycondition", feature = "wtypes")))]
    CreateStringLeaf: usize,
    #[cfg(all(feature = "structuredquerycondition", feature = "wtypes"))]
    pub CreateIntegerLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, super::structuredquerycondition::CONDITION_OPERATION, i32, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "structuredquerycondition", feature = "wtypes")))]
    CreateIntegerLeaf: usize,
    #[cfg(all(feature = "structuredquerycondition", feature = "wtypes"))]
    pub CreateBooleanLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, super::structuredquerycondition::CONDITION_OPERATION, windows_core::BOOL, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "structuredquerycondition", feature = "wtypes")))]
    CreateBooleanLeaf: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
    pub CreateLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, super::structuredquerycondition::CONDITION_OPERATION, *const super::propidlbase::PROPVARIANT, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CONDITION_CREATION_OPTIONS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase")))]
    CreateLeaf: usize,
    #[cfg(all(feature = "minwinbase", feature = "objidl", feature = "structuredquerycondition"))]
    pub ResolveCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, STRUCTURED_QUERY_RESOLVE_OPTION, *const super::minwinbase::SYSTEMTIME, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "objidl", feature = "structuredquerycondition")))]
    ResolveCondition: usize,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objectarray", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
pub trait IConditionFactory2_Impl: IConditionFactory_Impl {
    fn CreateTrueFalse(&self, fval: windows_core::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateNegation(&self, pcsub: windows_core::Ref<super::structuredquerycondition::ICondition>, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateCompoundFromObjectArray(&self, ct: super::structuredquerycondition::CONDITION_TYPE, poasubs: windows_core::Ref<super::objectarray::IObjectArray>, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateCompoundFromArray(&self, ct: super::structuredquerycondition::CONDITION_TYPE, ppcondsubs: *const Option<super::structuredquerycondition::ICondition>, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateStringLeaf(&self, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, pszvalue: &windows_core::PCWSTR, pszlocalename: &windows_core::PCWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateIntegerLeaf(&self, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateBooleanLeaf(&self, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, fvalue: windows_core::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateLeaf(&self, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, propvar: *const super::propidlbase::PROPVARIANT, pszsemantictype: &windows_core::PCWSTR, pszlocalename: &windows_core::PCWSTR, ppropertynameterm: windows_core::Ref<super::structuredquerycondition::IRichChunk>, poperationterm: windows_core::Ref<super::structuredquerycondition::IRichChunk>, pvalueterm: windows_core::Ref<super::structuredquerycondition::IRichChunk>, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ResolveCondition(&self, pc: windows_core::Ref<super::structuredquerycondition::ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::minwinbase::SYSTEMTIME, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objectarray", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl IConditionFactory2_Vtbl {
    pub const fn new<Identity: IConditionFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTrueFalse<Identity: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fval: windows_core::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionFactory2_Impl::CreateTrueFalse(this, core::mem::transmute_copy(&fval), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateNegation<Identity: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsub: *mut core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionFactory2_Impl::CreateNegation(this, core::mem::transmute_copy(&pcsub), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateCompoundFromObjectArray<Identity: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: super::structuredquerycondition::CONDITION_TYPE, poasubs: *mut core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionFactory2_Impl::CreateCompoundFromObjectArray(this, core::mem::transmute_copy(&ct), core::mem::transmute_copy(&poasubs), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateCompoundFromArray<Identity: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: super::structuredquerycondition::CONDITION_TYPE, ppcondsubs: *const *mut core::ffi::c_void, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionFactory2_Impl::CreateCompoundFromArray(this, core::mem::transmute_copy(&ct), core::mem::transmute_copy(&ppcondsubs), core::mem::transmute_copy(&csubs), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateStringLeaf<Identity: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, pszvalue: windows_core::PCWSTR, pszlocalename: windows_core::PCWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionFactory2_Impl::CreateStringLeaf(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&cop), core::mem::transmute(&pszvalue), core::mem::transmute(&pszlocalename), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateIntegerLeaf<Identity: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionFactory2_Impl::CreateIntegerLeaf(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&cop), core::mem::transmute_copy(&lvalue), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateBooleanLeaf<Identity: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, fvalue: windows_core::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionFactory2_Impl::CreateBooleanLeaf(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&cop), core::mem::transmute_copy(&fvalue), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateLeaf<Identity: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const super::wtypes::PROPERTYKEY, cop: super::structuredquerycondition::CONDITION_OPERATION, propvar: *const super::propidlbase::PROPVARIANT, pszsemantictype: windows_core::PCWSTR, pszlocalename: windows_core::PCWSTR, ppropertynameterm: *mut core::ffi::c_void, poperationterm: *mut core::ffi::c_void, pvalueterm: *mut core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionFactory2_Impl::CreateLeaf(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&cop), core::mem::transmute_copy(&propvar), core::mem::transmute(&pszsemantictype), core::mem::transmute(&pszlocalename), core::mem::transmute_copy(&ppropertynameterm), core::mem::transmute_copy(&poperationterm), core::mem::transmute_copy(&pvalueterm), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn ResolveCondition<Identity: IConditionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pc: *mut core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::minwinbase::SYSTEMTIME, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionFactory2_Impl::ResolveCondition(this, core::mem::transmute_copy(&pc), core::mem::transmute_copy(&sqro), core::mem::transmute_copy(&pstreferencetime), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: IConditionFactory_Vtbl::new::<Identity, OFFSET>(),
            CreateTrueFalse: CreateTrueFalse::<Identity, OFFSET>,
            CreateNegation: CreateNegation::<Identity, OFFSET>,
            CreateCompoundFromObjectArray: CreateCompoundFromObjectArray::<Identity, OFFSET>,
            CreateCompoundFromArray: CreateCompoundFromArray::<Identity, OFFSET>,
            CreateStringLeaf: CreateStringLeaf::<Identity, OFFSET>,
            CreateIntegerLeaf: CreateIntegerLeaf::<Identity, OFFSET>,
            CreateBooleanLeaf: CreateBooleanLeaf::<Identity, OFFSET>,
            CreateLeaf: CreateLeaf::<Identity, OFFSET>,
            ResolveCondition: ResolveCondition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConditionFactory2 as windows_core::Interface>::IID || iid == &<IConditionFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objectarray", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IConditionFactory2 {}
windows_core::imp::define_interface!(IConditionGenerator, IConditionGenerator_Vtbl, 0x92d2cc58_4386_45a3_b98c_7e0ce64a4117);
windows_core::imp::interface_hierarchy!(IConditionGenerator, windows_core::IUnknown);
impl IConditionGenerator {
    pub unsafe fn Initialize<P0>(&self, pschemaprovider: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISchemaProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pschemaprovider.param().abi()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn RecognizeNamedEntities<P0, P2>(&self, pszinputstring: P0, lciduserlocale: super::winnt::LCID, ptokencollection: P2, pnamedentities: &Option<INamedEntityCollector>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<ITokenCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).RecognizeNamedEntities)(windows_core::Interface::as_raw(self), pszinputstring.param().abi(), lciduserlocale, ptokencollection.param().abi(), core::mem::transmute_copy(pnamedentities)) }
    }
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub unsafe fn GenerateForLeaf<P0, P1, P3, P4, P5, P6, P7, P8>(&self, pconditionfactory: P0, pszpropertyname: P1, cop: super::structuredquerycondition::CONDITION_OPERATION, pszvaluetype: P3, pszvalue: P4, pszvalue2: P5, ppropertynameterm: P6, poperationterm: P7, pvalueterm: P8, automaticwildcard: bool, pnostringquery: *mut windows_core::BOOL) -> windows_core::Result<super::structuredquerycondition::ICondition>
    where
        P0: windows_core::Param<IConditionFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<super::structuredquerycondition::IRichChunk>,
        P7: windows_core::Param<super::structuredquerycondition::IRichChunk>,
        P8: windows_core::Param<super::structuredquerycondition::IRichChunk>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateForLeaf)(windows_core::Interface::as_raw(self), pconditionfactory.param().abi(), pszpropertyname.param().abi(), cop, pszvaluetype.param().abi(), pszvalue.param().abi(), pszvalue2.param().abi(), ppropertynameterm.param().abi(), poperationterm.param().abi(), pvalueterm.param().abi(), automaticwildcard.into(), pnostringquery as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn DefaultPhrase<P0>(&self, pszvaluetype: P0, ppropvar: *const super::propidlbase::PROPVARIANT, fuseenglish: bool) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultPhrase)(windows_core::Interface::as_raw(self), pszvaluetype.param().abi(), ppropvar, fuseenglish.into(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionGenerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub RecognizeNamedEntities: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::winnt::LCID, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    RecognizeNamedEntities: usize,
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub GenerateForLeaf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, super::structuredquerycondition::CONDITION_OPERATION, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "structuredquerycondition")))]
    GenerateForLeaf: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub DefaultPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::propidlbase::PROPVARIANT, windows_core::BOOL, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    DefaultPhrase: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IConditionGenerator_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pschemaprovider: windows_core::Ref<ISchemaProvider>) -> windows_core::Result<()>;
    fn RecognizeNamedEntities(&self, pszinputstring: &windows_core::PCWSTR, lciduserlocale: super::winnt::LCID, ptokencollection: windows_core::Ref<ITokenCollection>, pnamedentities: windows_core::OutRef<INamedEntityCollector>) -> windows_core::Result<()>;
    fn GenerateForLeaf(&self, pconditionfactory: windows_core::Ref<IConditionFactory>, pszpropertyname: &windows_core::PCWSTR, cop: super::structuredquerycondition::CONDITION_OPERATION, pszvaluetype: &windows_core::PCWSTR, pszvalue: &windows_core::PCWSTR, pszvalue2: &windows_core::PCWSTR, ppropertynameterm: windows_core::Ref<super::structuredquerycondition::IRichChunk>, poperationterm: windows_core::Ref<super::structuredquerycondition::IRichChunk>, pvalueterm: windows_core::Ref<super::structuredquerycondition::IRichChunk>, automaticwildcard: windows_core::BOOL, pnostringquery: *mut windows_core::BOOL) -> windows_core::Result<super::structuredquerycondition::ICondition>;
    fn DefaultPhrase(&self, pszvaluetype: &windows_core::PCWSTR, ppropvar: *const super::propidlbase::PROPVARIANT, fuseenglish: windows_core::BOOL) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IConditionGenerator_Vtbl {
    pub const fn new<Identity: IConditionGenerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pschemaprovider: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionGenerator_Impl::Initialize(this, core::mem::transmute_copy(&pschemaprovider)).into()
            }
        }
        unsafe extern "system" fn RecognizeNamedEntities<Identity: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinputstring: windows_core::PCWSTR, lciduserlocale: super::winnt::LCID, ptokencollection: *mut core::ffi::c_void, pnamedentities: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConditionGenerator_Impl::RecognizeNamedEntities(this, core::mem::transmute(&pszinputstring), core::mem::transmute_copy(&lciduserlocale), core::mem::transmute_copy(&ptokencollection), core::mem::transmute(&pnamedentities)).into()
            }
        }
        unsafe extern "system" fn GenerateForLeaf<Identity: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconditionfactory: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, cop: super::structuredquerycondition::CONDITION_OPERATION, pszvaluetype: windows_core::PCWSTR, pszvalue: windows_core::PCWSTR, pszvalue2: windows_core::PCWSTR, ppropertynameterm: *mut core::ffi::c_void, poperationterm: *mut core::ffi::c_void, pvalueterm: *mut core::ffi::c_void, automaticwildcard: windows_core::BOOL, pnostringquery: *mut windows_core::BOOL, ppqueryexpression: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConditionGenerator_Impl::GenerateForLeaf(this, core::mem::transmute_copy(&pconditionfactory), core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&cop), core::mem::transmute(&pszvaluetype), core::mem::transmute(&pszvalue), core::mem::transmute(&pszvalue2), core::mem::transmute_copy(&ppropertynameterm), core::mem::transmute_copy(&poperationterm), core::mem::transmute_copy(&pvalueterm), core::mem::transmute_copy(&automaticwildcard), core::mem::transmute_copy(&pnostringquery)) {
                    Ok(ok__) => {
                        ppqueryexpression.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: IConditionGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvaluetype: windows_core::PCWSTR, ppropvar: *const super::propidlbase::PROPVARIANT, fuseenglish: windows_core::BOOL, ppszphrase: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConditionGenerator_Impl::DefaultPhrase(this, core::mem::transmute(&pszvaluetype), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&fuseenglish)) {
                    Ok(ok__) => {
                        ppszphrase.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            RecognizeNamedEntities: RecognizeNamedEntities::<Identity, OFFSET>,
            GenerateForLeaf: GenerateForLeaf::<Identity, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConditionGenerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IConditionGenerator {}
windows_core::imp::define_interface!(IEntity, IEntity_Vtbl, 0x24264891_e80b_4fd3_b7ce_4ff2fae8931f);
windows_core::imp::interface_hierarchy!(IEntity, windows_core::IUnknown);
impl IEntity {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Base(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Base)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Relationships<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Relationships)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetRelationship<P0>(&self, pszrelationname: P0) -> windows_core::Result<IRelationship>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelationship)(windows_core::Interface::as_raw(self), pszrelationname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MetaData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).MetaData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn NamedEntities<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).NamedEntities)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetNamedEntity<P0>(&self, pszvalue: P0) -> windows_core::Result<INamedEntity>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNamedEntity)(windows_core::Interface::as_raw(self), pszvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DefaultPhrase(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultPhrase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Base: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Relationships: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRelationship: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamedEntities: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamedEntity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefaultPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IEntity_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Base(&self) -> windows_core::Result<IEntity>;
    fn Relationships(&self, riid: *const windows_core::GUID, prelationships: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetRelationship(&self, pszrelationname: &windows_core::PCWSTR) -> windows_core::Result<IRelationship>;
    fn MetaData(&self, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn NamedEntities(&self, riid: *const windows_core::GUID, pnamedentities: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetNamedEntity(&self, pszvalue: &windows_core::PCWSTR) -> windows_core::Result<INamedEntity>;
    fn DefaultPhrase(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IEntity_Vtbl {
    pub const fn new<Identity: IEntity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEntity_Impl::Name(this) {
                    Ok(ok__) => {
                        ppszname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Base<Identity: IEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbaseentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEntity_Impl::Base(this) {
                    Ok(ok__) => {
                        pbaseentity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Relationships<Identity: IEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, prelationships: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEntity_Impl::Relationships(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&prelationships)).into()
            }
        }
        unsafe extern "system" fn GetRelationship<Identity: IEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrelationname: windows_core::PCWSTR, prelationship: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEntity_Impl::GetRelationship(this, core::mem::transmute(&pszrelationname)) {
                    Ok(ok__) => {
                        prelationship.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MetaData<Identity: IEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEntity_Impl::MetaData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pmetadata)).into()
            }
        }
        unsafe extern "system" fn NamedEntities<Identity: IEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pnamedentities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEntity_Impl::NamedEntities(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pnamedentities)).into()
            }
        }
        unsafe extern "system" fn GetNamedEntity<Identity: IEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: windows_core::PCWSTR, ppnamedentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEntity_Impl::GetNamedEntity(this, core::mem::transmute(&pszvalue)) {
                    Ok(ok__) => {
                        ppnamedentity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: IEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszphrase: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEntity_Impl::DefaultPhrase(this) {
                    Ok(ok__) => {
                        ppszphrase.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Base: Base::<Identity, OFFSET>,
            Relationships: Relationships::<Identity, OFFSET>,
            GetRelationship: GetRelationship::<Identity, OFFSET>,
            MetaData: MetaData::<Identity, OFFSET>,
            NamedEntities: NamedEntities::<Identity, OFFSET>,
            GetNamedEntity: GetNamedEntity::<Identity, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEntity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEntity {}
windows_core::imp::define_interface!(IInterval, IInterval_Vtbl, 0x6bf0a714_3c18_430b_8b5d_83b1c234d3db);
windows_core::imp::interface_hierarchy!(IInterval, windows_core::IUnknown);
impl IInterval {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetLimits(&self, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::propidlbase::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLimits)(windows_core::Interface::as_raw(self), pilklower as _, ppropvarlower, pilkupper as _, ppropvarupper) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInterval_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetLimits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut INTERVAL_LIMIT_KIND, *mut super::propidlbase::PROPVARIANT, *mut INTERVAL_LIMIT_KIND, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetLimits: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInterval_Impl: windows_core::IUnknownImpl {
    fn GetLimits(&self, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::propidlbase::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IInterval_Vtbl {
    pub const fn new<Identity: IInterval_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLimits<Identity: IInterval_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::propidlbase::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInterval_Impl::GetLimits(this, core::mem::transmute_copy(&pilklower), core::mem::transmute_copy(&ppropvarlower), core::mem::transmute_copy(&pilkupper), core::mem::transmute_copy(&ppropvarupper)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetLimits: GetLimits::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInterval as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IInterval {}
pub const ILK_EXPLICIT_EXCLUDED: INTERVAL_LIMIT_KIND = 1;
pub const ILK_EXPLICIT_INCLUDED: INTERVAL_LIMIT_KIND = 0;
pub const ILK_NEGATIVE_INFINITY: INTERVAL_LIMIT_KIND = 2;
pub const ILK_POSITIVE_INFINITY: INTERVAL_LIMIT_KIND = 3;
windows_core::imp::define_interface!(IMetaData, IMetaData_Vtbl, 0x780102b0_c43b_4876_bc7b_5e9ba5c88794);
windows_core::imp::interface_hierarchy!(IMetaData, windows_core::IUnknown);
impl IMetaData {
    pub unsafe fn GetData(&self, ppszkey: *mut windows_core::PWSTR, ppszvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), ppszkey as _, ppszvalue as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMetaData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IMetaData_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, ppszkey: *mut windows_core::PWSTR, ppszvalue: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl IMetaData_Vtbl {
    pub const fn new<Identity: IMetaData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: IMetaData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszkey: *mut windows_core::PWSTR, ppszvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMetaData_Impl::GetData(this, core::mem::transmute_copy(&ppszkey), core::mem::transmute_copy(&ppszvalue)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMetaData {}
pub type INTERVAL_LIMIT_KIND = i32;
windows_core::imp::define_interface!(INamedEntity, INamedEntity_Vtbl, 0xabdbd0b1_7d54_49fb_ab5c_bff4130004cd);
windows_core::imp::interface_hierarchy!(INamedEntity, windows_core::IUnknown);
impl INamedEntity {
    pub unsafe fn GetValue(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DefaultPhrase(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultPhrase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedEntity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub DefaultPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait INamedEntity_Impl: windows_core::IUnknownImpl {
    fn GetValue(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn DefaultPhrase(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl INamedEntity_Vtbl {
    pub const fn new<Identity: INamedEntity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetValue<Identity: INamedEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INamedEntity_Impl::GetValue(this) {
                    Ok(ok__) => {
                        ppszvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: INamedEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszphrase: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INamedEntity_Impl::DefaultPhrase(this) {
                    Ok(ok__) => {
                        ppszphrase.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetValue: GetValue::<Identity, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INamedEntity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INamedEntity {}
windows_core::imp::define_interface!(INamedEntityCollector, INamedEntityCollector_Vtbl, 0xaf2440f6_8afc_47d0_9a7f_396a0acfb43d);
windows_core::imp::interface_hierarchy!(INamedEntityCollector, windows_core::IUnknown);
impl INamedEntityCollector {
    pub unsafe fn Add<P4, P5>(&self, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: P4, pszvalue: P5, certainty: NAMED_ENTITY_CERTAINTY) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IEntity>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), beginspan, endspan, beginactual, endactual, ptype.param().abi(), pszvalue.param().abi(), certainty) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedEntityCollector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void, windows_core::PCWSTR, NAMED_ENTITY_CERTAINTY) -> windows_core::HRESULT,
}
pub trait INamedEntityCollector_Impl: windows_core::IUnknownImpl {
    fn Add(&self, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: windows_core::Ref<IEntity>, pszvalue: &windows_core::PCWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> windows_core::Result<()>;
}
impl INamedEntityCollector_Vtbl {
    pub const fn new<Identity: INamedEntityCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: INamedEntityCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: *mut core::ffi::c_void, pszvalue: windows_core::PCWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INamedEntityCollector_Impl::Add(this, core::mem::transmute_copy(&beginspan), core::mem::transmute_copy(&endspan), core::mem::transmute_copy(&beginactual), core::mem::transmute_copy(&endactual), core::mem::transmute_copy(&ptype), core::mem::transmute(&pszvalue), core::mem::transmute_copy(&certainty)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INamedEntityCollector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INamedEntityCollector {}
windows_core::imp::define_interface!(IQueryParser, IQueryParser_Vtbl, 0x2ebdee67_3505_43f8_9946_ea44abc8e5b0);
windows_core::imp::interface_hierarchy!(IQueryParser, windows_core::IUnknown);
impl IQueryParser {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Parse<P0, P1>(&self, pszinputstring: P0, pcustomproperties: P1) -> windows_core::Result<IQuerySolution>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::objidlbase::IEnumUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parse)(windows_core::Interface::as_raw(self), pszinputstring.param().abi(), pcustomproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), option, poptionvalue) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOption)(windows_core::Interface::as_raw(self), option, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetMultiOption<P1>(&self, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: P1, poptionvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMultiOption)(windows_core::Interface::as_raw(self), option, pszoptionkey.param().abi(), poptionvalue) }
    }
    pub unsafe fn GetSchemaProvider(&self) -> windows_core::Result<ISchemaProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSchemaProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub unsafe fn RestateToString<P0>(&self, pcondition: P0, fuseenglish: bool) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<super::structuredquerycondition::ICondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RestateToString)(windows_core::Interface::as_raw(self), pcondition.param().abi(), fuseenglish.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ParsePropertyValue<P0, P1>(&self, pszpropertyname: P0, pszinputstring: P1) -> windows_core::Result<IQuerySolution>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParsePropertyValue)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), pszinputstring.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub unsafe fn RestatePropertyValueToString<P0>(&self, pcondition: P0, fuseenglish: bool, ppszpropertyname: *mut windows_core::PWSTR, ppszquerystring: *mut windows_core::PWSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::structuredquerycondition::ICondition>,
    {
        unsafe { (windows_core::Interface::vtable(self).RestatePropertyValueToString)(windows_core::Interface::as_raw(self), pcondition.param().abi(), fuseenglish.into(), ppszpropertyname as _, ppszquerystring as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryParser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Parse: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, STRUCTURED_QUERY_SINGLE_OPTION, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetOption: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetOption: unsafe extern "system" fn(*mut core::ffi::c_void, STRUCTURED_QUERY_SINGLE_OPTION, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetOption: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetMultiOption: unsafe extern "system" fn(*mut core::ffi::c_void, STRUCTURED_QUERY_MULTIOPTION, windows_core::PCWSTR, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetMultiOption: usize,
    pub GetSchemaProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub RestateToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "structuredquerycondition")))]
    RestateToString: usize,
    pub ParsePropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub RestatePropertyValueToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "structuredquerycondition")))]
    RestatePropertyValueToString: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
pub trait IQueryParser_Impl: windows_core::IUnknownImpl {
    fn Parse(&self, pszinputstring: &windows_core::PCWSTR, pcustomproperties: windows_core::Ref<super::objidlbase::IEnumUnknown>) -> windows_core::Result<IQuerySolution>;
    fn SetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn GetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn SetMultiOption(&self, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: &windows_core::PCWSTR, poptionvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn GetSchemaProvider(&self) -> windows_core::Result<ISchemaProvider>;
    fn RestateToString(&self, pcondition: windows_core::Ref<super::structuredquerycondition::ICondition>, fuseenglish: windows_core::BOOL) -> windows_core::Result<windows_core::PWSTR>;
    fn ParsePropertyValue(&self, pszpropertyname: &windows_core::PCWSTR, pszinputstring: &windows_core::PCWSTR) -> windows_core::Result<IQuerySolution>;
    fn RestatePropertyValueToString(&self, pcondition: windows_core::Ref<super::structuredquerycondition::ICondition>, fuseenglish: windows_core::BOOL, ppszpropertyname: *mut windows_core::PWSTR, ppszquerystring: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl IQueryParser_Vtbl {
    pub const fn new<Identity: IQueryParser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Parse<Identity: IQueryParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinputstring: windows_core::PCWSTR, pcustomproperties: *mut core::ffi::c_void, ppsolution: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryParser_Impl::Parse(this, core::mem::transmute(&pszinputstring), core::mem::transmute_copy(&pcustomproperties)) {
                    Ok(ok__) => {
                        ppsolution.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOption<Identity: IQueryParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryParser_Impl::SetOption(this, core::mem::transmute_copy(&option), core::mem::transmute_copy(&poptionvalue)).into()
            }
        }
        unsafe extern "system" fn GetOption<Identity: IQueryParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryParser_Impl::GetOption(this, core::mem::transmute_copy(&option)) {
                    Ok(ok__) => {
                        poptionvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMultiOption<Identity: IQueryParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: windows_core::PCWSTR, poptionvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryParser_Impl::SetMultiOption(this, core::mem::transmute_copy(&option), core::mem::transmute(&pszoptionkey), core::mem::transmute_copy(&poptionvalue)).into()
            }
        }
        unsafe extern "system" fn GetSchemaProvider<Identity: IQueryParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppschemaprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryParser_Impl::GetSchemaProvider(this) {
                    Ok(ok__) => {
                        ppschemaprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestateToString<Identity: IQueryParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcondition: *mut core::ffi::c_void, fuseenglish: windows_core::BOOL, ppszquerystring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryParser_Impl::RestateToString(this, core::mem::transmute_copy(&pcondition), core::mem::transmute_copy(&fuseenglish)) {
                    Ok(ok__) => {
                        ppszquerystring.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParsePropertyValue<Identity: IQueryParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, pszinputstring: windows_core::PCWSTR, ppsolution: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryParser_Impl::ParsePropertyValue(this, core::mem::transmute(&pszpropertyname), core::mem::transmute(&pszinputstring)) {
                    Ok(ok__) => {
                        ppsolution.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestatePropertyValueToString<Identity: IQueryParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcondition: *mut core::ffi::c_void, fuseenglish: windows_core::BOOL, ppszpropertyname: *mut windows_core::PWSTR, ppszquerystring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryParser_Impl::RestatePropertyValueToString(this, core::mem::transmute_copy(&pcondition), core::mem::transmute_copy(&fuseenglish), core::mem::transmute_copy(&ppszpropertyname), core::mem::transmute_copy(&ppszquerystring)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Parse: Parse::<Identity, OFFSET>,
            SetOption: SetOption::<Identity, OFFSET>,
            GetOption: GetOption::<Identity, OFFSET>,
            SetMultiOption: SetMultiOption::<Identity, OFFSET>,
            GetSchemaProvider: GetSchemaProvider::<Identity, OFFSET>,
            RestateToString: RestateToString::<Identity, OFFSET>,
            ParsePropertyValue: ParsePropertyValue::<Identity, OFFSET>,
            RestatePropertyValueToString: RestatePropertyValueToString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryParser as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IQueryParser {}
windows_core::imp::define_interface!(IQueryParserManager, IQueryParserManager_Vtbl, 0xa879e3c4_af77_44fb_8f37_ebd1487cf920);
windows_core::imp::interface_hierarchy!(IQueryParserManager, windows_core::IUnknown);
impl IQueryParserManager {
    #[cfg(feature = "winnt")]
    pub unsafe fn CreateLoadedParser<P0, T>(&self, pszcatalog: P0, langidforkeywords: super::winnt::LANGID) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateLoadedParser)(windows_core::Interface::as_raw(self), pszcatalog.param().abi(), langidforkeywords, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn InitializeOptions<P2>(&self, funderstandnqs: bool, fautowildcard: bool, pqueryparser: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IQueryParser>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeOptions)(windows_core::Interface::as_raw(self), funderstandnqs.into(), fautowildcard.into(), pqueryparser.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetOption(&self, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), option, poptionvalue) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryParserManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub CreateLoadedParser: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::winnt::LANGID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    CreateLoadedParser: usize,
    pub InitializeOptions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, QUERY_PARSER_MANAGER_OPTION, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetOption: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IQueryParserManager_Impl: windows_core::IUnknownImpl {
    fn CreateLoadedParser(&self, pszcatalog: &windows_core::PCWSTR, langidforkeywords: super::winnt::LANGID, riid: *const windows_core::GUID, ppqueryparser: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn InitializeOptions(&self, funderstandnqs: windows_core::BOOL, fautowildcard: windows_core::BOOL, pqueryparser: windows_core::Ref<IQueryParser>) -> windows_core::Result<()>;
    fn SetOption(&self, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IQueryParserManager_Vtbl {
    pub const fn new<Identity: IQueryParserManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateLoadedParser<Identity: IQueryParserManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcatalog: windows_core::PCWSTR, langidforkeywords: super::winnt::LANGID, riid: *const windows_core::GUID, ppqueryparser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryParserManager_Impl::CreateLoadedParser(this, core::mem::transmute(&pszcatalog), core::mem::transmute_copy(&langidforkeywords), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppqueryparser)).into()
            }
        }
        unsafe extern "system" fn InitializeOptions<Identity: IQueryParserManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, funderstandnqs: windows_core::BOOL, fautowildcard: windows_core::BOOL, pqueryparser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryParserManager_Impl::InitializeOptions(this, core::mem::transmute_copy(&funderstandnqs), core::mem::transmute_copy(&fautowildcard), core::mem::transmute_copy(&pqueryparser)).into()
            }
        }
        unsafe extern "system" fn SetOption<Identity: IQueryParserManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryParserManager_Impl::SetOption(this, core::mem::transmute_copy(&option), core::mem::transmute_copy(&poptionvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateLoadedParser: CreateLoadedParser::<Identity, OFFSET>,
            InitializeOptions: InitializeOptions::<Identity, OFFSET>,
            SetOption: SetOption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryParserManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IQueryParserManager {}
windows_core::imp::define_interface!(IQuerySolution, IQuerySolution_Vtbl, 0xd6ebc66b_8921_4193_afdd_a1789fb7ff57);
impl core::ops::Deref for IQuerySolution {
    type Target = IConditionFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IQuerySolution, windows_core::IUnknown, IConditionFactory);
impl IQuerySolution {
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub unsafe fn GetQuery(&self, ppquerynode: Option<*mut Option<super::structuredquerycondition::ICondition>>, ppmaintype: Option<*mut Option<IEntity>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetQuery)(windows_core::Interface::as_raw(self), ppquerynode.unwrap_or(core::mem::zeroed()) as _, ppmaintype.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetErrors<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetErrors)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLexicalData(&self, ppszinputstring: *mut windows_core::PWSTR, pptokens: Option<*mut Option<ITokenCollection>>, plcid: Option<*mut super::winnt::LCID>, ppwordbreaker: Option<*mut Option<windows_core::IUnknown>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLexicalData)(windows_core::Interface::as_raw(self), ppszinputstring as _, pptokens.unwrap_or(core::mem::zeroed()) as _, plcid.unwrap_or(core::mem::zeroed()) as _, ppwordbreaker.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuerySolution_Vtbl {
    pub base__: IConditionFactory_Vtbl,
    #[cfg(all(feature = "objidl", feature = "structuredquerycondition"))]
    pub GetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "structuredquerycondition")))]
    GetQuery: usize,
    pub GetErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetLexicalData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut *mut core::ffi::c_void, *mut super::winnt::LCID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLexicalData: usize,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IQuerySolution_Impl: IConditionFactory_Impl {
    fn GetQuery(&self, ppquerynode: windows_core::OutRef<super::structuredquerycondition::ICondition>, ppmaintype: windows_core::OutRef<IEntity>) -> windows_core::Result<()>;
    fn GetErrors(&self, riid: *const windows_core::GUID, ppparseerrors: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetLexicalData(&self, ppszinputstring: *mut windows_core::PWSTR, pptokens: windows_core::OutRef<ITokenCollection>, plcid: *mut super::winnt::LCID, ppwordbreaker: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IQuerySolution_Vtbl {
    pub const fn new<Identity: IQuerySolution_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetQuery<Identity: IQuerySolution_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppquerynode: *mut *mut core::ffi::c_void, ppmaintype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQuerySolution_Impl::GetQuery(this, core::mem::transmute_copy(&ppquerynode), core::mem::transmute_copy(&ppmaintype)).into()
            }
        }
        unsafe extern "system" fn GetErrors<Identity: IQuerySolution_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparseerrors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQuerySolution_Impl::GetErrors(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparseerrors)).into()
            }
        }
        unsafe extern "system" fn GetLexicalData<Identity: IQuerySolution_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszinputstring: *mut windows_core::PWSTR, pptokens: *mut *mut core::ffi::c_void, plcid: *mut super::winnt::LCID, ppwordbreaker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQuerySolution_Impl::GetLexicalData(this, core::mem::transmute_copy(&ppszinputstring), core::mem::transmute_copy(&pptokens), core::mem::transmute_copy(&plcid), core::mem::transmute_copy(&ppwordbreaker)).into()
            }
        }
        Self {
            base__: IConditionFactory_Vtbl::new::<Identity, OFFSET>(),
            GetQuery: GetQuery::<Identity, OFFSET>,
            GetErrors: GetErrors::<Identity, OFFSET>,
            GetLexicalData: GetLexicalData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQuerySolution as windows_core::Interface>::IID || iid == &<IConditionFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "structuredquerycondition", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IQuerySolution {}
windows_core::imp::define_interface!(IRelationship, IRelationship_Vtbl, 0x2769280b_5108_498c_9c7f_a51239b63147);
windows_core::imp::interface_hierarchy!(IRelationship, windows_core::IUnknown);
impl IRelationship {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsReal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsReal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Destination(&self) -> windows_core::Result<IEntity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Destination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MetaData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).MetaData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn DefaultPhrase(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultPhrase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelationship_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsReal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Destination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefaultPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IRelationship_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsReal(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Destination(&self) -> windows_core::Result<IEntity>;
    fn MetaData(&self, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DefaultPhrase(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IRelationship_Vtbl {
    pub const fn new<Identity: IRelationship_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRelationship_Impl::Name(this) {
                    Ok(ok__) => {
                        ppszname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsReal<Identity: IRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisreal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRelationship_Impl::IsReal(this) {
                    Ok(ok__) => {
                        pisreal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Destination<Identity: IRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRelationship_Impl::Destination(this) {
                    Ok(ok__) => {
                        pdestinationentity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MetaData<Identity: IRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRelationship_Impl::MetaData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pmetadata)).into()
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: IRelationship_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszphrase: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRelationship_Impl::DefaultPhrase(this) {
                    Ok(ok__) => {
                        ppszphrase.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            IsReal: IsReal::<Identity, OFFSET>,
            Destination: Destination::<Identity, OFFSET>,
            MetaData: MetaData::<Identity, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRelationship as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRelationship {}
windows_core::imp::define_interface!(ISchemaLocalizerSupport, ISchemaLocalizerSupport_Vtbl, 0xca3fdca2_bfbe_4eed_90d7_0caef0a1bda1);
windows_core::imp::interface_hierarchy!(ISchemaLocalizerSupport, windows_core::IUnknown);
impl ISchemaLocalizerSupport {
    pub unsafe fn Localize<P0>(&self, pszglobalstring: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Localize)(windows_core::Interface::as_raw(self), pszglobalstring.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaLocalizerSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Localize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ISchemaLocalizerSupport_Impl: windows_core::IUnknownImpl {
    fn Localize(&self, pszglobalstring: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl ISchemaLocalizerSupport_Vtbl {
    pub const fn new<Identity: ISchemaLocalizerSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Localize<Identity: ISchemaLocalizerSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszglobalstring: windows_core::PCWSTR, ppszlocalstring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaLocalizerSupport_Impl::Localize(this, core::mem::transmute(&pszglobalstring)) {
                    Ok(ok__) => {
                        ppszlocalstring.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Localize: Localize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaLocalizerSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISchemaLocalizerSupport {}
windows_core::imp::define_interface!(ISchemaProvider, ISchemaProvider_Vtbl, 0x8cf89bcb_394c_49b2_ae28_a59dd4ed7f68);
windows_core::imp::interface_hierarchy!(ISchemaProvider, windows_core::IUnknown);
impl ISchemaProvider {
    pub unsafe fn Entities<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Entities)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RootEntity(&self) -> windows_core::Result<IEntity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RootEntity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEntity<P0>(&self, pszentityname: P0) -> windows_core::Result<IEntity>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntity)(windows_core::Interface::as_raw(self), pszentityname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MetaData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).MetaData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn Localize<P1>(&self, lcid: super::winnt::LCID, pschemalocalizersupport: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ISchemaLocalizerSupport>,
    {
        unsafe { (windows_core::Interface::vtable(self).Localize)(windows_core::Interface::as_raw(self), lcid, pschemalocalizersupport.param().abi()) }
    }
    pub unsafe fn SaveBinary<P0>(&self, pszschemabinarypath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveBinary)(windows_core::Interface::as_raw(self), pszschemabinarypath.param().abi()) }
    }
    pub unsafe fn LookupAuthoredNamedEntity<P0, P1, P2>(&self, pentity: P0, pszinputstring: P1, ptokencollection: P2, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IEntity>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<ITokenCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).LookupAuthoredNamedEntity)(windows_core::Interface::as_raw(self), pentity.param().abi(), pszinputstring.param().abi(), ptokencollection.param().abi(), ctokensbegin, pctokenslength as _, ppszvalue as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Entities: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RootEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEntity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MetaData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub Localize: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Localize: usize,
    pub SaveBinary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub LookupAuthoredNamedEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait ISchemaProvider_Impl: windows_core::IUnknownImpl {
    fn Entities(&self, riid: *const windows_core::GUID, pentities: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RootEntity(&self) -> windows_core::Result<IEntity>;
    fn GetEntity(&self, pszentityname: &windows_core::PCWSTR) -> windows_core::Result<IEntity>;
    fn MetaData(&self, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Localize(&self, lcid: super::winnt::LCID, pschemalocalizersupport: windows_core::Ref<ISchemaLocalizerSupport>) -> windows_core::Result<()>;
    fn SaveBinary(&self, pszschemabinarypath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LookupAuthoredNamedEntity(&self, pentity: windows_core::Ref<IEntity>, pszinputstring: &windows_core::PCWSTR, ptokencollection: windows_core::Ref<ITokenCollection>, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl ISchemaProvider_Vtbl {
    pub const fn new<Identity: ISchemaProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Entities<Identity: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pentities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchemaProvider_Impl::Entities(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pentities)).into()
            }
        }
        unsafe extern "system" fn RootEntity<Identity: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prootentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaProvider_Impl::RootEntity(this) {
                    Ok(ok__) => {
                        prootentity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEntity<Identity: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszentityname: windows_core::PCWSTR, pentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaProvider_Impl::GetEntity(this, core::mem::transmute(&pszentityname)) {
                    Ok(ok__) => {
                        pentity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MetaData<Identity: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchemaProvider_Impl::MetaData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pmetadata)).into()
            }
        }
        unsafe extern "system" fn Localize<Identity: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::winnt::LCID, pschemalocalizersupport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchemaProvider_Impl::Localize(this, core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pschemalocalizersupport)).into()
            }
        }
        unsafe extern "system" fn SaveBinary<Identity: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszschemabinarypath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchemaProvider_Impl::SaveBinary(this, core::mem::transmute(&pszschemabinarypath)).into()
            }
        }
        unsafe extern "system" fn LookupAuthoredNamedEntity<Identity: ISchemaProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentity: *mut core::ffi::c_void, pszinputstring: windows_core::PCWSTR, ptokencollection: *mut core::ffi::c_void, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchemaProvider_Impl::LookupAuthoredNamedEntity(this, core::mem::transmute_copy(&pentity), core::mem::transmute(&pszinputstring), core::mem::transmute_copy(&ptokencollection), core::mem::transmute_copy(&ctokensbegin), core::mem::transmute_copy(&pctokenslength), core::mem::transmute_copy(&ppszvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Entities: Entities::<Identity, OFFSET>,
            RootEntity: RootEntity::<Identity, OFFSET>,
            GetEntity: GetEntity::<Identity, OFFSET>,
            MetaData: MetaData::<Identity, OFFSET>,
            Localize: Localize::<Identity, OFFSET>,
            SaveBinary: SaveBinary::<Identity, OFFSET>,
            LookupAuthoredNamedEntity: LookupAuthoredNamedEntity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for ISchemaProvider {}
windows_core::imp::define_interface!(ITokenCollection, ITokenCollection_Vtbl, 0x22d8b4f2_f577_4adb_a335_c2ae88416fab);
windows_core::imp::interface_hierarchy!(ITokenCollection, windows_core::IUnknown);
impl ITokenCollection {
    pub unsafe fn NumberOfTokens(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumberOfTokens)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetToken(&self, i: u32, pbegin: Option<*mut u32>, plength: Option<*mut u32>, ppsz: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetToken)(windows_core::Interface::as_raw(self), i, pbegin.unwrap_or(core::mem::zeroed()) as _, plength.unwrap_or(core::mem::zeroed()) as _, ppsz as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITokenCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NumberOfTokens: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ITokenCollection_Impl: windows_core::IUnknownImpl {
    fn NumberOfTokens(&self) -> windows_core::Result<u32>;
    fn GetToken(&self, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl ITokenCollection_Vtbl {
    pub const fn new<Identity: ITokenCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NumberOfTokens<Identity: ITokenCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITokenCollection_Impl::NumberOfTokens(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetToken<Identity: ITokenCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITokenCollection_Impl::GetToken(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&pbegin), core::mem::transmute_copy(&plength), core::mem::transmute_copy(&ppsz)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NumberOfTokens: NumberOfTokens::<Identity, OFFSET>,
            GetToken: GetToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITokenCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITokenCollection {}
pub const Interval: windows_core::GUID = windows_core::GUID::from_u128(0xd957171f_4bf9_4de2_bcd5_c70a7ca55836);
pub const LeafCondition: windows_core::GUID = windows_core::GUID::from_u128(0x52f15c89_5a17_48e1_bbcd_46a3f89c7cc2);
pub type NAMED_ENTITY_CERTAINTY = i32;
pub const NEC_HIGH: NAMED_ENTITY_CERTAINTY = 2;
pub const NEC_LOW: NAMED_ENTITY_CERTAINTY = 0;
pub const NEC_MEDIUM: NAMED_ENTITY_CERTAINTY = 1;
pub const NegationCondition: windows_core::GUID = windows_core::GUID::from_u128(0x8de9c74c_605a_4acd_bee3_2b222aa2d23d);
pub const QPMO_APPEND_LCID_TO_LOCALIZED_PATH: QUERY_PARSER_MANAGER_OPTION = 4;
pub const QPMO_LOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = 3;
pub const QPMO_LOCALIZER_SUPPORT: QUERY_PARSER_MANAGER_OPTION = 5;
pub const QPMO_PRELOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = 1;
pub const QPMO_SCHEMA_BINARY_NAME: QUERY_PARSER_MANAGER_OPTION = 0;
pub const QPMO_UNLOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = 2;
pub type QUERY_PARSER_MANAGER_OPTION = i32;
pub const QueryParser: windows_core::GUID = windows_core::GUID::from_u128(0xb72f8fd8_0fab_4dd9_bdbf_245a6ce1485b);
pub const QueryParserManager: windows_core::GUID = windows_core::GUID::from_u128(0x5088b39a_29b4_4d9d_8245_4ee289222f66);
pub const SQMO_DEFAULT_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = 1;
pub const SQMO_GENERATOR_FOR_TYPE: STRUCTURED_QUERY_MULTIOPTION = 2;
pub const SQMO_MAP_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = 3;
pub const SQMO_VIRTUAL_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = 0;
pub const SQPE_EXTRA_CLOSING_PARENTHESIS: STRUCTURED_QUERY_PARSE_ERROR = 2;
pub const SQPE_EXTRA_OPENING_PARENTHESIS: STRUCTURED_QUERY_PARSE_ERROR = 1;
pub const SQPE_IGNORED_CONNECTOR: STRUCTURED_QUERY_PARSE_ERROR = 4;
pub const SQPE_IGNORED_KEYWORD: STRUCTURED_QUERY_PARSE_ERROR = 5;
pub const SQPE_IGNORED_MODIFIER: STRUCTURED_QUERY_PARSE_ERROR = 3;
pub const SQPE_NONE: STRUCTURED_QUERY_PARSE_ERROR = 0;
pub const SQPE_UNHANDLED: STRUCTURED_QUERY_PARSE_ERROR = 6;
pub const SQRO_ADD_ROBUST_ITEM_NAME: STRUCTURED_QUERY_RESOLVE_OPTION = 512;
pub const SQRO_ADD_VALUE_TYPE_FOR_PLAIN_VALUES: STRUCTURED_QUERY_RESOLVE_OPTION = 256;
pub const SQRO_ALWAYS_ONE_INTERVAL: STRUCTURED_QUERY_RESOLVE_OPTION = 2;
pub const SQRO_DEFAULT: STRUCTURED_QUERY_RESOLVE_OPTION = 0;
pub const SQRO_DONT_MAP_RELATIONS: STRUCTURED_QUERY_RESOLVE_OPTION = 8;
pub const SQRO_DONT_REMOVE_UNRESTRICTED_KEYWORDS: STRUCTURED_QUERY_RESOLVE_OPTION = 32;
pub const SQRO_DONT_RESOLVE_DATETIME: STRUCTURED_QUERY_RESOLVE_OPTION = 1;
pub const SQRO_DONT_RESOLVE_RANGES: STRUCTURED_QUERY_RESOLVE_OPTION = 16;
pub const SQRO_DONT_SIMPLIFY_CONDITION_TREES: STRUCTURED_QUERY_RESOLVE_OPTION = 4;
pub const SQRO_DONT_SPLIT_WORDS: STRUCTURED_QUERY_RESOLVE_OPTION = 64;
pub const SQRO_IGNORE_PHRASE_ORDER: STRUCTURED_QUERY_RESOLVE_OPTION = 128;
pub const SQSO_AUTOMATIC_WILDCARD: STRUCTURED_QUERY_SINGLE_OPTION = 4;
pub const SQSO_CONNECTOR_CASE: STRUCTURED_QUERY_SINGLE_OPTION = 10;
pub const SQSO_IMPLICIT_CONNECTOR: STRUCTURED_QUERY_SINGLE_OPTION = 9;
pub const SQSO_LANGUAGE_KEYWORDS: STRUCTURED_QUERY_SINGLE_OPTION = 6;
pub const SQSO_LOCALE_WORD_BREAKING: STRUCTURED_QUERY_SINGLE_OPTION = 1;
pub const SQSO_NATURAL_SYNTAX: STRUCTURED_QUERY_SINGLE_OPTION = 3;
pub const SQSO_SCHEMA: STRUCTURED_QUERY_SINGLE_OPTION = 0;
pub const SQSO_SYNTAX: STRUCTURED_QUERY_SINGLE_OPTION = 7;
pub const SQSO_TIME_ZONE: STRUCTURED_QUERY_SINGLE_OPTION = 8;
pub const SQSO_TRACE_LEVEL: STRUCTURED_QUERY_SINGLE_OPTION = 5;
pub const SQSO_WORD_BREAKER: STRUCTURED_QUERY_SINGLE_OPTION = 2;
pub const SQS_ADVANCED_QUERY_SYNTAX: STRUCTURED_QUERY_SYNTAX = 1;
pub const SQS_NATURAL_QUERY_SYNTAX: STRUCTURED_QUERY_SYNTAX = 2;
pub const SQS_NO_SYNTAX: STRUCTURED_QUERY_SYNTAX = 0;
pub type STRUCTURED_QUERY_MULTIOPTION = i32;
pub type STRUCTURED_QUERY_PARSE_ERROR = i32;
pub type STRUCTURED_QUERY_RESOLVE_OPTION = u32;
pub type STRUCTURED_QUERY_SINGLE_OPTION = i32;
pub type STRUCTURED_QUERY_SYNTAX = i32;
