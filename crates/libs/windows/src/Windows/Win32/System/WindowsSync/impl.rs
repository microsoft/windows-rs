#[cfg(feature = "Win32_Foundation")]
pub trait IAsynchronousDataRetrieverImpl: Sized {
    fn GetIdParameters(&mut self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
    fn RegisterCallback(&mut self, pdataretrievercallback: ::core::option::Option<IDataRetrieverCallback>) -> ::windows::core::Result<()>;
    fn RevokeCallback(&mut self, pdataretrievercallback: ::core::option::Option<IDataRetrieverCallback>) -> ::windows::core::Result<()>;
    fn LoadChangeData(&mut self, ploadchangecontext: ::core::option::Option<ILoadChangeContext>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAsynchronousDataRetrieverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsynchronousDataRetrieverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsynchronousDataRetrieverVtbl {
        unsafe extern "system" fn GetIdParameters<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn RegisterCallback<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCallback(::core::mem::transmute(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn RevokeCallback<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeCallback(::core::mem::transmute(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn LoadChangeData<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IChangeConflictImpl: Sized {
    fn GetDestinationProviderConflictingChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSourceProviderConflictingData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetResolveActionForChange(&mut self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetResolveActionForChange(&mut self, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn GetResolveActionForChangeUnit(&mut self, pchangeunit: ::core::option::Option<ISyncChangeUnit>, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetResolveActionForChangeUnit(&mut self, pchangeunit: ::core::option::Option<ISyncChangeUnit>, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>;
}
impl IChangeConflictVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflictImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChangeConflictVtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolveActionForChange<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResolveActionForChange(::core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChange<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResolveActionForChange(::core::mem::transmute_copy(&resolveaction)).into()
        }
        unsafe extern "system" fn GetResolveActionForChangeUnit<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChangeUnit<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
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
pub trait IChangeUnitExceptionImpl: Sized {
    fn GetItemId(&mut self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitId(&mut self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClockVector(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IChangeUnitExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitExceptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChangeUnitExceptionVtbl {
        unsafe extern "system" fn GetItemId<Impl: IChangeUnitExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemId(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: IChangeUnitExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitId(::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Impl: IChangeUnitExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IChangeUnitListFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn Initialize(&mut self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitIdCount(&mut self, pdwchangeunitidcount: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitId(&mut self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
}
impl IChangeUnitListFilterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitListFilterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChangeUnitListFilterInfoVtbl {
        unsafe extern "system" fn Initialize<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&ppbchangeunitids), ::core::mem::transmute_copy(&dwchangeunitcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitIdCount<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitIdCount(::core::mem::transmute_copy(&pdwchangeunitidcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitId(::core::mem::transmute_copy(&dwchangeunitidindex), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base: ISyncFilterInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetChangeUnitIdCount: GetChangeUnitIdCount::<Impl, IMPL_OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeUnitListFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IClockVectorImpl: Sized {
    fn GetClockVectorElements(&mut self, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetClockVectorElementCount(&mut self, pdwcount: *mut u32) -> ::windows::core::Result<()>;
}
impl IClockVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClockVectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClockVectorVtbl {
        unsafe extern "system" fn GetClockVectorElements<Impl: IClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClockVectorElements(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppienumclockvector)).into()
        }
        unsafe extern "system" fn GetClockVectorElementCount<Impl: IClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IClockVectorElementImpl: Sized {
    fn GetReplicaKey(&mut self, pdwreplicakey: *mut u32) -> ::windows::core::Result<()>;
    fn GetTickCount(&mut self, pulltickcount: *mut u64) -> ::windows::core::Result<()>;
}
impl IClockVectorElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClockVectorElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClockVectorElementVtbl {
        unsafe extern "system" fn GetReplicaKey<Impl: IClockVectorElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetReplicaKey(::core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn GetTickCount<Impl: IClockVectorElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulltickcount: *mut u64) -> ::windows::core::HRESULT {
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
pub trait ICombinedFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn GetFilterCount(&mut self, pdwfiltercount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilterInfo(&mut self, dwfilterindex: u32) -> ::windows::core::Result<ISyncFilterInfo>;
    fn GetFilterCombinationType(&mut self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::Result<()>;
}
impl ICombinedFilterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICombinedFilterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICombinedFilterInfoVtbl {
        unsafe extern "system" fn GetFilterCount<Impl: ICombinedFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterCount(::core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterInfo<Impl: ICombinedFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterInfo(::core::mem::transmute_copy(&dwfilterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifilterinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterCombinationType<Impl: ICombinedFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterCombinationType(::core::mem::transmute_copy(&pfiltercombinationtype)).into()
        }
        Self {
            base: ISyncFilterInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFilterCount: GetFilterCount::<Impl, IMPL_OFFSET>,
            GetFilterInfo: GetFilterInfo::<Impl, IMPL_OFFSET>,
            GetFilterCombinationType: GetFilterCombinationType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICombinedFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IConstraintConflictImpl: Sized {
    fn GetDestinationProviderConflictingChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetDestinationProviderOriginalChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSourceProviderConflictingData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetDestinationProviderOriginalData(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetConstraintResolveActionForChange(&mut self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetConstraintResolveActionForChange(&mut self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn GetConstraintResolveActionForChangeUnit(&mut self, pchangeunit: ::core::option::Option<ISyncChangeUnit>, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn SetConstraintResolveActionForChangeUnit(&mut self, pchangeunit: ::core::option::Option<ISyncChangeUnit>, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>;
    fn GetConstraintConflictReason(&mut self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::Result<()>;
    fn IsTemporary(&mut self) -> ::windows::core::Result<()>;
}
impl IConstraintConflictVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflictImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConstraintConflictVtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginalchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderOriginalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pporiginalchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconflictingdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalData<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderOriginalData() {
                ::core::result::Result::Ok(ok__) => {
                    *pporiginaldata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstraintResolveActionForChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstraintResolveActionForChange(::core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConstraintResolveActionForChange(::core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintResolveActionForChangeUnit<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstraintResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChangeUnit<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConstraintResolveActionForChangeUnit(::core::mem::transmute(&pchangeunit), ::core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintConflictReason<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConstraintConflictReason(::core::mem::transmute_copy(&pconstraintconflictreason)).into()
        }
        unsafe extern "system" fn IsTemporary<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IConstructReplicaKeyMapImpl: Sized {
    fn FindOrAddReplica(&mut self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::Result<()>;
}
impl IConstructReplicaKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConstructReplicaKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConstructReplicaKeyMapVtbl {
        unsafe extern "system" fn FindOrAddReplica<Impl: IConstructReplicaKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindOrAddReplica(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FindOrAddReplica: FindOrAddReplica::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConstructReplicaKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait ICoreFragmentImpl: Sized {
    fn NextColumn(&mut self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::Result<()>;
    fn NextRange(&mut self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::core::option::Option<IClockVector>) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn GetColumnCount(&mut self, pcolumncount: *mut u32) -> ::windows::core::Result<()>;
    fn GetRangeCount(&mut self, prangecount: *mut u32) -> ::windows::core::Result<()>;
}
impl ICoreFragmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFragmentVtbl {
        unsafe extern "system" fn NextColumn<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextColumn(::core::mem::transmute_copy(&pchangeunitid), ::core::mem::transmute_copy(&pchangeunitidsize)).into()
        }
        unsafe extern "system" fn NextRange<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextRange(::core::mem::transmute_copy(&pitemid), ::core::mem::transmute_copy(&pitemidsize), ::core::mem::transmute_copy(&piclockvector)).into()
        }
        unsafe extern "system" fn Reset<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn GetColumnCount<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolumncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColumnCount(::core::mem::transmute_copy(&pcolumncount)).into()
        }
        unsafe extern "system" fn GetRangeCount<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ICoreFragmentInspectorImpl: Sized {
    fn NextCoreFragments(&mut self, requestedcount: u32, ppicorefragments: *mut ::core::option::Option<ICoreFragment>, pfetchedcount: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
impl ICoreFragmentInspectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragmentInspectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFragmentInspectorVtbl {
        unsafe extern "system" fn NextCoreFragments<Impl: ICoreFragmentInspectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut ::windows::core::RawPtr, pfetchedcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextCoreFragments(::core::mem::transmute_copy(&requestedcount), ::core::mem::transmute_copy(&ppicorefragments), ::core::mem::transmute_copy(&pfetchedcount)).into()
        }
        unsafe extern "system" fn Reset<Impl: ICoreFragmentInspectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ICustomFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn GetSyncFilter(&mut self) -> ::windows::core::Result<ISyncFilter>;
}
impl ICustomFilterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomFilterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomFilterInfoVtbl {
        unsafe extern "system" fn GetSyncFilter<Impl: ICustomFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *pisyncfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ISyncFilterInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetSyncFilter: GetSyncFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IDataRetrieverCallbackImpl: Sized {
    fn LoadChangeDataComplete(&mut self, punkdata: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn LoadChangeDataError(&mut self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IDataRetrieverCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRetrieverCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataRetrieverCallbackVtbl {
        unsafe extern "system" fn LoadChangeDataComplete<Impl: IDataRetrieverCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadChangeDataComplete(::core::mem::transmute(&punkdata)).into()
        }
        unsafe extern "system" fn LoadChangeDataError<Impl: IDataRetrieverCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
pub trait IEnumChangeUnitExceptionsImpl: Sized {
    fn Next(&mut self, cexceptions: u32, ppchangeunitexception: *mut ::core::option::Option<IChangeUnitException>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cexceptions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumChangeUnitExceptions>;
}
impl IEnumChangeUnitExceptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumChangeUnitExceptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumChangeUnitExceptionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&ppchangeunitexception), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumClockVectorImpl: Sized {
    fn Next(&mut self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IClockVectorElement>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, csyncversions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumClockVector>;
}
impl IEnumClockVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumClockVectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumClockVectorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cclockvectorelements), ::core::mem::transmute_copy(&ppiclockvectorelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumFeedClockVectorImpl: Sized {
    fn Next(&mut self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, csyncversions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumFeedClockVector>;
}
impl IEnumFeedClockVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFeedClockVectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumFeedClockVectorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cclockvectorelements), ::core::mem::transmute_copy(&ppiclockvectorelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumItemIdsImpl: Sized {
    fn Next(&mut self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::Result<()>;
}
impl IEnumItemIdsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemIdsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumItemIdsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumItemIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumItemIds as ::windows::core::Interface>::IID
    }
}
pub trait IEnumRangeExceptionsImpl: Sized {
    fn Next(&mut self, cexceptions: u32, pprangeexception: *mut ::core::option::Option<IRangeException>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cexceptions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumRangeExceptions>;
}
impl IEnumRangeExceptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRangeExceptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumRangeExceptionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumRangeExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, pprangeexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&pprangeexception), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumRangeExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumRangeExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumRangeExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumSingleItemExceptionsImpl: Sized {
    fn Next(&mut self, cexceptions: u32, ppsingleitemexception: *mut ::core::option::Option<ISingleItemException>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cexceptions: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSingleItemExceptions>;
}
impl IEnumSingleItemExceptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSingleItemExceptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSingleItemExceptionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&ppsingleitemexception), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumSyncChangeUnitsImpl: Sized {
    fn Next(&mut self, cchanges: u32, ppchangeunit: *mut ::core::option::Option<ISyncChangeUnit>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cchanges: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSyncChangeUnits>;
}
impl IEnumSyncChangeUnitsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangeUnitsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncChangeUnitsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchangeunit: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&ppchangeunit), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumSyncChangesImpl: Sized {
    fn Next(&mut self, cchanges: u32, ppchange: *mut ::core::option::Option<ISyncChange>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cchanges: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSyncChanges>;
}
impl IEnumSyncChangesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncChangesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncChangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchange: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&ppchange), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncChangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncChangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncChangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumSyncProviderConfigUIInfosImpl: Sized {
    fn Next(&mut self, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::core::option::Option<ISyncProviderConfigUIInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cfactories: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSyncProviderConfigUIInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderConfigUIInfosVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderConfigUIInfosImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncProviderConfigUIInfosVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cfactories), ::core::mem::transmute_copy(&ppsyncproviderconfiguiinfo), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfactories: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cfactories)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumSyncProviderInfosImpl: Sized {
    fn Next(&mut self, cinstances: u32, ppsyncproviderinfo: *mut ::core::option::Option<ISyncProviderInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cinstances: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSyncProviderInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderInfosVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderInfosImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncProviderInfosVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncProviderInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cinstances), ::core::mem::transmute_copy(&ppsyncproviderinfo), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncProviderInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinstances: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cinstances)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncProviderInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncProviderInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFeedClockVectorImpl: Sized + IClockVectorImpl {
    fn GetUpdateCount(&mut self, pdwupdatecount: *mut u32) -> ::windows::core::Result<()>;
    fn IsNoConflictsSpecified(&mut self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFeedClockVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedClockVectorVtbl {
        unsafe extern "system" fn GetUpdateCount<Impl: IFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwupdatecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUpdateCount(::core::mem::transmute_copy(&pdwupdatecount)).into()
        }
        unsafe extern "system" fn IsNoConflictsSpecified<Impl: IFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsNoConflictsSpecified(::core::mem::transmute_copy(&pfisnoconflictsspecified)).into()
        }
        Self {
            base: IClockVectorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetUpdateCount: GetUpdateCount::<Impl, IMPL_OFFSET>,
            IsNoConflictsSpecified: IsNoConflictsSpecified::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IFeedClockVectorElementImpl: Sized + IClockVectorElementImpl {
    fn GetSyncTime(&mut self, psynctime: *mut SYNC_TIME) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self, pbflags: *mut u8) -> ::windows::core::Result<()>;
}
impl IFeedClockVectorElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVectorElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedClockVectorElementVtbl {
        unsafe extern "system" fn GetSyncTime<Impl: IFeedClockVectorElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynctime: *mut SYNC_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSyncTime(::core::mem::transmute_copy(&psynctime)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: IFeedClockVectorElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbflags: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pbflags)).into()
        }
        Self {
            base: IClockVectorElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSyncTime: GetSyncTime::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedClockVectorElement as ::windows::core::Interface>::IID
    }
}
pub trait IFilterKeyMapImpl: Sized {
    fn GetCount(&mut self, pdwcount: *mut u32) -> ::windows::core::Result<()>;
    fn AddFilter(&mut self, pisyncfilter: ::core::option::Option<ISyncFilter>, pdwfilterkey: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilter(&mut self, dwfilterkey: u32) -> ::windows::core::Result<ISyncFilter>;
    fn Serialize(&mut self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::Result<()>;
}
impl IFilterKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterKeyMapVtbl {
        unsafe extern "system" fn GetCount<Impl: IFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn AddFilter<Impl: IFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncfilter: ::windows::core::RawPtr, pdwfilterkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFilter(::core::mem::transmute(&pisyncfilter), ::core::mem::transmute_copy(&pdwfilterkey)).into()
        }
        unsafe extern "system" fn GetFilter<Impl: IFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilter(::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppisyncfilter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: IFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IFilterRequestCallbackImpl: Sized {
    fn RequestFilter(&mut self, pfilter: ::core::option::Option<::windows::core::IUnknown>, filteringtype: FILTERING_TYPE) -> ::windows::core::Result<()>;
}
impl IFilterRequestCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterRequestCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterRequestCallbackVtbl {
        unsafe extern "system" fn RequestFilter<Impl: IFilterRequestCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestFilter(::core::mem::transmute(&pfilter), ::core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), RequestFilter: RequestFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IFilterTrackingProviderImpl: Sized {
    fn SpecifyTrackedFilters(&mut self, pcallback: ::core::option::Option<IFilterTrackingRequestCallback>) -> ::windows::core::Result<()>;
    fn AddTrackedFilter(&mut self, pfilter: ::core::option::Option<ISyncFilter>) -> ::windows::core::Result<()>;
}
impl IFilterTrackingProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterTrackingProviderVtbl {
        unsafe extern "system" fn SpecifyTrackedFilters<Impl: IFilterTrackingProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SpecifyTrackedFilters(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn AddTrackedFilter<Impl: IFilterTrackingProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFilterTrackingRequestCallbackImpl: Sized {
    fn RequestTrackedFilter(&mut self, pfilter: ::core::option::Option<ISyncFilter>) -> ::windows::core::Result<()>;
}
impl IFilterTrackingRequestCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingRequestCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterTrackingRequestCallbackVtbl {
        unsafe extern "system" fn RequestTrackedFilter<Impl: IFilterTrackingRequestCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFilterTrackingSyncChangeBuilderImpl: Sized {
    fn AddFilterChange(&mut self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::Result<()>;
    fn SetAllChangeUnitsPresentFlag(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFilterTrackingSyncChangeBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingSyncChangeBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterTrackingSyncChangeBuilderVtbl {
        unsafe extern "system" fn AddFilterChange<Impl: IFilterTrackingSyncChangeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFilterChange(::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn SetAllChangeUnitsPresentFlag<Impl: IFilterTrackingSyncChangeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IForgottenKnowledgeImpl: Sized + ISyncKnowledgeImpl {
    fn ForgetToVersion(&mut self, pknowledge: ::core::option::Option<ISyncKnowledge>, pversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IForgottenKnowledgeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForgottenKnowledgeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IForgottenKnowledgeVtbl {
        unsafe extern "system" fn ForgetToVersion<Impl: IForgottenKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForgetToVersion(::core::mem::transmute(&pknowledge), ::core::mem::transmute_copy(&pversion)).into()
        }
        Self { base: ISyncKnowledgeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ForgetToVersion: ForgetToVersion::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForgottenKnowledge as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKnowledgeSyncProviderImpl: Sized + ISyncProviderImpl {
    fn BeginSession(&mut self, role: SYNC_PROVIDER_ROLE, psessionstate: ::core::option::Option<ISyncSessionState>) -> ::windows::core::Result<()>;
    fn GetSyncBatchParameters(&mut self, ppsyncknowledge: *mut ::core::option::Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeBatch(&mut self, dwbatchsize: u32, psyncknowledge: ::core::option::Option<ISyncKnowledge>, ppsyncchangebatch: *mut ::core::option::Option<ISyncChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetFullEnumerationChangeBatch(&mut self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: ::core::option::Option<ISyncKnowledge>, ppsyncchangebatch: *mut ::core::option::Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ProcessChangeBatch(&mut self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::core::option::Option<ISyncChangeBatch>, punkdataretriever: ::core::option::Option<::windows::core::IUnknown>, pcallback: ::core::option::Option<ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::Result<()>;
    fn ProcessFullEnumerationChangeBatch(&mut self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::core::option::Option<ISyncFullEnumerationChangeBatch>, punkdataretriever: ::core::option::Option<::windows::core::IUnknown>, pcallback: ::core::option::Option<ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::Result<()>;
    fn EndSession(&mut self, psessionstate: ::core::option::Option<ISyncSessionState>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKnowledgeSyncProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnowledgeSyncProviderVtbl {
        unsafe extern "system" fn BeginSession<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginSession(::core::mem::transmute_copy(&role), ::core::mem::transmute(&psessionstate)).into()
        }
        unsafe extern "system" fn GetSyncBatchParameters<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncknowledge: *mut ::windows::core::RawPtr, pdwrequestedbatchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSyncBatchParameters(::core::mem::transmute_copy(&ppsyncknowledge), ::core::mem::transmute_copy(&pdwrequestedbatchsize)).into()
        }
        unsafe extern "system" fn GetChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeBatch(::core::mem::transmute_copy(&dwbatchsize), ::core::mem::transmute(&psyncknowledge), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn GetFullEnumerationChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFullEnumerationChangeBatch(::core::mem::transmute_copy(&dwbatchsize), ::core::mem::transmute_copy(&pblowerenumerationbound), ::core::mem::transmute(&psyncknowledge), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn ProcessChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessChangeBatch(::core::mem::transmute_copy(&resolutionpolicy), ::core::mem::transmute(&psourcechangebatch), ::core::mem::transmute(&punkdataretriever), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn ProcessFullEnumerationChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessFullEnumerationChangeBatch(::core::mem::transmute_copy(&resolutionpolicy), ::core::mem::transmute(&psourcechangebatch), ::core::mem::transmute(&punkdataretriever), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn EndSession<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndSession(::core::mem::transmute(&psessionstate)).into()
        }
        Self {
            base: ISyncProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<IKnowledgeSyncProvider as ::windows::core::Interface>::IID
    }
}
pub trait ILoadChangeContextImpl: Sized {
    fn GetSyncChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn SetRecoverableErrorOnChange(&mut self, hrerror: ::windows::core::HRESULT, perrordata: ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
    fn SetRecoverableErrorOnChangeUnit(&mut self, hrerror: ::windows::core::HRESULT, pchangeunit: ::core::option::Option<ISyncChangeUnit>, perrordata: ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
}
impl ILoadChangeContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoadChangeContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoadChangeContextVtbl {
        unsafe extern "system" fn GetSyncChange<Impl: ILoadChangeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoverableErrorOnChange<Impl: ILoadChangeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecoverableErrorOnChange(::core::mem::transmute_copy(&hrerror), ::core::mem::transmute(&perrordata)).into()
        }
        unsafe extern "system" fn SetRecoverableErrorOnChangeUnit<Impl: ILoadChangeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, pchangeunit: ::windows::core::RawPtr, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IProviderConverterImpl: Sized {
    fn Initialize(&mut self, pisyncprovider: ::core::option::Option<ISyncProvider>) -> ::windows::core::Result<()>;
}
impl IProviderConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderConverterVtbl {
        unsafe extern "system" fn Initialize<Impl: IProviderConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pisyncprovider)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderConverter as ::windows::core::Interface>::IID
    }
}
pub trait IRangeExceptionImpl: Sized {
    fn GetClosedRangeStart(&mut self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClosedRangeEnd(&mut self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClockVector(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IRangeExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeExceptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeExceptionVtbl {
        unsafe extern "system" fn GetClosedRangeStart<Impl: IRangeExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosedRangeStart(::core::mem::transmute_copy(&pbclosedrangestart), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedRangeEnd<Impl: IRangeExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosedRangeEnd(::core::mem::transmute_copy(&pbclosedrangeend), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Impl: IRangeExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IRecoverableErrorImpl: Sized {
    fn GetStage(&mut self, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::Result<()>;
    fn GetProvider(&mut self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::Result<()>;
    fn GetChangeWithRecoverableError(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetRecoverableErrorDataForChange(&mut self, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
    fn GetRecoverableErrorDataForChangeUnit(&mut self, pchangeunit: ::core::option::Option<ISyncChangeUnit>, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>;
}
impl IRecoverableErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRecoverableErrorVtbl {
        unsafe extern "system" fn GetStage<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStage(::core::mem::transmute_copy(&pstage)).into()
        }
        unsafe extern "system" fn GetProvider<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProvider(::core::mem::transmute_copy(&pproviderrole)).into()
        }
        unsafe extern "system" fn GetChangeWithRecoverableError<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppchangewithrecoverableerror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeWithRecoverableError() {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangewithrecoverableerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChange<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRecoverableErrorDataForChange(::core::mem::transmute_copy(&phrerror), ::core::mem::transmute_copy(&pperrordata)).into()
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChangeUnit<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRecoverableErrorDataImpl: Sized {
    fn Initialize(&mut self, pcszitemdisplayname: super::super::Foundation::PWSTR, pcszerrordescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetItemDisplayName(&mut self, pszitemdisplayname: super::super::Foundation::PWSTR, pcchitemdisplayname: *mut u32) -> ::windows::core::Result<()>;
    fn GetErrorDescription(&mut self, pszerrordescription: super::super::Foundation::PWSTR, pccherrordescription: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRecoverableErrorDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableErrorDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRecoverableErrorDataVtbl {
        unsafe extern "system" fn Initialize<Impl: IRecoverableErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszitemdisplayname: super::super::Foundation::PWSTR, pcszerrordescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pcszitemdisplayname), ::core::mem::transmute_copy(&pcszerrordescription)).into()
        }
        unsafe extern "system" fn GetItemDisplayName<Impl: IRecoverableErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitemdisplayname: super::super::Foundation::PWSTR, pcchitemdisplayname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemDisplayName(::core::mem::transmute_copy(&pszitemdisplayname), ::core::mem::transmute_copy(&pcchitemdisplayname)).into()
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IRecoverableErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszerrordescription: super::super::Foundation::PWSTR, pccherrordescription: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IRegisteredSyncProviderImpl: Sized {
    fn Init(&mut self, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn GetInstanceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IRegisteredSyncProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredSyncProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegisteredSyncProviderVtbl {
        unsafe extern "system" fn Init<Impl: IRegisteredSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute(&pcontextpropertystore)).into()
        }
        unsafe extern "system" fn GetInstanceId<Impl: IRegisteredSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRegisteredSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IReplicaKeyMapImpl: Sized {
    fn LookupReplicaKey(&mut self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::Result<()>;
    fn LookupReplicaId(&mut self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::Result<()>;
}
impl IReplicaKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReplicaKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReplicaKeyMapVtbl {
        unsafe extern "system" fn LookupReplicaKey<Impl: IReplicaKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LookupReplicaKey(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn LookupReplicaId<Impl: IReplicaKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LookupReplicaId(::core::mem::transmute_copy(&dwreplicakey), ::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Impl: IReplicaKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IRequestFilteredSyncImpl: Sized {
    fn SpecifyFilter(&mut self, pcallback: ::core::option::Option<IFilterRequestCallback>) -> ::windows::core::Result<()>;
}
impl IRequestFilteredSyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRequestFilteredSyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRequestFilteredSyncVtbl {
        unsafe extern "system" fn SpecifyFilter<Impl: IRequestFilteredSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SpecifyFilter(::core::mem::transmute(&pcallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SpecifyFilter: SpecifyFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRequestFilteredSync as ::windows::core::Interface>::IID
    }
}
pub trait ISingleItemExceptionImpl: Sized {
    fn GetItemId(&mut self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClockVector(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ISingleItemExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISingleItemExceptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISingleItemExceptionVtbl {
        unsafe extern "system" fn GetItemId<Impl: ISingleItemExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemId(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Impl: ISingleItemExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ISupportFilteredSyncImpl: Sized {
    fn AddFilter(&mut self, pfilter: ::core::option::Option<::windows::core::IUnknown>, filteringtype: FILTERING_TYPE) -> ::windows::core::Result<()>;
}
impl ISupportFilteredSyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportFilteredSyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISupportFilteredSyncVtbl {
        unsafe extern "system" fn AddFilter<Impl: ISupportFilteredSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFilter(::core::mem::transmute(&pfilter), ::core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddFilter: AddFilter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportFilteredSync as ::windows::core::Interface>::IID
    }
}
pub trait ISupportLastWriteTimeImpl: Sized {
    fn GetItemChangeTime(&mut self, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::Result<()>;
    fn GetChangeUnitChangeTime(&mut self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::Result<()>;
}
impl ISupportLastWriteTimeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportLastWriteTimeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISupportLastWriteTimeVtbl {
        unsafe extern "system" fn GetItemChangeTime<Impl: ISupportLastWriteTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetItemChangeTime(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pulltimestamp)).into()
        }
        unsafe extern "system" fn GetChangeUnitChangeTime<Impl: ISupportLastWriteTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
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
pub trait ISyncCallbackImpl: Sized {
    fn OnProgress(&mut self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()>;
    fn OnChange(&mut self, psyncchange: ::core::option::Option<ISyncChange>) -> ::windows::core::Result<()>;
    fn OnConflict(&mut self, pconflict: ::core::option::Option<IChangeConflict>) -> ::windows::core::Result<()>;
    fn OnFullEnumerationNeeded(&mut self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::Result<()>;
    fn OnRecoverableError(&mut self, precoverableerror: ::core::option::Option<IRecoverableError>) -> ::windows::core::Result<()>;
}
impl ISyncCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncCallbackVtbl {
        unsafe extern "system" fn OnProgress<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&provider), ::core::mem::transmute_copy(&syncstage), ::core::mem::transmute_copy(&dwcompletedwork), ::core::mem::transmute_copy(&dwtotalwork)).into()
        }
        unsafe extern "system" fn OnChange<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChange(::core::mem::transmute(&psyncchange)).into()
        }
        unsafe extern "system" fn OnConflict<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConflict(::core::mem::transmute(&pconflict)).into()
        }
        unsafe extern "system" fn OnFullEnumerationNeeded<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFullEnumerationNeeded(::core::mem::transmute_copy(&pfullenumerationaction)).into()
        }
        unsafe extern "system" fn OnRecoverableError<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precoverableerror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISyncCallback2Impl: Sized + ISyncCallbackImpl {
    fn OnChangeApplied(&mut self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::Result<()>;
    fn OnChangeFailed(&mut self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::Result<()>;
}
impl ISyncCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncCallback2Vtbl {
        unsafe extern "system" fn OnChangeApplied<Impl: ISyncCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChangeApplied(::core::mem::transmute_copy(&dwchangesapplied), ::core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        unsafe extern "system" fn OnChangeFailed<Impl: ISyncCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChangeFailed(::core::mem::transmute_copy(&dwchangesapplied), ::core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        Self {
            base: ISyncCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnChangeApplied: OnChangeApplied::<Impl, IMPL_OFFSET>,
            OnChangeFailed: OnChangeFailed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncCallback2 as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeImpl: Sized {
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
impl ISyncChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeVtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOwnerReplicaId(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetRootItemId<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRootItemId(::core::mem::transmute_copy(&pbrootitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeVersion<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeVersion(::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetCreationVersion<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCreationVersion(::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetWorkEstimate<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwork: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWorkEstimate(::core::mem::transmute_copy(&pdwwork)).into()
        }
        unsafe extern "system" fn GetChangeUnits<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeUnits() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMadeWithKnowledge<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmadewithknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMadeWithKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmadewithknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkEstimate<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwork: u32) -> ::windows::core::HRESULT {
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
pub trait ISyncChangeBatchImpl: Sized + ISyncChangeBatchBaseImpl {
    fn BeginUnorderedGroup(&mut self) -> ::windows::core::Result<()>;
    fn EndUnorderedGroup(&mut self, pmadewithknowledge: ::core::option::Option<ISyncKnowledge>, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AddLoggedConflict(&mut self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchVtbl {
        unsafe extern "system" fn BeginUnorderedGroup<Impl: ISyncChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUnorderedGroup().into()
        }
        unsafe extern "system" fn EndUnorderedGroup<Impl: ISyncChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmadewithknowledge: ::windows::core::RawPtr, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndUnorderedGroup(::core::mem::transmute(&pmadewithknowledge), ::core::mem::transmute_copy(&fallchangesforknowledge)).into()
        }
        unsafe extern "system" fn AddLoggedConflict<Impl: ISyncChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ISyncChangeBatchBaseVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginUnorderedGroup: BeginUnorderedGroup::<Impl, IMPL_OFFSET>,
            EndUnorderedGroup: EndUnorderedGroup::<Impl, IMPL_OFFSET>,
            AddLoggedConflict: AddLoggedConflict::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatch2Impl: Sized + ISyncChangeBatchBaseImpl + ISyncChangeBatchImpl {
    fn AddMergeTombstoneMetadataToGroup(&mut self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder>;
    fn AddMergeTombstoneLoggedConflict(&mut self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatch2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatch2Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Impl: ISyncChangeBatch2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMergeTombstoneMetadataToGroup(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMergeTombstoneLoggedConflict<Impl: ISyncChangeBatch2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ISyncChangeBatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Impl, IMPL_OFFSET>,
            AddMergeTombstoneLoggedConflict: AddMergeTombstoneLoggedConflict::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatch2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchAdvancedImpl: Sized {
    fn GetFilterInfo(&mut self) -> ::windows::core::Result<ISyncFilterInfo>;
    fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&mut self) -> ::windows::core::Result<ISyncChangeBatch>;
    fn GetUpperBoundItemId(&mut self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetBatchLevelKnowledgeShouldBeApplied(&mut self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchAdvancedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchAdvancedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchAdvancedVtbl {
        unsafe extern "system" fn GetFilterInfo<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfilterinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFullEnumerationChangeBatchToRegularChangeBatch<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppchangebatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertFullEnumerationChangeBatchToRegularChangeBatch() {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperBoundItemId<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUpperBoundItemId(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetBatchLevelKnowledgeShouldBeApplied<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait ISyncChangeBatchBaseImpl: Sized {
    fn GetChangeEnumerator(&mut self) -> ::windows::core::Result<IEnumSyncChanges>;
    fn GetIsLastBatch(&mut self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetWorkEstimateForBatch(&mut self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()>;
    fn GetRemainingWorkEstimateForSession(&mut self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()>;
    fn BeginOrderedGroup(&mut self, pblowerbound: *const u8) -> ::windows::core::Result<()>;
    fn EndOrderedGroup(&mut self, pbupperbound: *const u8, pmadewithknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
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
impl ISyncChangeBatchBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchBaseVtbl {
        unsafe extern "system" fn GetChangeEnumerator<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsLastBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIsLastBatch(::core::mem::transmute_copy(&pflastbatch)).into()
        }
        unsafe extern "system" fn GetWorkEstimateForBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwworkforbatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWorkEstimateForBatch(::core::mem::transmute_copy(&pdwworkforbatch)).into()
        }
        unsafe extern "system" fn GetRemainingWorkEstimateForSession<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemainingWorkEstimateForSession(::core::mem::transmute_copy(&pdwremainingworkforsession)).into()
        }
        unsafe extern "system" fn BeginOrderedGroup<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblowerbound: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginOrderedGroup(::core::mem::transmute_copy(&pblowerbound)).into()
        }
        unsafe extern "system" fn EndOrderedGroup<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndOrderedGroup(::core::mem::transmute_copy(&pbupperbound), ::core::mem::transmute(&pmadewithknowledge)).into()
        }
        unsafe extern "system" fn AddItemMetadataToGroup<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddItemMetadataToGroup(::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchangebuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisiteKnowledge<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprerequisteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrerequisiteKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprerequisteknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceForgottenKnowledge<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsourceforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceForgottenKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsourceforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastBatch().into()
        }
        unsafe extern "system" fn SetWorkEstimateForBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwworkforbatch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkEstimateForBatch(::core::mem::transmute_copy(&dwworkforbatch)).into()
        }
        unsafe extern "system" fn SetRemainingWorkEstimateForSession<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwremainingworkforsession: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemainingWorkEstimateForSession(::core::mem::transmute_copy(&dwremainingworkforsession)).into()
        }
        unsafe extern "system" fn Serialize<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ISyncChangeBatchBase2Impl: Sized + ISyncChangeBatchBaseImpl {
    fn SerializeWithOptions(&mut self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchBase2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchBase2Vtbl {
        unsafe extern "system" fn SerializeWithOptions<Impl: ISyncChangeBatchBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SerializeWithOptions(::core::mem::transmute_copy(&targetformatversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        Self {
            base: ISyncChangeBatchBaseVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SerializeWithOptions: SerializeWithOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase2 as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeBatchWithFilterKeyMapImpl: Sized {
    fn GetFilterKeyMap(&mut self) -> ::windows::core::Result<IFilterKeyMap>;
    fn SetFilterKeyMap(&mut self, pifilterkeymap: ::core::option::Option<IFilterKeyMap>) -> ::windows::core::Result<()>;
    fn SetFilterForgottenKnowledge(&mut self, dwfilterkey: u32, pfilterforgottenknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn GetFilteredReplicaLearnedKnowledge(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
}
impl ISyncChangeBatchWithFilterKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchWithFilterKeyMapVtbl {
        unsafe extern "system" fn GetFilterKeyMap<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifilterkeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterKeyMap() {
                ::core::result::Result::Ok(ok__) => {
                    *ppifilterkeymap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterKeyMap<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifilterkeymap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilterKeyMap(::core::mem::transmute(&pifilterkeymap)).into()
        }
        unsafe extern "system" fn SetFilterForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilterForgottenKnowledge(::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute(&pfilterforgottenknowledge)).into()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISyncChangeBatchWithPrerequisiteImpl: Sized + ISyncChangeBatchBaseImpl {
    fn SetPrerequisiteKnowledge(&mut self, pprerequisiteknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn GetLearnedKnowledgeWithPrerequisite(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&mut self) -> ::windows::core::Result<IForgottenKnowledge>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchWithPrerequisiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithPrerequisiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchWithPrerequisiteVtbl {
        unsafe extern "system" fn SetPrerequisiteKnowledge<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrerequisiteKnowledge(::core::mem::transmute(&pprerequisiteknowledge)).into()
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedwithprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeWithPrerequisite(::core::mem::transmute(&pdestinationknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedwithprerequisiteknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ISyncChangeBatchBaseVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPrerequisiteKnowledge: SetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Impl, IMPL_OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithPrerequisite as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeBuilderImpl: Sized {
    fn AddChangeUnitMetadata(&mut self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
}
impl ISyncChangeBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBuilderVtbl {
        unsafe extern "system" fn AddChangeUnitMetadata<Impl: ISyncChangeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddChangeUnitMetadata(::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pchangeunitversion)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AddChangeUnitMetadata: AddChangeUnitMetadata::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBuilder as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeUnitImpl: Sized {
    fn GetItemChange(&mut self) -> ::windows::core::Result<ISyncChange>;
    fn GetChangeUnitId(&mut self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetChangeUnitVersion(&mut self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
}
impl ISyncChangeUnitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeUnitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeUnitVtbl {
        unsafe extern "system" fn GetItemChange<Impl: ISyncChangeUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemChange() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncchange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: ISyncChangeUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitId(::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitVersion<Impl: ISyncChangeUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
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
pub trait ISyncChangeWithFilterKeyMapImpl: Sized {
    fn GetFilterCount(&mut self, pdwfiltercount: *mut u32) -> ::windows::core::Result<()>;
    fn GetFilterChange(&mut self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::Result<()>;
    fn GetAllChangeUnitsPresentFlag(&mut self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFilterForgottenKnowledge(&mut self, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedKnowledge(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>, pnewmoveins: ::core::option::Option<IEnumItemIds>, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeWithFilterKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeWithFilterKeyMapVtbl {
        unsafe extern "system" fn GetFilterCount<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterCount(::core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterChange<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterChange(::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn GetAllChangeUnitsPresentFlag<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAllChangeUnitsPresentFlag(::core::mem::transmute_copy(&pfallchangeunitspresent)).into()
        }
        unsafe extern "system" fn GetFilterForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterForgottenKnowledge(::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedfilterforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledge(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(::core::mem::transmute(&pdestinationknowledge), ::core::mem::transmute(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedforgottenknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISyncChangeWithPrerequisiteImpl: Sized {
    fn GetPrerequisiteKnowledge(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledgeWithPrerequisite(&mut self, pdestinationknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
}
impl ISyncChangeWithPrerequisiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithPrerequisiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeWithPrerequisiteVtbl {
        unsafe extern "system" fn GetPrerequisiteKnowledge<Impl: ISyncChangeWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrerequisiteKnowledge() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprerequisiteknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Impl: ISyncChangeWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedknowledgewithprerequisite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISyncConstraintCallbackImpl: Sized {
    fn OnConstraintConflict(&mut self, pconflict: ::core::option::Option<IConstraintConflict>) -> ::windows::core::Result<()>;
}
impl ISyncConstraintCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncConstraintCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncConstraintCallbackVtbl {
        unsafe extern "system" fn OnConstraintConflict<Impl: ISyncConstraintCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConstraintConflict(::core::mem::transmute(&pconflict)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnConstraintConflict: OnConstraintConflict::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncConstraintCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISyncDataConverterImpl: Sized {
    fn ConvertDataRetrieverFromProviderFormat(&mut self, punkdataretrieverin: ::core::option::Option<::windows::core::IUnknown>, penumsyncchanges: ::core::option::Option<IEnumSyncChanges>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ConvertDataRetrieverToProviderFormat(&mut self, punkdataretrieverin: ::core::option::Option<::windows::core::IUnknown>, penumsyncchanges: ::core::option::Option<IEnumSyncChanges>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ConvertDataFromProviderFormat(&mut self, pdatacontext: ::core::option::Option<ILoadChangeContext>, punkdatain: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ConvertDataToProviderFormat(&mut self, pdatacontext: ::core::option::Option<ILoadChangeContext>, punkdataout: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ISyncDataConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncDataConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncDataConverterVtbl {
        unsafe extern "system" fn ConvertDataRetrieverFromProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertDataRetrieverFromProviderFormat(::core::mem::transmute(&punkdataretrieverin), ::core::mem::transmute(&penumsyncchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataRetrieverToProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertDataRetrieverToProviderFormat(::core::mem::transmute(&punkdataretrieverin), ::core::mem::transmute(&penumsyncchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataFromProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertDataFromProviderFormat(::core::mem::transmute(&pdatacontext), ::core::mem::transmute(&punkdatain)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataToProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ISyncFilterImpl: Sized {
    fn IsIdentical(&mut self, psyncfilter: ::core::option::Option<ISyncFilter>) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterVtbl {
        unsafe extern "system" fn IsIdentical<Impl: ISyncFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsIdentical(::core::mem::transmute(&psyncfilter)).into()
        }
        unsafe extern "system" fn Serialize<Impl: ISyncFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ISyncFilterDeserializerImpl: Sized {
    fn DeserializeSyncFilter(&mut self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> ::windows::core::Result<ISyncFilter>;
}
impl ISyncFilterDeserializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterDeserializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterDeserializerVtbl {
        unsafe extern "system" fn DeserializeSyncFilter<Impl: ISyncFilterDeserializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISyncFilterInfoImpl: Sized {
    fn Serialize(&mut self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncFilterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterInfoVtbl {
        unsafe extern "system" fn Serialize<Impl: ISyncFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pcbbuffer)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Serialize: Serialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterInfo2Impl: Sized + ISyncFilterInfoImpl {
    fn GetFlags(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncFilterInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterInfo2Vtbl {
        unsafe extern "system" fn GetFlags<Impl: ISyncFilterInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base: ISyncFilterInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetFlags: GetFlags::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterInfo2 as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFullEnumerationChangeImpl: Sized {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&mut self) -> ::windows::core::Result<IForgottenKnowledge>;
}
impl ISyncFullEnumerationChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFullEnumerationChangeVtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Impl: ISyncFullEnumerationChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeAfterRecoveryComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Impl: ISyncFullEnumerationChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISyncFullEnumerationChangeBatchImpl: Sized + ISyncChangeBatchBaseImpl {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn GetClosedLowerBoundItemId(&mut self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetClosedUpperBoundItemId(&mut self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncFullEnumerationChangeBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFullEnumerationChangeBatchVtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeAfterRecoveryComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *pplearnedknowledgeafterrecoverycomplete = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosedLowerBoundItemId<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosedLowerBoundItemId(::core::mem::transmute_copy(&pbclosedlowerbounditemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedUpperBoundItemId<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClosedUpperBoundItemId(::core::mem::transmute_copy(&pbclosedupperbounditemid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base: ISyncChangeBatchBaseVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
            GetClosedLowerBoundItemId: GetClosedLowerBoundItemId::<Impl, IMPL_OFFSET>,
            GetClosedUpperBoundItemId: GetClosedUpperBoundItemId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncFullEnumerationChangeBatch2Impl: Sized + ISyncChangeBatchBaseImpl + ISyncFullEnumerationChangeBatchImpl {
    fn AddMergeTombstoneMetadataToGroup(&mut self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncFullEnumerationChangeBatch2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFullEnumerationChangeBatch2Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Impl: ISyncFullEnumerationChangeBatch2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ISyncFullEnumerationChangeBatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncKnowledgeImpl: Sized {
    fn GetOwnerReplicaId(&mut self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
    fn Serialize(&mut self, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::Result<()>;
    fn SetLocalTickCount(&mut self, ulltickcount: u64) -> ::windows::core::Result<()>;
    fn ContainsChange(&mut self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
    fn ContainsChangeUnit(&mut self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()>;
    fn GetScopeVector(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetReplicaKeyMap(&mut self) -> ::windows::core::Result<IReplicaKeyMap>;
    fn Clone(&mut self) -> ::windows::core::Result<ISyncKnowledge>;
    fn ConvertVersion(&mut self, pknowledgein: ::core::option::Option<ISyncKnowledge>, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::Result<()>;
    fn MapRemoteToLocal(&mut self, premoteknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn Union(&mut self, pknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn ProjectOntoItem(&mut self, pbitemid: *const u8) -> ::windows::core::Result<ISyncKnowledge>;
    fn ProjectOntoChangeUnit(&mut self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<ISyncKnowledge>;
    fn ProjectOntoRange(&mut self, psrngsyncrange: *const SYNC_RANGE) -> ::windows::core::Result<ISyncKnowledge>;
    fn ExcludeItem(&mut self, pbitemid: *const u8) -> ::windows::core::Result<()>;
    fn ExcludeChangeUnit(&mut self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()>;
    fn ContainsKnowledge(&mut self, pknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn FindMinTickCountForReplica(&mut self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::Result<()>;
    fn GetRangeExceptions(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetSingleItemExceptions(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetChangeUnitExceptions(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FindClockVectorForItem(&mut self, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FindClockVectorForChangeUnit(&mut self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetVersion(&mut self, pdwversion: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncKnowledgeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledgeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncKnowledgeVtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOwnerReplicaId(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&fserializereplicakeymap), ::core::mem::transmute_copy(&pbknowledge), ::core::mem::transmute_copy(&pcbknowledge)).into()
        }
        unsafe extern "system" fn SetLocalTickCount<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltickcount: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalTickCount(::core::mem::transmute_copy(&ulltickcount)).into()
        }
        unsafe extern "system" fn ContainsChange<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsChange(::core::mem::transmute_copy(&pbversionownerreplicaid), ::core::mem::transmute_copy(&pgiditemid), ::core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn ContainsChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsChangeUnit(::core::mem::transmute_copy(&pbversionownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn GetScopeVector<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScopeVector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetReplicaKeyMap<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreplicakeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReplicaKeyMap() {
                ::core::result::Result::Ok(ok__) => {
                    *ppreplicakeymap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclonedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppclonedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertVersion<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledgein: ::windows::core::RawPtr, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertVersion(::core::mem::transmute(&pknowledgein), ::core::mem::transmute_copy(&pbcurrentownerid), ::core::mem::transmute_copy(&pversionin), ::core::mem::transmute_copy(&pbnewownerid), ::core::mem::transmute_copy(&pcbidsize), ::core::mem::transmute_copy(&pversionout)).into()
        }
        unsafe extern "system" fn MapRemoteToLocal<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteknowledge: ::windows::core::RawPtr, ppmappedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapRemoteToLocal(::core::mem::transmute(&premoteknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmappedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Union<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Union(::core::mem::transmute(&pknowledge)).into()
        }
        unsafe extern "system" fn ProjectOntoItem<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoItem(::core::mem::transmute_copy(&pbitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoChangeUnit(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoRange<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoRange(::core::mem::transmute_copy(&psrngsyncrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcludeItem<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExcludeItem(::core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ExcludeChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExcludeChangeUnit(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ContainsKnowledge<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsKnowledge(::core::mem::transmute(&pknowledge)).into()
        }
        unsafe extern "system" fn FindMinTickCountForReplica<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindMinTickCountForReplica(::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pullreplicatickcount)).into()
        }
        unsafe extern "system" fn GetRangeExceptions<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRangeExceptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetSingleItemExceptions<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSingleItemExceptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetChangeUnitExceptions<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChangeUnitExceptions(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForItem<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindClockVectorForItem(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindClockVectorForChangeUnit(::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetVersion<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ISyncKnowledge2Impl: Sized + ISyncKnowledgeImpl {
    fn GetIdParameters(&mut self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
    fn ProjectOntoColumnSet(&mut self, ppcolumns: *const *const u8, count: u32) -> ::windows::core::Result<ISyncKnowledge2>;
    fn SerializeWithOptions(&mut self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetLowestUncontainedId(&mut self, pisyncknowledge: ::core::option::Option<ISyncKnowledge2>, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetInspector(&mut self, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetMinimumSupportedVersion(&mut self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::Result<()>;
    fn GetStatistics(&mut self, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::Result<()>;
    fn ContainsKnowledgeForItem(&mut self, pknowledge: ::core::option::Option<ISyncKnowledge>, pbitemid: *const u8) -> ::windows::core::Result<()>;
    fn ContainsKnowledgeForChangeUnit(&mut self, pknowledge: ::core::option::Option<ISyncKnowledge>, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()>;
    fn ProjectOntoKnowledgeWithPrerequisite(&mut self, pprerequisiteknowledge: ::core::option::Option<ISyncKnowledge>, ptemplateknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn Complement(&mut self, psyncknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<ISyncKnowledge>;
    fn IntersectsWithKnowledge(&mut self, psyncknowledge: ::core::option::Option<ISyncKnowledge>) -> ::windows::core::Result<()>;
    fn GetKnowledgeCookie(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CompareToKnowledgeCookie(&mut self, pknowledgecookie: ::core::option::Option<::windows::core::IUnknown>, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncKnowledge2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncKnowledge2Vtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn ProjectOntoColumnSet<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoColumnSet(::core::mem::transmute_copy(&ppcolumns), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiknowledgeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerializeWithOptions<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SerializeWithOptions(::core::mem::transmute_copy(&targetformatversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        unsafe extern "system" fn GetLowestUncontainedId<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncknowledge: ::windows::core::RawPtr, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLowestUncontainedId(::core::mem::transmute(&pisyncknowledge), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        unsafe extern "system" fn GetInspector<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInspector(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppiinspector)).into()
        }
        unsafe extern "system" fn GetMinimumSupportedVersion<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMinimumSupportedVersion(::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetStatistics<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&which), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForItem<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsKnowledgeForItem(::core::mem::transmute(&pknowledge), ::core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForChangeUnit<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainsKnowledgeForChangeUnit(::core::mem::transmute(&pknowledge), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ProjectOntoKnowledgeWithPrerequisite<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr, ptemplateknowledge: ::windows::core::RawPtr, ppprojectedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectOntoKnowledgeWithPrerequisite(::core::mem::transmute(&pprerequisiteknowledge), ::core::mem::transmute(&ptemplateknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprojectedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complement<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr, ppcomplementedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Complement(::core::mem::transmute(&psyncknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomplementedknowledge = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntersectsWithKnowledge<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IntersectsWithKnowledge(::core::mem::transmute(&psyncknowledge)).into()
        }
        unsafe extern "system" fn GetKnowledgeCookie<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKnowledgeCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *ppknowledgecookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareToKnowledgeCookie<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompareToKnowledgeCookie(::core::mem::transmute(&pknowledgecookie), ::core::mem::transmute_copy(&presult)).into()
        }
        Self {
            base: ISyncKnowledgeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ISyncKnowledge2 as ::windows::core::Interface>::IID
    }
}
pub trait ISyncMergeTombstoneChangeImpl: Sized {
    fn GetWinnerItemId(&mut self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()>;
}
impl ISyncMergeTombstoneChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncMergeTombstoneChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncMergeTombstoneChangeVtbl {
        unsafe extern "system" fn GetWinnerItemId<Impl: ISyncMergeTombstoneChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ISyncProviderImpl: Sized {
    fn GetIdParameters(&mut self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderVtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
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
pub trait ISyncProviderConfigUIImpl: Sized {
    fn Init(&mut self, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn GetRegisteredProperties(&mut self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateAndRegisterNewSyncProvider(&mut self, hwndparent: super::super::Foundation::HWND, punkcontext: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<ISyncProviderInfo>;
    fn ModifySyncProvider(&mut self, hwndparent: super::super::Foundation::HWND, punkcontext: ::core::option::Option<::windows::core::IUnknown>, pproviderinfo: ::core::option::Option<ISyncProviderInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderConfigUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderConfigUIVtbl {
        unsafe extern "system" fn Init<Impl: ISyncProviderConfigUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute(&pconfigurationproperties)).into()
        }
        unsafe extern "system" fn GetRegisteredProperties<Impl: ISyncProviderConfigUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconfiguiproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisteredProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfiguiproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndRegisterNewSyncProvider<Impl: ISyncProviderConfigUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAndRegisterNewSyncProvider(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&punkcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifySyncProvider<Impl: ISyncProviderConfigUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISyncProviderConfigUIInfoImpl: Sized + IPropertyStoreImpl {
    fn GetSyncProviderConfigUI(&mut self, dwclscontext: u32) -> ::windows::core::Result<ISyncProviderConfigUI>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderConfigUIInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUIInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderConfigUIInfoVtbl {
        unsafe extern "system" fn GetSyncProviderConfigUI<Impl: ISyncProviderConfigUIInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IPropertyStoreVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSyncProviderConfigUI: GetSyncProviderConfigUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderConfigUIInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderInfoImpl: Sized + IPropertyStoreImpl {
    fn GetSyncProvider(&mut self, dwclscontext: u32) -> ::windows::core::Result<IRegisteredSyncProvider>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderInfoVtbl {
        unsafe extern "system" fn GetSyncProvider<Impl: ISyncProviderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProvider(::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IPropertyStoreVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetSyncProvider: GetSyncProvider::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderRegistrationImpl: Sized {
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
impl ISyncProviderRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderRegistrationVtbl {
        unsafe extern "system" fn CreateSyncProviderConfigUIRegistrationInstance<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSyncProviderConfigUIRegistrationInstance(::core::mem::transmute_copy(&pconfiguiconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfiguiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProviderConfigUI<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterSyncProviderConfigUI(::core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn EnumerateSyncProviderConfigUIs<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSyncProviderConfigUIs(::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute_copy(&dwsupportedarchitecture)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsyncproviderconfiguiinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSyncProviderRegistrationInstance<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSyncProviderRegistrationInstance(::core::mem::transmute_copy(&pproviderconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProvider<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterSyncProvider(::core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfoforProvider<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidproviderinstanceid: *const ::windows::core::GUID, ppproviderconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUIInfoforProvider(::core::mem::transmute_copy(&pguidproviderinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderconfiguiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSyncProviders<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSyncProviders(::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute_copy(&dwstateflagstofiltermask), ::core::mem::transmute_copy(&dwstateflagstofilter), ::core::mem::transmute_copy(&refproviderclsid), ::core::mem::transmute_copy(&dwsupportedarchitecture)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsyncproviderinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderInfo<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderInfo(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproviderinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderFromInstanceId<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderFromInstanceId(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsyncprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfo<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUIInfo(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfiguiinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIFromInstanceId<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUIFromInstanceId(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfigui = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderState<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pdwstateflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderState(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstateflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncProviderState<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncProviderState(::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwstateflagsmask), ::core::mem::transmute_copy(&dwstateflags)).into()
        }
        unsafe extern "system" fn RegisterForEvent<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterForEvent(::core::mem::transmute_copy(&phevent)).into()
        }
        unsafe extern "system" fn RevokeEvent<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeEvent(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn GetChange<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISyncRegistrationChangeImpl: Sized {
    fn GetEvent(&mut self) -> ::windows::core::Result<SYNC_REGISTRATION_EVENT>;
    fn GetInstanceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ISyncRegistrationChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncRegistrationChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncRegistrationChangeVtbl {
        unsafe extern "system" fn GetEvent<Impl: ISyncRegistrationChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *psreevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Impl: ISyncRegistrationChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait ISyncSessionExtendedErrorInfoImpl: Sized {
    fn GetSyncProviderWithError(&mut self) -> ::windows::core::Result<ISyncProvider>;
}
impl ISyncSessionExtendedErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionExtendedErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncSessionExtendedErrorInfoVtbl {
        unsafe extern "system" fn GetSyncProviderWithError<Impl: ISyncSessionExtendedErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproviderwitherror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISyncSessionStateImpl: Sized {
    fn IsCanceled(&mut self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetInfoForChangeApplication(&mut self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::Result<()>;
    fn LoadInfoFromChangeApplication(&mut self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeStart(&mut self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeEnd(&mut self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::Result<()>;
    fn SetForgottenKnowledgeRecoveryRange(&mut self, prange: *const SYNC_RANGE) -> ::windows::core::Result<()>;
    fn OnProgress(&mut self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncSessionStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncSessionStateVtbl {
        unsafe extern "system" fn IsCanceled<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsCanceled(::core::mem::transmute_copy(&pfiscanceled)).into()
        }
        unsafe extern "system" fn GetInfoForChangeApplication<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInfoForChangeApplication(::core::mem::transmute_copy(&pbchangeapplierinfo), ::core::mem::transmute_copy(&pcbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn LoadInfoFromChangeApplication<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadInfoFromChangeApplication(::core::mem::transmute_copy(&pbchangeapplierinfo), ::core::mem::transmute_copy(&cbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeStart<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForgottenKnowledgeRecoveryRangeStart(::core::mem::transmute_copy(&pbrangestart), ::core::mem::transmute_copy(&pcbrangestart)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeEnd<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForgottenKnowledgeRecoveryRangeEnd(::core::mem::transmute_copy(&pbrangeend), ::core::mem::transmute_copy(&pcbrangeend)).into()
        }
        unsafe extern "system" fn SetForgottenKnowledgeRecoveryRange<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *const SYNC_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForgottenKnowledgeRecoveryRange(::core::mem::transmute_copy(&prange)).into()
        }
        unsafe extern "system" fn OnProgress<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
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
pub trait ISyncSessionState2Impl: Sized + ISyncSessionStateImpl {
    fn SetProviderWithError(&mut self, fself: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSessionErrorStatus(&mut self, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncSessionState2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncSessionState2Vtbl {
        unsafe extern "system" fn SetProviderWithError<Impl: ISyncSessionState2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fself: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderWithError(::core::mem::transmute_copy(&fself)).into()
        }
        unsafe extern "system" fn GetSessionErrorStatus<Impl: ISyncSessionState2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSessionErrorStatus(::core::mem::transmute_copy(&phrsessionerror)).into()
        }
        Self {
            base: ISyncSessionStateVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProviderWithError: SetProviderWithError::<Impl, IMPL_OFFSET>,
            GetSessionErrorStatus: GetSessionErrorStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionState2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronousDataRetrieverImpl: Sized {
    fn GetIdParameters(&mut self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()>;
    fn LoadChangeData(&mut self, ploadchangecontext: ::core::option::Option<ILoadChangeContext>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISynchronousDataRetrieverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronousDataRetrieverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronousDataRetrieverVtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdParameters(::core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn LoadChangeData<Impl: ISynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
