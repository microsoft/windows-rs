#[cfg(feature = "Win32_Foundation")]
pub trait IAsynchronousDataRetriever_Impl: Sized {
    fn GetIdParameters(&mut self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
    fn RegisterCallback(&mut self, pdataretrievercallback: &::core::option::Option<IDataRetrieverCallback>) -> ::windows::core::Result<()>;
    fn RevokeCallback(&mut self, pdataretrievercallback: &::core::option::Option<IDataRetrieverCallback>) -> ::windows::core::Result<()>;
    fn LoadChangeData(&mut self, ploadchangecontext: &::core::option::Option<ILoadChangeContext>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAsynchronousDataRetriever_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsynchronousDataRetriever_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsynchronousDataRetriever_Vtbl {
        unsafe extern "system" fn GetIdParameters<Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn RegisterCallback<Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCallback(::core::mem::transmute(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn RevokeCallback<Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeCallback(::core::mem::transmute(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn LoadChangeData<Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadChangeData(::core::mem::transmute(&ploadchangecontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIdParameters: GetIdParameters::<Impl, IMPL_OFFSET>,
            RegisterCallback: RegisterCallback::<Impl, IMPL_OFFSET>,
            RevokeCallback: RevokeCallback::<Impl, IMPL_OFFSET>,
            LoadChangeData: LoadChangeData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsynchronousDataRetriever as ::windows::core::Interface>::IID
    }
}
pub trait IChangeConflict_Impl: Sized {
    fn GetDestinationProviderConflictingChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSourceProviderConflictingData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetResolveActionForChange(&mut self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetResolveActionForChange(&mut self, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn GetResolveActionForChangeUnit(&mut self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetResolveActionForChangeUnit(&mut self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
}
impl IChangeConflict_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflict_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChangeConflict_Vtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolveActionForChange<Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResolveActionForChange(::core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChange<Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResolveActionForChange(::core::mem::transmute_copy(&resolveaction)).into()
        }
        unsafe extern "system" fn GetResolveActionForChangeUnit<Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChangeUnit<Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&resolveaction)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Impl, IMPL_OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Impl, IMPL_OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Impl, IMPL_OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Impl, IMPL_OFFSET>,
            GetResolveActionForChange: GetResolveActionForChange::<Impl, IMPL_OFFSET>,
            SetResolveActionForChange: SetResolveActionForChange::<Impl, IMPL_OFFSET>,
            GetResolveActionForChangeUnit: GetResolveActionForChangeUnit::<Impl, IMPL_OFFSET>,
            SetResolveActionForChangeUnit: SetResolveActionForChangeUnit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeConflict as ::windows::core::Interface>::IID
    }
}
pub trait IChangeUnitException_Impl: Sized {
    fn GetItemId(&mut self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitId(&mut self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClockVector(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IChangeUnitException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitException_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChangeUnitException_Vtbl {
        unsafe extern "system" fn GetItemId<Impl: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemId(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitId(::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Impl: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClockVector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetItemId: GetItemId::<Impl, IMPL_OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Impl, IMPL_OFFSET>,
            GetClockVector: GetClockVector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeUnitException as ::windows::core::Interface>::IID
    }
}
pub trait IChangeUnitListFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn Initialize(&mut self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitIdCount(&mut self, pdwchangeunitidcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitId(&mut self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
}
impl IChangeUnitListFilterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitListFilterInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChangeUnitListFilterInfo_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&ppbchangeunitids), ::core::mem::transmute_copy(&dwchangeunitcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitIdCount<Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitIdCount(::core::mem::transmute_copy(&pdwchangeunitidcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitId(::core::mem::transmute_copy(&dwchangeunitidindex), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base: ISyncFilterInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetChangeUnitIdCount: GetChangeUnitIdCount::<Impl, IMPL_OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeUnitListFilterInfo as ::windows::core::Interface>::IID || iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IClockVector_Impl: Sized {
    fn GetClockVectorElements(&mut self, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetClockVectorElementCount(&mut self, pdwcount: *mut u32) -> ::windows::core::Result<()>;
}
impl IClockVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClockVector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClockVector_Vtbl {
        unsafe extern "system" fn GetClockVectorElements<Impl: IClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClockVectorElements(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppienumclockvector)).into()
        }
        unsafe extern "system" fn GetClockVectorElementCount<Impl: IClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClockVectorElementCount(::core::mem::transmute_copy(&pdwcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClockVectorElements: GetClockVectorElements::<Impl, IMPL_OFFSET>,
            GetClockVectorElementCount: GetClockVectorElementCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IClockVectorElement_Impl: Sized {
    fn GetReplicaKey(&mut self, pdwreplicakey: *mut u32) -> ::windows::core::Result<()>;
    fn GetTickCount(&mut self, pulltickcount: *mut u64) -> ::windows::core::Result<()>;
}
impl IClockVectorElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClockVectorElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClockVectorElement_Vtbl {
        unsafe extern "system" fn GetReplicaKey<Impl: IClockVectorElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetReplicaKey(::core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn GetTickCount<Impl: IClockVectorElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulltickcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTickCount(::core::mem::transmute_copy(&pulltickcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetReplicaKey: GetReplicaKey::<Impl, IMPL_OFFSET>,
            GetTickCount: GetTickCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClockVectorElement as ::windows::core::Interface>::IID
    }
}
pub trait ICombinedFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetFilterCount(&mut self, pdwfiltercount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilterInfo(&mut self, dwfilterindex: u32) -> ::windows::core::Result<ISyncFilterInfo>;
    fn GetFilterCombinationType(&mut self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::Result<()>;
}
impl ICombinedFilterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICombinedFilterInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICombinedFilterInfo_Vtbl {
        unsafe extern "system" fn GetFilterCount<Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterCount(::core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterInfo<Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterInfo(::core::mem::transmute_copy(&dwfilterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifilterinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterCombinationType<Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterCombinationType(::core::mem::transmute_copy(&pfiltercombinationtype)).into()
        }
        Self {
            base: ISyncFilterInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFilterCount: GetFilterCount::<Impl, IMPL_OFFSET>,
            GetFilterInfo: GetFilterInfo::<Impl, IMPL_OFFSET>,
            GetFilterCombinationType: GetFilterCombinationType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICombinedFilterInfo as ::windows::core::Interface>::IID || iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IConstraintConflict_Impl: Sized {
    fn GetDestinationProviderConflictingChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetDestinationProviderOriginalChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSourceProviderConflictingData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetDestinationProviderOriginalData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetConstraintResolveActionForChange(&mut self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetConstraintResolveActionForChange(&mut self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn GetConstraintResolveActionForChangeUnit(&mut self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetConstraintResolveActionForChangeUnit(&mut self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn GetConstraintConflictReason(&mut self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::Result<()>;
    fn IsTemporary(&mut self) -> ::windows::core::Result<()>;
}
impl IConstraintConflict_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflict_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConstraintConflict_Vtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalChange<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginalchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderOriginalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pporiginalchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalData<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderOriginalData() {
                ::core::result::Result::Ok(ok__) => {
                    *pporiginaldata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstraintResolveActionForChange<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstraintResolveActionForChange(::core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChange<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConstraintResolveActionForChange(::core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintResolveActionForChangeUnit<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstraintResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChangeUnit<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConstraintResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintConflictReason<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstraintConflictReason(::core::mem::transmute_copy(&pconstraintconflictreason)).into()
        }
        unsafe extern "system" fn IsTemporary<Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsTemporary().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Impl, IMPL_OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Impl, IMPL_OFFSET>,
            GetDestinationProviderOriginalChange: GetDestinationProviderOriginalChange::<Impl, IMPL_OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Impl, IMPL_OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Impl, IMPL_OFFSET>,
            GetDestinationProviderOriginalData: GetDestinationProviderOriginalData::<Impl, IMPL_OFFSET>,
            GetConstraintResolveActionForChange: GetConstraintResolveActionForChange::<Impl, IMPL_OFFSET>,
            SetConstraintResolveActionForChange: SetConstraintResolveActionForChange::<Impl, IMPL_OFFSET>,
            GetConstraintResolveActionForChangeUnit: GetConstraintResolveActionForChangeUnit::<Impl, IMPL_OFFSET>,
            SetConstraintResolveActionForChangeUnit: SetConstraintResolveActionForChangeUnit::<Impl, IMPL_OFFSET>,
            GetConstraintConflictReason: GetConstraintConflictReason::<Impl, IMPL_OFFSET>,
            IsTemporary: IsTemporary::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConstraintConflict as ::windows::core::Interface>::IID
    }
}
pub trait IConstructReplicaKeyMap_Impl: Sized {
    fn FindOrAddReplica(&mut self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::Result<()>;
}
impl IConstructReplicaKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConstructReplicaKeyMap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConstructReplicaKeyMap_Vtbl {
        unsafe extern "system" fn FindOrAddReplica<Impl: IConstructReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindOrAddReplica(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FindOrAddReplica: FindOrAddReplica::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConstructReplicaKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait ICoreFragment_Impl: Sized {
    fn NextColumn(&mut self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::Result<()>;
    fn NextRange(&mut self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::core::option::Option<IClockVector>) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn GetColumnCount(&mut self, pcolumncount: *mut u32) -> ::windows::core::Result<()>;
    fn GetRangeCount(&mut self, prangecount: *mut u32) -> ::windows::core::Result<()>;
}
impl ICoreFragment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFragment_Vtbl {
        unsafe extern "system" fn NextColumn<Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextColumn(::core::mem::transmute_copy(&pchangeunitid), ::core::mem::transmute_copy(&pchangeunitidsize)).into()
        }
        unsafe extern "system" fn NextRange<Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextRange(::core::mem::transmute_copy(&pitemid), ::core::mem::transmute_copy(&pitemidsize), ::core::mem::transmute_copy(&piclockvector)).into()
        }
        unsafe extern "system" fn Reset<Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn GetColumnCount<Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolumncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumnCount(::core::mem::transmute_copy(&pcolumncount)).into()
        }
        unsafe extern "system" fn GetRangeCount<Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRangeCount(::core::mem::transmute_copy(&prangecount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NextColumn: NextColumn::<Impl, IMPL_OFFSET>,
            NextRange: NextRange::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            GetColumnCount: GetColumnCount::<Impl, IMPL_OFFSET>,
            GetRangeCount: GetRangeCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFragment as ::windows::core::Interface>::IID
    }
}
pub trait ICoreFragmentInspector_Impl: Sized {
    fn NextCoreFragments(&mut self, requestedcount: u32, ppicorefragments: *mut ::core::option::Option<ICoreFragment>, pfetchedcount: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
impl ICoreFragmentInspector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragmentInspector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFragmentInspector_Vtbl {
        unsafe extern "system" fn NextCoreFragments<Impl: ICoreFragmentInspector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut ::windows::core::RawPtr, pfetchedcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextCoreFragments(::core::mem::transmute_copy(&requestedcount), ::core::mem::transmute_copy(&ppicorefragments), ::core::mem::transmute_copy(&pfetchedcount)).into()
        }
        unsafe extern "system" fn Reset<Impl: ICoreFragmentInspector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NextCoreFragments: NextCoreFragments::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFragmentInspector as ::windows::core::Interface>::IID
    }
}
pub trait ICustomFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetSyncFilter(&mut self) -> ::windows::core::Result<ISyncFilter>;
}
impl ICustomFilterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomFilterInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomFilterInfo_Vtbl {
        unsafe extern "system" fn GetSyncFilter<Impl: ICustomFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *pisyncfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ISyncFilterInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetSyncFilter: GetSyncFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomFilterInfo as ::windows::core::Interface>::IID || iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IDataRetrieverCallback_Impl: Sized {
    fn LoadChangeDataComplete(&mut self, punkdata: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn LoadChangeDataError(&mut self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IDataRetrieverCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRetrieverCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataRetrieverCallback_Vtbl {
        unsafe extern "system" fn LoadChangeDataComplete<Impl: IDataRetrieverCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadChangeDataComplete(::core::mem::transmute(&punkdata)).into()
        }
        unsafe extern "system" fn LoadChangeDataError<Impl: IDataRetrieverCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadChangeDataError(::core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LoadChangeDataComplete: LoadChangeDataComplete::<Impl, IMPL_OFFSET>,
            LoadChangeDataError: LoadChangeDataError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataRetrieverCallback as ::windows::core::Interface>::IID
    }
}
pub trait IEnumChangeUnitExceptions_Impl: Sized {
    fn Next(&mut self, cexceptions: u32, ppchangeunitexception: *mut ::core::option::Option<IChangeUnitException>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cexceptions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumChangeUnitExceptions>;
}
impl IEnumChangeUnitExceptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumChangeUnitExceptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumChangeUnitExceptions_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&ppchangeunitexception), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumChangeUnitExceptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumClockVector_Impl: Sized {
    fn Next(&mut self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IClockVectorElement>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, csyncversions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumClockVector>;
}
impl IEnumClockVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumClockVector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumClockVector_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cclockvectorelements), ::core::mem::transmute_copy(&ppiclockvectorelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IEnumFeedClockVector_Impl: Sized {
    fn Next(&mut self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, csyncversions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumFeedClockVector>;
}
impl IEnumFeedClockVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFeedClockVector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumFeedClockVector_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cclockvectorelements), ::core::mem::transmute_copy(&ppiclockvectorelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppienum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumFeedClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IEnumItemIds_Impl: Sized {
    fn Next(&mut self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::Result<()>;
}
impl IEnumItemIds_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemIds_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumItemIds_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumItemIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumItemIds as ::windows::core::Interface>::IID
    }
}
pub trait IEnumRangeExceptions_Impl: Sized {
    fn Next(&mut self, cexceptions: u32, pprangeexception: *mut ::core::option::Option<IRangeException>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cexceptions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumRangeExceptions>;
}
impl IEnumRangeExceptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRangeExceptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumRangeExceptions_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, pprangeexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&pprangeexception), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumRangeExceptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSingleItemExceptions_Impl: Sized {
    fn Next(&mut self, cexceptions: u32, ppsingleitemexception: *mut ::core::option::Option<ISingleItemException>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cexceptions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSingleItemExceptions>;
}
impl IEnumSingleItemExceptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSingleItemExceptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSingleItemExceptions_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&ppsingleitemexception), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSingleItemExceptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSyncChangeUnits_Impl: Sized {
    fn Next(&mut self, cchanges: u32, ppchangeunit: *mut ::core::option::Option<ISyncChangeUnit>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cchanges: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSyncChangeUnits>;
}
impl IEnumSyncChangeUnits_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangeUnits_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncChangeUnits_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchangeunit: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&ppchangeunit), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncChangeUnits as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSyncChanges_Impl: Sized {
    fn Next(&mut self, cchanges: u32, ppchange: *mut ::core::option::Option<ISyncChange>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cchanges: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSyncChanges>;
}
impl IEnumSyncChanges_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChanges_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncChanges_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchange: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&ppchange), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncChanges as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderConfigUIInfos_Impl: Sized {
    fn Next(&mut self, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::core::option::Option<ISyncProviderConfigUIInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cfactories: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSyncProviderConfigUIInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderConfigUIInfos_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderConfigUIInfos_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncProviderConfigUIInfos_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cfactories), ::core::mem::transmute_copy(&ppsyncproviderconfiguiinfo), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfactories: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cfactories)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncProviderConfigUIInfos as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderInfos_Impl: Sized {
    fn Next(&mut self, cinstances: u32, ppsyncproviderinfo: *mut ::core::option::Option<ISyncProviderInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cinstances: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSyncProviderInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderInfos_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderInfos_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncProviderInfos_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cinstances), ::core::mem::transmute_copy(&ppsyncproviderinfo), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinstances: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cinstances)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncProviderInfos as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFeedClockVector_Impl: Sized + IClockVector_Impl {
    fn GetUpdateCount(&mut self, pdwupdatecount: *mut u32) -> ::windows::core::Result<()>;
    fn IsNoConflictsSpecified(&mut self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFeedClockVector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedClockVector_Vtbl {
        unsafe extern "system" fn GetUpdateCount<Impl: IFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwupdatecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUpdateCount(::core::mem::transmute_copy(&pdwupdatecount)).into()
        }
        unsafe extern "system" fn IsNoConflictsSpecified<Impl: IFeedClockVector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsNoConflictsSpecified(::core::mem::transmute_copy(&pfisnoconflictsspecified)).into()
        }
        Self {
            base: IClockVector_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetUpdateCount: GetUpdateCount::<Impl, IMPL_OFFSET>,
            IsNoConflictsSpecified: IsNoConflictsSpecified::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedClockVector as ::windows::core::Interface>::IID || iid == &<IClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IFeedClockVectorElement_Impl: Sized + IClockVectorElement_Impl {
    fn GetSyncTime(&mut self, psynctime: *mut SYNC_TIME) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self, pbflags: *mut u8) -> ::windows::core::Result<()>;
}
impl IFeedClockVectorElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVectorElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedClockVectorElement_Vtbl {
        unsafe extern "system" fn GetSyncTime<Impl: IFeedClockVectorElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynctime: *mut SYNC_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSyncTime(::core::mem::transmute_copy(&psynctime)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: IFeedClockVectorElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbflags: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pbflags)).into()
        }
        Self {
            base: IClockVectorElement_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSyncTime: GetSyncTime::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedClockVectorElement as ::windows::core::Interface>::IID || iid == &<IClockVectorElement as ::windows::core::Interface>::IID
    }
}
pub trait IFilterKeyMap_Impl: Sized {
    fn GetCount(&mut self, pdwcount: *mut u32) -> ::windows::core::Result<()>;
    fn AddFilter(&mut self, pisyncfilter: &::core::option::Option<ISyncFilter>, pdwfilterkey: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilter(&mut self, dwfilterkey: u32) -> ::windows::core::Result<ISyncFilter>;
    fn Serialize(&mut self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::Result<()>;
}
impl IFilterKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterKeyMap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn AddFilter<Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncfilter: ::windows::core::RawPtr, pdwfilterkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFilter(::core::mem::transmute(&pisyncfilter), ::core::mem::transmute_copy(&pdwfilterkey)).into()
        }
        unsafe extern "system" fn GetFilter<Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilter(::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppisyncfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbfilterkeymap), ::core::mem::transmute_copy(&pcbfilterkeymap)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            AddFilter: AddFilter::<Impl, IMPL_OFFSET>,
            GetFilter: GetFilter::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait IFilterRequestCallback_Impl: Sized {
    fn RequestFilter(&mut self, pfilter: &::core::option::Option<::windows::core::IUnknown>, filteringtype: FILTERING_TYPE) -> ::windows::core::Result<()>;
}
impl IFilterRequestCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterRequestCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterRequestCallback_Vtbl {
        unsafe extern "system" fn RequestFilter<Impl: IFilterRequestCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestFilter(::core::mem::transmute(&pfilter), ::core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), RequestFilter: RequestFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IFilterTrackingProvider_Impl: Sized {
    fn SpecifyTrackedFilters(&mut self, pcallback: &::core::option::Option<IFilterTrackingRequestCallback>) -> ::windows::core::Result<()>;
    fn AddTrackedFilter(&mut self, pfilter: &::core::option::Option<ISyncFilter>) -> ::windows::core::Result<()>;
}
impl IFilterTrackingProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterTrackingProvider_Vtbl {
        unsafe extern "system" fn SpecifyTrackedFilters<Impl: IFilterTrackingProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SpecifyTrackedFilters(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn AddTrackedFilter<Impl: IFilterTrackingProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTrackedFilter(::core::mem::transmute(&pfilter)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SpecifyTrackedFilters: SpecifyTrackedFilters::<Impl, IMPL_OFFSET>,
            AddTrackedFilter: AddTrackedFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterTrackingProvider as ::windows::core::Interface>::IID
    }
}
pub trait IFilterTrackingRequestCallback_Impl: Sized {
    fn RequestTrackedFilter(&mut self, pfilter: &::core::option::Option<ISyncFilter>) -> ::windows::core::Result<()>;
}
impl IFilterTrackingRequestCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingRequestCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterTrackingRequestCallback_Vtbl {
        unsafe extern "system" fn RequestTrackedFilter<Impl: IFilterTrackingRequestCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestTrackedFilter(::core::mem::transmute(&pfilter)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), RequestTrackedFilter: RequestTrackedFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterTrackingRequestCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFilterTrackingSyncChangeBuilder_Impl: Sized {
    fn AddFilterChange(&mut self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::Result<()>;
    fn SetAllChangeUnitsPresentFlag(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFilterTrackingSyncChangeBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingSyncChangeBuilder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterTrackingSyncChangeBuilder_Vtbl {
        unsafe extern "system" fn AddFilterChange<Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFilterChange(::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn SetAllChangeUnitsPresentFlag<Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllChangeUnitsPresentFlag().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddFilterChange: AddFilterChange::<Impl, IMPL_OFFSET>,
            SetAllChangeUnitsPresentFlag: SetAllChangeUnitsPresentFlag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterTrackingSyncChangeBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IForgottenKnowledge_Impl: Sized + ISyncKnowledge_Impl {
    fn ForgetToVersion(&mut self, pknowledge: &::core::option::Option<ISyncKnowledge>, pversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IForgottenKnowledge_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForgottenKnowledge_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IForgottenKnowledge_Vtbl {
        unsafe extern "system" fn ForgetToVersion<Impl: IForgottenKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForgetToVersion(::core::mem::transmute(&pknowledge), ::core::mem::transmute_copy(&pversion)).into()
        }
        Self { base: ISyncKnowledge_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ForgetToVersion: ForgetToVersion::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForgottenKnowledge as ::windows::core::Interface>::IID || iid == &<ISyncKnowledge as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKnowledgeSyncProvider_Impl: Sized + ISyncProvider_Impl {
    fn BeginSession(&mut self, role: SYNC_PROVIDER_ROLE, psessionstate: &::core::option::Option<ISyncSessionState>) -> ::windows::core::Result<()>;
    fn GetSyncBatchParameters(&mut self, ppsyncknowledge: *mut ::core::option::Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeBatch(&mut self, dwbatchsize: u32, psyncknowledge: &::core::option::Option<ISyncKnowledge>, ppsyncchangebatch: *mut ::core::option::Option<ISyncChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetFullEnumerationChangeBatch(&mut self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: &::core::option::Option<ISyncKnowledge>, ppsyncchangebatch: *mut ::core::option::Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ProcessChangeBatch(&mut self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: &::core::option::Option<ISyncChangeBatch>, punkdataretriever: &::core::option::Option<::windows::core::IUnknown>, pcallback: &::core::option::Option<ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::Result<()>;
    fn ProcessFullEnumerationChangeBatch(&mut self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: &::core::option::Option<ISyncFullEnumerationChangeBatch>, punkdataretriever: &::core::option::Option<::windows::core::IUnknown>, pcallback: &::core::option::Option<ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::Result<()>;
    fn EndSession(&mut self, psessionstate: &::core::option::Option<ISyncSessionState>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKnowledgeSyncProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnowledgeSyncProvider_Vtbl {
        unsafe extern "system" fn BeginSession<Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginSession(::core::mem::transmute_copy(&role), ::core::mem::transmute(&psessionstate)).into()
        }
        unsafe extern "system" fn GetSyncBatchParameters<Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncknowledge: *mut ::windows::core::RawPtr, pdwrequestedbatchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSyncBatchParameters(::core::mem::transmute_copy(&ppsyncknowledge), ::core::mem::transmute_copy(&pdwrequestedbatchsize)).into()
        }
        unsafe extern "system" fn GetChangeBatch<Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeBatch(::core::mem::transmute_copy(&dwbatchsize), ::core::mem::transmute(&psyncknowledge), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn GetFullEnumerationChangeBatch<Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFullEnumerationChangeBatch(::core::mem::transmute_copy(&dwbatchsize), ::core::mem::transmute_copy(&pblowerenumerationbound), ::core::mem::transmute(&psyncknowledge), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn ProcessChangeBatch<Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessChangeBatch(::core::mem::transmute_copy(&resolutionpolicy), ::core::mem::transmute(&psourcechangebatch), ::core::mem::transmute(&punkdataretriever), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn ProcessFullEnumerationChangeBatch<Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessFullEnumerationChangeBatch(::core::mem::transmute_copy(&resolutionpolicy), ::core::mem::transmute(&psourcechangebatch), ::core::mem::transmute(&punkdataretriever), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn EndSession<Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndSession(::core::mem::transmute(&psessionstate)).into()
        }
        Self {
            base: ISyncProvider_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginSession: BeginSession::<Impl, IMPL_OFFSET>,
            GetSyncBatchParameters: GetSyncBatchParameters::<Impl, IMPL_OFFSET>,
            GetChangeBatch: GetChangeBatch::<Impl, IMPL_OFFSET>,
            GetFullEnumerationChangeBatch: GetFullEnumerationChangeBatch::<Impl, IMPL_OFFSET>,
            ProcessChangeBatch: ProcessChangeBatch::<Impl, IMPL_OFFSET>,
            ProcessFullEnumerationChangeBatch: ProcessFullEnumerationChangeBatch::<Impl, IMPL_OFFSET>,
            EndSession: EndSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnowledgeSyncProvider as ::windows::core::Interface>::IID || iid == &<ISyncProvider as ::windows::core::Interface>::IID
    }
}
pub trait ILoadChangeContext_Impl: Sized {
    fn GetSyncChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn SetRecoverableErrorOnChange(&mut self, hrerror: ::windows::core::HRESULT, perrordata: &::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
    fn SetRecoverableErrorOnChangeUnit(&mut self, hrerror: ::windows::core::HRESULT, pchangeunit: &::core::option::Option<ISyncChangeUnit>, perrordata: &::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
}
impl ILoadChangeContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoadChangeContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoadChangeContext_Vtbl {
        unsafe extern "system" fn GetSyncChange<Impl: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoverableErrorOnChange<Impl: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecoverableErrorOnChange(::core::mem::transmute_copy(&hrerror), ::core::mem::transmute(&perrordata)).into()
        }
        unsafe extern "system" fn SetRecoverableErrorOnChangeUnit<Impl: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, pchangeunit: ::windows::core::RawPtr, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecoverableErrorOnChangeUnit(::core::mem::transmute_copy(&hrerror), ::core::mem::transmute(&pchangeunit), ::core::mem::transmute(&perrordata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSyncChange: GetSyncChange::<Impl, IMPL_OFFSET>,
            SetRecoverableErrorOnChange: SetRecoverableErrorOnChange::<Impl, IMPL_OFFSET>,
            SetRecoverableErrorOnChangeUnit: SetRecoverableErrorOnChangeUnit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadChangeContext as ::windows::core::Interface>::IID
    }
}
pub trait IProviderConverter_Impl: Sized {
    fn Initialize(&mut self, pisyncprovider: &::core::option::Option<ISyncProvider>) -> ::windows::core::Result<()>;
}
impl IProviderConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderConverter_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IProviderConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisyncprovider)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderConverter as ::windows::core::Interface>::IID
    }
}
pub trait IRangeException_Impl: Sized {
    fn GetClosedRangeStart(&mut self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClosedRangeEnd(&mut self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClockVector(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IRangeException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeException_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeException_Vtbl {
        unsafe extern "system" fn GetClosedRangeStart<Impl: IRangeException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosedRangeStart(::core::mem::transmute_copy(&pbclosedrangestart), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedRangeEnd<Impl: IRangeException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosedRangeEnd(::core::mem::transmute_copy(&pbclosedrangeend), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Impl: IRangeException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClockVector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClosedRangeStart: GetClosedRangeStart::<Impl, IMPL_OFFSET>,
            GetClosedRangeEnd: GetClosedRangeEnd::<Impl, IMPL_OFFSET>,
            GetClockVector: GetClockVector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeException as ::windows::core::Interface>::IID
    }
}
pub trait IRecoverableError_Impl: Sized {
    fn GetStage(&mut self, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::Result<()>;
    fn GetProvider(&mut self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::Result<()>;
    fn GetChangeWithRecoverableError(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetRecoverableErrorDataForChange(&mut self, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
    fn GetRecoverableErrorDataForChangeUnit(&mut self, pchangeunit: &::core::option::Option<ISyncChangeUnit>, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
}
impl IRecoverableError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableError_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRecoverableError_Vtbl {
        unsafe extern "system" fn GetStage<Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStage(::core::mem::transmute_copy(&pstage)).into()
        }
        unsafe extern "system" fn GetProvider<Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProvider(::core::mem::transmute_copy(&pproviderrole)).into()
        }
        unsafe extern "system" fn GetChangeWithRecoverableError<Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppchangewithrecoverableerror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeWithRecoverableError() {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangewithrecoverableerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChange<Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRecoverableErrorDataForChange(::core::mem::transmute_copy(&phrerror), ::core::mem::transmute_copy(&pperrordata)).into()
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChangeUnit<Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRecoverableErrorDataForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&phrerror), ::core::mem::transmute_copy(&pperrordata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStage: GetStage::<Impl, IMPL_OFFSET>,
            GetProvider: GetProvider::<Impl, IMPL_OFFSET>,
            GetChangeWithRecoverableError: GetChangeWithRecoverableError::<Impl, IMPL_OFFSET>,
            GetRecoverableErrorDataForChange: GetRecoverableErrorDataForChange::<Impl, IMPL_OFFSET>,
            GetRecoverableErrorDataForChangeUnit: GetRecoverableErrorDataForChangeUnit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRecoverableError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRecoverableErrorData_Impl: Sized {
    fn Initialize(&mut self, pcszitemdisplayname: super::super::Foundation::PWSTR, pcszerrordescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetItemDisplayName(&mut self, pszitemdisplayname: super::super::Foundation::PWSTR, pcchitemdisplayname: *mut u32) -> ::windows::core::Result<()>;
    fn GetErrorDescription(&mut self, pszerrordescription: super::super::Foundation::PWSTR, pccherrordescription: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRecoverableErrorData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableErrorData_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRecoverableErrorData_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszitemdisplayname: super::super::Foundation::PWSTR, pcszerrordescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pcszitemdisplayname), ::core::mem::transmute_copy(&pcszerrordescription)).into()
        }
        unsafe extern "system" fn GetItemDisplayName<Impl: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitemdisplayname: super::super::Foundation::PWSTR, pcchitemdisplayname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemDisplayName(::core::mem::transmute_copy(&pszitemdisplayname), ::core::mem::transmute_copy(&pcchitemdisplayname)).into()
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszerrordescription: super::super::Foundation::PWSTR, pccherrordescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetErrorDescription(::core::mem::transmute_copy(&pszerrordescription), ::core::mem::transmute_copy(&pccherrordescription)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetItemDisplayName: GetItemDisplayName::<Impl, IMPL_OFFSET>,
            GetErrorDescription: GetErrorDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRecoverableErrorData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IRegisteredSyncProvider_Impl: Sized {
    fn Init(&mut self, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn GetInstanceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IRegisteredSyncProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredSyncProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegisteredSyncProvider_Vtbl {
        unsafe extern "system" fn Init<Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute(&pcontextpropertystore)).into()
        }
        unsafe extern "system" fn GetInstanceId<Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            GetInstanceId: GetInstanceId::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredSyncProvider as ::windows::core::Interface>::IID
    }
}
pub trait IReplicaKeyMap_Impl: Sized {
    fn LookupReplicaKey(&mut self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::Result<()>;
    fn LookupReplicaId(&mut self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::Result<()>;
}
impl IReplicaKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReplicaKeyMap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReplicaKeyMap_Vtbl {
        unsafe extern "system" fn LookupReplicaKey<Impl: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LookupReplicaKey(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn LookupReplicaId<Impl: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LookupReplicaId(::core::mem::transmute_copy(&dwreplicakey), ::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Impl: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbreplicakeymap), ::core::mem::transmute_copy(&pcbreplicakeymap)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LookupReplicaKey: LookupReplicaKey::<Impl, IMPL_OFFSET>,
            LookupReplicaId: LookupReplicaId::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReplicaKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait IRequestFilteredSync_Impl: Sized {
    fn SpecifyFilter(&mut self, pcallback: &::core::option::Option<IFilterRequestCallback>) -> ::windows::core::Result<()>;
}
impl IRequestFilteredSync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRequestFilteredSync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRequestFilteredSync_Vtbl {
        unsafe extern "system" fn SpecifyFilter<Impl: IRequestFilteredSync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SpecifyFilter(::core::mem::transmute(&pcallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SpecifyFilter: SpecifyFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRequestFilteredSync as ::windows::core::Interface>::IID
    }
}
pub trait ISingleItemException_Impl: Sized {
    fn GetItemId(&mut self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClockVector(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ISingleItemException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISingleItemException_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISingleItemException_Vtbl {
        unsafe extern "system" fn GetItemId<Impl: ISingleItemException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemId(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Impl: ISingleItemException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClockVector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetItemId: GetItemId::<Impl, IMPL_OFFSET>,
            GetClockVector: GetClockVector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISingleItemException as ::windows::core::Interface>::IID
    }
}
pub trait ISupportFilteredSync_Impl: Sized {
    fn AddFilter(&mut self, pfilter: &::core::option::Option<::windows::core::IUnknown>, filteringtype: FILTERING_TYPE) -> ::windows::core::Result<()>;
}
impl ISupportFilteredSync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportFilteredSync_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISupportFilteredSync_Vtbl {
        unsafe extern "system" fn AddFilter<Impl: ISupportFilteredSync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFilter(::core::mem::transmute(&pfilter), ::core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddFilter: AddFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportFilteredSync as ::windows::core::Interface>::IID
    }
}
pub trait ISupportLastWriteTime_Impl: Sized {
    fn GetItemChangeTime(&mut self, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::Result<()>;
    fn GetChangeUnitChangeTime(&mut self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::Result<()>;
}
impl ISupportLastWriteTime_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportLastWriteTime_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISupportLastWriteTime_Vtbl {
        unsafe extern "system" fn GetItemChangeTime<Impl: ISupportLastWriteTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemChangeTime(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pulltimestamp)).into()
        }
        unsafe extern "system" fn GetChangeUnitChangeTime<Impl: ISupportLastWriteTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitChangeTime(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pulltimestamp)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetItemChangeTime: GetItemChangeTime::<Impl, IMPL_OFFSET>,
            GetChangeUnitChangeTime: GetChangeUnitChangeTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportLastWriteTime as ::windows::core::Interface>::IID
    }
}
pub trait ISyncCallback_Impl: Sized {
    fn OnProgress(&mut self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()>;
    fn OnChange(&mut self, psyncchange: &::core::option::Option<ISyncChange>) -> ::windows::core::Result<()>;
    fn OnConflict(&mut self, pconflict: &::core::option::Option<IChangeConflict>) -> ::windows::core::Result<()>;
    fn OnFullEnumerationNeeded(&mut self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::Result<()>;
    fn OnRecoverableError(&mut self, precoverableerror: &::core::option::Option<IRecoverableError>) -> ::windows::core::Result<()>;
}
impl ISyncCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncCallback_Vtbl {
        unsafe extern "system" fn OnProgress<Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&provider), ::core::mem::transmute_copy(&syncstage), ::core::mem::transmute_copy(&dwcompletedwork), ::core::mem::transmute_copy(&dwtotalwork)).into()
        }
        unsafe extern "system" fn OnChange<Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChange(::core::mem::transmute(&psyncchange)).into()
        }
        unsafe extern "system" fn OnConflict<Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConflict(::core::mem::transmute(&pconflict)).into()
        }
        unsafe extern "system" fn OnFullEnumerationNeeded<Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFullEnumerationNeeded(::core::mem::transmute_copy(&pfullenumerationaction)).into()
        }
        unsafe extern "system" fn OnRecoverableError<Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precoverableerror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRecoverableError(::core::mem::transmute(&precoverableerror)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnProgress: OnProgress::<Impl, IMPL_OFFSET>,
            OnChange: OnChange::<Impl, IMPL_OFFSET>,
            OnConflict: OnConflict::<Impl, IMPL_OFFSET>,
            OnFullEnumerationNeeded: OnFullEnumerationNeeded::<Impl, IMPL_OFFSET>,
            OnRecoverableError: OnRecoverableError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISyncCallback2_Impl: Sized + ISyncCallback_Impl {
    fn OnChangeApplied(&mut self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::Result<()>;
    fn OnChangeFailed(&mut self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::Result<()>;
}
impl ISyncCallback2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncCallback2_Vtbl {
        unsafe extern "system" fn OnChangeApplied<Impl: ISyncCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChangeApplied(::core::mem::transmute_copy(&dwchangesapplied), ::core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        unsafe extern "system" fn OnChangeFailed<Impl: ISyncCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChangeFailed(::core::mem::transmute_copy(&dwchangesapplied), ::core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        Self {
            base: ISyncCallback_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnChangeApplied: OnChangeApplied::<Impl, IMPL_OFFSET>,
            OnChangeFailed: OnChangeFailed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncCallback2 as ::windows::core::Interface>::IID || iid == &<ISyncCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChange_Impl: Sized {
    fn GetOwnerReplicaId(&mut self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetRootItemId(&mut self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeVersion(&mut self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
    fn GetCreationVersion(&mut self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetWorkEstimate(&mut self, pdwwork: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnits(&mut self) -> ::windows::core::Result<IEnumSyncChangeUnits>;
    fn GetMadeWithKnowledge(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledge(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn SetWorkEstimate(&mut self, dwwork: u32) -> ::windows::core::Result<()>;
}
impl ISyncChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChange_Vtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOwnerReplicaId(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetRootItemId<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRootItemId(::core::mem::transmute_copy(&pbrootitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeVersion<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeVersion(::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetCreationVersion<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCreationVersion(::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetWorkEstimate<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwork: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWorkEstimate(::core::mem::transmute_copy(&pdwwork)).into()
        }
        unsafe extern "system" fn GetChangeUnits<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeUnits() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMadeWithKnowledge<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmadewithknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMadeWithKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmadewithknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkEstimate<Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkEstimate(::core::mem::transmute_copy(&dwwork)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Impl, IMPL_OFFSET>,
            GetRootItemId: GetRootItemId::<Impl, IMPL_OFFSET>,
            GetChangeVersion: GetChangeVersion::<Impl, IMPL_OFFSET>,
            GetCreationVersion: GetCreationVersion::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            GetWorkEstimate: GetWorkEstimate::<Impl, IMPL_OFFSET>,
            GetChangeUnits: GetChangeUnits::<Impl, IMPL_OFFSET>,
            GetMadeWithKnowledge: GetMadeWithKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            SetWorkEstimate: SetWorkEstimate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatch_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn BeginUnorderedGroup(&mut self) -> ::windows::core::Result<()>;
    fn EndUnorderedGroup(&mut self, pmadewithknowledge: &::core::option::Option<ISyncKnowledge>, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddLoggedConflict(&mut self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatch_Vtbl {
        unsafe extern "system" fn BeginUnorderedGroup<Impl: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUnorderedGroup().into()
        }
        unsafe extern "system" fn EndUnorderedGroup<Impl: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmadewithknowledge: ::windows::core::RawPtr, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndUnorderedGroup(::core::mem::transmute(&pmadewithknowledge), ::core::mem::transmute_copy(&fallchangesforknowledge)).into()
        }
        unsafe extern "system" fn AddLoggedConflict<Impl: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddLoggedConflict(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwworkforchange), ::core::mem::transmute(&pconflictknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginUnorderedGroup: BeginUnorderedGroup::<Impl, IMPL_OFFSET>,
            EndUnorderedGroup: EndUnorderedGroup::<Impl, IMPL_OFFSET>,
            AddLoggedConflict: AddLoggedConflict::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatch as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatch2_Impl: Sized + ISyncChangeBatchBase_Impl + ISyncChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(&mut self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder>;
    fn AddMergeTombstoneLoggedConflict(&mut self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatch2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatch2_Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Impl: ISyncChangeBatch2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMergeTombstoneMetadataToGroup(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMergeTombstoneLoggedConflict<Impl: ISyncChangeBatch2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMergeTombstoneLoggedConflict(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange), ::core::mem::transmute(&pconflictknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISyncChangeBatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Impl, IMPL_OFFSET>,
            AddMergeTombstoneLoggedConflict: AddMergeTombstoneLoggedConflict::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatch2 as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchAdvanced_Impl: Sized {
    fn GetFilterInfo(&mut self) -> ::windows::core::Result<ISyncFilterInfo>;
    fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&mut self) -> ::windows::core::Result<ISyncChangeBatch>;
    fn GetUpperBoundItemId(&mut self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetBatchLevelKnowledgeShouldBeApplied(&mut self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchAdvanced_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchAdvanced_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchAdvanced_Vtbl {
        unsafe extern "system" fn GetFilterInfo<Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfilterinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFullEnumerationChangeBatchToRegularChangeBatch<Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppchangebatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertFullEnumerationChangeBatchToRegularChangeBatch() {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperBoundItemId<Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUpperBoundItemId(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetBatchLevelKnowledgeShouldBeApplied<Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBatchLevelKnowledgeShouldBeApplied(::core::mem::transmute_copy(&pfbatchknowledgeshouldbeapplied)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFilterInfo: GetFilterInfo::<Impl, IMPL_OFFSET>,
            ConvertFullEnumerationChangeBatchToRegularChangeBatch: ConvertFullEnumerationChangeBatchToRegularChangeBatch::<Impl, IMPL_OFFSET>,
            GetUpperBoundItemId: GetUpperBoundItemId::<Impl, IMPL_OFFSET>,
            GetBatchLevelKnowledgeShouldBeApplied: GetBatchLevelKnowledgeShouldBeApplied::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchBase_Impl: Sized {
    fn GetChangeEnumerator(&mut self) -> ::windows::core::Result<IEnumSyncChanges>;
    fn GetIsLastBatch(&mut self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetWorkEstimateForBatch(&mut self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()>;
    fn GetRemainingWorkEstimateForSession(&mut self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()>;
    fn BeginOrderedGroup(&mut self, pblowerbound: *const u8) -> ::windows::core::Result<()>;
    fn EndOrderedGroup(&mut self, pbupperbound: *const u8, pmadewithknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn AddItemMetadataToGroup(&mut self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder>;
    fn GetLearnedKnowledge(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetPrerequisiteKnowledge(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetSourceForgottenKnowledge(&mut self) -> ::windows::core::Result<IForgottenKnowledge>;
    fn SetLastBatch(&mut self) -> ::windows::core::Result<()>;
    fn SetWorkEstimateForBatch(&mut self, dwworkforbatch: u32) -> ::windows::core::Result<()>;
    fn SetRemainingWorkEstimateForSession(&mut self, dwremainingworkforsession: u32) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchBase_Vtbl {
        unsafe extern "system" fn GetChangeEnumerator<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsLastBatch<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIsLastBatch(::core::mem::transmute_copy(&pflastbatch)).into()
        }
        unsafe extern "system" fn GetWorkEstimateForBatch<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwworkforbatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWorkEstimateForBatch(::core::mem::transmute_copy(&pdwworkforbatch)).into()
        }
        unsafe extern "system" fn GetRemainingWorkEstimateForSession<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemainingWorkEstimateForSession(::core::mem::transmute_copy(&pdwremainingworkforsession)).into()
        }
        unsafe extern "system" fn BeginOrderedGroup<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblowerbound: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginOrderedGroup(::core::mem::transmute_copy(&pblowerbound)).into()
        }
        unsafe extern "system" fn EndOrderedGroup<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndOrderedGroup(::core::mem::transmute_copy(&pbupperbound), ::core::mem::transmute(&pmadewithknowledge)).into()
        }
        unsafe extern "system" fn AddItemMetadataToGroup<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddItemMetadataToGroup(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisiteKnowledge<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprerequisteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrerequisiteKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprerequisteknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceForgottenKnowledge<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsourceforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceForgottenKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsourceforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastBatch<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastBatch().into()
        }
        unsafe extern "system" fn SetWorkEstimateForBatch<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwworkforbatch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkEstimateForBatch(::core::mem::transmute_copy(&dwworkforbatch)).into()
        }
        unsafe extern "system" fn SetRemainingWorkEstimateForSession<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwremainingworkforsession: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemainingWorkEstimateForSession(::core::mem::transmute_copy(&dwremainingworkforsession)).into()
        }
        unsafe extern "system" fn Serialize<Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbchangebatch), ::core::mem::transmute_copy(&pcbchangebatch)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetChangeEnumerator: GetChangeEnumerator::<Impl, IMPL_OFFSET>,
            GetIsLastBatch: GetIsLastBatch::<Impl, IMPL_OFFSET>,
            GetWorkEstimateForBatch: GetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            GetRemainingWorkEstimateForSession: GetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            BeginOrderedGroup: BeginOrderedGroup::<Impl, IMPL_OFFSET>,
            EndOrderedGroup: EndOrderedGroup::<Impl, IMPL_OFFSET>,
            AddItemMetadataToGroup: AddItemMetadataToGroup::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetSourceForgottenKnowledge: GetSourceForgottenKnowledge::<Impl, IMPL_OFFSET>,
            SetLastBatch: SetLastBatch::<Impl, IMPL_OFFSET>,
            SetWorkEstimateForBatch: SetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            SetRemainingWorkEstimateForSession: SetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchBase2_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn SerializeWithOptions(&mut self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchBase2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchBase2_Vtbl {
        unsafe extern "system" fn SerializeWithOptions<Impl: ISyncChangeBatchBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SerializeWithOptions(::core::mem::transmute_copy(&targetformatversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        Self {
            base: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SerializeWithOptions: SerializeWithOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase2 as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeBatchWithFilterKeyMap_Impl: Sized {
    fn GetFilterKeyMap(&mut self) -> ::windows::core::Result<IFilterKeyMap>;
    fn SetFilterKeyMap(&mut self, pifilterkeymap: &::core::option::Option<IFilterKeyMap>) -> ::windows::core::Result<()>;
    fn SetFilterForgottenKnowledge(&mut self, dwfilterkey: u32, pfilterforgottenknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn GetFilteredReplicaLearnedKnowledge(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
}
impl ISyncChangeBatchWithFilterKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchWithFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetFilterKeyMap<Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifilterkeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterKeyMap() {
                ::core::result::Result::Ok(ok__) => {
                    *ppifilterkeymap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterKeyMap<Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifilterkeymap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilterKeyMap(::core::mem::transmute(&pifilterkeymap)).into()
        }
        unsafe extern "system" fn SetFilterForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilterForgottenKnowledge(::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute(&pfilterforgottenknowledge)).into()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFilterKeyMap: GetFilterKeyMap::<Impl, IMPL_OFFSET>,
            SetFilterKeyMap: SetFilterKeyMap::<Impl, IMPL_OFFSET>,
            SetFilterForgottenKnowledge: SetFilterForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithFilterKeyMap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchWithPrerequisite_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn SetPrerequisiteKnowledge(&mut self, pprerequisiteknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn GetLearnedKnowledgeWithPrerequisite(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&mut self) -> ::windows::core::Result<IForgottenKnowledge>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchWithPrerequisite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithPrerequisite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchWithPrerequisite_Vtbl {
        unsafe extern "system" fn SetPrerequisiteKnowledge<Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrerequisiteKnowledge(::core::mem::transmute(&pprerequisiteknowledge)).into()
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedwithprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeWithPrerequisite(::core::mem::transmute(&pdestinationknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedwithprerequisiteknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedForgottenKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPrerequisiteKnowledge: SetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Impl, IMPL_OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithPrerequisite as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeBuilder_Impl: Sized {
    fn AddChangeUnitMetadata(&mut self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
}
impl ISyncChangeBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBuilder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBuilder_Vtbl {
        unsafe extern "system" fn AddChangeUnitMetadata<Impl: ISyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChangeUnitMetadata(::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pchangeunitversion)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddChangeUnitMetadata: AddChangeUnitMetadata::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBuilder as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeUnit_Impl: Sized {
    fn GetItemChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetChangeUnitId(&mut self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitVersion(&mut self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
}
impl ISyncChangeUnit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeUnit_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeUnit_Vtbl {
        unsafe extern "system" fn GetItemChange<Impl: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitId(::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitVersion<Impl: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitVersion(::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetItemChange: GetItemChange::<Impl, IMPL_OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Impl, IMPL_OFFSET>,
            GetChangeUnitVersion: GetChangeUnitVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeUnit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeWithFilterKeyMap_Impl: Sized {
    fn GetFilterCount(&mut self, pdwfiltercount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilterChange(&mut self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::Result<()>;
    fn GetAllChangeUnitsPresentFlag(&mut self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFilterForgottenKnowledge(&mut self, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedKnowledge(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>, pnewmoveins: &::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeWithFilterKeyMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeWithFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetFilterCount<Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterCount(::core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterChange<Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterChange(::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn GetAllChangeUnitsPresentFlag<Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAllChangeUnitsPresentFlag(::core::mem::transmute_copy(&pfallchangeunitspresent)).into()
        }
        unsafe extern "system" fn GetFilterForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterForgottenKnowledge(::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFilterCount: GetFilterCount::<Impl, IMPL_OFFSET>,
            GetFilterChange: GetFilterChange::<Impl, IMPL_OFFSET>,
            GetAllChangeUnitsPresentFlag: GetAllChangeUnitsPresentFlag::<Impl, IMPL_OFFSET>,
            GetFilterForgottenKnowledge: GetFilterForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeWithFilterKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeWithPrerequisite_Impl: Sized {
    fn GetPrerequisiteKnowledge(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledgeWithPrerequisite(&mut self, pdestinationknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
}
impl ISyncChangeWithPrerequisite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithPrerequisite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeWithPrerequisite_Vtbl {
        unsafe extern "system" fn GetPrerequisiteKnowledge<Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrerequisiteKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprerequisiteknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedknowledgewithprerequisite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeWithPrerequisite(::core::mem::transmute(&pdestinationknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledgewithprerequisite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeWithPrerequisite as ::windows::core::Interface>::IID
    }
}
pub trait ISyncConstraintCallback_Impl: Sized {
    fn OnConstraintConflict(&mut self, pconflict: &::core::option::Option<IConstraintConflict>) -> ::windows::core::Result<()>;
}
impl ISyncConstraintCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncConstraintCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncConstraintCallback_Vtbl {
        unsafe extern "system" fn OnConstraintConflict<Impl: ISyncConstraintCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConstraintConflict(::core::mem::transmute(&pconflict)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnConstraintConflict: OnConstraintConflict::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncConstraintCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISyncDataConverter_Impl: Sized {
    fn ConvertDataRetrieverFromProviderFormat(&mut self, punkdataretrieverin: &::core::option::Option<::windows::core::IUnknown>, penumsyncchanges: &::core::option::Option<IEnumSyncChanges>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ConvertDataRetrieverToProviderFormat(&mut self, punkdataretrieverin: &::core::option::Option<::windows::core::IUnknown>, penumsyncchanges: &::core::option::Option<IEnumSyncChanges>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ConvertDataFromProviderFormat(&mut self, pdatacontext: &::core::option::Option<ILoadChangeContext>, punkdatain: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ConvertDataToProviderFormat(&mut self, pdatacontext: &::core::option::Option<ILoadChangeContext>, punkdataout: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ISyncDataConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncDataConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncDataConverter_Vtbl {
        unsafe extern "system" fn ConvertDataRetrieverFromProviderFormat<Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertDataRetrieverFromProviderFormat(::core::mem::transmute(&punkdataretrieverin), ::core::mem::transmute(&penumsyncchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataRetrieverToProviderFormat<Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertDataRetrieverToProviderFormat(::core::mem::transmute(&punkdataretrieverin), ::core::mem::transmute(&penumsyncchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataFromProviderFormat<Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertDataFromProviderFormat(::core::mem::transmute(&pdatacontext), ::core::mem::transmute(&punkdatain)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataToProviderFormat<Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertDataToProviderFormat(::core::mem::transmute(&pdatacontext), ::core::mem::transmute(&punkdataout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConvertDataRetrieverFromProviderFormat: ConvertDataRetrieverFromProviderFormat::<Impl, IMPL_OFFSET>,
            ConvertDataRetrieverToProviderFormat: ConvertDataRetrieverToProviderFormat::<Impl, IMPL_OFFSET>,
            ConvertDataFromProviderFormat: ConvertDataFromProviderFormat::<Impl, IMPL_OFFSET>,
            ConvertDataToProviderFormat: ConvertDataToProviderFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncDataConverter as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilter_Impl: Sized {
    fn IsIdentical(&mut self, psyncfilter: &::core::option::Option<ISyncFilter>) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilter_Vtbl {
        unsafe extern "system" fn IsIdentical<Impl: ISyncFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsIdentical(::core::mem::transmute(&psyncfilter)).into()
        }
        unsafe extern "system" fn Serialize<Impl: ISyncFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbsyncfilter), ::core::mem::transmute_copy(&pcbsyncfilter)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsIdentical: IsIdentical::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilter as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterDeserializer_Impl: Sized {
    fn DeserializeSyncFilter(&mut self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> ::windows::core::Result<ISyncFilter>;
}
impl ISyncFilterDeserializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterDeserializer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterDeserializer_Vtbl {
        unsafe extern "system" fn DeserializeSyncFilter<Impl: ISyncFilterDeserializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeserializeSyncFilter(::core::mem::transmute_copy(&pbsyncfilter), ::core::mem::transmute_copy(&dwcbsyncfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppisyncfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), DeserializeSyncFilter: DeserializeSyncFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterDeserializer as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterInfo_Impl: Sized {
    fn Serialize(&mut self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncFilterInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterInfo_Vtbl {
        unsafe extern "system" fn Serialize<Impl: ISyncFilterInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pcbbuffer)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Serialize: Serialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterInfo2_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetFlags(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncFilterInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterInfo2_Vtbl {
        unsafe extern "system" fn GetFlags<Impl: ISyncFilterInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base: ISyncFilterInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetFlags: GetFlags::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterInfo2 as ::windows::core::Interface>::IID || iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFullEnumerationChange_Impl: Sized {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&mut self) -> ::windows::core::Result<IForgottenKnowledge>;
}
impl ISyncFullEnumerationChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFullEnumerationChange_Vtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Impl: ISyncFullEnumerationChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeAfterRecoveryComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Impl: ISyncFullEnumerationChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedForgottenKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncFullEnumerationChangeBatch_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetClosedLowerBoundItemId(&mut self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClosedUpperBoundItemId(&mut self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncFullEnumerationChangeBatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFullEnumerationChangeBatch_Vtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeAfterRecoveryComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledgeafterrecoverycomplete = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosedLowerBoundItemId<Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosedLowerBoundItemId(::core::mem::transmute_copy(&pbclosedlowerbounditemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedUpperBoundItemId<Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosedUpperBoundItemId(::core::mem::transmute_copy(&pbclosedupperbounditemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
            GetClosedLowerBoundItemId: GetClosedLowerBoundItemId::<Impl, IMPL_OFFSET>,
            GetClosedUpperBoundItemId: GetClosedUpperBoundItemId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncFullEnumerationChangeBatch2_Impl: Sized + ISyncChangeBatchBase_Impl + ISyncFullEnumerationChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(&mut self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncFullEnumerationChangeBatch2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFullEnumerationChangeBatch2_Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Impl: ISyncFullEnumerationChangeBatch2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMergeTombstoneMetadataToGroup(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISyncFullEnumerationChangeBatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch2 as ::windows::core::Interface>::IID || iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID || iid == &<ISyncFullEnumerationChangeBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncKnowledge_Impl: Sized {
    fn GetOwnerReplicaId(&mut self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::Result<()>;
    fn SetLocalTickCount(&mut self, ulltickcount: u64) -> ::windows::core::Result<()>;
    fn ContainsChange(&mut self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
    fn ContainsChangeUnit(&mut self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
    fn GetScopeVector(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetReplicaKeyMap(&mut self) -> ::windows::core::Result<IReplicaKeyMap>;
    fn Clone(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn ConvertVersion(&mut self, pknowledgein: &::core::option::Option<ISyncKnowledge>, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
    fn MapRemoteToLocal(&mut self, premoteknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn Union(&mut self, pknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn ProjectOntoItem(&mut self, pbitemid: *const u8) -> ::windows::core::Result<ISyncKnowledge>;
    fn ProjectOntoChangeUnit(&mut self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<ISyncKnowledge>;
    fn ProjectOntoRange(&mut self, psrngsyncrange: *const SYNC_RANGE) -> ::windows::core::Result<ISyncKnowledge>;
    fn ExcludeItem(&mut self, pbitemid: *const u8) -> ::windows::core::Result<()>;
    fn ExcludeChangeUnit(&mut self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()>;
    fn ContainsKnowledge(&mut self, pknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn FindMinTickCountForReplica(&mut self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::Result<()>;
    fn GetRangeExceptions(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetSingleItemExceptions(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetChangeUnitExceptions(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FindClockVectorForItem(&mut self, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FindClockVectorForChangeUnit(&mut self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetVersion(&mut self, pdwversion: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncKnowledge_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncKnowledge_Vtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOwnerReplicaId(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&fserializereplicakeymap), ::core::mem::transmute_copy(&pbknowledge), ::core::mem::transmute_copy(&pcbknowledge)).into()
        }
        unsafe extern "system" fn SetLocalTickCount<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltickcount: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalTickCount(::core::mem::transmute_copy(&ulltickcount)).into()
        }
        unsafe extern "system" fn ContainsChange<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsChange(::core::mem::transmute_copy(&pbversionownerreplicaid), ::core::mem::transmute_copy(&pgiditemid), ::core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn ContainsChangeUnit<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsChangeUnit(::core::mem::transmute_copy(&pbversionownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn GetScopeVector<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScopeVector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetReplicaKeyMap<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreplicakeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReplicaKeyMap() {
                ::core::result::Result::Ok(ok__) => {
                    *ppreplicakeymap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclonedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclonedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertVersion<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledgein: ::windows::core::RawPtr, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertVersion(::core::mem::transmute(&pknowledgein), ::core::mem::transmute_copy(&pbcurrentownerid), ::core::mem::transmute_copy(&pversionin), ::core::mem::transmute_copy(&pbnewownerid), ::core::mem::transmute_copy(&pcbidsize), ::core::mem::transmute_copy(&pversionout)).into()
        }
        unsafe extern "system" fn MapRemoteToLocal<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteknowledge: ::windows::core::RawPtr, ppmappedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapRemoteToLocal(::core::mem::transmute(&premoteknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmappedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Union<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Union(::core::mem::transmute(&pknowledge)).into()
        }
        unsafe extern "system" fn ProjectOntoItem<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoItem(::core::mem::transmute_copy(&pbitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoChangeUnit<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoChangeUnit(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoRange<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoRange(::core::mem::transmute_copy(&psrngsyncrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcludeItem<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExcludeItem(::core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ExcludeChangeUnit<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExcludeChangeUnit(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ContainsKnowledge<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsKnowledge(::core::mem::transmute(&pknowledge)).into()
        }
        unsafe extern "system" fn FindMinTickCountForReplica<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindMinTickCountForReplica(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pullreplicatickcount)).into()
        }
        unsafe extern "system" fn GetRangeExceptions<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRangeExceptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetSingleItemExceptions<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSingleItemExceptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetChangeUnitExceptions<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitExceptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForItem<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindClockVectorForItem(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForChangeUnit<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindClockVectorForChangeUnit(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetVersion<Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVersion(::core::mem::transmute_copy(&pdwversion)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
            SetLocalTickCount: SetLocalTickCount::<Impl, IMPL_OFFSET>,
            ContainsChange: ContainsChange::<Impl, IMPL_OFFSET>,
            ContainsChangeUnit: ContainsChangeUnit::<Impl, IMPL_OFFSET>,
            GetScopeVector: GetScopeVector::<Impl, IMPL_OFFSET>,
            GetReplicaKeyMap: GetReplicaKeyMap::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            ConvertVersion: ConvertVersion::<Impl, IMPL_OFFSET>,
            MapRemoteToLocal: MapRemoteToLocal::<Impl, IMPL_OFFSET>,
            Union: Union::<Impl, IMPL_OFFSET>,
            ProjectOntoItem: ProjectOntoItem::<Impl, IMPL_OFFSET>,
            ProjectOntoChangeUnit: ProjectOntoChangeUnit::<Impl, IMPL_OFFSET>,
            ProjectOntoRange: ProjectOntoRange::<Impl, IMPL_OFFSET>,
            ExcludeItem: ExcludeItem::<Impl, IMPL_OFFSET>,
            ExcludeChangeUnit: ExcludeChangeUnit::<Impl, IMPL_OFFSET>,
            ContainsKnowledge: ContainsKnowledge::<Impl, IMPL_OFFSET>,
            FindMinTickCountForReplica: FindMinTickCountForReplica::<Impl, IMPL_OFFSET>,
            GetRangeExceptions: GetRangeExceptions::<Impl, IMPL_OFFSET>,
            GetSingleItemExceptions: GetSingleItemExceptions::<Impl, IMPL_OFFSET>,
            GetChangeUnitExceptions: GetChangeUnitExceptions::<Impl, IMPL_OFFSET>,
            FindClockVectorForItem: FindClockVectorForItem::<Impl, IMPL_OFFSET>,
            FindClockVectorForChangeUnit: FindClockVectorForChangeUnit::<Impl, IMPL_OFFSET>,
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncKnowledge as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncKnowledge2_Impl: Sized + ISyncKnowledge_Impl {
    fn GetIdParameters(&mut self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
    fn ProjectOntoColumnSet(&mut self, ppcolumns: *const *const u8, count: u32) -> ::windows::core::Result<ISyncKnowledge2>;
    fn SerializeWithOptions(&mut self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetLowestUncontainedId(&mut self, pisyncknowledge: &::core::option::Option<ISyncKnowledge2>, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetInspector(&mut self, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetMinimumSupportedVersion(&mut self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::Result<()>;
    fn GetStatistics(&mut self, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::Result<()>;
    fn ContainsKnowledgeForItem(&mut self, pknowledge: &::core::option::Option<ISyncKnowledge>, pbitemid: *const u8) -> ::windows::core::Result<()>;
    fn ContainsKnowledgeForChangeUnit(&mut self, pknowledge: &::core::option::Option<ISyncKnowledge>, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()>;
    fn ProjectOntoKnowledgeWithPrerequisite(&mut self, pprerequisiteknowledge: &::core::option::Option<ISyncKnowledge>, ptemplateknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn Complement(&mut self, psyncknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn IntersectsWithKnowledge(&mut self, psyncknowledge: &::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn GetKnowledgeCookie(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CompareToKnowledgeCookie(&mut self, pknowledgecookie: &::core::option::Option<::windows::core::IUnknown>, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncKnowledge2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncKnowledge2_Vtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn ProjectOntoColumnSet<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoColumnSet(::core::mem::transmute_copy(&ppcolumns), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerializeWithOptions<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SerializeWithOptions(::core::mem::transmute_copy(&targetformatversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        unsafe extern "system" fn GetLowestUncontainedId<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncknowledge: ::windows::core::RawPtr, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLowestUncontainedId(::core::mem::transmute(&pisyncknowledge), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        unsafe extern "system" fn GetInspector<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInspector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppiinspector)).into()
        }
        unsafe extern "system" fn GetMinimumSupportedVersion<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMinimumSupportedVersion(::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetStatistics<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&which), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForItem<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsKnowledgeForItem(::core::mem::transmute(&pknowledge), ::core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForChangeUnit<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsKnowledgeForChangeUnit(::core::mem::transmute(&pknowledge), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ProjectOntoKnowledgeWithPrerequisite<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr, ptemplateknowledge: ::windows::core::RawPtr, ppprojectedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoKnowledgeWithPrerequisite(::core::mem::transmute(&pprerequisiteknowledge), ::core::mem::transmute(&ptemplateknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprojectedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complement<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr, ppcomplementedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Complement(::core::mem::transmute(&psyncknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomplementedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntersectsWithKnowledge<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IntersectsWithKnowledge(::core::mem::transmute(&psyncknowledge)).into()
        }
        unsafe extern "system" fn GetKnowledgeCookie<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKnowledgeCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgecookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareToKnowledgeCookie<Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompareToKnowledgeCookie(::core::mem::transmute(&pknowledgecookie), ::core::mem::transmute_copy(&presult)).into()
        }
        Self {
            base: ISyncKnowledge_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetIdParameters: GetIdParameters::<Impl, IMPL_OFFSET>,
            ProjectOntoColumnSet: ProjectOntoColumnSet::<Impl, IMPL_OFFSET>,
            SerializeWithOptions: SerializeWithOptions::<Impl, IMPL_OFFSET>,
            GetLowestUncontainedId: GetLowestUncontainedId::<Impl, IMPL_OFFSET>,
            GetInspector: GetInspector::<Impl, IMPL_OFFSET>,
            GetMinimumSupportedVersion: GetMinimumSupportedVersion::<Impl, IMPL_OFFSET>,
            GetStatistics: GetStatistics::<Impl, IMPL_OFFSET>,
            ContainsKnowledgeForItem: ContainsKnowledgeForItem::<Impl, IMPL_OFFSET>,
            ContainsKnowledgeForChangeUnit: ContainsKnowledgeForChangeUnit::<Impl, IMPL_OFFSET>,
            ProjectOntoKnowledgeWithPrerequisite: ProjectOntoKnowledgeWithPrerequisite::<Impl, IMPL_OFFSET>,
            Complement: Complement::<Impl, IMPL_OFFSET>,
            IntersectsWithKnowledge: IntersectsWithKnowledge::<Impl, IMPL_OFFSET>,
            GetKnowledgeCookie: GetKnowledgeCookie::<Impl, IMPL_OFFSET>,
            CompareToKnowledgeCookie: CompareToKnowledgeCookie::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncKnowledge2 as ::windows::core::Interface>::IID || iid == &<ISyncKnowledge as ::windows::core::Interface>::IID
    }
}
pub trait ISyncMergeTombstoneChange_Impl: Sized {
    fn GetWinnerItemId(&mut self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncMergeTombstoneChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncMergeTombstoneChange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncMergeTombstoneChange_Vtbl {
        unsafe extern "system" fn GetWinnerItemId<Impl: ISyncMergeTombstoneChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWinnerItemId(::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetWinnerItemId: GetWinnerItemId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncMergeTombstoneChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncProvider_Impl: Sized {
    fn GetIdParameters(&mut self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProvider_Vtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISyncProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIdParameters: GetIdParameters::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderConfigUI_Impl: Sized {
    fn Init(&mut self, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn GetRegisteredProperties(&mut self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateAndRegisterNewSyncProvider(&mut self, hwndparent: super::super::Foundation::HWND, punkcontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<ISyncProviderInfo>;
    fn ModifySyncProvider(&mut self, hwndparent: super::super::Foundation::HWND, punkcontext: &::core::option::Option<::windows::core::IUnknown>, pproviderinfo: &::core::option::Option<ISyncProviderInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderConfigUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUI_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderConfigUI_Vtbl {
        unsafe extern "system" fn Init<Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute(&pconfigurationproperties)).into()
        }
        unsafe extern "system" fn GetRegisteredProperties<Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconfiguiproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisteredProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfiguiproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndRegisterNewSyncProvider<Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAndRegisterNewSyncProvider(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&punkcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifySyncProvider<Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifySyncProvider(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&punkcontext), ::core::mem::transmute(&pproviderinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            GetRegisteredProperties: GetRegisteredProperties::<Impl, IMPL_OFFSET>,
            CreateAndRegisterNewSyncProvider: CreateAndRegisterNewSyncProvider::<Impl, IMPL_OFFSET>,
            ModifySyncProvider: ModifySyncProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderConfigUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderConfigUIInfo_Impl: Sized + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProviderConfigUI(&mut self, dwclscontext: u32) -> ::windows::core::Result<ISyncProviderConfigUI>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderConfigUIInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUIInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderConfigUIInfo_Vtbl {
        unsafe extern "system" fn GetSyncProviderConfigUI<Impl: ISyncProviderConfigUIInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUI(::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncproviderconfigui = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSyncProviderConfigUI: GetSyncProviderConfigUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderConfigUIInfo as ::windows::core::Interface>::IID || iid == &<super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderInfo_Impl: Sized + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProvider(&mut self, dwclscontext: u32) -> ::windows::core::Result<IRegisteredSyncProvider>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderInfo_Vtbl {
        unsafe extern "system" fn GetSyncProvider<Impl: ISyncProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProvider(::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSyncProvider: GetSyncProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderInfo as ::windows::core::Interface>::IID || iid == &<super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderRegistration_Impl: Sized {
    fn CreateSyncProviderConfigUIRegistrationInstance(&mut self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> ::windows::core::Result<ISyncProviderConfigUIInfo>;
    fn UnregisterSyncProviderConfigUI(&mut self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumerateSyncProviderConfigUIs(&mut self, pguidcontenttype: *const ::windows::core::GUID, dwsupportedarchitecture: u32) -> ::windows::core::Result<IEnumSyncProviderConfigUIInfos>;
    fn CreateSyncProviderRegistrationInstance(&mut self, pproviderconfiguration: *const SyncProviderConfiguration) -> ::windows::core::Result<ISyncProviderInfo>;
    fn UnregisterSyncProvider(&mut self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetSyncProviderConfigUIInfoforProvider(&mut self, pguidproviderinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<ISyncProviderConfigUIInfo>;
    fn EnumerateSyncProviders(&mut self, pguidcontenttype: *const ::windows::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::core::GUID, dwsupportedarchitecture: u32) -> ::windows::core::Result<IEnumSyncProviderInfos>;
    fn GetSyncProviderInfo(&mut self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<ISyncProviderInfo>;
    fn GetSyncProviderFromInstanceId(&mut self, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32) -> ::windows::core::Result<IRegisteredSyncProvider>;
    fn GetSyncProviderConfigUIInfo(&mut self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<ISyncProviderConfigUIInfo>;
    fn GetSyncProviderConfigUIFromInstanceId(&mut self, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32) -> ::windows::core::Result<ISyncProviderConfigUI>;
    fn GetSyncProviderState(&mut self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn SetSyncProviderState(&mut self, pguidinstanceid: *const ::windows::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::core::Result<()>;
    fn RegisterForEvent(&mut self, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn RevokeEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetChange(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<ISyncRegistrationChange>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderRegistration_Vtbl {
        unsafe extern "system" fn CreateSyncProviderConfigUIRegistrationInstance<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSyncProviderConfigUIRegistrationInstance(::core::mem::transmute_copy(&pconfiguiconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfiguiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProviderConfigUI<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterSyncProviderConfigUI(::core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn EnumerateSyncProviderConfigUIs<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSyncProviderConfigUIs(::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute_copy(&dwsupportedarchitecture)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsyncproviderconfiguiinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSyncProviderRegistrationInstance<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSyncProviderRegistrationInstance(::core::mem::transmute_copy(&pproviderconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProvider<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterSyncProvider(::core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfoforProvider<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidproviderinstanceid: *const ::windows::core::GUID, ppproviderconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUIInfoforProvider(::core::mem::transmute_copy(&pguidproviderinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderconfiguiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSyncProviders<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSyncProviders(::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute_copy(&dwstateflagstofiltermask), ::core::mem::transmute_copy(&dwstateflagstofilter), ::core::mem::transmute_copy(&refproviderclsid), ::core::mem::transmute_copy(&dwsupportedarchitecture)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsyncproviderinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderInfo<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderInfo(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderFromInstanceId<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderFromInstanceId(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfo<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUIInfo(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfiguiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIFromInstanceId<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUIFromInstanceId(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfigui = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderState<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pdwstateflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderState(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstateflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncProviderState<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncProviderState(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwstateflagsmask), ::core::mem::transmute_copy(&dwstateflags)).into()
        }
        unsafe extern "system" fn RegisterForEvent<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterForEvent(::core::mem::transmute_copy(&phevent)).into()
        }
        unsafe extern "system" fn RevokeEvent<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeEvent(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn GetChange<Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChange(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateSyncProviderConfigUIRegistrationInstance: CreateSyncProviderConfigUIRegistrationInstance::<Impl, IMPL_OFFSET>,
            UnregisterSyncProviderConfigUI: UnregisterSyncProviderConfigUI::<Impl, IMPL_OFFSET>,
            EnumerateSyncProviderConfigUIs: EnumerateSyncProviderConfigUIs::<Impl, IMPL_OFFSET>,
            CreateSyncProviderRegistrationInstance: CreateSyncProviderRegistrationInstance::<Impl, IMPL_OFFSET>,
            UnregisterSyncProvider: UnregisterSyncProvider::<Impl, IMPL_OFFSET>,
            GetSyncProviderConfigUIInfoforProvider: GetSyncProviderConfigUIInfoforProvider::<Impl, IMPL_OFFSET>,
            EnumerateSyncProviders: EnumerateSyncProviders::<Impl, IMPL_OFFSET>,
            GetSyncProviderInfo: GetSyncProviderInfo::<Impl, IMPL_OFFSET>,
            GetSyncProviderFromInstanceId: GetSyncProviderFromInstanceId::<Impl, IMPL_OFFSET>,
            GetSyncProviderConfigUIInfo: GetSyncProviderConfigUIInfo::<Impl, IMPL_OFFSET>,
            GetSyncProviderConfigUIFromInstanceId: GetSyncProviderConfigUIFromInstanceId::<Impl, IMPL_OFFSET>,
            GetSyncProviderState: GetSyncProviderState::<Impl, IMPL_OFFSET>,
            SetSyncProviderState: SetSyncProviderState::<Impl, IMPL_OFFSET>,
            RegisterForEvent: RegisterForEvent::<Impl, IMPL_OFFSET>,
            RevokeEvent: RevokeEvent::<Impl, IMPL_OFFSET>,
            GetChange: GetChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderRegistration as ::windows::core::Interface>::IID
    }
}
pub trait ISyncRegistrationChange_Impl: Sized {
    fn GetEvent(&mut self) -> ::windows::core::Result<SYNC_REGISTRATION_EVENT>;
    fn GetInstanceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ISyncRegistrationChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncRegistrationChange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncRegistrationChange_Vtbl {
        unsafe extern "system" fn GetEvent<Impl: ISyncRegistrationChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *psreevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Impl: ISyncRegistrationChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEvent: GetEvent::<Impl, IMPL_OFFSET>,
            GetInstanceId: GetInstanceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncRegistrationChange as ::windows::core::Interface>::IID
    }
}
pub trait ISyncSessionExtendedErrorInfo_Impl: Sized {
    fn GetSyncProviderWithError(&mut self) -> ::windows::core::Result<ISyncProvider>;
}
impl ISyncSessionExtendedErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionExtendedErrorInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncSessionExtendedErrorInfo_Vtbl {
        unsafe extern "system" fn GetSyncProviderWithError<Impl: ISyncSessionExtendedErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproviderwitherror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderWithError() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderwitherror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetSyncProviderWithError: GetSyncProviderWithError::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionExtendedErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncSessionState_Impl: Sized {
    fn IsCanceled(&mut self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetInfoForChangeApplication(&mut self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::Result<()>;
    fn LoadInfoFromChangeApplication(&mut self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeStart(&mut self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeEnd(&mut self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::Result<()>;
    fn SetForgottenKnowledgeRecoveryRange(&mut self, prange: *const SYNC_RANGE) -> ::windows::core::Result<()>;
    fn OnProgress(&mut self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncSessionState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncSessionState_Vtbl {
        unsafe extern "system" fn IsCanceled<Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsCanceled(::core::mem::transmute_copy(&pfiscanceled)).into()
        }
        unsafe extern "system" fn GetInfoForChangeApplication<Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInfoForChangeApplication(::core::mem::transmute_copy(&pbchangeapplierinfo), ::core::mem::transmute_copy(&pcbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn LoadInfoFromChangeApplication<Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadInfoFromChangeApplication(::core::mem::transmute_copy(&pbchangeapplierinfo), ::core::mem::transmute_copy(&cbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeStart<Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForgottenKnowledgeRecoveryRangeStart(::core::mem::transmute_copy(&pbrangestart), ::core::mem::transmute_copy(&pcbrangestart)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeEnd<Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForgottenKnowledgeRecoveryRangeEnd(::core::mem::transmute_copy(&pbrangeend), ::core::mem::transmute_copy(&pcbrangeend)).into()
        }
        unsafe extern "system" fn SetForgottenKnowledgeRecoveryRange<Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *const SYNC_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForgottenKnowledgeRecoveryRange(::core::mem::transmute_copy(&prange)).into()
        }
        unsafe extern "system" fn OnProgress<Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&provider), ::core::mem::transmute_copy(&syncstage), ::core::mem::transmute_copy(&dwcompletedwork), ::core::mem::transmute_copy(&dwtotalwork)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsCanceled: IsCanceled::<Impl, IMPL_OFFSET>,
            GetInfoForChangeApplication: GetInfoForChangeApplication::<Impl, IMPL_OFFSET>,
            LoadInfoFromChangeApplication: LoadInfoFromChangeApplication::<Impl, IMPL_OFFSET>,
            GetForgottenKnowledgeRecoveryRangeStart: GetForgottenKnowledgeRecoveryRangeStart::<Impl, IMPL_OFFSET>,
            GetForgottenKnowledgeRecoveryRangeEnd: GetForgottenKnowledgeRecoveryRangeEnd::<Impl, IMPL_OFFSET>,
            SetForgottenKnowledgeRecoveryRange: SetForgottenKnowledgeRecoveryRange::<Impl, IMPL_OFFSET>,
            OnProgress: OnProgress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncSessionState2_Impl: Sized + ISyncSessionState_Impl {
    fn SetProviderWithError(&mut self, fself: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSessionErrorStatus(&mut self, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncSessionState2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncSessionState2_Vtbl {
        unsafe extern "system" fn SetProviderWithError<Impl: ISyncSessionState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fself: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderWithError(::core::mem::transmute_copy(&fself)).into()
        }
        unsafe extern "system" fn GetSessionErrorStatus<Impl: ISyncSessionState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSessionErrorStatus(::core::mem::transmute_copy(&phrsessionerror)).into()
        }
        Self {
            base: ISyncSessionState_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProviderWithError: SetProviderWithError::<Impl, IMPL_OFFSET>,
            GetSessionErrorStatus: GetSessionErrorStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionState2 as ::windows::core::Interface>::IID || iid == &<ISyncSessionState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronousDataRetriever_Impl: Sized {
    fn GetIdParameters(&mut self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
    fn LoadChangeData(&mut self, ploadchangecontext: &::core::option::Option<ILoadChangeContext>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISynchronousDataRetriever_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronousDataRetriever_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronousDataRetriever_Vtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn LoadChangeData<Impl: ISynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadChangeData(::core::mem::transmute(&ploadchangecontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIdParameters: GetIdParameters::<Impl, IMPL_OFFSET>,
            LoadChangeData: LoadChangeData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronousDataRetriever as ::windows::core::Interface>::IID
    }
}
