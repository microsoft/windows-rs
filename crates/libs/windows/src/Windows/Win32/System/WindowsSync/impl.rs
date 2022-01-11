#[cfg(feature = "Win32_Foundation")]
pub trait IAsynchronousDataRetrieverImpl: Sized {
    fn GetIdParameters();
    fn RegisterCallback();
    fn RevokeCallback();
    fn LoadChangeData();
}
#[cfg(feature = "Win32_Foundation")]
impl IAsynchronousDataRetrieverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsynchronousDataRetrieverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsynchronousDataRetrieverVtbl {
        unsafe extern "system" fn GetIdParameters<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterCallback<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevokeCallback<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadChangeData<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIdParameters::<Impl, IMPL_OFFSET>, RegisterCallback::<Impl, IMPL_OFFSET>, RevokeCallback::<Impl, IMPL_OFFSET>, LoadChangeData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsynchronousDataRetriever as ::windows::core::Interface>::IID
    }
}
pub trait IChangeConflictImpl: Sized {
    fn GetDestinationProviderConflictingChange();
    fn GetSourceProviderConflictingChange();
    fn GetDestinationProviderConflictingData();
    fn GetSourceProviderConflictingData();
    fn GetResolveActionForChange();
    fn SetResolveActionForChange();
    fn GetResolveActionForChangeUnit();
    fn SetResolveActionForChangeUnit();
}
impl IChangeConflictVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeConflictImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChangeConflictVtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResolveActionForChange<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResolveActionForChange<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResolveActionForChangeUnit<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResolveActionForChangeUnit<Impl: IChangeConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDestinationProviderConflictingChange::<Impl, IMPL_OFFSET>,
            GetSourceProviderConflictingChange::<Impl, IMPL_OFFSET>,
            GetDestinationProviderConflictingData::<Impl, IMPL_OFFSET>,
            GetSourceProviderConflictingData::<Impl, IMPL_OFFSET>,
            GetResolveActionForChange::<Impl, IMPL_OFFSET>,
            SetResolveActionForChange::<Impl, IMPL_OFFSET>,
            GetResolveActionForChangeUnit::<Impl, IMPL_OFFSET>,
            SetResolveActionForChangeUnit::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeConflict as ::windows::core::Interface>::IID
    }
}
pub trait IChangeUnitExceptionImpl: Sized {
    fn GetItemId();
    fn GetChangeUnitId();
    fn GetClockVector();
}
impl IChangeUnitExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitExceptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChangeUnitExceptionVtbl {
        unsafe extern "system" fn GetItemId<Impl: IChangeUnitExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: IChangeUnitExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClockVector<Impl: IChangeUnitExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemId::<Impl, IMPL_OFFSET>, GetChangeUnitId::<Impl, IMPL_OFFSET>, GetClockVector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeUnitException as ::windows::core::Interface>::IID
    }
}
pub trait IChangeUnitListFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn Initialize();
    fn GetChangeUnitIdCount();
    fn GetChangeUnitId();
}
impl IChangeUnitListFilterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChangeUnitListFilterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChangeUnitListFilterInfoVtbl {
        unsafe extern "system" fn Initialize<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeUnitIdCount<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Serialize::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, GetChangeUnitIdCount::<Impl, IMPL_OFFSET>, GetChangeUnitId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChangeUnitListFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IClockVectorImpl: Sized {
    fn GetClockVectorElements();
    fn GetClockVectorElementCount();
}
impl IClockVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClockVectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClockVectorVtbl {
        unsafe extern "system" fn GetClockVectorElements<Impl: IClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClockVectorElementCount<Impl: IClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClockVectorElements::<Impl, IMPL_OFFSET>, GetClockVectorElementCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IClockVectorElementImpl: Sized {
    fn GetReplicaKey();
    fn GetTickCount();
}
impl IClockVectorElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClockVectorElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClockVectorElementVtbl {
        unsafe extern "system" fn GetReplicaKey<Impl: IClockVectorElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTickCount<Impl: IClockVectorElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulltickcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetReplicaKey::<Impl, IMPL_OFFSET>, GetTickCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClockVectorElement as ::windows::core::Interface>::IID
    }
}
pub trait ICombinedFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn GetFilterCount();
    fn GetFilterInfo();
    fn GetFilterCombinationType();
}
impl ICombinedFilterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICombinedFilterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICombinedFilterInfoVtbl {
        unsafe extern "system" fn GetFilterCount<Impl: ICombinedFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilterInfo<Impl: ICombinedFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilterCombinationType<Impl: ICombinedFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Serialize::<Impl, IMPL_OFFSET>, GetFilterCount::<Impl, IMPL_OFFSET>, GetFilterInfo::<Impl, IMPL_OFFSET>, GetFilterCombinationType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICombinedFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IConstraintConflictImpl: Sized {
    fn GetDestinationProviderConflictingChange();
    fn GetSourceProviderConflictingChange();
    fn GetDestinationProviderOriginalChange();
    fn GetDestinationProviderConflictingData();
    fn GetSourceProviderConflictingData();
    fn GetDestinationProviderOriginalData();
    fn GetConstraintResolveActionForChange();
    fn SetConstraintResolveActionForChange();
    fn GetConstraintResolveActionForChangeUnit();
    fn SetConstraintResolveActionForChangeUnit();
    fn GetConstraintConflictReason();
    fn IsTemporary();
}
impl IConstraintConflictVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConstraintConflictImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConstraintConflictVtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDestinationProviderOriginalChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginalchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDestinationProviderOriginalData<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstraintResolveActionForChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChange<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstraintResolveActionForChangeUnit<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChangeUnit<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConstraintConflictReason<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTemporary<Impl: IConstraintConflictImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDestinationProviderConflictingChange::<Impl, IMPL_OFFSET>,
            GetSourceProviderConflictingChange::<Impl, IMPL_OFFSET>,
            GetDestinationProviderOriginalChange::<Impl, IMPL_OFFSET>,
            GetDestinationProviderConflictingData::<Impl, IMPL_OFFSET>,
            GetSourceProviderConflictingData::<Impl, IMPL_OFFSET>,
            GetDestinationProviderOriginalData::<Impl, IMPL_OFFSET>,
            GetConstraintResolveActionForChange::<Impl, IMPL_OFFSET>,
            SetConstraintResolveActionForChange::<Impl, IMPL_OFFSET>,
            GetConstraintResolveActionForChangeUnit::<Impl, IMPL_OFFSET>,
            SetConstraintResolveActionForChangeUnit::<Impl, IMPL_OFFSET>,
            GetConstraintConflictReason::<Impl, IMPL_OFFSET>,
            IsTemporary::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConstraintConflict as ::windows::core::Interface>::IID
    }
}
pub trait IConstructReplicaKeyMapImpl: Sized {
    fn FindOrAddReplica();
}
impl IConstructReplicaKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConstructReplicaKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConstructReplicaKeyMapVtbl {
        unsafe extern "system" fn FindOrAddReplica<Impl: IConstructReplicaKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FindOrAddReplica::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConstructReplicaKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait ICoreFragmentImpl: Sized {
    fn NextColumn();
    fn NextRange();
    fn Reset();
    fn GetColumnCount();
    fn GetRangeCount();
}
impl ICoreFragmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFragmentVtbl {
        unsafe extern "system" fn NextColumn<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextRange<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnCount<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolumncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRangeCount<Impl: ICoreFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, NextColumn::<Impl, IMPL_OFFSET>, NextRange::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, GetColumnCount::<Impl, IMPL_OFFSET>, GetRangeCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFragment as ::windows::core::Interface>::IID
    }
}
pub trait ICoreFragmentInspectorImpl: Sized {
    fn NextCoreFragments();
    fn Reset();
}
impl ICoreFragmentInspectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFragmentInspectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFragmentInspectorVtbl {
        unsafe extern "system" fn NextCoreFragments<Impl: ICoreFragmentInspectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut ::windows::core::RawPtr, pfetchedcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ICoreFragmentInspectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, NextCoreFragments::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFragmentInspector as ::windows::core::Interface>::IID
    }
}
pub trait ICustomFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn GetSyncFilter();
}
impl ICustomFilterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomFilterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomFilterInfoVtbl {
        unsafe extern "system" fn GetSyncFilter<Impl: ICustomFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Serialize::<Impl, IMPL_OFFSET>, GetSyncFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait IDataRetrieverCallbackImpl: Sized {
    fn LoadChangeDataComplete();
    fn LoadChangeDataError();
}
impl IDataRetrieverCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataRetrieverCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataRetrieverCallbackVtbl {
        unsafe extern "system" fn LoadChangeDataComplete<Impl: IDataRetrieverCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadChangeDataError<Impl: IDataRetrieverCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, LoadChangeDataComplete::<Impl, IMPL_OFFSET>, LoadChangeDataError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataRetrieverCallback as ::windows::core::Interface>::IID
    }
}
pub trait IEnumChangeUnitExceptionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumChangeUnitExceptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumChangeUnitExceptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumChangeUnitExceptionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumChangeUnitExceptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumClockVectorImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumClockVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumClockVectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumClockVectorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IEnumFeedClockVectorImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumFeedClockVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFeedClockVectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumFeedClockVectorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumFeedClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IEnumItemIdsImpl: Sized {
    fn Next();
}
impl IEnumItemIdsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumItemIdsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumItemIdsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumItemIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumItemIds as ::windows::core::Interface>::IID
    }
}
pub trait IEnumRangeExceptionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumRangeExceptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRangeExceptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumRangeExceptionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumRangeExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, pprangeexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumRangeExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumRangeExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumRangeExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumRangeExceptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSingleItemExceptionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumSingleItemExceptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSingleItemExceptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSingleItemExceptionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSingleItemExceptions as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSyncChangeUnitsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumSyncChangeUnitsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangeUnitsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncChangeUnitsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchangeunit: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncChangeUnits as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSyncChangesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumSyncChangesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncChangesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncChangesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncChangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchange: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncChangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncChangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncChangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncChanges as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSyncProviderConfigUIInfosImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumSyncProviderConfigUIInfosVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderConfigUIInfosImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncProviderConfigUIInfosVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfactories: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncProviderConfigUIInfos as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSyncProviderInfosImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumSyncProviderInfosVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSyncProviderInfosImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSyncProviderInfosVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncProviderInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncProviderInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cinstances: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncProviderInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncProviderInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSyncProviderInfos as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFeedClockVectorImpl: Sized + IClockVectorImpl {
    fn GetUpdateCount();
    fn IsNoConflictsSpecified();
}
#[cfg(feature = "Win32_Foundation")]
impl IFeedClockVectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedClockVectorVtbl {
        unsafe extern "system" fn GetUpdateCount<Impl: IFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwupdatecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsNoConflictsSpecified<Impl: IFeedClockVectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClockVectorElements::<Impl, IMPL_OFFSET>, GetClockVectorElementCount::<Impl, IMPL_OFFSET>, GetUpdateCount::<Impl, IMPL_OFFSET>, IsNoConflictsSpecified::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedClockVector as ::windows::core::Interface>::IID
    }
}
pub trait IFeedClockVectorElementImpl: Sized + IClockVectorElementImpl {
    fn GetSyncTime();
    fn GetFlags();
}
impl IFeedClockVectorElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFeedClockVectorElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFeedClockVectorElementVtbl {
        unsafe extern "system" fn GetSyncTime<Impl: IFeedClockVectorElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psynctime: *mut SYNC_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IFeedClockVectorElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbflags: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetReplicaKey::<Impl, IMPL_OFFSET>, GetTickCount::<Impl, IMPL_OFFSET>, GetSyncTime::<Impl, IMPL_OFFSET>, GetFlags::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFeedClockVectorElement as ::windows::core::Interface>::IID
    }
}
pub trait IFilterKeyMapImpl: Sized {
    fn GetCount();
    fn AddFilter();
    fn GetFilter();
    fn Serialize();
}
impl IFilterKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterKeyMapVtbl {
        unsafe extern "system" fn GetCount<Impl: IFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFilter<Impl: IFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncfilter: ::windows::core::RawPtr, pdwfilterkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilter<Impl: IFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Serialize<Impl: IFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, AddFilter::<Impl, IMPL_OFFSET>, GetFilter::<Impl, IMPL_OFFSET>, Serialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait IFilterRequestCallbackImpl: Sized {
    fn RequestFilter();
}
impl IFilterRequestCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterRequestCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterRequestCallbackVtbl {
        unsafe extern "system" fn RequestFilter<Impl: IFilterRequestCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RequestFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IFilterTrackingProviderImpl: Sized {
    fn SpecifyTrackedFilters();
    fn AddTrackedFilter();
}
impl IFilterTrackingProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterTrackingProviderVtbl {
        unsafe extern "system" fn SpecifyTrackedFilters<Impl: IFilterTrackingProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTrackedFilter<Impl: IFilterTrackingProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SpecifyTrackedFilters::<Impl, IMPL_OFFSET>, AddTrackedFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterTrackingProvider as ::windows::core::Interface>::IID
    }
}
pub trait IFilterTrackingRequestCallbackImpl: Sized {
    fn RequestTrackedFilter();
}
impl IFilterTrackingRequestCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingRequestCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterTrackingRequestCallbackVtbl {
        unsafe extern "system" fn RequestTrackedFilter<Impl: IFilterTrackingRequestCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RequestTrackedFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterTrackingRequestCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFilterTrackingSyncChangeBuilderImpl: Sized {
    fn AddFilterChange();
    fn SetAllChangeUnitsPresentFlag();
}
#[cfg(feature = "Win32_Foundation")]
impl IFilterTrackingSyncChangeBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterTrackingSyncChangeBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterTrackingSyncChangeBuilderVtbl {
        unsafe extern "system" fn AddFilterChange<Impl: IFilterTrackingSyncChangeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllChangeUnitsPresentFlag<Impl: IFilterTrackingSyncChangeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddFilterChange::<Impl, IMPL_OFFSET>, SetAllChangeUnitsPresentFlag::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilterTrackingSyncChangeBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IForgottenKnowledgeImpl: Sized + ISyncKnowledgeImpl {
    fn ForgetToVersion();
}
#[cfg(feature = "Win32_Foundation")]
impl IForgottenKnowledgeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForgottenKnowledgeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IForgottenKnowledgeVtbl {
        unsafe extern "system" fn ForgetToVersion<Impl: IForgottenKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwnerReplicaId::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            SetLocalTickCount::<Impl, IMPL_OFFSET>,
            ContainsChange::<Impl, IMPL_OFFSET>,
            ContainsChangeUnit::<Impl, IMPL_OFFSET>,
            GetScopeVector::<Impl, IMPL_OFFSET>,
            GetReplicaKeyMap::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            ConvertVersion::<Impl, IMPL_OFFSET>,
            MapRemoteToLocal::<Impl, IMPL_OFFSET>,
            Union::<Impl, IMPL_OFFSET>,
            ProjectOntoItem::<Impl, IMPL_OFFSET>,
            ProjectOntoChangeUnit::<Impl, IMPL_OFFSET>,
            ProjectOntoRange::<Impl, IMPL_OFFSET>,
            ExcludeItem::<Impl, IMPL_OFFSET>,
            ExcludeChangeUnit::<Impl, IMPL_OFFSET>,
            ContainsKnowledge::<Impl, IMPL_OFFSET>,
            FindMinTickCountForReplica::<Impl, IMPL_OFFSET>,
            GetRangeExceptions::<Impl, IMPL_OFFSET>,
            GetSingleItemExceptions::<Impl, IMPL_OFFSET>,
            GetChangeUnitExceptions::<Impl, IMPL_OFFSET>,
            FindClockVectorForItem::<Impl, IMPL_OFFSET>,
            FindClockVectorForChangeUnit::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            ForgetToVersion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForgottenKnowledge as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKnowledgeSyncProviderImpl: Sized + ISyncProviderImpl {
    fn BeginSession();
    fn GetSyncBatchParameters();
    fn GetChangeBatch();
    fn GetFullEnumerationChangeBatch();
    fn ProcessChangeBatch();
    fn ProcessFullEnumerationChangeBatch();
    fn EndSession();
}
#[cfg(feature = "Win32_Foundation")]
impl IKnowledgeSyncProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnowledgeSyncProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnowledgeSyncProviderVtbl {
        unsafe extern "system" fn BeginSession<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSyncBatchParameters<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncknowledge: *mut ::windows::core::RawPtr, pdwrequestedbatchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFullEnumerationChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessFullEnumerationChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndSession<Impl: IKnowledgeSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetIdParameters::<Impl, IMPL_OFFSET>,
            BeginSession::<Impl, IMPL_OFFSET>,
            GetSyncBatchParameters::<Impl, IMPL_OFFSET>,
            GetChangeBatch::<Impl, IMPL_OFFSET>,
            GetFullEnumerationChangeBatch::<Impl, IMPL_OFFSET>,
            ProcessChangeBatch::<Impl, IMPL_OFFSET>,
            ProcessFullEnumerationChangeBatch::<Impl, IMPL_OFFSET>,
            EndSession::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnowledgeSyncProvider as ::windows::core::Interface>::IID
    }
}
pub trait ILoadChangeContextImpl: Sized {
    fn GetSyncChange();
    fn SetRecoverableErrorOnChange();
    fn SetRecoverableErrorOnChangeUnit();
}
impl ILoadChangeContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoadChangeContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoadChangeContextVtbl {
        unsafe extern "system" fn GetSyncChange<Impl: ILoadChangeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRecoverableErrorOnChange<Impl: ILoadChangeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRecoverableErrorOnChangeUnit<Impl: ILoadChangeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, pchangeunit: ::windows::core::RawPtr, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSyncChange::<Impl, IMPL_OFFSET>, SetRecoverableErrorOnChange::<Impl, IMPL_OFFSET>, SetRecoverableErrorOnChangeUnit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoadChangeContext as ::windows::core::Interface>::IID
    }
}
pub trait IProviderConverterImpl: Sized {
    fn Initialize();
}
impl IProviderConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderConverterVtbl {
        unsafe extern "system" fn Initialize<Impl: IProviderConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderConverter as ::windows::core::Interface>::IID
    }
}
pub trait IRangeExceptionImpl: Sized {
    fn GetClosedRangeStart();
    fn GetClosedRangeEnd();
    fn GetClockVector();
}
impl IRangeExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeExceptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeExceptionVtbl {
        unsafe extern "system" fn GetClosedRangeStart<Impl: IRangeExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClosedRangeEnd<Impl: IRangeExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClockVector<Impl: IRangeExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClosedRangeStart::<Impl, IMPL_OFFSET>, GetClosedRangeEnd::<Impl, IMPL_OFFSET>, GetClockVector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeException as ::windows::core::Interface>::IID
    }
}
pub trait IRecoverableErrorImpl: Sized {
    fn GetStage();
    fn GetProvider();
    fn GetChangeWithRecoverableError();
    fn GetRecoverableErrorDataForChange();
    fn GetRecoverableErrorDataForChangeUnit();
}
impl IRecoverableErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRecoverableErrorVtbl {
        unsafe extern "system" fn GetStage<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProvider<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeWithRecoverableError<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppchangewithrecoverableerror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChange<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChangeUnit<Impl: IRecoverableErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetStage::<Impl, IMPL_OFFSET>, GetProvider::<Impl, IMPL_OFFSET>, GetChangeWithRecoverableError::<Impl, IMPL_OFFSET>, GetRecoverableErrorDataForChange::<Impl, IMPL_OFFSET>, GetRecoverableErrorDataForChangeUnit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRecoverableError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRecoverableErrorDataImpl: Sized {
    fn Initialize();
    fn GetItemDisplayName();
    fn GetErrorDescription();
}
#[cfg(feature = "Win32_Foundation")]
impl IRecoverableErrorDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRecoverableErrorDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRecoverableErrorDataVtbl {
        unsafe extern "system" fn Initialize<Impl: IRecoverableErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcszitemdisplayname: super::super::Foundation::PWSTR, pcszerrordescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemDisplayName<Impl: IRecoverableErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszitemdisplayname: super::super::Foundation::PWSTR, pcchitemdisplayname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IRecoverableErrorDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszerrordescription: super::super::Foundation::PWSTR, pccherrordescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, GetItemDisplayName::<Impl, IMPL_OFFSET>, GetErrorDescription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRecoverableErrorData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IRegisteredSyncProviderImpl: Sized {
    fn Init();
    fn GetInstanceId();
    fn Reset();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IRegisteredSyncProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredSyncProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegisteredSyncProviderVtbl {
        unsafe extern "system" fn Init<Impl: IRegisteredSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInstanceId<Impl: IRegisteredSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IRegisteredSyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Init::<Impl, IMPL_OFFSET>, GetInstanceId::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredSyncProvider as ::windows::core::Interface>::IID
    }
}
pub trait IReplicaKeyMapImpl: Sized {
    fn LookupReplicaKey();
    fn LookupReplicaId();
    fn Serialize();
}
impl IReplicaKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReplicaKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReplicaKeyMapVtbl {
        unsafe extern "system" fn LookupReplicaKey<Impl: IReplicaKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LookupReplicaId<Impl: IReplicaKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Serialize<Impl: IReplicaKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, LookupReplicaKey::<Impl, IMPL_OFFSET>, LookupReplicaId::<Impl, IMPL_OFFSET>, Serialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReplicaKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait IRequestFilteredSyncImpl: Sized {
    fn SpecifyFilter();
}
impl IRequestFilteredSyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRequestFilteredSyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRequestFilteredSyncVtbl {
        unsafe extern "system" fn SpecifyFilter<Impl: IRequestFilteredSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SpecifyFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRequestFilteredSync as ::windows::core::Interface>::IID
    }
}
pub trait ISingleItemExceptionImpl: Sized {
    fn GetItemId();
    fn GetClockVector();
}
impl ISingleItemExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISingleItemExceptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISingleItemExceptionVtbl {
        unsafe extern "system" fn GetItemId<Impl: ISingleItemExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClockVector<Impl: ISingleItemExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemId::<Impl, IMPL_OFFSET>, GetClockVector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISingleItemException as ::windows::core::Interface>::IID
    }
}
pub trait ISupportFilteredSyncImpl: Sized {
    fn AddFilter();
}
impl ISupportFilteredSyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportFilteredSyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISupportFilteredSyncVtbl {
        unsafe extern "system" fn AddFilter<Impl: ISupportFilteredSyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportFilteredSync as ::windows::core::Interface>::IID
    }
}
pub trait ISupportLastWriteTimeImpl: Sized {
    fn GetItemChangeTime();
    fn GetChangeUnitChangeTime();
}
impl ISupportLastWriteTimeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportLastWriteTimeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISupportLastWriteTimeVtbl {
        unsafe extern "system" fn GetItemChangeTime<Impl: ISupportLastWriteTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeUnitChangeTime<Impl: ISupportLastWriteTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemChangeTime::<Impl, IMPL_OFFSET>, GetChangeUnitChangeTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportLastWriteTime as ::windows::core::Interface>::IID
    }
}
pub trait ISyncCallbackImpl: Sized {
    fn OnProgress();
    fn OnChange();
    fn OnConflict();
    fn OnFullEnumerationNeeded();
    fn OnRecoverableError();
}
impl ISyncCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncCallbackVtbl {
        unsafe extern "system" fn OnProgress<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnChange<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnConflict<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnFullEnumerationNeeded<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnRecoverableError<Impl: ISyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precoverableerror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnProgress::<Impl, IMPL_OFFSET>, OnChange::<Impl, IMPL_OFFSET>, OnConflict::<Impl, IMPL_OFFSET>, OnFullEnumerationNeeded::<Impl, IMPL_OFFSET>, OnRecoverableError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISyncCallback2Impl: Sized + ISyncCallbackImpl {
    fn OnChangeApplied();
    fn OnChangeFailed();
}
impl ISyncCallback2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncCallback2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncCallback2Vtbl {
        unsafe extern "system" fn OnChangeApplied<Impl: ISyncCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnChangeFailed<Impl: ISyncCallback2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnProgress::<Impl, IMPL_OFFSET>, OnChange::<Impl, IMPL_OFFSET>, OnConflict::<Impl, IMPL_OFFSET>, OnFullEnumerationNeeded::<Impl, IMPL_OFFSET>, OnRecoverableError::<Impl, IMPL_OFFSET>, OnChangeApplied::<Impl, IMPL_OFFSET>, OnChangeFailed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncCallback2 as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeImpl: Sized {
    fn GetOwnerReplicaId();
    fn GetRootItemId();
    fn GetChangeVersion();
    fn GetCreationVersion();
    fn GetFlags();
    fn GetWorkEstimate();
    fn GetChangeUnits();
    fn GetMadeWithKnowledge();
    fn GetLearnedKnowledge();
    fn SetWorkEstimate();
}
impl ISyncChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeVtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRootItemId<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeVersion<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreationVersion<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWorkEstimate<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwork: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeUnits<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMadeWithKnowledge<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmadewithknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedKnowledge<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWorkEstimate<Impl: ISyncChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwnerReplicaId::<Impl, IMPL_OFFSET>,
            GetRootItemId::<Impl, IMPL_OFFSET>,
            GetChangeVersion::<Impl, IMPL_OFFSET>,
            GetCreationVersion::<Impl, IMPL_OFFSET>,
            GetFlags::<Impl, IMPL_OFFSET>,
            GetWorkEstimate::<Impl, IMPL_OFFSET>,
            GetChangeUnits::<Impl, IMPL_OFFSET>,
            GetMadeWithKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            SetWorkEstimate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchImpl: Sized + ISyncChangeBatchBaseImpl {
    fn BeginUnorderedGroup();
    fn EndUnorderedGroup();
    fn AddLoggedConflict();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchVtbl {
        unsafe extern "system" fn BeginUnorderedGroup<Impl: ISyncChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndUnorderedGroup<Impl: ISyncChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmadewithknowledge: ::windows::core::RawPtr, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddLoggedConflict<Impl: ISyncChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetChangeEnumerator::<Impl, IMPL_OFFSET>,
            GetIsLastBatch::<Impl, IMPL_OFFSET>,
            GetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            GetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            BeginOrderedGroup::<Impl, IMPL_OFFSET>,
            EndOrderedGroup::<Impl, IMPL_OFFSET>,
            AddItemMetadataToGroup::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetSourceForgottenKnowledge::<Impl, IMPL_OFFSET>,
            SetLastBatch::<Impl, IMPL_OFFSET>,
            SetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            SetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            BeginUnorderedGroup::<Impl, IMPL_OFFSET>,
            EndUnorderedGroup::<Impl, IMPL_OFFSET>,
            AddLoggedConflict::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatch2Impl: Sized + ISyncChangeBatchImpl + ISyncChangeBatchBaseImpl {
    fn AddMergeTombstoneMetadataToGroup();
    fn AddMergeTombstoneLoggedConflict();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatch2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatch2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatch2Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Impl: ISyncChangeBatch2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMergeTombstoneLoggedConflict<Impl: ISyncChangeBatch2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetChangeEnumerator::<Impl, IMPL_OFFSET>,
            GetIsLastBatch::<Impl, IMPL_OFFSET>,
            GetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            GetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            BeginOrderedGroup::<Impl, IMPL_OFFSET>,
            EndOrderedGroup::<Impl, IMPL_OFFSET>,
            AddItemMetadataToGroup::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetSourceForgottenKnowledge::<Impl, IMPL_OFFSET>,
            SetLastBatch::<Impl, IMPL_OFFSET>,
            SetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            SetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            BeginUnorderedGroup::<Impl, IMPL_OFFSET>,
            EndUnorderedGroup::<Impl, IMPL_OFFSET>,
            AddLoggedConflict::<Impl, IMPL_OFFSET>,
            AddMergeTombstoneMetadataToGroup::<Impl, IMPL_OFFSET>,
            AddMergeTombstoneLoggedConflict::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatch2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchAdvancedImpl: Sized {
    fn GetFilterInfo();
    fn ConvertFullEnumerationChangeBatchToRegularChangeBatch();
    fn GetUpperBoundItemId();
    fn GetBatchLevelKnowledgeShouldBeApplied();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchAdvancedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchAdvancedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchAdvancedVtbl {
        unsafe extern "system" fn GetFilterInfo<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertFullEnumerationChangeBatchToRegularChangeBatch<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppchangebatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUpperBoundItemId<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBatchLevelKnowledgeShouldBeApplied<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFilterInfo::<Impl, IMPL_OFFSET>, ConvertFullEnumerationChangeBatchToRegularChangeBatch::<Impl, IMPL_OFFSET>, GetUpperBoundItemId::<Impl, IMPL_OFFSET>, GetBatchLevelKnowledgeShouldBeApplied::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchBaseImpl: Sized {
    fn GetChangeEnumerator();
    fn GetIsLastBatch();
    fn GetWorkEstimateForBatch();
    fn GetRemainingWorkEstimateForSession();
    fn BeginOrderedGroup();
    fn EndOrderedGroup();
    fn AddItemMetadataToGroup();
    fn GetLearnedKnowledge();
    fn GetPrerequisiteKnowledge();
    fn GetSourceForgottenKnowledge();
    fn SetLastBatch();
    fn SetWorkEstimateForBatch();
    fn SetRemainingWorkEstimateForSession();
    fn Serialize();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchBaseVtbl {
        unsafe extern "system" fn GetChangeEnumerator<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsLastBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWorkEstimateForBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwworkforbatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRemainingWorkEstimateForSession<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginOrderedGroup<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblowerbound: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndOrderedGroup<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddItemMetadataToGroup<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedKnowledge<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrerequisiteKnowledge<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprerequisteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceForgottenKnowledge<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsourceforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWorkEstimateForBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwworkforbatch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemainingWorkEstimateForSession<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwremainingworkforsession: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Serialize<Impl: ISyncChangeBatchBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetChangeEnumerator::<Impl, IMPL_OFFSET>,
            GetIsLastBatch::<Impl, IMPL_OFFSET>,
            GetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            GetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            BeginOrderedGroup::<Impl, IMPL_OFFSET>,
            EndOrderedGroup::<Impl, IMPL_OFFSET>,
            AddItemMetadataToGroup::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetSourceForgottenKnowledge::<Impl, IMPL_OFFSET>,
            SetLastBatch::<Impl, IMPL_OFFSET>,
            SetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            SetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchBase2Impl: Sized + ISyncChangeBatchBaseImpl {
    fn SerializeWithOptions();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchBase2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchBase2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchBase2Vtbl {
        unsafe extern "system" fn SerializeWithOptions<Impl: ISyncChangeBatchBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetChangeEnumerator::<Impl, IMPL_OFFSET>,
            GetIsLastBatch::<Impl, IMPL_OFFSET>,
            GetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            GetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            BeginOrderedGroup::<Impl, IMPL_OFFSET>,
            EndOrderedGroup::<Impl, IMPL_OFFSET>,
            AddItemMetadataToGroup::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetSourceForgottenKnowledge::<Impl, IMPL_OFFSET>,
            SetLastBatch::<Impl, IMPL_OFFSET>,
            SetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            SetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            SerializeWithOptions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase2 as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeBatchWithFilterKeyMapImpl: Sized {
    fn GetFilterKeyMap();
    fn SetFilterKeyMap();
    fn SetFilterForgottenKnowledge();
    fn GetFilteredReplicaLearnedKnowledge();
    fn GetLearnedFilterForgottenKnowledge();
    fn GetFilteredReplicaLearnedForgottenKnowledge();
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete();
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete();
}
impl ISyncChangeBatchWithFilterKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithFilterKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchWithFilterKeyMapVtbl {
        unsafe extern "system" fn GetFilterKeyMap<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifilterkeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFilterKeyMap<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifilterkeymap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFilterForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFilterKeyMap::<Impl, IMPL_OFFSET>,
            SetFilterKeyMap::<Impl, IMPL_OFFSET>,
            SetFilterForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedFilterForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithFilterKeyMap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchWithPrerequisiteImpl: Sized + ISyncChangeBatchBaseImpl {
    fn SetPrerequisiteKnowledge();
    fn GetLearnedKnowledgeWithPrerequisite();
    fn GetLearnedForgottenKnowledge();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeBatchWithPrerequisiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBatchWithPrerequisiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBatchWithPrerequisiteVtbl {
        unsafe extern "system" fn SetPrerequisiteKnowledge<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedwithprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetChangeEnumerator::<Impl, IMPL_OFFSET>,
            GetIsLastBatch::<Impl, IMPL_OFFSET>,
            GetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            GetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            BeginOrderedGroup::<Impl, IMPL_OFFSET>,
            EndOrderedGroup::<Impl, IMPL_OFFSET>,
            AddItemMetadataToGroup::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetSourceForgottenKnowledge::<Impl, IMPL_OFFSET>,
            SetLastBatch::<Impl, IMPL_OFFSET>,
            SetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            SetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            SetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledgeWithPrerequisite::<Impl, IMPL_OFFSET>,
            GetLearnedForgottenKnowledge::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithPrerequisite as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeBuilderImpl: Sized {
    fn AddChangeUnitMetadata();
}
impl ISyncChangeBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeBuilderVtbl {
        unsafe extern "system" fn AddChangeUnitMetadata<Impl: ISyncChangeBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddChangeUnitMetadata::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeBuilder as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeUnitImpl: Sized {
    fn GetItemChange();
    fn GetChangeUnitId();
    fn GetChangeUnitVersion();
}
impl ISyncChangeUnitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeUnitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeUnitVtbl {
        unsafe extern "system" fn GetItemChange<Impl: ISyncChangeUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: ISyncChangeUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeUnitVersion<Impl: ISyncChangeUnitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemChange::<Impl, IMPL_OFFSET>, GetChangeUnitId::<Impl, IMPL_OFFSET>, GetChangeUnitVersion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeUnit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeWithFilterKeyMapImpl: Sized {
    fn GetFilterCount();
    fn GetFilterChange();
    fn GetAllChangeUnitsPresentFlag();
    fn GetFilterForgottenKnowledge();
    fn GetFilteredReplicaLearnedKnowledge();
    fn GetLearnedFilterForgottenKnowledge();
    fn GetFilteredReplicaLearnedForgottenKnowledge();
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete();
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncChangeWithFilterKeyMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithFilterKeyMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeWithFilterKeyMapVtbl {
        unsafe extern "system" fn GetFilterCount<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilterChange<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllChangeUnitsPresentFlag<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilterForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetFilterCount::<Impl, IMPL_OFFSET>,
            GetFilterChange::<Impl, IMPL_OFFSET>,
            GetAllChangeUnitsPresentFlag::<Impl, IMPL_OFFSET>,
            GetFilterForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetLearnedFilterForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge::<Impl, IMPL_OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeWithFilterKeyMap as ::windows::core::Interface>::IID
    }
}
pub trait ISyncChangeWithPrerequisiteImpl: Sized {
    fn GetPrerequisiteKnowledge();
    fn GetLearnedKnowledgeWithPrerequisite();
}
impl ISyncChangeWithPrerequisiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncChangeWithPrerequisiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncChangeWithPrerequisiteVtbl {
        unsafe extern "system" fn GetPrerequisiteKnowledge<Impl: ISyncChangeWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Impl: ISyncChangeWithPrerequisiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedknowledgewithprerequisite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>, GetLearnedKnowledgeWithPrerequisite::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncChangeWithPrerequisite as ::windows::core::Interface>::IID
    }
}
pub trait ISyncConstraintCallbackImpl: Sized {
    fn OnConstraintConflict();
}
impl ISyncConstraintCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncConstraintCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncConstraintCallbackVtbl {
        unsafe extern "system" fn OnConstraintConflict<Impl: ISyncConstraintCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnConstraintConflict::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncConstraintCallback as ::windows::core::Interface>::IID
    }
}
pub trait ISyncDataConverterImpl: Sized {
    fn ConvertDataRetrieverFromProviderFormat();
    fn ConvertDataRetrieverToProviderFormat();
    fn ConvertDataFromProviderFormat();
    fn ConvertDataToProviderFormat();
}
impl ISyncDataConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncDataConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncDataConverterVtbl {
        unsafe extern "system" fn ConvertDataRetrieverFromProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertDataRetrieverToProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertDataFromProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertDataToProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ConvertDataRetrieverFromProviderFormat::<Impl, IMPL_OFFSET>, ConvertDataRetrieverToProviderFormat::<Impl, IMPL_OFFSET>, ConvertDataFromProviderFormat::<Impl, IMPL_OFFSET>, ConvertDataToProviderFormat::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncDataConverter as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterImpl: Sized {
    fn IsIdentical();
    fn Serialize();
}
impl ISyncFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterVtbl {
        unsafe extern "system" fn IsIdentical<Impl: ISyncFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Serialize<Impl: ISyncFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsIdentical::<Impl, IMPL_OFFSET>, Serialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilter as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterDeserializerImpl: Sized {
    fn DeserializeSyncFilter();
}
impl ISyncFilterDeserializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterDeserializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterDeserializerVtbl {
        unsafe extern "system" fn DeserializeSyncFilter<Impl: ISyncFilterDeserializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DeserializeSyncFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterDeserializer as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterInfoImpl: Sized {
    fn Serialize();
}
impl ISyncFilterInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterInfoVtbl {
        unsafe extern "system" fn Serialize<Impl: ISyncFilterInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Serialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterInfo as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFilterInfo2Impl: Sized + ISyncFilterInfoImpl {
    fn GetFlags();
}
impl ISyncFilterInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFilterInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFilterInfo2Vtbl {
        unsafe extern "system" fn GetFlags<Impl: ISyncFilterInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Serialize::<Impl, IMPL_OFFSET>, GetFlags::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFilterInfo2 as ::windows::core::Interface>::IID
    }
}
pub trait ISyncFullEnumerationChangeImpl: Sized {
    fn GetLearnedKnowledgeAfterRecoveryComplete();
    fn GetLearnedForgottenKnowledge();
}
impl ISyncFullEnumerationChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFullEnumerationChangeVtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Impl: ISyncFullEnumerationChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Impl: ISyncFullEnumerationChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetLearnedKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>, GetLearnedForgottenKnowledge::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncFullEnumerationChangeBatchImpl: Sized + ISyncChangeBatchBaseImpl {
    fn GetLearnedKnowledgeAfterRecoveryComplete();
    fn GetClosedLowerBoundItemId();
    fn GetClosedUpperBoundItemId();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncFullEnumerationChangeBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFullEnumerationChangeBatchVtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClosedLowerBoundItemId<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClosedUpperBoundItemId<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetChangeEnumerator::<Impl, IMPL_OFFSET>,
            GetIsLastBatch::<Impl, IMPL_OFFSET>,
            GetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            GetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            BeginOrderedGroup::<Impl, IMPL_OFFSET>,
            EndOrderedGroup::<Impl, IMPL_OFFSET>,
            AddItemMetadataToGroup::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetSourceForgottenKnowledge::<Impl, IMPL_OFFSET>,
            SetLastBatch::<Impl, IMPL_OFFSET>,
            SetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            SetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
            GetClosedLowerBoundItemId::<Impl, IMPL_OFFSET>,
            GetClosedUpperBoundItemId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncFullEnumerationChangeBatch2Impl: Sized + ISyncFullEnumerationChangeBatchImpl + ISyncChangeBatchBaseImpl {
    fn AddMergeTombstoneMetadataToGroup();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncFullEnumerationChangeBatch2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncFullEnumerationChangeBatch2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncFullEnumerationChangeBatch2Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Impl: ISyncFullEnumerationChangeBatch2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetChangeEnumerator::<Impl, IMPL_OFFSET>,
            GetIsLastBatch::<Impl, IMPL_OFFSET>,
            GetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            GetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            BeginOrderedGroup::<Impl, IMPL_OFFSET>,
            EndOrderedGroup::<Impl, IMPL_OFFSET>,
            AddItemMetadataToGroup::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledge::<Impl, IMPL_OFFSET>,
            GetPrerequisiteKnowledge::<Impl, IMPL_OFFSET>,
            GetSourceForgottenKnowledge::<Impl, IMPL_OFFSET>,
            SetLastBatch::<Impl, IMPL_OFFSET>,
            SetWorkEstimateForBatch::<Impl, IMPL_OFFSET>,
            SetRemainingWorkEstimateForSession::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            GetLearnedKnowledgeAfterRecoveryComplete::<Impl, IMPL_OFFSET>,
            GetClosedLowerBoundItemId::<Impl, IMPL_OFFSET>,
            GetClosedUpperBoundItemId::<Impl, IMPL_OFFSET>,
            AddMergeTombstoneMetadataToGroup::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncKnowledgeImpl: Sized {
    fn GetOwnerReplicaId();
    fn Serialize();
    fn SetLocalTickCount();
    fn ContainsChange();
    fn ContainsChangeUnit();
    fn GetScopeVector();
    fn GetReplicaKeyMap();
    fn Clone();
    fn ConvertVersion();
    fn MapRemoteToLocal();
    fn Union();
    fn ProjectOntoItem();
    fn ProjectOntoChangeUnit();
    fn ProjectOntoRange();
    fn ExcludeItem();
    fn ExcludeChangeUnit();
    fn ContainsKnowledge();
    fn FindMinTickCountForReplica();
    fn GetRangeExceptions();
    fn GetSingleItemExceptions();
    fn GetChangeUnitExceptions();
    fn FindClockVectorForItem();
    fn FindClockVectorForChangeUnit();
    fn GetVersion();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncKnowledgeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledgeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncKnowledgeVtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Serialize<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalTickCount<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltickcount: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainsChange<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainsChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScopeVector<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReplicaKeyMap<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreplicakeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppclonedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertVersion<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledgein: ::windows::core::RawPtr, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MapRemoteToLocal<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, premoteknowledge: ::windows::core::RawPtr, ppmappedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Union<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProjectOntoItem<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProjectOntoChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProjectOntoRange<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExcludeItem<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExcludeChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainsKnowledge<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindMinTickCountForReplica<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRangeExceptions<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSingleItemExceptions<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChangeUnitExceptions<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindClockVectorForItem<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindClockVectorForChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVersion<Impl: ISyncKnowledgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwnerReplicaId::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            SetLocalTickCount::<Impl, IMPL_OFFSET>,
            ContainsChange::<Impl, IMPL_OFFSET>,
            ContainsChangeUnit::<Impl, IMPL_OFFSET>,
            GetScopeVector::<Impl, IMPL_OFFSET>,
            GetReplicaKeyMap::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            ConvertVersion::<Impl, IMPL_OFFSET>,
            MapRemoteToLocal::<Impl, IMPL_OFFSET>,
            Union::<Impl, IMPL_OFFSET>,
            ProjectOntoItem::<Impl, IMPL_OFFSET>,
            ProjectOntoChangeUnit::<Impl, IMPL_OFFSET>,
            ProjectOntoRange::<Impl, IMPL_OFFSET>,
            ExcludeItem::<Impl, IMPL_OFFSET>,
            ExcludeChangeUnit::<Impl, IMPL_OFFSET>,
            ContainsKnowledge::<Impl, IMPL_OFFSET>,
            FindMinTickCountForReplica::<Impl, IMPL_OFFSET>,
            GetRangeExceptions::<Impl, IMPL_OFFSET>,
            GetSingleItemExceptions::<Impl, IMPL_OFFSET>,
            GetChangeUnitExceptions::<Impl, IMPL_OFFSET>,
            FindClockVectorForItem::<Impl, IMPL_OFFSET>,
            FindClockVectorForChangeUnit::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncKnowledge as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncKnowledge2Impl: Sized + ISyncKnowledgeImpl {
    fn GetIdParameters();
    fn ProjectOntoColumnSet();
    fn SerializeWithOptions();
    fn GetLowestUncontainedId();
    fn GetInspector();
    fn GetMinimumSupportedVersion();
    fn GetStatistics();
    fn ContainsKnowledgeForItem();
    fn ContainsKnowledgeForChangeUnit();
    fn ProjectOntoKnowledgeWithPrerequisite();
    fn Complement();
    fn IntersectsWithKnowledge();
    fn GetKnowledgeCookie();
    fn CompareToKnowledgeCookie();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncKnowledge2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncKnowledge2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncKnowledge2Vtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProjectOntoColumnSet<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SerializeWithOptions<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLowestUncontainedId<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisyncknowledge: ::windows::core::RawPtr, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInspector<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinimumSupportedVersion<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatistics<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainsKnowledgeForItem<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainsKnowledgeForChangeUnit<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProjectOntoKnowledgeWithPrerequisite<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr, ptemplateknowledge: ::windows::core::RawPtr, ppprojectedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Complement<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr, ppcomplementedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IntersectsWithKnowledge<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKnowledgeCookie<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareToKnowledgeCookie<Impl: ISyncKnowledge2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetOwnerReplicaId::<Impl, IMPL_OFFSET>,
            Serialize::<Impl, IMPL_OFFSET>,
            SetLocalTickCount::<Impl, IMPL_OFFSET>,
            ContainsChange::<Impl, IMPL_OFFSET>,
            ContainsChangeUnit::<Impl, IMPL_OFFSET>,
            GetScopeVector::<Impl, IMPL_OFFSET>,
            GetReplicaKeyMap::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            ConvertVersion::<Impl, IMPL_OFFSET>,
            MapRemoteToLocal::<Impl, IMPL_OFFSET>,
            Union::<Impl, IMPL_OFFSET>,
            ProjectOntoItem::<Impl, IMPL_OFFSET>,
            ProjectOntoChangeUnit::<Impl, IMPL_OFFSET>,
            ProjectOntoRange::<Impl, IMPL_OFFSET>,
            ExcludeItem::<Impl, IMPL_OFFSET>,
            ExcludeChangeUnit::<Impl, IMPL_OFFSET>,
            ContainsKnowledge::<Impl, IMPL_OFFSET>,
            FindMinTickCountForReplica::<Impl, IMPL_OFFSET>,
            GetRangeExceptions::<Impl, IMPL_OFFSET>,
            GetSingleItemExceptions::<Impl, IMPL_OFFSET>,
            GetChangeUnitExceptions::<Impl, IMPL_OFFSET>,
            FindClockVectorForItem::<Impl, IMPL_OFFSET>,
            FindClockVectorForChangeUnit::<Impl, IMPL_OFFSET>,
            GetVersion::<Impl, IMPL_OFFSET>,
            GetIdParameters::<Impl, IMPL_OFFSET>,
            ProjectOntoColumnSet::<Impl, IMPL_OFFSET>,
            SerializeWithOptions::<Impl, IMPL_OFFSET>,
            GetLowestUncontainedId::<Impl, IMPL_OFFSET>,
            GetInspector::<Impl, IMPL_OFFSET>,
            GetMinimumSupportedVersion::<Impl, IMPL_OFFSET>,
            GetStatistics::<Impl, IMPL_OFFSET>,
            ContainsKnowledgeForItem::<Impl, IMPL_OFFSET>,
            ContainsKnowledgeForChangeUnit::<Impl, IMPL_OFFSET>,
            ProjectOntoKnowledgeWithPrerequisite::<Impl, IMPL_OFFSET>,
            Complement::<Impl, IMPL_OFFSET>,
            IntersectsWithKnowledge::<Impl, IMPL_OFFSET>,
            GetKnowledgeCookie::<Impl, IMPL_OFFSET>,
            CompareToKnowledgeCookie::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncKnowledge2 as ::windows::core::Interface>::IID
    }
}
pub trait ISyncMergeTombstoneChangeImpl: Sized {
    fn GetWinnerItemId();
}
impl ISyncMergeTombstoneChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncMergeTombstoneChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncMergeTombstoneChangeVtbl {
        unsafe extern "system" fn GetWinnerItemId<Impl: ISyncMergeTombstoneChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWinnerItemId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncMergeTombstoneChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncProviderImpl: Sized {
    fn GetIdParameters();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderVtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISyncProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIdParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderConfigUIImpl: Sized {
    fn Init();
    fn GetRegisteredProperties();
    fn CreateAndRegisterNewSyncProvider();
    fn ModifySyncProvider();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderConfigUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderConfigUIVtbl {
        unsafe extern "system" fn Init<Impl: ISyncProviderConfigUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisteredProperties<Impl: ISyncProviderConfigUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconfiguiproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAndRegisterNewSyncProvider<Impl: ISyncProviderConfigUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifySyncProvider<Impl: ISyncProviderConfigUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Init::<Impl, IMPL_OFFSET>, GetRegisteredProperties::<Impl, IMPL_OFFSET>, CreateAndRegisterNewSyncProvider::<Impl, IMPL_OFFSET>, ModifySyncProvider::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderConfigUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderConfigUIInfoImpl: Sized + IPropertyStoreImpl {
    fn GetSyncProviderConfigUI();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderConfigUIInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderConfigUIInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderConfigUIInfoVtbl {
        unsafe extern "system" fn GetSyncProviderConfigUI<Impl: ISyncProviderConfigUIInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, GetSyncProviderConfigUI::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderConfigUIInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderInfoImpl: Sized + IPropertyStoreImpl {
    fn GetSyncProvider();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderInfoVtbl {
        unsafe extern "system" fn GetSyncProvider<Impl: ISyncProviderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, GetSyncProvider::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncProviderRegistrationImpl: Sized {
    fn CreateSyncProviderConfigUIRegistrationInstance();
    fn UnregisterSyncProviderConfigUI();
    fn EnumerateSyncProviderConfigUIs();
    fn CreateSyncProviderRegistrationInstance();
    fn UnregisterSyncProvider();
    fn GetSyncProviderConfigUIInfoforProvider();
    fn EnumerateSyncProviders();
    fn GetSyncProviderInfo();
    fn GetSyncProviderFromInstanceId();
    fn GetSyncProviderConfigUIInfo();
    fn GetSyncProviderConfigUIFromInstanceId();
    fn GetSyncProviderState();
    fn SetSyncProviderState();
    fn RegisterForEvent();
    fn RevokeEvent();
    fn GetChange();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncProviderRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncProviderRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncProviderRegistrationVtbl {
        unsafe extern "system" fn CreateSyncProviderConfigUIRegistrationInstance<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterSyncProviderConfigUI<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateSyncProviderConfigUIs<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSyncProviderRegistrationInstance<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterSyncProvider<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfoforProvider<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidproviderinstanceid: *const ::windows::core::GUID, ppproviderconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateSyncProviders<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSyncProviderInfo<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSyncProviderFromInstanceId<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfo<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSyncProviderConfigUIFromInstanceId<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSyncProviderState<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pdwstateflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSyncProviderState<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterForEvent<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevokeEvent<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChange<Impl: ISyncProviderRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateSyncProviderConfigUIRegistrationInstance::<Impl, IMPL_OFFSET>,
            UnregisterSyncProviderConfigUI::<Impl, IMPL_OFFSET>,
            EnumerateSyncProviderConfigUIs::<Impl, IMPL_OFFSET>,
            CreateSyncProviderRegistrationInstance::<Impl, IMPL_OFFSET>,
            UnregisterSyncProvider::<Impl, IMPL_OFFSET>,
            GetSyncProviderConfigUIInfoforProvider::<Impl, IMPL_OFFSET>,
            EnumerateSyncProviders::<Impl, IMPL_OFFSET>,
            GetSyncProviderInfo::<Impl, IMPL_OFFSET>,
            GetSyncProviderFromInstanceId::<Impl, IMPL_OFFSET>,
            GetSyncProviderConfigUIInfo::<Impl, IMPL_OFFSET>,
            GetSyncProviderConfigUIFromInstanceId::<Impl, IMPL_OFFSET>,
            GetSyncProviderState::<Impl, IMPL_OFFSET>,
            SetSyncProviderState::<Impl, IMPL_OFFSET>,
            RegisterForEvent::<Impl, IMPL_OFFSET>,
            RevokeEvent::<Impl, IMPL_OFFSET>,
            GetChange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncProviderRegistration as ::windows::core::Interface>::IID
    }
}
pub trait ISyncRegistrationChangeImpl: Sized {
    fn GetEvent();
    fn GetInstanceId();
}
impl ISyncRegistrationChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncRegistrationChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncRegistrationChangeVtbl {
        unsafe extern "system" fn GetEvent<Impl: ISyncRegistrationChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInstanceId<Impl: ISyncRegistrationChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetEvent::<Impl, IMPL_OFFSET>, GetInstanceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncRegistrationChange as ::windows::core::Interface>::IID
    }
}
pub trait ISyncSessionExtendedErrorInfoImpl: Sized {
    fn GetSyncProviderWithError();
}
impl ISyncSessionExtendedErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionExtendedErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncSessionExtendedErrorInfoVtbl {
        unsafe extern "system" fn GetSyncProviderWithError<Impl: ISyncSessionExtendedErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproviderwitherror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSyncProviderWithError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionExtendedErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncSessionStateImpl: Sized {
    fn IsCanceled();
    fn GetInfoForChangeApplication();
    fn LoadInfoFromChangeApplication();
    fn GetForgottenKnowledgeRecoveryRangeStart();
    fn GetForgottenKnowledgeRecoveryRangeEnd();
    fn SetForgottenKnowledgeRecoveryRange();
    fn OnProgress();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncSessionStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncSessionStateVtbl {
        unsafe extern "system" fn IsCanceled<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInfoForChangeApplication<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadInfoFromChangeApplication<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeStart<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeEnd<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetForgottenKnowledgeRecoveryRange<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: *const SYNC_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnProgress<Impl: ISyncSessionStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            IsCanceled::<Impl, IMPL_OFFSET>,
            GetInfoForChangeApplication::<Impl, IMPL_OFFSET>,
            LoadInfoFromChangeApplication::<Impl, IMPL_OFFSET>,
            GetForgottenKnowledgeRecoveryRangeStart::<Impl, IMPL_OFFSET>,
            GetForgottenKnowledgeRecoveryRangeEnd::<Impl, IMPL_OFFSET>,
            SetForgottenKnowledgeRecoveryRange::<Impl, IMPL_OFFSET>,
            OnProgress::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncSessionState2Impl: Sized + ISyncSessionStateImpl {
    fn SetProviderWithError();
    fn GetSessionErrorStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl ISyncSessionState2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISyncSessionState2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISyncSessionState2Vtbl {
        unsafe extern "system" fn SetProviderWithError<Impl: ISyncSessionState2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fself: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSessionErrorStatus<Impl: ISyncSessionState2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            IsCanceled::<Impl, IMPL_OFFSET>,
            GetInfoForChangeApplication::<Impl, IMPL_OFFSET>,
            LoadInfoFromChangeApplication::<Impl, IMPL_OFFSET>,
            GetForgottenKnowledgeRecoveryRangeStart::<Impl, IMPL_OFFSET>,
            GetForgottenKnowledgeRecoveryRangeEnd::<Impl, IMPL_OFFSET>,
            SetForgottenKnowledgeRecoveryRange::<Impl, IMPL_OFFSET>,
            OnProgress::<Impl, IMPL_OFFSET>,
            SetProviderWithError::<Impl, IMPL_OFFSET>,
            GetSessionErrorStatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISyncSessionState2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronousDataRetrieverImpl: Sized {
    fn GetIdParameters();
    fn LoadChangeData();
}
#[cfg(feature = "Win32_Foundation")]
impl ISynchronousDataRetrieverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronousDataRetrieverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronousDataRetrieverVtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadChangeData<Impl: ISynchronousDataRetrieverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIdParameters::<Impl, IMPL_OFFSET>, LoadChangeData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronousDataRetriever as ::windows::core::Interface>::IID
    }
}
