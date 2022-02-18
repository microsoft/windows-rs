#[cfg(feature = "Win32_Foundation")]
pub trait IAsynchronousDataRetriever_Impl: Sized {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
    fn RegisterCallback(&self, pdataretrievercallback: &::core::option::Option<IDataRetrieverCallback>) -> ::windows::core::Result<()>;
    fn RevokeCallback(&self, pdataretrievercallback: &::core::option::Option<IDataRetrieverCallback>) -> ::windows::core::Result<()>;
    fn LoadChangeData(&self, ploadchangecontext: &::core::option::Option<ILoadChangeContext>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAsynchronousDataRetriever_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>() -> IAsynchronousDataRetriever_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: ::windows::core::IUnknownImpl, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn RegisterCallback<Identity: ::windows::core::IUnknownImpl, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterCallback(::core::mem::transmute(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn RevokeCallback<Identity: ::windows::core::IUnknownImpl, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RevokeCallback(::core::mem::transmute(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn LoadChangeData<Identity: ::windows::core::IUnknownImpl, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadChangeData(::core::mem::transmute(&ploadchangecontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
            RegisterCallback: RegisterCallback::<Identity, Impl, OFFSET>,
            RevokeCallback: RevokeCallback::<Identity, Impl, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsynchronousDataRetriever as ::windows::core::Interface>::IID
    }
}
pub trait IChangeConflict_Impl: Sized {
    fn GetDestinationProviderConflictingChange(&self) -> ::windows::core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&self) -> ::windows::core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSourceProviderConflictingData(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetResolveActionForChange(&self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetResolveActionForChange(&self, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn GetResolveActionForChangeUnit(&self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetResolveActionForChangeUnit(&self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
}
impl IChangeConflict_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const OFFSET: isize>() -> IChangeConflict_Vtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDestinationProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDestinationProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolveActionForChange<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResolveActionForChange(::core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChange<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResolveActionForChange(::core::mem::transmute_copy(&resolveaction)).into()
        }
        unsafe extern "system" fn GetResolveActionForChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&resolveaction)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, Impl, OFFSET>,
            GetResolveActionForChange: GetResolveActionForChange::<Identity, Impl, OFFSET>,
            SetResolveActionForChange: SetResolveActionForChange::<Identity, Impl, OFFSET>,
            GetResolveActionForChangeUnit: GetResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
            SetResolveActionForChangeUnit: SetResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeConflict as ::windows::core::Interface>::IID
    }
}
pub trait IChangeUnitException_Impl: Sized {
    fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClockVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IChangeUnitException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitException_Impl, const OFFSET: isize>() -> IChangeUnitException_Vtbl {
        unsafe extern "system" fn GetItemId<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetItemId(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChangeUnitId(::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClockVector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetItemId: GetItemId::<Identity, Impl, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, Impl, OFFSET>,
            GetClockVector: GetClockVector::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeUnitException as ::windows::core::Interface>::IID
    }
}
pub trait IChangeUnitListFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn Initialize(&self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitIdCount(&self, pdwchangeunitidcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitId(&self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
}
impl IChangeUnitListFilterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>() -> IChangeUnitListFilterInfo_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&ppbchangeunitids), ::core::mem::transmute_copy(&dwchangeunitcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitIdCount<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChangeUnitIdCount(::core::mem::transmute_copy(&pdwchangeunitidcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChangeUnitId(::core::mem::transmute_copy(&dwchangeunitidindex), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base: ISyncFilterInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetChangeUnitIdCount: GetChangeUnitIdCount::<Identity, Impl, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeUnitListFilterInfo as ::windows::core::Interface>::IID || iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IClockVector_Impl: Sized {
    fn GetClockVectorElements(&self, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> ::windows::core::Result<()>;
}
impl IClockVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClockVector_Impl, const OFFSET: isize>() -> IClockVector_Vtbl {
        unsafe extern "system" fn GetClockVectorElements<Identity: ::windows::core::IUnknownImpl, Impl: IClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClockVectorElements(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppienumclockvector)).into()
        }
        unsafe extern "system" fn GetClockVectorElementCount<Identity: ::windows::core::IUnknownImpl, Impl: IClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClockVectorElementCount(::core::mem::transmute_copy(&pdwcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClockVectorElements: GetClockVectorElements::<Identity, Impl, OFFSET>,
            GetClockVectorElementCount: GetClockVectorElementCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IClockVectorElement_Impl: Sized {
    fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> ::windows::core::Result<()>;
    fn GetTickCount(&self, pulltickcount: *mut u64) -> ::windows::core::Result<()>;
}
impl IClockVectorElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClockVectorElement_Impl, const OFFSET: isize>() -> IClockVectorElement_Vtbl {
        unsafe extern "system" fn GetReplicaKey<Identity: ::windows::core::IUnknownImpl, Impl: IClockVectorElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetReplicaKey(::core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn GetTickCount<Identity: ::windows::core::IUnknownImpl, Impl: IClockVectorElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulltickcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTickCount(::core::mem::transmute_copy(&pulltickcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetReplicaKey: GetReplicaKey::<Identity, Impl, OFFSET>,
            GetTickCount: GetTickCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClockVectorElement as ::windows::core::Interface>::IID
    }
}
pub trait ICombinedFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilterInfo(&self, dwfilterindex: u32) -> ::windows::core::Result<ISyncFilterInfo>;
    fn GetFilterCombinationType(&self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::Result<()>;
}
impl ICombinedFilterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>() -> ICombinedFilterInfo_Vtbl {
        unsafe extern "system" fn GetFilterCount<Identity: ::windows::core::IUnknownImpl, Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilterCount(::core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterInfo<Identity: ::windows::core::IUnknownImpl, Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilterInfo(::core::mem::transmute_copy(&dwfilterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifilterinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterCombinationType<Identity: ::windows::core::IUnknownImpl, Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilterCombinationType(::core::mem::transmute_copy(&pfiltercombinationtype)).into()
        }
        Self {
            base: ISyncFilterInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFilterCount: GetFilterCount::<Identity, Impl, OFFSET>,
            GetFilterInfo: GetFilterInfo::<Identity, Impl, OFFSET>,
            GetFilterCombinationType: GetFilterCombinationType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICombinedFilterInfo as ::windows::core::Interface>::IID || iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IConstraintConflict_Impl: Sized {
    fn GetDestinationProviderConflictingChange(&self) -> ::windows::core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&self) -> ::windows::core::Result<ISyncChange>;
    fn GetDestinationProviderOriginalChange(&self) -> ::windows::core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSourceProviderConflictingData(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetDestinationProviderOriginalData(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetConstraintResolveActionForChange(&self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetConstraintResolveActionForChange(&self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn GetConstraintResolveActionForChangeUnit(&self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetConstraintResolveActionForChangeUnit(&self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn GetConstraintConflictReason(&self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::Result<()>;
    fn IsTemporary(&self) -> ::windows::core::Result<()>;
}
impl IConstraintConflict_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>() -> IConstraintConflict_Vtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDestinationProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalChange<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginalchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDestinationProviderOriginalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pporiginalchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDestinationProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalData<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDestinationProviderOriginalData() {
                ::core::result::Result::Ok(ok__) => {
                    *pporiginaldata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstraintResolveActionForChange<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstraintResolveActionForChange(::core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChange<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConstraintResolveActionForChange(::core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintResolveActionForChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstraintResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConstraintResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintConflictReason<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConstraintConflictReason(::core::mem::transmute_copy(&pconstraintconflictreason)).into()
        }
        unsafe extern "system" fn IsTemporary<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsTemporary().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetDestinationProviderOriginalChange: GetDestinationProviderOriginalChange::<Identity, Impl, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, Impl, OFFSET>,
            GetDestinationProviderOriginalData: GetDestinationProviderOriginalData::<Identity, Impl, OFFSET>,
            GetConstraintResolveActionForChange: GetConstraintResolveActionForChange::<Identity, Impl, OFFSET>,
            SetConstraintResolveActionForChange: SetConstraintResolveActionForChange::<Identity, Impl, OFFSET>,
            GetConstraintResolveActionForChangeUnit: GetConstraintResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
            SetConstraintResolveActionForChangeUnit: SetConstraintResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
            GetConstraintConflictReason: GetConstraintConflictReason::<Identity, Impl, OFFSET>,
            IsTemporary: IsTemporary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConstraintConflict as ::windows::core::Interface>::IID
    }
}
pub trait IConstructReplicaKeyMap_Impl: Sized {
    fn FindOrAddReplica(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::Result<()>;
}
impl IConstructReplicaKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConstructReplicaKeyMap_Impl, const OFFSET: isize>() -> IConstructReplicaKeyMap_Vtbl {
        unsafe extern "system" fn FindOrAddReplica<Identity: ::windows::core::IUnknownImpl, Impl: IConstructReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindOrAddReplica(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), FindOrAddReplica: FindOrAddReplica::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConstructReplicaKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait ICoreFragment_Impl: Sized {
    fn NextColumn(&self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::Result<()>;
    fn NextRange(&self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::core::option::Option<IClockVector>) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn GetColumnCount(&self, pcolumncount: *mut u32) -> ::windows::core::Result<()>;
    fn GetRangeCount(&self, prangecount: *mut u32) -> ::windows::core::Result<()>;
}
impl ICoreFragment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragment_Impl, const OFFSET: isize>() -> ICoreFragment_Vtbl {
        unsafe extern "system" fn NextColumn<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NextColumn(::core::mem::transmute_copy(&pchangeunitid), ::core::mem::transmute_copy(&pchangeunitidsize)).into()
        }
        unsafe extern "system" fn NextRange<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NextRange(::core::mem::transmute_copy(&pitemid), ::core::mem::transmute_copy(&pitemidsize), ::core::mem::transmute_copy(&piclockvector)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn GetColumnCount<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolumncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColumnCount(::core::mem::transmute_copy(&pcolumncount)).into()
        }
        unsafe extern "system" fn GetRangeCount<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRangeCount(::core::mem::transmute_copy(&prangecount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            NextColumn: NextColumn::<Identity, Impl, OFFSET>,
            NextRange: NextRange::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, Impl, OFFSET>,
            GetRangeCount: GetRangeCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFragment as ::windows::core::Interface>::IID
    }
}
pub trait ICoreFragmentInspector_Impl: Sized {
    fn NextCoreFragments(&self, requestedcount: u32, ppicorefragments: *mut ::core::option::Option<ICoreFragment>, pfetchedcount: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
impl ICoreFragmentInspector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragmentInspector_Impl, const OFFSET: isize>() -> ICoreFragmentInspector_Vtbl {
        unsafe extern "system" fn NextCoreFragments<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragmentInspector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut ::windows::core::RawPtr, pfetchedcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NextCoreFragments(::core::mem::transmute_copy(&requestedcount), ::core::mem::transmute_copy(&ppicorefragments), ::core::mem::transmute_copy(&pfetchedcount)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragmentInspector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            NextCoreFragments: NextCoreFragments::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFragmentInspector as ::windows::core::Interface>::IID
    }
}
pub trait ICustomFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetSyncFilter(&self) -> ::windows::core::Result<ISyncFilter>;
}
impl ICustomFilterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomFilterInfo_Impl, const OFFSET: isize>() -> ICustomFilterInfo_Vtbl {
        unsafe extern "system" fn GetSyncFilter<Identity: ::windows::core::IUnknownImpl, Impl: ICustomFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *pisyncfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ISyncFilterInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetSyncFilter: GetSyncFilter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomFilterInfo as ::windows::core::Interface>::IID || iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IDataRetrieverCallback_Impl: Sized {
    fn LoadChangeDataComplete(&self, punkdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn LoadChangeDataError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IDataRetrieverCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRetrieverCallback_Impl, const OFFSET: isize>() -> IDataRetrieverCallback_Vtbl {
        unsafe extern "system" fn LoadChangeDataComplete<Identity: ::windows::core::IUnknownImpl, Impl: IDataRetrieverCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadChangeDataComplete(::core::mem::transmute(&punkdata)).into()
        }
        unsafe extern "system" fn LoadChangeDataError<Identity: ::windows::core::IUnknownImpl, Impl: IDataRetrieverCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadChangeDataError(::core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            LoadChangeDataComplete: LoadChangeDataComplete::<Identity, Impl, OFFSET>,
            LoadChangeDataError: LoadChangeDataError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataRetrieverCallback as ::windows::core::Interface>::IID
    }
}
pub trait IEnumChangeUnitExceptions_Impl: Sized {
    fn Next(&self, cexceptions: u32, ppchangeunitexception: *mut ::core::option::Option<IChangeUnitException>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cexceptions: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumChangeUnitExceptions>;
}
impl IEnumChangeUnitExceptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>() -> IEnumChangeUnitExceptions_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&ppchangeunitexception), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumChangeUnitExceptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumClockVector_Impl: Sized {
    fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IClockVectorElement>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, csyncversions: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumClockVector>;
}
impl IEnumClockVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumClockVector_Impl, const OFFSET: isize>() -> IEnumClockVector_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cclockvectorelements), ::core::mem::transmute_copy(&ppiclockvectorelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IEnumFeedClockVector_Impl: Sized {
    fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, csyncversions: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumFeedClockVector>;
}
impl IEnumFeedClockVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>() -> IEnumFeedClockVector_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cclockvectorelements), ::core::mem::transmute_copy(&ppiclockvectorelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumFeedClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IEnumItemIds_Impl: Sized {
    fn Next(&self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::Result<()>;
}
impl IEnumItemIds_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemIds_Impl, const OFFSET: isize>() -> IEnumItemIds_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumItemIds as ::windows::core::Interface>::IID
    }
}
pub trait IEnumRangeExceptions_Impl: Sized {
    fn Next(&self, cexceptions: u32, pprangeexception: *mut ::core::option::Option<IRangeException>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cexceptions: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumRangeExceptions>;
}
impl IEnumRangeExceptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>() -> IEnumRangeExceptions_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, pprangeexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&pprangeexception), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumRangeExceptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSingleItemExceptions_Impl: Sized {
    fn Next(&self, cexceptions: u32, ppsingleitemexception: *mut ::core::option::Option<ISingleItemException>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cexceptions: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSingleItemExceptions>;
}
impl IEnumSingleItemExceptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>() -> IEnumSingleItemExceptions_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&ppsingleitemexception), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSingleItemExceptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSyncChangeUnits_Impl: Sized {
    fn Next(&self, cchanges: u32, ppchangeunit: *mut ::core::option::Option<ISyncChangeUnit>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cchanges: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSyncChangeUnits>;
}
impl IEnumSyncChangeUnits_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>() -> IEnumSyncChangeUnits_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchangeunit: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&ppchangeunit), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncChangeUnits as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSyncChanges_Impl: Sized {
    fn Next(&self, cchanges: u32, ppchange: *mut ::core::option::Option<ISyncChange>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cchanges: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSyncChanges>;
}
impl IEnumSyncChanges_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>() -> IEnumSyncChanges_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchange: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&ppchange), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncChanges as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderConfigUIInfos_Impl: Sized {
    fn Next(&self, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::core::option::Option<ISyncProviderConfigUIInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cfactories: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSyncProviderConfigUIInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderConfigUIInfos_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>() -> IEnumSyncProviderConfigUIInfos_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cfactories), ::core::mem::transmute_copy(&ppsyncproviderconfiguiinfo), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfactories: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cfactories)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncProviderConfigUIInfos as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderInfos_Impl: Sized {
    fn Next(&self, cinstances: u32, ppsyncproviderinfo: *mut ::core::option::Option<ISyncProviderInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cinstances: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSyncProviderInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderInfos_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>() -> IEnumSyncProviderInfos_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cinstances), ::core::mem::transmute_copy(&ppsyncproviderinfo), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinstances: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cinstances)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncProviderInfos as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFeedClockVector_Impl: Sized + IClockVector_Impl {
    fn GetUpdateCount(&self, pdwupdatecount: *mut u32) -> ::windows::core::Result<()>;
    fn IsNoConflictsSpecified(&self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFeedClockVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVector_Impl, const OFFSET: isize>() -> IFeedClockVector_Vtbl {
        unsafe extern "system" fn GetUpdateCount<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwupdatecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUpdateCount(::core::mem::transmute_copy(&pdwupdatecount)).into()
        }
        unsafe extern "system" fn IsNoConflictsSpecified<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsNoConflictsSpecified(::core::mem::transmute_copy(&pfisnoconflictsspecified)).into()
        }
        Self {
            base: IClockVector_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetUpdateCount: GetUpdateCount::<Identity, Impl, OFFSET>,
            IsNoConflictsSpecified: IsNoConflictsSpecified::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedClockVector as ::windows::core::Interface>::IID || iid == &<IClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IFeedClockVectorElement_Impl: Sized + IClockVectorElement_Impl {
    fn GetSyncTime(&self, psynctime: *mut SYNC_TIME) -> ::windows::core::Result<()>;
    fn GetFlags(&self, pbflags: *mut u8) -> ::windows::core::Result<()>;
}
impl IFeedClockVectorElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVectorElement_Impl, const OFFSET: isize>() -> IFeedClockVectorElement_Vtbl {
        unsafe extern "system" fn GetSyncTime<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVectorElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynctime: *mut SYNC_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSyncTime(::core::mem::transmute_copy(&psynctime)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVectorElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbflags: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pbflags)).into()
        }
        Self {
            base: IClockVectorElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSyncTime: GetSyncTime::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedClockVectorElement as ::windows::core::Interface>::IID || iid == &<IClockVectorElement as ::windows::core::Interface>::IID
    }
}
pub trait IFilterKeyMap_Impl: Sized {
    fn GetCount(&self, pdwcount: *mut u32) -> ::windows::core::Result<()>;
    fn AddFilter(&self, pisyncfilter: &::core::option::Option<ISyncFilter>, pdwfilterkey: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilter(&self, dwfilterkey: u32) -> ::windows::core::Result<ISyncFilter>;
    fn Serialize(&self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::Result<()>;
}
impl IFilterKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterKeyMap_Impl, const OFFSET: isize>() -> IFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn AddFilter<Identity: ::windows::core::IUnknownImpl, Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncfilter: ::windows::core::RawPtr, pdwfilterkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFilter(::core::mem::transmute(&pisyncfilter), ::core::mem::transmute_copy(&pdwfilterkey)).into()
        }
        unsafe extern "system" fn GetFilter<Identity: ::windows::core::IUnknownImpl, Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilter(::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppisyncfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl, Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbfilterkeymap), ::core::mem::transmute_copy(&pcbfilterkeymap)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            AddFilter: AddFilter::<Identity, Impl, OFFSET>,
            GetFilter: GetFilter::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait IFilterRequestCallback_Impl: Sized {
    fn RequestFilter(&self, pfilter: &::core::option::Option<::windows::core::IUnknown>, filteringtype: FILTERING_TYPE) -> ::windows::core::Result<()>;
}
impl IFilterRequestCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterRequestCallback_Impl, const OFFSET: isize>() -> IFilterRequestCallback_Vtbl {
        unsafe extern "system" fn RequestFilter<Identity: ::windows::core::IUnknownImpl, Impl: IFilterRequestCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestFilter(::core::mem::transmute(&pfilter), ::core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), RequestFilter: RequestFilter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IFilterTrackingProvider_Impl: Sized {
    fn SpecifyTrackedFilters(&self, pcallback: &::core::option::Option<IFilterTrackingRequestCallback>) -> ::windows::core::Result<()>;
    fn AddTrackedFilter(&self, pfilter: &::core::option::Option<ISyncFilter>) -> ::windows::core::Result<()>;
}
impl IFilterTrackingProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingProvider_Impl, const OFFSET: isize>() -> IFilterTrackingProvider_Vtbl {
        unsafe extern "system" fn SpecifyTrackedFilters<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SpecifyTrackedFilters(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn AddTrackedFilter<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTrackedFilter(::core::mem::transmute(&pfilter)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SpecifyTrackedFilters: SpecifyTrackedFilters::<Identity, Impl, OFFSET>,
            AddTrackedFilter: AddTrackedFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterTrackingProvider as ::windows::core::Interface>::IID
    }
}
pub trait IFilterTrackingRequestCallback_Impl: Sized {
    fn RequestTrackedFilter(&self, pfilter: &::core::option::Option<ISyncFilter>) -> ::windows::core::Result<()>;
}
impl IFilterTrackingRequestCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingRequestCallback_Impl, const OFFSET: isize>() -> IFilterTrackingRequestCallback_Vtbl {
        unsafe extern "system" fn RequestTrackedFilter<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingRequestCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestTrackedFilter(::core::mem::transmute(&pfilter)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), RequestTrackedFilter: RequestTrackedFilter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterTrackingRequestCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFilterTrackingSyncChangeBuilder_Impl: Sized {
    fn AddFilterChange(&self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::Result<()>;
    fn SetAllChangeUnitsPresentFlag(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFilterTrackingSyncChangeBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>() -> IFilterTrackingSyncChangeBuilder_Vtbl {
        unsafe extern "system" fn AddFilterChange<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFilterChange(::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn SetAllChangeUnitsPresentFlag<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllChangeUnitsPresentFlag().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddFilterChange: AddFilterChange::<Identity, Impl, OFFSET>,
            SetAllChangeUnitsPresentFlag: SetAllChangeUnitsPresentFlag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterTrackingSyncChangeBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IForgottenKnowledge_Impl: Sized + ISyncKnowledge_Impl {
    fn ForgetToVersion(&self, pknowledge: &::core::option::Option<ISyncKnowledge>, pversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IForgottenKnowledge_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForgottenKnowledge_Impl, const OFFSET: isize>() -> IForgottenKnowledge_Vtbl {
        unsafe extern "system" fn ForgetToVersion<Identity: ::windows::core::IUnknownImpl, Impl: IForgottenKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ForgetToVersion(::core::mem::transmute(&pknowledge), ::core::mem::transmute_copy(&pversion)).into()
        }
        Self { base: ISyncKnowledge_Vtbl::new::<Identity, Impl, OFFSET>(), ForgetToVersion: ForgetToVersion::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForgottenKnowledge as ::windows::core::Interface>::IID || iid == &<ISyncKnowledge as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKnowledgeSyncProvider_Impl: Sized + ISyncProvider_Impl {
    fn BeginSession(&self, role: SYNC_PROVIDER_ROLE, psessionstate: &::core::option::Option<ISyncSessionState>) -> ::windows::core::Result<()>;
    fn GetSyncBatchParameters(&self, ppsyncknowledge: *mut ::core::option::Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeBatch(&self, dwbatchsize: u32, psyncknowledge: &::core::option::Option<ISyncKnowledge>, ppsyncchangebatch: *mut ::core::option::Option<ISyncChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetFullEnumerationChangeBatch(&self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: &::core::option::Option<ISyncKnowledge>, ppsyncchangebatch: *mut ::core::option::Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ProcessChangeBatch(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: &::core::option::Option<ISyncChangeBatch>, punkdataretriever: &::core::option::Option<::windows::core::IUnknown>, pcallback: &::core::option::Option<ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::Result<()>;
    fn ProcessFullEnumerationChangeBatch(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: &::core::option::Option<ISyncFullEnumerationChangeBatch>, punkdataretriever: &::core::option::Option<::windows::core::IUnknown>, pcallback: &::core::option::Option<ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::Result<()>;
    fn EndSession(&self, psessionstate: &::core::option::Option<ISyncSessionState>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKnowledgeSyncProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>() -> IKnowledgeSyncProvider_Vtbl {
        unsafe extern "system" fn BeginSession<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginSession(::core::mem::transmute_copy(&role), ::core::mem::transmute(&psessionstate)).into()
        }
        unsafe extern "system" fn GetSyncBatchParameters<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncknowledge: *mut ::windows::core::RawPtr, pdwrequestedbatchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSyncBatchParameters(::core::mem::transmute_copy(&ppsyncknowledge), ::core::mem::transmute_copy(&pdwrequestedbatchsize)).into()
        }
        unsafe extern "system" fn GetChangeBatch<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChangeBatch(::core::mem::transmute_copy(&dwbatchsize), ::core::mem::transmute(&psyncknowledge), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn GetFullEnumerationChangeBatch<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFullEnumerationChangeBatch(::core::mem::transmute_copy(&dwbatchsize), ::core::mem::transmute_copy(&pblowerenumerationbound), ::core::mem::transmute(&psyncknowledge), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn ProcessChangeBatch<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessChangeBatch(::core::mem::transmute_copy(&resolutionpolicy), ::core::mem::transmute(&psourcechangebatch), ::core::mem::transmute(&punkdataretriever), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn ProcessFullEnumerationChangeBatch<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessFullEnumerationChangeBatch(::core::mem::transmute_copy(&resolutionpolicy), ::core::mem::transmute(&psourcechangebatch), ::core::mem::transmute(&punkdataretriever), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn EndSession<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndSession(::core::mem::transmute(&psessionstate)).into()
        }
        Self {
            base: ISyncProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginSession: BeginSession::<Identity, Impl, OFFSET>,
            GetSyncBatchParameters: GetSyncBatchParameters::<Identity, Impl, OFFSET>,
            GetChangeBatch: GetChangeBatch::<Identity, Impl, OFFSET>,
            GetFullEnumerationChangeBatch: GetFullEnumerationChangeBatch::<Identity, Impl, OFFSET>,
            ProcessChangeBatch: ProcessChangeBatch::<Identity, Impl, OFFSET>,
            ProcessFullEnumerationChangeBatch: ProcessFullEnumerationChangeBatch::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnowledgeSyncProvider as ::windows::core::Interface>::IID || iid == &<ISyncProvider as ::windows::core::Interface>::IID
    }
}
pub trait ILoadChangeContext_Impl: Sized {
    fn GetSyncChange(&self) -> ::windows::core::Result<ISyncChange>;
    fn SetRecoverableErrorOnChange(&self, hrerror: ::windows::core::HRESULT, perrordata: &::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
    fn SetRecoverableErrorOnChangeUnit(&self, hrerror: ::windows::core::HRESULT, pchangeunit: &::core::option::Option<ISyncChangeUnit>, perrordata: &::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
}
impl ILoadChangeContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoadChangeContext_Impl, const OFFSET: isize>() -> ILoadChangeContext_Vtbl {
        unsafe extern "system" fn GetSyncChange<Identity: ::windows::core::IUnknownImpl, Impl: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoverableErrorOnChange<Identity: ::windows::core::IUnknownImpl, Impl: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecoverableErrorOnChange(::core::mem::transmute_copy(&hrerror), ::core::mem::transmute(&perrordata)).into()
        }
        unsafe extern "system" fn SetRecoverableErrorOnChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, pchangeunit: ::windows::core::RawPtr, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecoverableErrorOnChangeUnit(::core::mem::transmute_copy(&hrerror), ::core::mem::transmute(&pchangeunit), ::core::mem::transmute(&perrordata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSyncChange: GetSyncChange::<Identity, Impl, OFFSET>,
            SetRecoverableErrorOnChange: SetRecoverableErrorOnChange::<Identity, Impl, OFFSET>,
            SetRecoverableErrorOnChangeUnit: SetRecoverableErrorOnChangeUnit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadChangeContext as ::windows::core::Interface>::IID
    }
}
pub trait IProviderConverter_Impl: Sized {
    fn Initialize(&self, pisyncprovider: &::core::option::Option<ISyncProvider>) -> ::windows::core::Result<()>;
}
impl IProviderConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderConverter_Impl, const OFFSET: isize>() -> IProviderConverter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IProviderConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisyncprovider)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderConverter as ::windows::core::Interface>::IID
    }
}
pub trait IRangeException_Impl: Sized {
    fn GetClosedRangeStart(&self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClosedRangeEnd(&self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClockVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IRangeException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeException_Impl, const OFFSET: isize>() -> IRangeException_Vtbl {
        unsafe extern "system" fn GetClosedRangeStart<Identity: ::windows::core::IUnknownImpl, Impl: IRangeException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClosedRangeStart(::core::mem::transmute_copy(&pbclosedrangestart), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedRangeEnd<Identity: ::windows::core::IUnknownImpl, Impl: IRangeException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClosedRangeEnd(::core::mem::transmute_copy(&pbclosedrangeend), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: ::windows::core::IUnknownImpl, Impl: IRangeException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClockVector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClosedRangeStart: GetClosedRangeStart::<Identity, Impl, OFFSET>,
            GetClosedRangeEnd: GetClosedRangeEnd::<Identity, Impl, OFFSET>,
            GetClockVector: GetClockVector::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeException as ::windows::core::Interface>::IID
    }
}
pub trait IRecoverableError_Impl: Sized {
    fn GetStage(&self, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::Result<()>;
    fn GetProvider(&self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::Result<()>;
    fn GetChangeWithRecoverableError(&self) -> ::windows::core::Result<ISyncChange>;
    fn GetRecoverableErrorDataForChange(&self, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
    fn GetRecoverableErrorDataForChangeUnit(&self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
}
impl IRecoverableError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableError_Impl, const OFFSET: isize>() -> IRecoverableError_Vtbl {
        unsafe extern "system" fn GetStage<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStage(::core::mem::transmute_copy(&pstage)).into()
        }
        unsafe extern "system" fn GetProvider<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProvider(::core::mem::transmute_copy(&pproviderrole)).into()
        }
        unsafe extern "system" fn GetChangeWithRecoverableError<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppchangewithrecoverableerror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChangeWithRecoverableError() {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangewithrecoverableerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChange<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRecoverableErrorDataForChange(::core::mem::transmute_copy(&phrerror), ::core::mem::transmute_copy(&pperrordata)).into()
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRecoverableErrorDataForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&phrerror), ::core::mem::transmute_copy(&pperrordata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStage: GetStage::<Identity, Impl, OFFSET>,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            GetChangeWithRecoverableError: GetChangeWithRecoverableError::<Identity, Impl, OFFSET>,
            GetRecoverableErrorDataForChange: GetRecoverableErrorDataForChange::<Identity, Impl, OFFSET>,
            GetRecoverableErrorDataForChangeUnit: GetRecoverableErrorDataForChangeUnit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRecoverableError as ::windows::core::Interface>::IID
    }
}
pub trait IRecoverableErrorData_Impl: Sized {
    fn Initialize(&self, pcszitemdisplayname: &::windows::core::PCWSTR, pcszerrordescription: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetItemDisplayName(&self, pszitemdisplayname: &::windows::core::PCWSTR, pcchitemdisplayname: *mut u32) -> ::windows::core::Result<()>;
    fn GetErrorDescription(&self, pszerrordescription: &::windows::core::PCWSTR, pccherrordescription: *mut u32) -> ::windows::core::Result<()>;
}
impl IRecoverableErrorData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableErrorData_Impl, const OFFSET: isize>() -> IRecoverableErrorData_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszitemdisplayname: ::windows::core::PCWSTR, pcszerrordescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pcszitemdisplayname), ::core::mem::transmute(&pcszerrordescription)).into()
        }
        unsafe extern "system" fn GetItemDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitemdisplayname: ::windows::core::PCWSTR, pcchitemdisplayname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetItemDisplayName(::core::mem::transmute(&pszitemdisplayname), ::core::mem::transmute_copy(&pcchitemdisplayname)).into()
        }
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszerrordescription: ::windows::core::PCWSTR, pccherrordescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetErrorDescription(::core::mem::transmute(&pszerrordescription), ::core::mem::transmute_copy(&pccherrordescription)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetItemDisplayName: GetItemDisplayName::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRecoverableErrorData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IRegisteredSyncProvider_Impl: Sized {
    fn Init(&self, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn GetInstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IRegisteredSyncProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>() -> IRegisteredSyncProvider_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute(&pcontextpropertystore)).into()
        }
        unsafe extern "system" fn GetInstanceId<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredSyncProvider as ::windows::core::Interface>::IID
    }
}
pub trait IReplicaKeyMap_Impl: Sized {
    fn LookupReplicaKey(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::Result<()>;
    fn LookupReplicaId(&self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn Serialize(&self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::Result<()>;
}
impl IReplicaKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReplicaKeyMap_Impl, const OFFSET: isize>() -> IReplicaKeyMap_Vtbl {
        unsafe extern "system" fn LookupReplicaKey<Identity: ::windows::core::IUnknownImpl, Impl: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LookupReplicaKey(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn LookupReplicaId<Identity: ::windows::core::IUnknownImpl, Impl: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LookupReplicaId(::core::mem::transmute_copy(&dwreplicakey), ::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl, Impl: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbreplicakeymap), ::core::mem::transmute_copy(&pcbreplicakeymap)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            LookupReplicaKey: LookupReplicaKey::<Identity, Impl, OFFSET>,
            LookupReplicaId: LookupReplicaId::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReplicaKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait IRequestFilteredSync_Impl: Sized {
    fn SpecifyFilter(&self, pcallback: &::core::option::Option<IFilterRequestCallback>) -> ::windows::core::Result<()>;
}
impl IRequestFilteredSync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRequestFilteredSync_Impl, const OFFSET: isize>() -> IRequestFilteredSync_Vtbl {
        unsafe extern "system" fn SpecifyFilter<Identity: ::windows::core::IUnknownImpl, Impl: IRequestFilteredSync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SpecifyFilter(::core::mem::transmute(&pcallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SpecifyFilter: SpecifyFilter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRequestFilteredSync as ::windows::core::Interface>::IID
    }
}
pub trait ISingleItemException_Impl: Sized {
    fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClockVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ISingleItemException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISingleItemException_Impl, const OFFSET: isize>() -> ISingleItemException_Vtbl {
        unsafe extern "system" fn GetItemId<Identity: ::windows::core::IUnknownImpl, Impl: ISingleItemException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetItemId(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: ::windows::core::IUnknownImpl, Impl: ISingleItemException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClockVector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetItemId: GetItemId::<Identity, Impl, OFFSET>,
            GetClockVector: GetClockVector::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISingleItemException as ::windows::core::Interface>::IID
    }
}
pub trait ISupportFilteredSync_Impl: Sized {
    fn AddFilter(&self, pfilter: &::core::option::Option<::windows::core::IUnknown>, filteringtype: FILTERING_TYPE) -> ::windows::core::Result<()>;
}
impl ISupportFilteredSync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportFilteredSync_Impl, const OFFSET: isize>() -> ISupportFilteredSync_Vtbl {
        unsafe extern "system" fn AddFilter<Identity: ::windows::core::IUnknownImpl, Impl: ISupportFilteredSync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFilter(::core::mem::transmute(&pfilter), ::core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AddFilter: AddFilter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportFilteredSync as ::windows::core::Interface>::IID
    }
}
pub trait ISupportLastWriteTime_Impl: Sized {
    fn GetItemChangeTime(&self, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::Result<()>;
    fn GetChangeUnitChangeTime(&self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::Result<()>;
}
impl ISupportLastWriteTime_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportLastWriteTime_Impl, const OFFSET: isize>() -> ISupportLastWriteTime_Vtbl {
        unsafe extern "system" fn GetItemChangeTime<Identity: ::windows::core::IUnknownImpl, Impl: ISupportLastWriteTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetItemChangeTime(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pulltimestamp)).into()
        }
        unsafe extern "system" fn GetChangeUnitChangeTime<Identity: ::windows::core::IUnknownImpl, Impl: ISupportLastWriteTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChangeUnitChangeTime(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pulltimestamp)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetItemChangeTime: GetItemChangeTime::<Identity, Impl, OFFSET>,
            GetChangeUnitChangeTime: GetChangeUnitChangeTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportLastWriteTime as ::windows::core::Interface>::IID
    }
}
pub trait ISyncCallback_Impl: Sized {
    fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()>;
    fn OnChange(&self, psyncchange: &::core::option::Option<ISyncChange>) -> ::windows::core::Result<()>;
    fn OnConflict(&self, pconflict: &::core::option::Option<IChangeConflict>) -> ::windows::core::Result<()>;
    fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::Result<()>;
    fn OnRecoverableError(&self, precoverableerror: &::core::option::Option<IRecoverableError>) -> ::windows::core::Result<()>;
}
impl ISyncCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback_Impl, const OFFSET: isize>() -> ISyncCallback_Vtbl {
        unsafe extern "system" fn OnProgress<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&provider), ::core::mem::transmute_copy(&syncstage), ::core::mem::transmute_copy(&dwcompletedwork), ::core::mem::transmute_copy(&dwtotalwork)).into()
        }
        unsafe extern "system" fn OnChange<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChange(::core::mem::transmute(&psyncchange)).into()
        }
        unsafe extern "system" fn OnConflict<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConflict(::core::mem::transmute(&pconflict)).into()
        }
        unsafe extern "system" fn OnFullEnumerationNeeded<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnFullEnumerationNeeded(::core::mem::transmute_copy(&pfullenumerationaction)).into()
        }
        unsafe extern "system" fn OnRecoverableError<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precoverableerror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRecoverableError(::core::mem::transmute(&precoverableerror)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnChange: OnChange::<Identity, Impl, OFFSET>,
            OnConflict: OnConflict::<Identity, Impl, OFFSET>,
            OnFullEnumerationNeeded: OnFullEnumerationNeeded::<Identity, Impl, OFFSET>,
            OnRecoverableError: OnRecoverableError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISyncCallback2_Impl: Sized + ISyncCallback_Impl {
    fn OnChangeApplied(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::Result<()>;
    fn OnChangeFailed(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::Result<()>;
}
impl ISyncCallback2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback2_Impl, const OFFSET: isize>() -> ISyncCallback2_Vtbl {
        unsafe extern "system" fn OnChangeApplied<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChangeApplied(::core::mem::transmute_copy(&dwchangesapplied), ::core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        unsafe extern "system" fn OnChangeFailed<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChangeFailed(::core::mem::transmute_copy(&dwchangesapplied), ::core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        Self {
            base: ISyncCallback_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnChangeApplied: OnChangeApplied::<Identity, Impl, OFFSET>,
            OnChangeFailed: OnChangeFailed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncCallback2 as ::windows::core::Interface>::IID || iid == &<ISyncCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChange_Impl: Sized {
    fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetRootItemId(&self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
    fn GetCreationVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
    fn GetFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetWorkEstimate(&self, pdwwork: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnits(&self) -> ::windows::core::Result<IEnumSyncChangeUnits>;
    fn GetMadeWithKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge>;
    fn SetWorkEstimate(&self, dwwork: u32) -> ::windows::core::Result<()>;
}
impl ISyncChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>() -> ISyncChange_Vtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOwnerReplicaId(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetRootItemId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRootItemId(::core::mem::transmute_copy(&pbrootitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeVersion<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChangeVersion(::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetCreationVersion<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCreationVersion(::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetWorkEstimate<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwork: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWorkEstimate(::core::mem::transmute_copy(&pdwwork)).into()
        }
        unsafe extern "system" fn GetChangeUnits<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChangeUnits() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMadeWithKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmadewithknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMadeWithKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmadewithknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkEstimate<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWorkEstimate(::core::mem::transmute_copy(&dwwork)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, Impl, OFFSET>,
            GetRootItemId: GetRootItemId::<Identity, Impl, OFFSET>,
            GetChangeVersion: GetChangeVersion::<Identity, Impl, OFFSET>,
            GetCreationVersion: GetCreationVersion::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetWorkEstimate: GetWorkEstimate::<Identity, Impl, OFFSET>,
            GetChangeUnits: GetChangeUnits::<Identity, Impl, OFFSET>,
            GetMadeWithKnowledge: GetMadeWithKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, Impl, OFFSET>,
            SetWorkEstimate: SetWorkEstimate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatch_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn BeginUnorderedGroup(&self) -> ::windows::core::Result<()>;
    fn EndUnorderedGroup(&self, pmadewithknowledge: &::core::option::Option<ISyncKnowledge>, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddLoggedConflict(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch_Impl, const OFFSET: isize>() -> ISyncChangeBatch_Vtbl {
        unsafe extern "system" fn BeginUnorderedGroup<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginUnorderedGroup().into()
        }
        unsafe extern "system" fn EndUnorderedGroup<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmadewithknowledge: ::windows::core::RawPtr, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndUnorderedGroup(::core::mem::transmute(&pmadewithknowledge), ::core::mem::transmute_copy(&fallchangesforknowledge)).into()
        }
        unsafe extern "system" fn AddLoggedConflict<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddLoggedConflict(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwworkforchange), ::core::mem::transmute(&pconflictknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUnorderedGroup: BeginUnorderedGroup::<Identity, Impl, OFFSET>,
            EndUnorderedGroup: EndUnorderedGroup::<Identity, Impl, OFFSET>,
            AddLoggedConflict: AddLoggedConflict::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatch as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatch2_Impl: Sized + ISyncChangeBatchBase_Impl + ISyncChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder>;
    fn AddMergeTombstoneLoggedConflict(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatch2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch2_Impl, const OFFSET: isize>() -> ISyncChangeBatch2_Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddMergeTombstoneMetadataToGroup(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMergeTombstoneLoggedConflict<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddMergeTombstoneLoggedConflict(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange), ::core::mem::transmute(&pconflictknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISyncChangeBatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, Impl, OFFSET>,
            AddMergeTombstoneLoggedConflict: AddMergeTombstoneLoggedConflict::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatch2 as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchAdvanced_Impl: Sized {
    fn GetFilterInfo(&self) -> ::windows::core::Result<ISyncFilterInfo>;
    fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&self) -> ::windows::core::Result<ISyncChangeBatch>;
    fn GetUpperBoundItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetBatchLevelKnowledgeShouldBeApplied(&self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchAdvanced_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>() -> ISyncChangeBatchAdvanced_Vtbl {
        unsafe extern "system" fn GetFilterInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilterInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfilterinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFullEnumerationChangeBatchToRegularChangeBatch<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppchangebatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConvertFullEnumerationChangeBatchToRegularChangeBatch() {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperBoundItemId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUpperBoundItemId(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetBatchLevelKnowledgeShouldBeApplied<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBatchLevelKnowledgeShouldBeApplied(::core::mem::transmute_copy(&pfbatchknowledgeshouldbeapplied)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFilterInfo: GetFilterInfo::<Identity, Impl, OFFSET>,
            ConvertFullEnumerationChangeBatchToRegularChangeBatch: ConvertFullEnumerationChangeBatchToRegularChangeBatch::<Identity, Impl, OFFSET>,
            GetUpperBoundItemId: GetUpperBoundItemId::<Identity, Impl, OFFSET>,
            GetBatchLevelKnowledgeShouldBeApplied: GetBatchLevelKnowledgeShouldBeApplied::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchBase_Impl: Sized {
    fn GetChangeEnumerator(&self) -> ::windows::core::Result<IEnumSyncChanges>;
    fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()>;
    fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()>;
    fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::core::Result<()>;
    fn EndOrderedGroup(&self, pbupperbound: *const u8, pmadewithknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder>;
    fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetSourceForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge>;
    fn SetLastBatch(&self) -> ::windows::core::Result<()>;
    fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::core::Result<()>;
    fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::core::Result<()>;
    fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>() -> ISyncChangeBatchBase_Vtbl {
        unsafe extern "system" fn GetChangeEnumerator<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChangeEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsLastBatch<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIsLastBatch(::core::mem::transmute_copy(&pflastbatch)).into()
        }
        unsafe extern "system" fn GetWorkEstimateForBatch<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwworkforbatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWorkEstimateForBatch(::core::mem::transmute_copy(&pdwworkforbatch)).into()
        }
        unsafe extern "system" fn GetRemainingWorkEstimateForSession<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRemainingWorkEstimateForSession(::core::mem::transmute_copy(&pdwremainingworkforsession)).into()
        }
        unsafe extern "system" fn BeginOrderedGroup<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblowerbound: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginOrderedGroup(::core::mem::transmute_copy(&pblowerbound)).into()
        }
        unsafe extern "system" fn EndOrderedGroup<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndOrderedGroup(::core::mem::transmute_copy(&pbupperbound), ::core::mem::transmute(&pmadewithknowledge)).into()
        }
        unsafe extern "system" fn AddItemMetadataToGroup<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddItemMetadataToGroup(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprerequisteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrerequisiteKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprerequisteknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceForgottenKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsourceforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceForgottenKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsourceforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastBatch<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLastBatch().into()
        }
        unsafe extern "system" fn SetWorkEstimateForBatch<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwworkforbatch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWorkEstimateForBatch(::core::mem::transmute_copy(&dwworkforbatch)).into()
        }
        unsafe extern "system" fn SetRemainingWorkEstimateForSession<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwremainingworkforsession: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemainingWorkEstimateForSession(::core::mem::transmute_copy(&dwremainingworkforsession)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbchangebatch), ::core::mem::transmute_copy(&pcbchangebatch)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetChangeEnumerator: GetChangeEnumerator::<Identity, Impl, OFFSET>,
            GetIsLastBatch: GetIsLastBatch::<Identity, Impl, OFFSET>,
            GetWorkEstimateForBatch: GetWorkEstimateForBatch::<Identity, Impl, OFFSET>,
            GetRemainingWorkEstimateForSession: GetRemainingWorkEstimateForSession::<Identity, Impl, OFFSET>,
            BeginOrderedGroup: BeginOrderedGroup::<Identity, Impl, OFFSET>,
            EndOrderedGroup: EndOrderedGroup::<Identity, Impl, OFFSET>,
            AddItemMetadataToGroup: AddItemMetadataToGroup::<Identity, Impl, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, Impl, OFFSET>,
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, Impl, OFFSET>,
            GetSourceForgottenKnowledge: GetSourceForgottenKnowledge::<Identity, Impl, OFFSET>,
            SetLastBatch: SetLastBatch::<Identity, Impl, OFFSET>,
            SetWorkEstimateForBatch: SetWorkEstimateForBatch::<Identity, Impl, OFFSET>,
            SetRemainingWorkEstimateForSession: SetRemainingWorkEstimateForSession::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchBase2_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchBase2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase2_Impl, const OFFSET: isize>() -> ISyncChangeBatchBase2_Vtbl {
        unsafe extern "system" fn SerializeWithOptions<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SerializeWithOptions(::core::mem::transmute_copy(&targetformatversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        Self { base: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, OFFSET>(), SerializeWithOptions: SerializeWithOptions::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase2 as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeBatchWithFilterKeyMap_Impl: Sized {
    fn GetFilterKeyMap(&self) -> ::windows::core::Result<IFilterKeyMap>;
    fn SetFilterKeyMap(&self, pifilterkeymap: &::core::option::Option<IFilterKeyMap>) -> ::windows::core::Result<()>;
    fn SetFilterForgottenKnowledge(&self, dwfilterkey: u32, pfilterforgottenknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn GetFilteredReplicaLearnedKnowledge(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
}
impl ISyncChangeBatchWithFilterKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>() -> ISyncChangeBatchWithFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetFilterKeyMap<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifilterkeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilterKeyMap() {
                ::core::result::Result::Ok(ok__) => {
                    *ppifilterkeymap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterKeyMap<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifilterkeymap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFilterKeyMap(::core::mem::transmute(&pifilterkeymap)).into()
        }
        unsafe extern "system" fn SetFilterForgottenKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFilterForgottenKnowledge(::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute(&pfilterforgottenknowledge)).into()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilteredReplicaLearnedKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFilterKeyMap: GetFilterKeyMap::<Identity, Impl, OFFSET>,
            SetFilterKeyMap: SetFilterKeyMap::<Identity, Impl, OFFSET>,
            SetFilterForgottenKnowledge: SetFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithFilterKeyMap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchWithPrerequisite_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn SetPrerequisiteKnowledge(&self, pprerequisiteknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn GetLearnedKnowledgeWithPrerequisite(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchWithPrerequisite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>() -> ISyncChangeBatchWithPrerequisite_Vtbl {
        unsafe extern "system" fn SetPrerequisiteKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrerequisiteKnowledge(::core::mem::transmute(&pprerequisiteknowledge)).into()
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedwithprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedKnowledgeWithPrerequisite(::core::mem::transmute(&pdestinationknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedwithprerequisiteknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedForgottenKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPrerequisiteKnowledge: SetPrerequisiteKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, Impl, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithPrerequisite as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeBuilder_Impl: Sized {
    fn AddChangeUnitMetadata(&self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
}
impl ISyncChangeBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBuilder_Impl, const OFFSET: isize>() -> ISyncChangeBuilder_Vtbl {
        unsafe extern "system" fn AddChangeUnitMetadata<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddChangeUnitMetadata(::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pchangeunitversion)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AddChangeUnitMetadata: AddChangeUnitMetadata::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBuilder as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeUnit_Impl: Sized {
    fn GetItemChange(&self) -> ::windows::core::Result<ISyncChange>;
    fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
}
impl ISyncChangeUnit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeUnit_Impl, const OFFSET: isize>() -> ISyncChangeUnit_Vtbl {
        unsafe extern "system" fn GetItemChange<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChangeUnitId(::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitVersion<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChangeUnitVersion(::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetItemChange: GetItemChange::<Identity, Impl, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, Impl, OFFSET>,
            GetChangeUnitVersion: GetChangeUnitVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeUnit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeWithFilterKeyMap_Impl: Sized {
    fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilterChange(&self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::Result<()>;
    fn GetAllChangeUnitsPresentFlag(&self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFilterForgottenKnowledge(&self, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedKnowledge(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeWithFilterKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>() -> ISyncChangeWithFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetFilterCount<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilterCount(::core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterChange<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilterChange(::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn GetAllChangeUnitsPresentFlag<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAllChangeUnitsPresentFlag(::core::mem::transmute_copy(&pfallchangeunitspresent)).into()
        }
        unsafe extern "system" fn GetFilterForgottenKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilterForgottenKnowledge(::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilteredReplicaLearnedKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFilterCount: GetFilterCount::<Identity, Impl, OFFSET>,
            GetFilterChange: GetFilterChange::<Identity, Impl, OFFSET>,
            GetAllChangeUnitsPresentFlag: GetAllChangeUnitsPresentFlag::<Identity, Impl, OFFSET>,
            GetFilterForgottenKnowledge: GetFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeWithFilterKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeWithPrerequisite_Impl: Sized {
    fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledgeWithPrerequisite(&self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
}
impl ISyncChangeWithPrerequisite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>() -> ISyncChangeWithPrerequisite_Vtbl {
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrerequisiteKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprerequisiteknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedknowledgewithprerequisite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedKnowledgeWithPrerequisite(::core::mem::transmute(&pdestinationknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledgewithprerequisite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeWithPrerequisite as ::windows::core::Interface>::IID
    }
}
pub trait ISyncConstraintCallback_Impl: Sized {
    fn OnConstraintConflict(&self, pconflict: &::core::option::Option<IConstraintConflict>) -> ::windows::core::Result<()>;
}
impl ISyncConstraintCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncConstraintCallback_Impl, const OFFSET: isize>() -> ISyncConstraintCallback_Vtbl {
        unsafe extern "system" fn OnConstraintConflict<Identity: ::windows::core::IUnknownImpl, Impl: ISyncConstraintCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConstraintConflict(::core::mem::transmute(&pconflict)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnConstraintConflict: OnConstraintConflict::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncConstraintCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISyncDataConverter_Impl: Sized {
    fn ConvertDataRetrieverFromProviderFormat(&self, punkdataretrieverin: &::core::option::Option<::windows::core::IUnknown>, penumsyncchanges: &::core::option::Option<IEnumSyncChanges>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ConvertDataRetrieverToProviderFormat(&self, punkdataretrieverin: &::core::option::Option<::windows::core::IUnknown>, penumsyncchanges: &::core::option::Option<IEnumSyncChanges>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ConvertDataFromProviderFormat(&self, pdatacontext: &::core::option::Option<ILoadChangeContext>, punkdatain: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ConvertDataToProviderFormat(&self, pdatacontext: &::core::option::Option<ILoadChangeContext>, punkdataout: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ISyncDataConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncDataConverter_Impl, const OFFSET: isize>() -> ISyncDataConverter_Vtbl {
        unsafe extern "system" fn ConvertDataRetrieverFromProviderFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConvertDataRetrieverFromProviderFormat(::core::mem::transmute(&punkdataretrieverin), ::core::mem::transmute(&penumsyncchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataRetrieverToProviderFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConvertDataRetrieverToProviderFormat(::core::mem::transmute(&punkdataretrieverin), ::core::mem::transmute(&penumsyncchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataFromProviderFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConvertDataFromProviderFormat(::core::mem::transmute(&pdatacontext), ::core::mem::transmute(&punkdatain)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataToProviderFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConvertDataToProviderFormat(::core::mem::transmute(&pdatacontext), ::core::mem::transmute(&punkdataout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConvertDataRetrieverFromProviderFormat: ConvertDataRetrieverFromProviderFormat::<Identity, Impl, OFFSET>,
            ConvertDataRetrieverToProviderFormat: ConvertDataRetrieverToProviderFormat::<Identity, Impl, OFFSET>,
            ConvertDataFromProviderFormat: ConvertDataFromProviderFormat::<Identity, Impl, OFFSET>,
            ConvertDataToProviderFormat: ConvertDataToProviderFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncDataConverter as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilter_Impl: Sized {
    fn IsIdentical(&self, psyncfilter: &::core::option::Option<ISyncFilter>) -> ::windows::core::Result<()>;
    fn Serialize(&self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilter_Impl, const OFFSET: isize>() -> ISyncFilter_Vtbl {
        unsafe extern "system" fn IsIdentical<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsIdentical(::core::mem::transmute(&psyncfilter)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbsyncfilter), ::core::mem::transmute_copy(&pcbsyncfilter)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsIdentical: IsIdentical::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilter as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterDeserializer_Impl: Sized {
    fn DeserializeSyncFilter(&self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> ::windows::core::Result<ISyncFilter>;
}
impl ISyncFilterDeserializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterDeserializer_Impl, const OFFSET: isize>() -> ISyncFilterDeserializer_Vtbl {
        unsafe extern "system" fn DeserializeSyncFilter<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterDeserializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeserializeSyncFilter(::core::mem::transmute_copy(&pbsyncfilter), ::core::mem::transmute_copy(&dwcbsyncfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppisyncfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), DeserializeSyncFilter: DeserializeSyncFilter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterDeserializer as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterInfo_Impl: Sized {
    fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncFilterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfo_Impl, const OFFSET: isize>() -> ISyncFilterInfo_Vtbl {
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pcbbuffer)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Serialize: Serialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterInfo2_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncFilterInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfo2_Impl, const OFFSET: isize>() -> ISyncFilterInfo2_Vtbl {
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base: ISyncFilterInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetFlags: GetFlags::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterInfo2 as ::windows::core::Interface>::IID || iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFullEnumerationChange_Impl: Sized {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge>;
}
impl ISyncFullEnumerationChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChange_Impl, const OFFSET: isize>() -> ISyncFullEnumerationChange_Vtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedKnowledgeAfterRecoveryComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedForgottenKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncFullEnumerationChangeBatch_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncFullEnumerationChangeBatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>() -> ISyncFullEnumerationChangeBatch_Vtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLearnedKnowledgeAfterRecoveryComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledgeafterrecoverycomplete = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosedLowerBoundItemId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClosedLowerBoundItemId(::core::mem::transmute_copy(&pbclosedlowerbounditemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedUpperBoundItemId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClosedUpperBoundItemId(::core::mem::transmute_copy(&pbclosedupperbounditemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetClosedLowerBoundItemId: GetClosedLowerBoundItemId::<Identity, Impl, OFFSET>,
            GetClosedUpperBoundItemId: GetClosedUpperBoundItemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncFullEnumerationChangeBatch2_Impl: Sized + ISyncChangeBatchBase_Impl + ISyncFullEnumerationChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncFullEnumerationChangeBatch2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch2_Impl, const OFFSET: isize>() -> ISyncFullEnumerationChangeBatch2_Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddMergeTombstoneMetadataToGroup(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISyncFullEnumerationChangeBatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch2 as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID || iid == &<ISyncFullEnumerationChangeBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncKnowledge_Impl: Sized {
    fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn Serialize(&self, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::Result<()>;
    fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows::core::Result<()>;
    fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
    fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
    fn GetScopeVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetReplicaKeyMap(&self) -> ::windows::core::Result<IReplicaKeyMap>;
    fn Clone(&self) -> ::windows::core::Result<ISyncKnowledge>;
    fn ConvertVersion(&self, pknowledgein: &::core::option::Option<ISyncKnowledge>, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
    fn MapRemoteToLocal(&self, premoteknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn Union(&self, pknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows::core::Result<ISyncKnowledge>;
    fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<ISyncKnowledge>;
    fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows::core::Result<ISyncKnowledge>;
    fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows::core::Result<()>;
    fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()>;
    fn ContainsKnowledge(&self, pknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::Result<()>;
    fn GetRangeExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetSingleItemExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetChangeUnitExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetVersion(&self, pdwversion: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncKnowledge_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>() -> ISyncKnowledge_Vtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOwnerReplicaId(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&fserializereplicakeymap), ::core::mem::transmute_copy(&pbknowledge), ::core::mem::transmute_copy(&pcbknowledge)).into()
        }
        unsafe extern "system" fn SetLocalTickCount<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltickcount: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalTickCount(::core::mem::transmute_copy(&ulltickcount)).into()
        }
        unsafe extern "system" fn ContainsChange<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ContainsChange(::core::mem::transmute_copy(&pbversionownerreplicaid), ::core::mem::transmute_copy(&pgiditemid), ::core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn ContainsChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ContainsChangeUnit(::core::mem::transmute_copy(&pbversionownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn GetScopeVector<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScopeVector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetReplicaKeyMap<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreplicakeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReplicaKeyMap() {
                ::core::result::Result::Ok(ok__) => {
                    *ppreplicakeymap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclonedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclonedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertVersion<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledgein: ::windows::core::RawPtr, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertVersion(::core::mem::transmute(&pknowledgein), ::core::mem::transmute_copy(&pbcurrentownerid), ::core::mem::transmute_copy(&pversionin), ::core::mem::transmute_copy(&pbnewownerid), ::core::mem::transmute_copy(&pcbidsize), ::core::mem::transmute_copy(&pversionout)).into()
        }
        unsafe extern "system" fn MapRemoteToLocal<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteknowledge: ::windows::core::RawPtr, ppmappedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MapRemoteToLocal(::core::mem::transmute(&premoteknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmappedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Union<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Union(::core::mem::transmute(&pknowledge)).into()
        }
        unsafe extern "system" fn ProjectOntoItem<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProjectOntoItem(::core::mem::transmute_copy(&pbitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProjectOntoChangeUnit(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoRange<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProjectOntoRange(::core::mem::transmute_copy(&psrngsyncrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcludeItem<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExcludeItem(::core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ExcludeChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExcludeChangeUnit(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ContainsKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ContainsKnowledge(::core::mem::transmute(&pknowledge)).into()
        }
        unsafe extern "system" fn FindMinTickCountForReplica<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindMinTickCountForReplica(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pullreplicatickcount)).into()
        }
        unsafe extern "system" fn GetRangeExceptions<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRangeExceptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetSingleItemExceptions<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSingleItemExceptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetChangeUnitExceptions<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChangeUnitExceptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForItem<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindClockVectorForItem(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindClockVectorForChangeUnit(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVersion(::core::mem::transmute_copy(&pdwversion)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            SetLocalTickCount: SetLocalTickCount::<Identity, Impl, OFFSET>,
            ContainsChange: ContainsChange::<Identity, Impl, OFFSET>,
            ContainsChangeUnit: ContainsChangeUnit::<Identity, Impl, OFFSET>,
            GetScopeVector: GetScopeVector::<Identity, Impl, OFFSET>,
            GetReplicaKeyMap: GetReplicaKeyMap::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            ConvertVersion: ConvertVersion::<Identity, Impl, OFFSET>,
            MapRemoteToLocal: MapRemoteToLocal::<Identity, Impl, OFFSET>,
            Union: Union::<Identity, Impl, OFFSET>,
            ProjectOntoItem: ProjectOntoItem::<Identity, Impl, OFFSET>,
            ProjectOntoChangeUnit: ProjectOntoChangeUnit::<Identity, Impl, OFFSET>,
            ProjectOntoRange: ProjectOntoRange::<Identity, Impl, OFFSET>,
            ExcludeItem: ExcludeItem::<Identity, Impl, OFFSET>,
            ExcludeChangeUnit: ExcludeChangeUnit::<Identity, Impl, OFFSET>,
            ContainsKnowledge: ContainsKnowledge::<Identity, Impl, OFFSET>,
            FindMinTickCountForReplica: FindMinTickCountForReplica::<Identity, Impl, OFFSET>,
            GetRangeExceptions: GetRangeExceptions::<Identity, Impl, OFFSET>,
            GetSingleItemExceptions: GetSingleItemExceptions::<Identity, Impl, OFFSET>,
            GetChangeUnitExceptions: GetChangeUnitExceptions::<Identity, Impl, OFFSET>,
            FindClockVectorForItem: FindClockVectorForItem::<Identity, Impl, OFFSET>,
            FindClockVectorForChangeUnit: FindClockVectorForChangeUnit::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncKnowledge as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncKnowledge2_Impl: Sized + ISyncKnowledge_Impl {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
    fn ProjectOntoColumnSet(&self, ppcolumns: *const *const u8, count: u32) -> ::windows::core::Result<ISyncKnowledge2>;
    fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetLowestUncontainedId(&self, pisyncknowledge: &::core::option::Option<ISyncKnowledge2>, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetInspector(&self, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetMinimumSupportedVersion(&self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::Result<()>;
    fn GetStatistics(&self, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::Result<()>;
    fn ContainsKnowledgeForItem(&self, pknowledge: &::core::option::Option<ISyncKnowledge>, pbitemid: *const u8) -> ::windows::core::Result<()>;
    fn ContainsKnowledgeForChangeUnit(&self, pknowledge: &::core::option::Option<ISyncKnowledge>, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()>;
    fn ProjectOntoKnowledgeWithPrerequisite(&self, pprerequisiteknowledge: &::core::option::Option<ISyncKnowledge>, ptemplateknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn Complement(&self, psyncknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn IntersectsWithKnowledge(&self, psyncknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn GetKnowledgeCookie(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CompareToKnowledgeCookie(&self, pknowledgecookie: &::core::option::Option<::windows::core::IUnknown>, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncKnowledge2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>() -> ISyncKnowledge2_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn ProjectOntoColumnSet<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProjectOntoColumnSet(::core::mem::transmute_copy(&ppcolumns), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerializeWithOptions<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SerializeWithOptions(::core::mem::transmute_copy(&targetformatversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        unsafe extern "system" fn GetLowestUncontainedId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncknowledge: ::windows::core::RawPtr, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLowestUncontainedId(::core::mem::transmute(&pisyncknowledge), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        unsafe extern "system" fn GetInspector<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInspector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppiinspector)).into()
        }
        unsafe extern "system" fn GetMinimumSupportedVersion<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMinimumSupportedVersion(::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&which), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForItem<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ContainsKnowledgeForItem(::core::mem::transmute(&pknowledge), ::core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForChangeUnit<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ContainsKnowledgeForChangeUnit(::core::mem::transmute(&pknowledge), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ProjectOntoKnowledgeWithPrerequisite<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr, ptemplateknowledge: ::windows::core::RawPtr, ppprojectedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProjectOntoKnowledgeWithPrerequisite(::core::mem::transmute(&pprerequisiteknowledge), ::core::mem::transmute(&ptemplateknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprojectedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complement<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr, ppcomplementedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Complement(::core::mem::transmute(&psyncknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomplementedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntersectsWithKnowledge<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IntersectsWithKnowledge(::core::mem::transmute(&psyncknowledge)).into()
        }
        unsafe extern "system" fn GetKnowledgeCookie<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKnowledgeCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgecookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareToKnowledgeCookie<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompareToKnowledgeCookie(::core::mem::transmute(&pknowledgecookie), ::core::mem::transmute_copy(&presult)).into()
        }
        Self {
            base: ISyncKnowledge_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
            ProjectOntoColumnSet: ProjectOntoColumnSet::<Identity, Impl, OFFSET>,
            SerializeWithOptions: SerializeWithOptions::<Identity, Impl, OFFSET>,
            GetLowestUncontainedId: GetLowestUncontainedId::<Identity, Impl, OFFSET>,
            GetInspector: GetInspector::<Identity, Impl, OFFSET>,
            GetMinimumSupportedVersion: GetMinimumSupportedVersion::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            ContainsKnowledgeForItem: ContainsKnowledgeForItem::<Identity, Impl, OFFSET>,
            ContainsKnowledgeForChangeUnit: ContainsKnowledgeForChangeUnit::<Identity, Impl, OFFSET>,
            ProjectOntoKnowledgeWithPrerequisite: ProjectOntoKnowledgeWithPrerequisite::<Identity, Impl, OFFSET>,
            Complement: Complement::<Identity, Impl, OFFSET>,
            IntersectsWithKnowledge: IntersectsWithKnowledge::<Identity, Impl, OFFSET>,
            GetKnowledgeCookie: GetKnowledgeCookie::<Identity, Impl, OFFSET>,
            CompareToKnowledgeCookie: CompareToKnowledgeCookie::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncKnowledge2 as ::windows::core::Interface>::IID || iid == &<ISyncKnowledge as ::windows::core::Interface>::IID
    }
}
pub trait ISyncMergeTombstoneChange_Impl: Sized {
    fn GetWinnerItemId(&self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncMergeTombstoneChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncMergeTombstoneChange_Impl, const OFFSET: isize>() -> ISyncMergeTombstoneChange_Vtbl {
        unsafe extern "system" fn GetWinnerItemId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncMergeTombstoneChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWinnerItemId(::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetWinnerItemId: GetWinnerItemId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncMergeTombstoneChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncProvider_Impl: Sized {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProvider_Impl, const OFFSET: isize>() -> ISyncProvider_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderConfigUI_Impl: Sized {
    fn Init(&self, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn GetRegisteredProperties(&self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateAndRegisterNewSyncProvider(&self, hwndparent: super::super::Foundation::HWND, punkcontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<ISyncProviderInfo>;
    fn ModifySyncProvider(&self, hwndparent: super::super::Foundation::HWND, punkcontext: &::core::option::Option<::windows::core::IUnknown>, pproviderinfo: &::core::option::Option<ISyncProviderInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderConfigUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>() -> ISyncProviderConfigUI_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute(&pconfigurationproperties)).into()
        }
        unsafe extern "system" fn GetRegisteredProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconfiguiproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegisteredProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfiguiproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndRegisterNewSyncProvider<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAndRegisterNewSyncProvider(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&punkcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifySyncProvider<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ModifySyncProvider(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&punkcontext), ::core::mem::transmute(&pproviderinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetRegisteredProperties: GetRegisteredProperties::<Identity, Impl, OFFSET>,
            CreateAndRegisterNewSyncProvider: CreateAndRegisterNewSyncProvider::<Identity, Impl, OFFSET>,
            ModifySyncProvider: ModifySyncProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderConfigUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderConfigUIInfo_Impl: Sized + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProviderConfigUI(&self, dwclscontext: u32) -> ::windows::core::Result<ISyncProviderConfigUI>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderConfigUIInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUIInfo_Impl, const OFFSET: isize>() -> ISyncProviderConfigUIInfo_Vtbl {
        unsafe extern "system" fn GetSyncProviderConfigUI<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncProviderConfigUI(::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncproviderconfigui = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSyncProviderConfigUI: GetSyncProviderConfigUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderConfigUIInfo as ::windows::core::Interface>::IID || iid == &<super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderInfo_Impl: Sized + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProvider(&self, dwclscontext: u32) -> ::windows::core::Result<IRegisteredSyncProvider>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderInfo_Impl, const OFFSET: isize>() -> ISyncProviderInfo_Vtbl {
        unsafe extern "system" fn GetSyncProvider<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncProvider(::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSyncProvider: GetSyncProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderInfo as ::windows::core::Interface>::IID || iid == &<super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderRegistration_Impl: Sized {
    fn CreateSyncProviderConfigUIRegistrationInstance(&self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> ::windows::core::Result<ISyncProviderConfigUIInfo>;
    fn UnregisterSyncProviderConfigUI(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumerateSyncProviderConfigUIs(&self, pguidcontenttype: *const ::windows::core::GUID, dwsupportedarchitecture: u32) -> ::windows::core::Result<IEnumSyncProviderConfigUIInfos>;
    fn CreateSyncProviderRegistrationInstance(&self, pproviderconfiguration: *const SyncProviderConfiguration) -> ::windows::core::Result<ISyncProviderInfo>;
    fn UnregisterSyncProvider(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetSyncProviderConfigUIInfoforProvider(&self, pguidproviderinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<ISyncProviderConfigUIInfo>;
    fn EnumerateSyncProviders(&self, pguidcontenttype: *const ::windows::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::core::GUID, dwsupportedarchitecture: u32) -> ::windows::core::Result<IEnumSyncProviderInfos>;
    fn GetSyncProviderInfo(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<ISyncProviderInfo>;
    fn GetSyncProviderFromInstanceId(&self, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32) -> ::windows::core::Result<IRegisteredSyncProvider>;
    fn GetSyncProviderConfigUIInfo(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<ISyncProviderConfigUIInfo>;
    fn GetSyncProviderConfigUIFromInstanceId(&self, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32) -> ::windows::core::Result<ISyncProviderConfigUI>;
    fn GetSyncProviderState(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn SetSyncProviderState(&self, pguidinstanceid: *const ::windows::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::core::Result<()>;
    fn RegisterForEvent(&self, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn RevokeEvent(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetChange(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<ISyncRegistrationChange>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>() -> ISyncProviderRegistration_Vtbl {
        unsafe extern "system" fn CreateSyncProviderConfigUIRegistrationInstance<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSyncProviderConfigUIRegistrationInstance(::core::mem::transmute_copy(&pconfiguiconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfiguiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProviderConfigUI<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterSyncProviderConfigUI(::core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn EnumerateSyncProviderConfigUIs<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateSyncProviderConfigUIs(::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute_copy(&dwsupportedarchitecture)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsyncproviderconfiguiinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSyncProviderRegistrationInstance<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSyncProviderRegistrationInstance(::core::mem::transmute_copy(&pproviderconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProvider<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterSyncProvider(::core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfoforProvider<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidproviderinstanceid: *const ::windows::core::GUID, ppproviderconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncProviderConfigUIInfoforProvider(::core::mem::transmute_copy(&pguidproviderinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderconfiguiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSyncProviders<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateSyncProviders(::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute_copy(&dwstateflagstofiltermask), ::core::mem::transmute_copy(&dwstateflagstofilter), ::core::mem::transmute_copy(&refproviderclsid), ::core::mem::transmute_copy(&dwsupportedarchitecture)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsyncproviderinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncProviderInfo(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderFromInstanceId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncProviderFromInstanceId(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncProviderConfigUIInfo(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfiguiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIFromInstanceId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncProviderConfigUIFromInstanceId(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfigui = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderState<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pdwstateflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncProviderState(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstateflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncProviderState<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSyncProviderState(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwstateflagsmask), ::core::mem::transmute_copy(&dwstateflags)).into()
        }
        unsafe extern "system" fn RegisterForEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterForEvent(::core::mem::transmute_copy(&phevent)).into()
        }
        unsafe extern "system" fn RevokeEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RevokeEvent(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn GetChange<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChange(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateSyncProviderConfigUIRegistrationInstance: CreateSyncProviderConfigUIRegistrationInstance::<Identity, Impl, OFFSET>,
            UnregisterSyncProviderConfigUI: UnregisterSyncProviderConfigUI::<Identity, Impl, OFFSET>,
            EnumerateSyncProviderConfigUIs: EnumerateSyncProviderConfigUIs::<Identity, Impl, OFFSET>,
            CreateSyncProviderRegistrationInstance: CreateSyncProviderRegistrationInstance::<Identity, Impl, OFFSET>,
            UnregisterSyncProvider: UnregisterSyncProvider::<Identity, Impl, OFFSET>,
            GetSyncProviderConfigUIInfoforProvider: GetSyncProviderConfigUIInfoforProvider::<Identity, Impl, OFFSET>,
            EnumerateSyncProviders: EnumerateSyncProviders::<Identity, Impl, OFFSET>,
            GetSyncProviderInfo: GetSyncProviderInfo::<Identity, Impl, OFFSET>,
            GetSyncProviderFromInstanceId: GetSyncProviderFromInstanceId::<Identity, Impl, OFFSET>,
            GetSyncProviderConfigUIInfo: GetSyncProviderConfigUIInfo::<Identity, Impl, OFFSET>,
            GetSyncProviderConfigUIFromInstanceId: GetSyncProviderConfigUIFromInstanceId::<Identity, Impl, OFFSET>,
            GetSyncProviderState: GetSyncProviderState::<Identity, Impl, OFFSET>,
            SetSyncProviderState: SetSyncProviderState::<Identity, Impl, OFFSET>,
            RegisterForEvent: RegisterForEvent::<Identity, Impl, OFFSET>,
            RevokeEvent: RevokeEvent::<Identity, Impl, OFFSET>,
            GetChange: GetChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderRegistration as ::windows::core::Interface>::IID
    }
}
pub trait ISyncRegistrationChange_Impl: Sized {
    fn GetEvent(&self) -> ::windows::core::Result<SYNC_REGISTRATION_EVENT>;
    fn GetInstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ISyncRegistrationChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncRegistrationChange_Impl, const OFFSET: isize>() -> ISyncRegistrationChange_Vtbl {
        unsafe extern "system" fn GetEvent<Identity: ::windows::core::IUnknownImpl, Impl: ISyncRegistrationChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *psreevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Identity: ::windows::core::IUnknownImpl, Impl: ISyncRegistrationChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncRegistrationChange as ::windows::core::Interface>::IID
    }
}
pub trait ISyncSessionExtendedErrorInfo_Impl: Sized {
    fn GetSyncProviderWithError(&self) -> ::windows::core::Result<ISyncProvider>;
}
impl ISyncSessionExtendedErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionExtendedErrorInfo_Impl, const OFFSET: isize>() -> ISyncSessionExtendedErrorInfo_Vtbl {
        unsafe extern "system" fn GetSyncProviderWithError<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionExtendedErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproviderwitherror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncProviderWithError() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderwitherror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSyncProviderWithError: GetSyncProviderWithError::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionExtendedErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncSessionState_Impl: Sized {
    fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::Result<()>;
    fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::Result<()>;
    fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> ::windows::core::Result<()>;
    fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncSessionState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState_Impl, const OFFSET: isize>() -> ISyncSessionState_Vtbl {
        unsafe extern "system" fn IsCanceled<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsCanceled(::core::mem::transmute_copy(&pfiscanceled)).into()
        }
        unsafe extern "system" fn GetInfoForChangeApplication<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInfoForChangeApplication(::core::mem::transmute_copy(&pbchangeapplierinfo), ::core::mem::transmute_copy(&pcbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn LoadInfoFromChangeApplication<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadInfoFromChangeApplication(::core::mem::transmute_copy(&pbchangeapplierinfo), ::core::mem::transmute_copy(&cbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeStart<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForgottenKnowledgeRecoveryRangeStart(::core::mem::transmute_copy(&pbrangestart), ::core::mem::transmute_copy(&pcbrangestart)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeEnd<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForgottenKnowledgeRecoveryRangeEnd(::core::mem::transmute_copy(&pbrangeend), ::core::mem::transmute_copy(&pcbrangeend)).into()
        }
        unsafe extern "system" fn SetForgottenKnowledgeRecoveryRange<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *const SYNC_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForgottenKnowledgeRecoveryRange(::core::mem::transmute_copy(&prange)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&provider), ::core::mem::transmute_copy(&syncstage), ::core::mem::transmute_copy(&dwcompletedwork), ::core::mem::transmute_copy(&dwtotalwork)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsCanceled: IsCanceled::<Identity, Impl, OFFSET>,
            GetInfoForChangeApplication: GetInfoForChangeApplication::<Identity, Impl, OFFSET>,
            LoadInfoFromChangeApplication: LoadInfoFromChangeApplication::<Identity, Impl, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeStart: GetForgottenKnowledgeRecoveryRangeStart::<Identity, Impl, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeEnd: GetForgottenKnowledgeRecoveryRangeEnd::<Identity, Impl, OFFSET>,
            SetForgottenKnowledgeRecoveryRange: SetForgottenKnowledgeRecoveryRange::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncSessionState2_Impl: Sized + ISyncSessionState_Impl {
    fn SetProviderWithError(&self, fself: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSessionErrorStatus(&self, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncSessionState2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState2_Impl, const OFFSET: isize>() -> ISyncSessionState2_Vtbl {
        unsafe extern "system" fn SetProviderWithError<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fself: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProviderWithError(::core::mem::transmute_copy(&fself)).into()
        }
        unsafe extern "system" fn GetSessionErrorStatus<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSessionErrorStatus(::core::mem::transmute_copy(&phrsessionerror)).into()
        }
        Self {
            base: ISyncSessionState_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProviderWithError: SetProviderWithError::<Identity, Impl, OFFSET>,
            GetSessionErrorStatus: GetSessionErrorStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionState2 as ::windows::core::Interface>::IID || iid == &<ISyncSessionState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronousDataRetriever_Impl: Sized {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
    fn LoadChangeData(&self, ploadchangecontext: &::core::option::Option<ILoadChangeContext>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISynchronousDataRetriever_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronousDataRetriever_Impl, const OFFSET: isize>() -> ISynchronousDataRetriever_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn LoadChangeData<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadChangeData(::core::mem::transmute(&ploadchangecontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronousDataRetriever as ::windows::core::Interface>::IID
    }
}
