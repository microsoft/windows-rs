pub trait IAsynchronousDataRetrieverImpl: Sized {
    fn GetIdParameters();
    fn RegisterCallback();
    fn RevokeCallback();
    fn LoadChangeData();
}
impl ::windows::core::RuntimeName for IAsynchronousDataRetriever {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IAsynchronousDataRetriever";
}
impl IAsynchronousDataRetrieverVtbl {
    pub const fn new<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAsynchronousDataRetrieverVtbl {
        unsafe extern "system" fn GetIdParameters<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIdParameters(&*(&pidparameters as *const <ID_PARAMETERS as ::windows::core::Abi>::Abi as *const <ID_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallback<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterCallback(&*(&pdataretrievercallback as *const <IDataRetrieverCallback as ::windows::core::Abi>::Abi as *const <IDataRetrieverCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeCallback<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevokeCallback(&*(&pdataretrievercallback as *const <IDataRetrieverCallback as ::windows::core::Abi>::Abi as *const <IDataRetrieverCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadChangeData<Impl: IAsynchronousDataRetrieverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadChangeData(&*(&ploadchangecontext as *const <ILoadChangeContext as ::windows::core::Abi>::Abi as *const <ILoadChangeContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAsynchronousDataRetriever>, base.5, GetIdParameters::<Impl, OFFSET>, RegisterCallback::<Impl, OFFSET>, RevokeCallback::<Impl, OFFSET>, LoadChangeData::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IChangeConflict {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IChangeConflict";
}
impl IChangeConflictVtbl {
    pub const fn new<Impl: IChangeConflictImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IChangeConflictVtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Impl: IChangeConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingChange(::core::mem::transmute_copy(&ppconflictingchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Impl: IChangeConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingChange(::core::mem::transmute_copy(&ppconflictingchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Impl: IChangeConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingData(::core::mem::transmute_copy(&ppconflictingdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Impl: IChangeConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingData(::core::mem::transmute_copy(&ppconflictingdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolveActionForChange<Impl: IChangeConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResolveActionForChange(presolveaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResolveActionForChange<Impl: IChangeConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetResolveActionForChange(resolveaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolveActionForChangeUnit<Impl: IChangeConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResolveActionForChangeUnit(&*(&pchangeunit as *const <ISyncChangeUnit as ::windows::core::Abi>::Abi as *const <ISyncChangeUnit as ::windows::core::DefaultType>::DefaultType), presolveaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResolveActionForChangeUnit<Impl: IChangeConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetResolveActionForChangeUnit(&*(&pchangeunit as *const <ISyncChangeUnit as ::windows::core::Abi>::Abi as *const <ISyncChangeUnit as ::windows::core::DefaultType>::DefaultType), resolveaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IChangeConflict>,
            base.5,
            GetDestinationProviderConflictingChange::<Impl, OFFSET>,
            GetSourceProviderConflictingChange::<Impl, OFFSET>,
            GetDestinationProviderConflictingData::<Impl, OFFSET>,
            GetSourceProviderConflictingData::<Impl, OFFSET>,
            GetResolveActionForChange::<Impl, OFFSET>,
            SetResolveActionForChange::<Impl, OFFSET>,
            GetResolveActionForChangeUnit::<Impl, OFFSET>,
            SetResolveActionForChangeUnit::<Impl, OFFSET>,
        )
    }
}
pub trait IChangeUnitExceptionImpl: Sized {
    fn GetItemId();
    fn GetChangeUnitId();
    fn GetClockVector();
}
impl ::windows::core::RuntimeName for IChangeUnitException {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IChangeUnitException";
}
impl IChangeUnitExceptionVtbl {
    pub const fn new<Impl: IChangeUnitExceptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IChangeUnitExceptionVtbl {
        unsafe extern "system" fn GetItemId<Impl: IChangeUnitExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemId(pbitemid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: IChangeUnitExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeUnitId(pbchangeunitid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClockVector<Impl: IChangeUnitExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClockVector(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppunk as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IChangeUnitException>, base.5, GetItemId::<Impl, OFFSET>, GetChangeUnitId::<Impl, OFFSET>, GetClockVector::<Impl, OFFSET>)
    }
}
pub trait IChangeUnitListFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn Initialize();
    fn GetChangeUnitIdCount();
    fn GetChangeUnitId();
}
impl ::windows::core::RuntimeName for IChangeUnitListFilterInfo {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IChangeUnitListFilterInfo";
}
impl IChangeUnitListFilterInfoVtbl {
    pub const fn new<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IChangeUnitListFilterInfoVtbl {
        unsafe extern "system" fn Initialize<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(ppbchangeunitids, dwchangeunitcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitIdCount<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeUnitIdCount(pdwchangeunitidcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: IChangeUnitListFilterInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeUnitId(dwchangeunitidindex, pbchangeunitid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IChangeUnitListFilterInfo>, base.5, Initialize::<Impl, OFFSET>, GetChangeUnitIdCount::<Impl, OFFSET>, GetChangeUnitId::<Impl, OFFSET>)
    }
}
pub trait IClockVectorImpl: Sized {
    fn GetClockVectorElements();
    fn GetClockVectorElementCount();
}
impl ::windows::core::RuntimeName for IClockVector {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IClockVector";
}
impl IClockVectorVtbl {
    pub const fn new<Impl: IClockVectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClockVectorVtbl {
        unsafe extern "system" fn GetClockVectorElements<Impl: IClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClockVectorElements(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppienumclockvector as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClockVectorElementCount<Impl: IClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClockVectorElementCount(pdwcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClockVector>, base.5, GetClockVectorElements::<Impl, OFFSET>, GetClockVectorElementCount::<Impl, OFFSET>)
    }
}
pub trait IClockVectorElementImpl: Sized {
    fn GetReplicaKey();
    fn GetTickCount();
}
impl ::windows::core::RuntimeName for IClockVectorElement {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IClockVectorElement";
}
impl IClockVectorElementVtbl {
    pub const fn new<Impl: IClockVectorElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IClockVectorElementVtbl {
        unsafe extern "system" fn GetReplicaKey<Impl: IClockVectorElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReplicaKey(pdwreplicakey) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTickCount<Impl: IClockVectorElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulltickcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTickCount(pulltickcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IClockVectorElement>, base.5, GetReplicaKey::<Impl, OFFSET>, GetTickCount::<Impl, OFFSET>)
    }
}
pub trait ICombinedFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn GetFilterCount();
    fn GetFilterInfo();
    fn GetFilterCombinationType();
}
impl ::windows::core::RuntimeName for ICombinedFilterInfo {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ICombinedFilterInfo";
}
impl ICombinedFilterInfoVtbl {
    pub const fn new<Impl: ICombinedFilterInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICombinedFilterInfoVtbl {
        unsafe extern "system" fn GetFilterCount<Impl: ICombinedFilterInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilterCount(pdwfiltercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterInfo<Impl: ICombinedFilterInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilterInfo(dwfilterindex, ::core::mem::transmute_copy(&ppifilterinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterCombinationType<Impl: ICombinedFilterInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilterCombinationType(pfiltercombinationtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICombinedFilterInfo>, base.5, GetFilterCount::<Impl, OFFSET>, GetFilterInfo::<Impl, OFFSET>, GetFilterCombinationType::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IConstraintConflict {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IConstraintConflict";
}
impl IConstraintConflictVtbl {
    pub const fn new<Impl: IConstraintConflictImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConstraintConflictVtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingChange(::core::mem::transmute_copy(&ppconflictingchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingChange(::core::mem::transmute_copy(&ppconflictingchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalChange<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pporiginalchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderOriginalChange(::core::mem::transmute_copy(&pporiginalchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderConflictingData(::core::mem::transmute_copy(&ppconflictingdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceProviderConflictingData(::core::mem::transmute_copy(&ppconflictingdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalData<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDestinationProviderOriginalData(::core::mem::transmute_copy(&pporiginaldata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstraintResolveActionForChange<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConstraintResolveActionForChange(pconstraintresolveaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConstraintResolveActionForChange<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConstraintResolveActionForChange(constraintresolveaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstraintResolveActionForChangeUnit<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConstraintResolveActionForChangeUnit(&*(&pchangeunit as *const <ISyncChangeUnit as ::windows::core::Abi>::Abi as *const <ISyncChangeUnit as ::windows::core::DefaultType>::DefaultType), pconstraintresolveaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConstraintResolveActionForChangeUnit<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConstraintResolveActionForChangeUnit(&*(&pchangeunit as *const <ISyncChangeUnit as ::windows::core::Abi>::Abi as *const <ISyncChangeUnit as ::windows::core::DefaultType>::DefaultType), constraintresolveaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstraintConflictReason<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConstraintConflictReason(pconstraintconflictreason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTemporary<Impl: IConstraintConflictImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTemporary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IConstraintConflict>,
            base.5,
            GetDestinationProviderConflictingChange::<Impl, OFFSET>,
            GetSourceProviderConflictingChange::<Impl, OFFSET>,
            GetDestinationProviderOriginalChange::<Impl, OFFSET>,
            GetDestinationProviderConflictingData::<Impl, OFFSET>,
            GetSourceProviderConflictingData::<Impl, OFFSET>,
            GetDestinationProviderOriginalData::<Impl, OFFSET>,
            GetConstraintResolveActionForChange::<Impl, OFFSET>,
            SetConstraintResolveActionForChange::<Impl, OFFSET>,
            GetConstraintResolveActionForChangeUnit::<Impl, OFFSET>,
            SetConstraintResolveActionForChangeUnit::<Impl, OFFSET>,
            GetConstraintConflictReason::<Impl, OFFSET>,
            IsTemporary::<Impl, OFFSET>,
        )
    }
}
pub trait IConstructReplicaKeyMapImpl: Sized {
    fn FindOrAddReplica();
}
impl ::windows::core::RuntimeName for IConstructReplicaKeyMap {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IConstructReplicaKeyMap";
}
impl IConstructReplicaKeyMapVtbl {
    pub const fn new<Impl: IConstructReplicaKeyMapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IConstructReplicaKeyMapVtbl {
        unsafe extern "system" fn FindOrAddReplica<Impl: IConstructReplicaKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindOrAddReplica(pbreplicaid, pdwreplicakey) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IConstructReplicaKeyMap>, base.5, FindOrAddReplica::<Impl, OFFSET>)
    }
}
pub trait ICoreFragmentImpl: Sized {
    fn NextColumn();
    fn NextRange();
    fn Reset();
    fn GetColumnCount();
    fn GetRangeCount();
}
impl ::windows::core::RuntimeName for ICoreFragment {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ICoreFragment";
}
impl ICoreFragmentVtbl {
    pub const fn new<Impl: ICoreFragmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreFragmentVtbl {
        unsafe extern "system" fn NextColumn<Impl: ICoreFragmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextColumn(pchangeunitid, pchangeunitidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRange<Impl: ICoreFragmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextRange(pitemid, pitemidsize, ::core::mem::transmute_copy(&piclockvector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ICoreFragmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnCount<Impl: ICoreFragmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolumncount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColumnCount(pcolumncount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeCount<Impl: ICoreFragmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRangeCount(prangecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreFragment>, base.5, NextColumn::<Impl, OFFSET>, NextRange::<Impl, OFFSET>, Reset::<Impl, OFFSET>, GetColumnCount::<Impl, OFFSET>, GetRangeCount::<Impl, OFFSET>)
    }
}
pub trait ICoreFragmentInspectorImpl: Sized {
    fn NextCoreFragments();
    fn Reset();
}
impl ::windows::core::RuntimeName for ICoreFragmentInspector {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ICoreFragmentInspector";
}
impl ICoreFragmentInspectorVtbl {
    pub const fn new<Impl: ICoreFragmentInspectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreFragmentInspectorVtbl {
        unsafe extern "system" fn NextCoreFragments<Impl: ICoreFragmentInspectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut ::windows::core::RawPtr, pfetchedcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextCoreFragments(requestedcount, ::core::mem::transmute_copy(&ppicorefragments), pfetchedcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ICoreFragmentInspectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreFragmentInspector>, base.5, NextCoreFragments::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait ICustomFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn GetSyncFilter();
}
impl ::windows::core::RuntimeName for ICustomFilterInfo {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ICustomFilterInfo";
}
impl ICustomFilterInfoVtbl {
    pub const fn new<Impl: ICustomFilterInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomFilterInfoVtbl {
        unsafe extern "system" fn GetSyncFilter<Impl: ICustomFilterInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncFilter(::core::mem::transmute_copy(&pisyncfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomFilterInfo>, base.5, GetSyncFilter::<Impl, OFFSET>)
    }
}
pub trait IDataRetrieverCallbackImpl: Sized {
    fn LoadChangeDataComplete();
    fn LoadChangeDataError();
}
impl ::windows::core::RuntimeName for IDataRetrieverCallback {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IDataRetrieverCallback";
}
impl IDataRetrieverCallbackVtbl {
    pub const fn new<Impl: IDataRetrieverCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataRetrieverCallbackVtbl {
        unsafe extern "system" fn LoadChangeDataComplete<Impl: IDataRetrieverCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadChangeDataComplete(&*(&punkdata as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadChangeDataError<Impl: IDataRetrieverCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadChangeDataError(hrerror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataRetrieverCallback>, base.5, LoadChangeDataComplete::<Impl, OFFSET>, LoadChangeDataError::<Impl, OFFSET>)
    }
}
pub trait IEnumChangeUnitExceptionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumChangeUnitExceptions {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumChangeUnitExceptions";
}
impl IEnumChangeUnitExceptionsVtbl {
    pub const fn new<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumChangeUnitExceptionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cexceptions, ::core::mem::transmute_copy(&ppchangeunitexception), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cexceptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumChangeUnitExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumChangeUnitExceptions>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumClockVectorImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumClockVector {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumClockVector";
}
impl IEnumClockVectorVtbl {
    pub const fn new<Impl: IEnumClockVectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumClockVectorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cclockvectorelements, ::core::mem::transmute_copy(&ppiclockvectorelements), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(csyncversions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumClockVector>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumFeedClockVectorImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumFeedClockVector {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumFeedClockVector";
}
impl IEnumFeedClockVectorVtbl {
    pub const fn new<Impl: IEnumFeedClockVectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumFeedClockVectorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumFeedClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cclockvectorelements, ::core::mem::transmute_copy(&ppiclockvectorelements), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumFeedClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(csyncversions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumFeedClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumFeedClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumFeedClockVector>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumItemIdsImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IEnumItemIds {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumItemIds";
}
impl IEnumItemIdsVtbl {
    pub const fn new<Impl: IEnumItemIdsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumItemIdsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumItemIdsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(pbitemid, pcbitemidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumItemIds>, base.5, Next::<Impl, OFFSET>)
    }
}
pub trait IEnumRangeExceptionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumRangeExceptions {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumRangeExceptions";
}
impl IEnumRangeExceptionsVtbl {
    pub const fn new<Impl: IEnumRangeExceptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumRangeExceptionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumRangeExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32, pprangeexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cexceptions, ::core::mem::transmute_copy(&pprangeexception), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumRangeExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cexceptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumRangeExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumRangeExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumRangeExceptions>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSingleItemExceptionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSingleItemExceptions {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumSingleItemExceptions";
}
impl IEnumSingleItemExceptionsVtbl {
    pub const fn new<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSingleItemExceptionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cexceptions, ::core::mem::transmute_copy(&ppsingleitemexception), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cexceptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSingleItemExceptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSingleItemExceptions>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSyncChangeUnitsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSyncChangeUnits {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumSyncChangeUnits";
}
impl IEnumSyncChangeUnitsVtbl {
    pub const fn new<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSyncChangeUnitsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchangeunit: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cchanges, ::core::mem::transmute_copy(&ppchangeunit), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cchanges) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncChangeUnitsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSyncChangeUnits>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSyncChangesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSyncChanges {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumSyncChanges";
}
impl IEnumSyncChangesVtbl {
    pub const fn new<Impl: IEnumSyncChangesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSyncChangesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncChangesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchange: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cchanges, ::core::mem::transmute_copy(&ppchange), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncChangesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cchanges) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncChangesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncChangesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSyncChanges>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSyncProviderConfigUIInfosImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSyncProviderConfigUIInfos {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumSyncProviderConfigUIInfos";
}
impl IEnumSyncProviderConfigUIInfosVtbl {
    pub const fn new<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSyncProviderConfigUIInfosVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cfactories, ::core::mem::transmute_copy(&ppsyncproviderconfiguiinfo), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfactories: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cfactories) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncProviderConfigUIInfosImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSyncProviderConfigUIInfos>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSyncProviderInfosImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSyncProviderInfos {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IEnumSyncProviderInfos";
}
impl IEnumSyncProviderInfosVtbl {
    pub const fn new<Impl: IEnumSyncProviderInfosImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSyncProviderInfosVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSyncProviderInfosImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cinstances, ::core::mem::transmute_copy(&ppsyncproviderinfo), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSyncProviderInfosImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cinstances: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cinstances) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSyncProviderInfosImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSyncProviderInfosImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSyncProviderInfos>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IFeedClockVectorImpl: Sized + IClockVectorImpl {
    fn GetUpdateCount();
    fn IsNoConflictsSpecified();
}
impl ::windows::core::RuntimeName for IFeedClockVector {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IFeedClockVector";
}
impl IFeedClockVectorVtbl {
    pub const fn new<Impl: IFeedClockVectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFeedClockVectorVtbl {
        unsafe extern "system" fn GetUpdateCount<Impl: IFeedClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwupdatecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUpdateCount(pdwupdatecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNoConflictsSpecified<Impl: IFeedClockVectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsNoConflictsSpecified(&*(&pfisnoconflictsspecified as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFeedClockVector>, base.5, GetUpdateCount::<Impl, OFFSET>, IsNoConflictsSpecified::<Impl, OFFSET>)
    }
}
pub trait IFeedClockVectorElementImpl: Sized + IClockVectorElementImpl {
    fn GetSyncTime();
    fn GetFlags();
}
impl ::windows::core::RuntimeName for IFeedClockVectorElement {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IFeedClockVectorElement";
}
impl IFeedClockVectorElementVtbl {
    pub const fn new<Impl: IFeedClockVectorElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFeedClockVectorElementVtbl {
        unsafe extern "system" fn GetSyncTime<Impl: IFeedClockVectorElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psynctime: *mut SYNC_TIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncTime(&*(&psynctime as *const <SYNC_TIME as ::windows::core::Abi>::Abi as *const <SYNC_TIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: IFeedClockVectorElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbflags: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlags(pbflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFeedClockVectorElement>, base.5, GetSyncTime::<Impl, OFFSET>, GetFlags::<Impl, OFFSET>)
    }
}
pub trait IFilterKeyMapImpl: Sized {
    fn GetCount();
    fn AddFilter();
    fn GetFilter();
    fn Serialize();
}
impl ::windows::core::RuntimeName for IFilterKeyMap {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IFilterKeyMap";
}
impl IFilterKeyMapVtbl {
    pub const fn new<Impl: IFilterKeyMapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFilterKeyMapVtbl {
        unsafe extern "system" fn GetCount<Impl: IFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(pdwcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFilter<Impl: IFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisyncfilter: ::windows::core::RawPtr, pdwfilterkey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddFilter(&*(&pisyncfilter as *const <ISyncFilter as ::windows::core::Abi>::Abi as *const <ISyncFilter as ::windows::core::DefaultType>::DefaultType), pdwfilterkey) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilter<Impl: IFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilter(dwfilterkey, ::core::mem::transmute_copy(&ppisyncfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: IFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Serialize(pbfilterkeymap, pcbfilterkeymap) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFilterKeyMap>, base.5, GetCount::<Impl, OFFSET>, AddFilter::<Impl, OFFSET>, GetFilter::<Impl, OFFSET>, Serialize::<Impl, OFFSET>)
    }
}
pub trait IFilterRequestCallbackImpl: Sized {
    fn RequestFilter();
}
impl ::windows::core::RuntimeName for IFilterRequestCallback {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IFilterRequestCallback";
}
impl IFilterRequestCallbackVtbl {
    pub const fn new<Impl: IFilterRequestCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFilterRequestCallbackVtbl {
        unsafe extern "system" fn RequestFilter<Impl: IFilterRequestCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestFilter(&*(&pfilter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), filteringtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFilterRequestCallback>, base.5, RequestFilter::<Impl, OFFSET>)
    }
}
pub trait IFilterTrackingProviderImpl: Sized {
    fn SpecifyTrackedFilters();
    fn AddTrackedFilter();
}
impl ::windows::core::RuntimeName for IFilterTrackingProvider {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IFilterTrackingProvider";
}
impl IFilterTrackingProviderVtbl {
    pub const fn new<Impl: IFilterTrackingProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFilterTrackingProviderVtbl {
        unsafe extern "system" fn SpecifyTrackedFilters<Impl: IFilterTrackingProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpecifyTrackedFilters(&*(&pcallback as *const <IFilterTrackingRequestCallback as ::windows::core::Abi>::Abi as *const <IFilterTrackingRequestCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrackedFilter<Impl: IFilterTrackingProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddTrackedFilter(&*(&pfilter as *const <ISyncFilter as ::windows::core::Abi>::Abi as *const <ISyncFilter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFilterTrackingProvider>, base.5, SpecifyTrackedFilters::<Impl, OFFSET>, AddTrackedFilter::<Impl, OFFSET>)
    }
}
pub trait IFilterTrackingRequestCallbackImpl: Sized {
    fn RequestTrackedFilter();
}
impl ::windows::core::RuntimeName for IFilterTrackingRequestCallback {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IFilterTrackingRequestCallback";
}
impl IFilterTrackingRequestCallbackVtbl {
    pub const fn new<Impl: IFilterTrackingRequestCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFilterTrackingRequestCallbackVtbl {
        unsafe extern "system" fn RequestTrackedFilter<Impl: IFilterTrackingRequestCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestTrackedFilter(&*(&pfilter as *const <ISyncFilter as ::windows::core::Abi>::Abi as *const <ISyncFilter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFilterTrackingRequestCallback>, base.5, RequestTrackedFilter::<Impl, OFFSET>)
    }
}
pub trait IFilterTrackingSyncChangeBuilderImpl: Sized {
    fn AddFilterChange();
    fn SetAllChangeUnitsPresentFlag();
}
impl ::windows::core::RuntimeName for IFilterTrackingSyncChangeBuilder {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IFilterTrackingSyncChangeBuilder";
}
impl IFilterTrackingSyncChangeBuilderVtbl {
    pub const fn new<Impl: IFilterTrackingSyncChangeBuilderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFilterTrackingSyncChangeBuilderVtbl {
        unsafe extern "system" fn AddFilterChange<Impl: IFilterTrackingSyncChangeBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddFilterChange(dwfilterkey, &*(&pfilterchange as *const <SYNC_FILTER_CHANGE as ::windows::core::Abi>::Abi as *const <SYNC_FILTER_CHANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllChangeUnitsPresentFlag<Impl: IFilterTrackingSyncChangeBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllChangeUnitsPresentFlag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFilterTrackingSyncChangeBuilder>, base.5, AddFilterChange::<Impl, OFFSET>, SetAllChangeUnitsPresentFlag::<Impl, OFFSET>)
    }
}
pub trait IForgottenKnowledgeImpl: Sized + ISyncKnowledgeImpl {
    fn ForgetToVersion();
}
impl ::windows::core::RuntimeName for IForgottenKnowledge {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IForgottenKnowledge";
}
impl IForgottenKnowledgeVtbl {
    pub const fn new<Impl: IForgottenKnowledgeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IForgottenKnowledgeVtbl {
        unsafe extern "system" fn ForgetToVersion<Impl: IForgottenKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForgetToVersion(&*(&pknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IForgottenKnowledge>, base.5, ForgetToVersion::<Impl, OFFSET>)
    }
}
pub trait IKnowledgeSyncProviderImpl: Sized + ISyncProviderImpl {
    fn BeginSession();
    fn GetSyncBatchParameters();
    fn GetChangeBatch();
    fn GetFullEnumerationChangeBatch();
    fn ProcessChangeBatch();
    fn ProcessFullEnumerationChangeBatch();
    fn EndSession();
}
impl ::windows::core::RuntimeName for IKnowledgeSyncProvider {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IKnowledgeSyncProvider";
}
impl IKnowledgeSyncProviderVtbl {
    pub const fn new<Impl: IKnowledgeSyncProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnowledgeSyncProviderVtbl {
        unsafe extern "system" fn BeginSession<Impl: IKnowledgeSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginSession(role, &*(&psessionstate as *const <ISyncSessionState as ::windows::core::Abi>::Abi as *const <ISyncSessionState as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncBatchParameters<Impl: IKnowledgeSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsyncknowledge: *mut ::windows::core::RawPtr, pdwrequestedbatchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncBatchParameters(::core::mem::transmute_copy(&ppsyncknowledge), pdwrequestedbatchsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeBatch(dwbatchsize, &*(&psyncknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullEnumerationChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: ::windows::core::RawPtr, ppsyncchangebatch: *mut ::windows::core::RawPtr, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFullEnumerationChangeBatch(dwbatchsize, pblowerenumerationbound, &*(&psyncknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessChangeBatch(
                resolutionpolicy,
                &*(&psourcechangebatch as *const <ISyncChangeBatch as ::windows::core::Abi>::Abi as *const <ISyncChangeBatch as ::windows::core::DefaultType>::DefaultType),
                &*(&punkdataretriever as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <ISyncCallback as ::windows::core::Abi>::Abi as *const <ISyncCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&psyncsessionstatistics as *const <SYNC_SESSION_STATISTICS as ::windows::core::Abi>::Abi as *const <SYNC_SESSION_STATISTICS as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessFullEnumerationChangeBatch<Impl: IKnowledgeSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::core::RawPtr, punkdataretriever: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessFullEnumerationChangeBatch(
                resolutionpolicy,
                &*(&psourcechangebatch as *const <ISyncFullEnumerationChangeBatch as ::windows::core::Abi>::Abi as *const <ISyncFullEnumerationChangeBatch as ::windows::core::DefaultType>::DefaultType),
                &*(&punkdataretriever as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <ISyncCallback as ::windows::core::Abi>::Abi as *const <ISyncCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&psyncsessionstatistics as *const <SYNC_SESSION_STATISTICS as ::windows::core::Abi>::Abi as *const <SYNC_SESSION_STATISTICS as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSession<Impl: IKnowledgeSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psessionstate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndSession(&*(&psessionstate as *const <ISyncSessionState as ::windows::core::Abi>::Abi as *const <ISyncSessionState as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKnowledgeSyncProvider>, base.5, BeginSession::<Impl, OFFSET>, GetSyncBatchParameters::<Impl, OFFSET>, GetChangeBatch::<Impl, OFFSET>, GetFullEnumerationChangeBatch::<Impl, OFFSET>, ProcessChangeBatch::<Impl, OFFSET>, ProcessFullEnumerationChangeBatch::<Impl, OFFSET>, EndSession::<Impl, OFFSET>)
    }
}
pub trait ILoadChangeContextImpl: Sized {
    fn GetSyncChange();
    fn SetRecoverableErrorOnChange();
    fn SetRecoverableErrorOnChangeUnit();
}
impl ::windows::core::RuntimeName for ILoadChangeContext {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ILoadChangeContext";
}
impl ILoadChangeContextVtbl {
    pub const fn new<Impl: ILoadChangeContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoadChangeContextVtbl {
        unsafe extern "system" fn GetSyncChange<Impl: ILoadChangeContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncChange(::core::mem::transmute_copy(&ppsyncchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoverableErrorOnChange<Impl: ILoadChangeContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRecoverableErrorOnChange(hrerror, &*(&perrordata as *const <IRecoverableErrorData as ::windows::core::Abi>::Abi as *const <IRecoverableErrorData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoverableErrorOnChangeUnit<Impl: ILoadChangeContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, pchangeunit: ::windows::core::RawPtr, perrordata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRecoverableErrorOnChangeUnit(hrerror, &*(&pchangeunit as *const <ISyncChangeUnit as ::windows::core::Abi>::Abi as *const <ISyncChangeUnit as ::windows::core::DefaultType>::DefaultType), &*(&perrordata as *const <IRecoverableErrorData as ::windows::core::Abi>::Abi as *const <IRecoverableErrorData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoadChangeContext>, base.5, GetSyncChange::<Impl, OFFSET>, SetRecoverableErrorOnChange::<Impl, OFFSET>, SetRecoverableErrorOnChangeUnit::<Impl, OFFSET>)
    }
}
pub trait IProviderConverterImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IProviderConverter {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IProviderConverter";
}
impl IProviderConverterVtbl {
    pub const fn new<Impl: IProviderConverterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProviderConverterVtbl {
        unsafe extern "system" fn Initialize<Impl: IProviderConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisyncprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pisyncprovider as *const <ISyncProvider as ::windows::core::Abi>::Abi as *const <ISyncProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProviderConverter>, base.5, Initialize::<Impl, OFFSET>)
    }
}
pub trait IRangeExceptionImpl: Sized {
    fn GetClosedRangeStart();
    fn GetClosedRangeEnd();
    fn GetClockVector();
}
impl ::windows::core::RuntimeName for IRangeException {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IRangeException";
}
impl IRangeExceptionVtbl {
    pub const fn new<Impl: IRangeExceptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRangeExceptionVtbl {
        unsafe extern "system" fn GetClosedRangeStart<Impl: IRangeExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClosedRangeStart(pbclosedrangestart, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosedRangeEnd<Impl: IRangeExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClosedRangeEnd(pbclosedrangeend, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClockVector<Impl: IRangeExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClockVector(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppunk as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRangeException>, base.5, GetClosedRangeStart::<Impl, OFFSET>, GetClosedRangeEnd::<Impl, OFFSET>, GetClockVector::<Impl, OFFSET>)
    }
}
pub trait IRecoverableErrorImpl: Sized {
    fn GetStage();
    fn GetProvider();
    fn GetChangeWithRecoverableError();
    fn GetRecoverableErrorDataForChange();
    fn GetRecoverableErrorDataForChangeUnit();
}
impl ::windows::core::RuntimeName for IRecoverableError {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IRecoverableError";
}
impl IRecoverableErrorVtbl {
    pub const fn new<Impl: IRecoverableErrorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRecoverableErrorVtbl {
        unsafe extern "system" fn GetStage<Impl: IRecoverableErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStage(pstage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProvider<Impl: IRecoverableErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProvider(pproviderrole) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeWithRecoverableError<Impl: IRecoverableErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppchangewithrecoverableerror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeWithRecoverableError(::core::mem::transmute_copy(&ppchangewithrecoverableerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChange<Impl: IRecoverableErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRecoverableErrorDataForChange(phrerror, ::core::mem::transmute_copy(&pperrordata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChangeUnit<Impl: IRecoverableErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: ::windows::core::RawPtr, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRecoverableErrorDataForChangeUnit(&*(&pchangeunit as *const <ISyncChangeUnit as ::windows::core::Abi>::Abi as *const <ISyncChangeUnit as ::windows::core::DefaultType>::DefaultType), phrerror, ::core::mem::transmute_copy(&pperrordata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRecoverableError>, base.5, GetStage::<Impl, OFFSET>, GetProvider::<Impl, OFFSET>, GetChangeWithRecoverableError::<Impl, OFFSET>, GetRecoverableErrorDataForChange::<Impl, OFFSET>, GetRecoverableErrorDataForChangeUnit::<Impl, OFFSET>)
    }
}
pub trait IRecoverableErrorDataImpl: Sized {
    fn Initialize();
    fn GetItemDisplayName();
    fn GetErrorDescription();
}
impl ::windows::core::RuntimeName for IRecoverableErrorData {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IRecoverableErrorData";
}
impl IRecoverableErrorDataVtbl {
    pub const fn new<Impl: IRecoverableErrorDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRecoverableErrorDataVtbl {
        unsafe extern "system" fn Initialize<Impl: IRecoverableErrorDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcszitemdisplayname: super::super::Foundation::PWSTR, pcszerrordescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pcszitemdisplayname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcszerrordescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemDisplayName<Impl: IRecoverableErrorDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszitemdisplayname: super::super::Foundation::PWSTR, pcchitemdisplayname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemDisplayName(&*(&pszitemdisplayname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcchitemdisplayname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Impl: IRecoverableErrorDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszerrordescription: super::super::Foundation::PWSTR, pccherrordescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorDescription(&*(&pszerrordescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pccherrordescription) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRecoverableErrorData>, base.5, Initialize::<Impl, OFFSET>, GetItemDisplayName::<Impl, OFFSET>, GetErrorDescription::<Impl, OFFSET>)
    }
}
pub trait IRegisteredSyncProviderImpl: Sized {
    fn Init();
    fn GetInstanceId();
    fn Reset();
}
impl ::windows::core::RuntimeName for IRegisteredSyncProvider {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IRegisteredSyncProvider";
}
impl IRegisteredSyncProviderVtbl {
    pub const fn new<Impl: IRegisteredSyncProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRegisteredSyncProviderVtbl {
        unsafe extern "system" fn Init<Impl: IRegisteredSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Init(
                &*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pguidcontenttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pcontextpropertystore as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Impl: IRegisteredSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInstanceId(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IRegisteredSyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRegisteredSyncProvider>, base.5, Init::<Impl, OFFSET>, GetInstanceId::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait IReplicaKeyMapImpl: Sized {
    fn LookupReplicaKey();
    fn LookupReplicaId();
    fn Serialize();
}
impl ::windows::core::RuntimeName for IReplicaKeyMap {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IReplicaKeyMap";
}
impl IReplicaKeyMapVtbl {
    pub const fn new<Impl: IReplicaKeyMapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReplicaKeyMapVtbl {
        unsafe extern "system" fn LookupReplicaKey<Impl: IReplicaKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LookupReplicaKey(pbreplicaid, pdwreplicakey) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupReplicaId<Impl: IReplicaKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LookupReplicaId(dwreplicakey, pbreplicaid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: IReplicaKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Serialize(pbreplicakeymap, pcbreplicakeymap) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReplicaKeyMap>, base.5, LookupReplicaKey::<Impl, OFFSET>, LookupReplicaId::<Impl, OFFSET>, Serialize::<Impl, OFFSET>)
    }
}
pub trait IRequestFilteredSyncImpl: Sized {
    fn SpecifyFilter();
}
impl ::windows::core::RuntimeName for IRequestFilteredSync {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.IRequestFilteredSync";
}
impl IRequestFilteredSyncVtbl {
    pub const fn new<Impl: IRequestFilteredSyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRequestFilteredSyncVtbl {
        unsafe extern "system" fn SpecifyFilter<Impl: IRequestFilteredSyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpecifyFilter(&*(&pcallback as *const <IFilterRequestCallback as ::windows::core::Abi>::Abi as *const <IFilterRequestCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRequestFilteredSync>, base.5, SpecifyFilter::<Impl, OFFSET>)
    }
}
pub trait ISingleItemExceptionImpl: Sized {
    fn GetItemId();
    fn GetClockVector();
}
impl ::windows::core::RuntimeName for ISingleItemException {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISingleItemException";
}
impl ISingleItemExceptionVtbl {
    pub const fn new<Impl: ISingleItemExceptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISingleItemExceptionVtbl {
        unsafe extern "system" fn GetItemId<Impl: ISingleItemExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemId(pbitemid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClockVector<Impl: ISingleItemExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClockVector(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppunk as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISingleItemException>, base.5, GetItemId::<Impl, OFFSET>, GetClockVector::<Impl, OFFSET>)
    }
}
pub trait ISupportFilteredSyncImpl: Sized {
    fn AddFilter();
}
impl ::windows::core::RuntimeName for ISupportFilteredSync {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISupportFilteredSync";
}
impl ISupportFilteredSyncVtbl {
    pub const fn new<Impl: ISupportFilteredSyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISupportFilteredSyncVtbl {
        unsafe extern "system" fn AddFilter<Impl: ISupportFilteredSyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddFilter(&*(&pfilter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), filteringtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISupportFilteredSync>, base.5, AddFilter::<Impl, OFFSET>)
    }
}
pub trait ISupportLastWriteTimeImpl: Sized {
    fn GetItemChangeTime();
    fn GetChangeUnitChangeTime();
}
impl ::windows::core::RuntimeName for ISupportLastWriteTime {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISupportLastWriteTime";
}
impl ISupportLastWriteTimeVtbl {
    pub const fn new<Impl: ISupportLastWriteTimeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISupportLastWriteTimeVtbl {
        unsafe extern "system" fn GetItemChangeTime<Impl: ISupportLastWriteTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemChangeTime(pbitemid, pulltimestamp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitChangeTime<Impl: ISupportLastWriteTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeUnitChangeTime(pbitemid, pbchangeunitid, pulltimestamp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISupportLastWriteTime>, base.5, GetItemChangeTime::<Impl, OFFSET>, GetChangeUnitChangeTime::<Impl, OFFSET>)
    }
}
pub trait ISyncCallbackImpl: Sized {
    fn OnProgress();
    fn OnChange();
    fn OnConflict();
    fn OnFullEnumerationNeeded();
    fn OnRecoverableError();
}
impl ::windows::core::RuntimeName for ISyncCallback {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncCallback";
}
impl ISyncCallbackVtbl {
    pub const fn new<Impl: ISyncCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncCallbackVtbl {
        unsafe extern "system" fn OnProgress<Impl: ISyncCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnProgress(provider, syncstage, dwcompletedwork, dwtotalwork) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnChange<Impl: ISyncCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnChange(&*(&psyncchange as *const <ISyncChange as ::windows::core::Abi>::Abi as *const <ISyncChange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConflict<Impl: ISyncCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnConflict(&*(&pconflict as *const <IChangeConflict as ::windows::core::Abi>::Abi as *const <IChangeConflict as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnFullEnumerationNeeded<Impl: ISyncCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnFullEnumerationNeeded(pfullenumerationaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnRecoverableError<Impl: ISyncCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precoverableerror: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRecoverableError(&*(&precoverableerror as *const <IRecoverableError as ::windows::core::Abi>::Abi as *const <IRecoverableError as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncCallback>, base.5, OnProgress::<Impl, OFFSET>, OnChange::<Impl, OFFSET>, OnConflict::<Impl, OFFSET>, OnFullEnumerationNeeded::<Impl, OFFSET>, OnRecoverableError::<Impl, OFFSET>)
    }
}
pub trait ISyncCallback2Impl: Sized + ISyncCallbackImpl {
    fn OnChangeApplied();
    fn OnChangeFailed();
}
impl ::windows::core::RuntimeName for ISyncCallback2 {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncCallback2";
}
impl ISyncCallback2Vtbl {
    pub const fn new<Impl: ISyncCallback2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncCallback2Vtbl {
        unsafe extern "system" fn OnChangeApplied<Impl: ISyncCallback2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnChangeApplied(dwchangesapplied, dwchangesfailed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnChangeFailed<Impl: ISyncCallback2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnChangeFailed(dwchangesapplied, dwchangesfailed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncCallback2>, base.5, OnChangeApplied::<Impl, OFFSET>, OnChangeFailed::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for ISyncChange {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChange";
}
impl ISyncChangeVtbl {
    pub const fn new<Impl: ISyncChangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeVtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwnerReplicaId(pbreplicaid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootItemId<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRootItemId(pbrootitemid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeVersion<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeVersion(pbcurrentreplicaid, &*(&pversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCreationVersion<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCreationVersion(pbcurrentreplicaid, &*(&pversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlags(pdwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWorkEstimate<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwwork: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWorkEstimate(pdwwork) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnits<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeUnits(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMadeWithKnowledge<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmadewithknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMadeWithKnowledge(::core::mem::transmute_copy(&ppmadewithknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledge(::core::mem::transmute_copy(&pplearnedknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkEstimate<Impl: ISyncChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetWorkEstimate(dwwork) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncChange>, base.5, GetOwnerReplicaId::<Impl, OFFSET>, GetRootItemId::<Impl, OFFSET>, GetChangeVersion::<Impl, OFFSET>, GetCreationVersion::<Impl, OFFSET>, GetFlags::<Impl, OFFSET>, GetWorkEstimate::<Impl, OFFSET>, GetChangeUnits::<Impl, OFFSET>, GetMadeWithKnowledge::<Impl, OFFSET>, GetLearnedKnowledge::<Impl, OFFSET>, SetWorkEstimate::<Impl, OFFSET>)
    }
}
pub trait ISyncChangeBatchImpl: Sized + ISyncChangeBatchBaseImpl {
    fn BeginUnorderedGroup();
    fn EndUnorderedGroup();
    fn AddLoggedConflict();
}
impl ::windows::core::RuntimeName for ISyncChangeBatch {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeBatch";
}
impl ISyncChangeBatchVtbl {
    pub const fn new<Impl: ISyncChangeBatchImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeBatchVtbl {
        unsafe extern "system" fn BeginUnorderedGroup<Impl: ISyncChangeBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginUnorderedGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnorderedGroup<Impl: ISyncChangeBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmadewithknowledge: ::windows::core::RawPtr, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndUnorderedGroup(&*(&pmadewithknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&fallchangesforknowledge as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggedConflict<Impl: ISyncChangeBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddLoggedConflict(
                pbownerreplicaid,
                pbitemid,
                &*(&pchangeversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType),
                &*(&pcreationversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                dwworkforchange,
                &*(&pconflictknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppchangebuilder),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncChangeBatch>, base.5, BeginUnorderedGroup::<Impl, OFFSET>, EndUnorderedGroup::<Impl, OFFSET>, AddLoggedConflict::<Impl, OFFSET>)
    }
}
pub trait ISyncChangeBatch2Impl: Sized + ISyncChangeBatchImpl + ISyncChangeBatchBaseImpl {
    fn AddMergeTombstoneMetadataToGroup();
    fn AddMergeTombstoneLoggedConflict();
}
impl ::windows::core::RuntimeName for ISyncChangeBatch2 {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeBatch2";
}
impl ISyncChangeBatch2Vtbl {
    pub const fn new<Impl: ISyncChangeBatch2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeBatch2Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Impl: ISyncChangeBatch2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddMergeTombstoneMetadataToGroup(pbownerreplicaid, pbwinneritemid, pbitemid, &*(&pchangeversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType), &*(&pcreationversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType), dwworkforchange, ::core::mem::transmute_copy(&ppchangebuilder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMergeTombstoneLoggedConflict<Impl: ISyncChangeBatch2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: ::windows::core::RawPtr, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddMergeTombstoneLoggedConflict(
                pbownerreplicaid,
                pbwinneritemid,
                pbitemid,
                &*(&pchangeversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType),
                &*(&pcreationversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType),
                dwworkforchange,
                &*(&pconflictknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppchangebuilder),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncChangeBatch2>, base.5, AddMergeTombstoneMetadataToGroup::<Impl, OFFSET>, AddMergeTombstoneLoggedConflict::<Impl, OFFSET>)
    }
}
pub trait ISyncChangeBatchAdvancedImpl: Sized {
    fn GetFilterInfo();
    fn ConvertFullEnumerationChangeBatchToRegularChangeBatch();
    fn GetUpperBoundItemId();
    fn GetBatchLevelKnowledgeShouldBeApplied();
}
impl ::windows::core::RuntimeName for ISyncChangeBatchAdvanced {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeBatchAdvanced";
}
impl ISyncChangeBatchAdvancedVtbl {
    pub const fn new<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeBatchAdvancedVtbl {
        unsafe extern "system" fn GetFilterInfo<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfilterinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilterInfo(::core::mem::transmute_copy(&ppfilterinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFullEnumerationChangeBatchToRegularChangeBatch<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppchangebatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertFullEnumerationChangeBatchToRegularChangeBatch(::core::mem::transmute_copy(&ppchangebatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperBoundItemId<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUpperBoundItemId(pbitemid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBatchLevelKnowledgeShouldBeApplied<Impl: ISyncChangeBatchAdvancedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBatchLevelKnowledgeShouldBeApplied(&*(&pfbatchknowledgeshouldbeapplied as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncChangeBatchAdvanced>, base.5, GetFilterInfo::<Impl, OFFSET>, ConvertFullEnumerationChangeBatchToRegularChangeBatch::<Impl, OFFSET>, GetUpperBoundItemId::<Impl, OFFSET>, GetBatchLevelKnowledgeShouldBeApplied::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ISyncChangeBatchBase {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeBatchBase";
}
impl ISyncChangeBatchBaseVtbl {
    pub const fn new<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeBatchBaseVtbl {
        unsafe extern "system" fn GetChangeEnumerator<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeEnumerator(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsLastBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsLastBatch(&*(&pflastbatch as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWorkEstimateForBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwworkforbatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWorkEstimateForBatch(pdwworkforbatch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemainingWorkEstimateForSession<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRemainingWorkEstimateForSession(pdwremainingworkforsession) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginOrderedGroup<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblowerbound: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginOrderedGroup(pblowerbound) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOrderedGroup<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndOrderedGroup(pbupperbound, &*(&pmadewithknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItemMetadataToGroup<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddItemMetadataToGroup(pbownerreplicaid, pbitemid, &*(&pchangeversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType), &*(&pcreationversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType), dwflags, dwworkforchange, ::core::mem::transmute_copy(&ppchangebuilder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledge(::core::mem::transmute_copy(&pplearnedknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisiteKnowledge<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprerequisteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrerequisiteKnowledge(::core::mem::transmute_copy(&ppprerequisteknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceForgottenKnowledge<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsourceforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceForgottenKnowledge(::core::mem::transmute_copy(&ppsourceforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLastBatch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkEstimateForBatch<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwworkforbatch: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetWorkEstimateForBatch(dwworkforbatch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemainingWorkEstimateForSession<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwremainingworkforsession: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRemainingWorkEstimateForSession(dwremainingworkforsession) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ISyncChangeBatchBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Serialize(pbchangebatch, pcbchangebatch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISyncChangeBatchBase>,
            base.5,
            GetChangeEnumerator::<Impl, OFFSET>,
            GetIsLastBatch::<Impl, OFFSET>,
            GetWorkEstimateForBatch::<Impl, OFFSET>,
            GetRemainingWorkEstimateForSession::<Impl, OFFSET>,
            BeginOrderedGroup::<Impl, OFFSET>,
            EndOrderedGroup::<Impl, OFFSET>,
            AddItemMetadataToGroup::<Impl, OFFSET>,
            GetLearnedKnowledge::<Impl, OFFSET>,
            GetPrerequisiteKnowledge::<Impl, OFFSET>,
            GetSourceForgottenKnowledge::<Impl, OFFSET>,
            SetLastBatch::<Impl, OFFSET>,
            SetWorkEstimateForBatch::<Impl, OFFSET>,
            SetRemainingWorkEstimateForSession::<Impl, OFFSET>,
            Serialize::<Impl, OFFSET>,
        )
    }
}
pub trait ISyncChangeBatchBase2Impl: Sized + ISyncChangeBatchBaseImpl {
    fn SerializeWithOptions();
}
impl ::windows::core::RuntimeName for ISyncChangeBatchBase2 {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeBatchBase2";
}
impl ISyncChangeBatchBase2Vtbl {
    pub const fn new<Impl: ISyncChangeBatchBase2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeBatchBase2Vtbl {
        unsafe extern "system" fn SerializeWithOptions<Impl: ISyncChangeBatchBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SerializeWithOptions(targetformatversion, dwflags, pbbuffer, pdwserializedsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncChangeBatchBase2>, base.5, SerializeWithOptions::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for ISyncChangeBatchWithFilterKeyMap {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeBatchWithFilterKeyMap";
}
impl ISyncChangeBatchWithFilterKeyMapVtbl {
    pub const fn new<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeBatchWithFilterKeyMapVtbl {
        unsafe extern "system" fn GetFilterKeyMap<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppifilterkeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilterKeyMap(::core::mem::transmute_copy(&ppifilterkeymap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterKeyMap<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifilterkeymap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFilterKeyMap(&*(&pifilterkeymap as *const <IFilterKeyMap as ::windows::core::Abi>::Abi as *const <IFilterKeyMap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFilterForgottenKnowledge(dwfilterkey, &*(&pfilterforgottenknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedKnowledge(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplearnedforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledge(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), dwfilterkey, ::core::mem::transmute_copy(&pplearnedfilterforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledge(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplearnedforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplearnedforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeBatchWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), dwfilterkey, ::core::mem::transmute_copy(&pplearnedfilterforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISyncChangeBatchWithFilterKeyMap>,
            base.5,
            GetFilterKeyMap::<Impl, OFFSET>,
            SetFilterKeyMap::<Impl, OFFSET>,
            SetFilterForgottenKnowledge::<Impl, OFFSET>,
            GetFilteredReplicaLearnedKnowledge::<Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledge::<Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge::<Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Impl, OFFSET>,
        )
    }
}
pub trait ISyncChangeBatchWithPrerequisiteImpl: Sized + ISyncChangeBatchBaseImpl {
    fn SetPrerequisiteKnowledge();
    fn GetLearnedKnowledgeWithPrerequisite();
    fn GetLearnedForgottenKnowledge();
}
impl ::windows::core::RuntimeName for ISyncChangeBatchWithPrerequisite {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeBatchWithPrerequisite";
}
impl ISyncChangeBatchWithPrerequisiteVtbl {
    pub const fn new<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeBatchWithPrerequisiteVtbl {
        unsafe extern "system" fn SetPrerequisiteKnowledge<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPrerequisiteKnowledge(&*(&pprerequisiteknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedwithprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeWithPrerequisite(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplearnedwithprerequisiteknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Impl: ISyncChangeBatchWithPrerequisiteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedForgottenKnowledge(::core::mem::transmute_copy(&pplearnedforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncChangeBatchWithPrerequisite>, base.5, SetPrerequisiteKnowledge::<Impl, OFFSET>, GetLearnedKnowledgeWithPrerequisite::<Impl, OFFSET>, GetLearnedForgottenKnowledge::<Impl, OFFSET>)
    }
}
pub trait ISyncChangeBuilderImpl: Sized {
    fn AddChangeUnitMetadata();
}
impl ::windows::core::RuntimeName for ISyncChangeBuilder {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeBuilder";
}
impl ISyncChangeBuilderVtbl {
    pub const fn new<Impl: ISyncChangeBuilderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeBuilderVtbl {
        unsafe extern "system" fn AddChangeUnitMetadata<Impl: ISyncChangeBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddChangeUnitMetadata(pbchangeunitid, &*(&pchangeunitversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncChangeBuilder>, base.5, AddChangeUnitMetadata::<Impl, OFFSET>)
    }
}
pub trait ISyncChangeUnitImpl: Sized {
    fn GetItemChange();
    fn GetChangeUnitId();
    fn GetChangeUnitVersion();
}
impl ::windows::core::RuntimeName for ISyncChangeUnit {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeUnit";
}
impl ISyncChangeUnitVtbl {
    pub const fn new<Impl: ISyncChangeUnitImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeUnitVtbl {
        unsafe extern "system" fn GetItemChange<Impl: ISyncChangeUnitImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemChange(::core::mem::transmute_copy(&ppsyncchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitId<Impl: ISyncChangeUnitImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeUnitId(pbchangeunitid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitVersion<Impl: ISyncChangeUnitImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeUnitVersion(pbcurrentreplicaid, &*(&pversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncChangeUnit>, base.5, GetItemChange::<Impl, OFFSET>, GetChangeUnitId::<Impl, OFFSET>, GetChangeUnitVersion::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ISyncChangeWithFilterKeyMap {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeWithFilterKeyMap";
}
impl ISyncChangeWithFilterKeyMapVtbl {
    pub const fn new<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeWithFilterKeyMapVtbl {
        unsafe extern "system" fn GetFilterCount<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilterCount(pdwfiltercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterChange<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilterChange(dwfilterkey, &*(&pfilterchange as *const <SYNC_FILTER_CHANGE as ::windows::core::Abi>::Abi as *const <SYNC_FILTER_CHANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllChangeUnitsPresentFlag<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllChangeUnitsPresentFlag(&*(&pfallchangeunitspresent as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilterForgottenKnowledge(dwfilterkey, ::core::mem::transmute_copy(&ppifilterforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedKnowledge(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplearnedknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledge(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), dwfilterkey, ::core::mem::transmute_copy(&pplearnedfilterforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledge(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplearnedforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplearnedforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Impl: ISyncChangeWithFilterKeyMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pnewmoveins: ::windows::core::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&pnewmoveins as *const <IEnumItemIds as ::windows::core::Abi>::Abi as *const <IEnumItemIds as ::windows::core::DefaultType>::DefaultType), dwfilterkey, ::core::mem::transmute_copy(&pplearnedfilterforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISyncChangeWithFilterKeyMap>,
            base.5,
            GetFilterCount::<Impl, OFFSET>,
            GetFilterChange::<Impl, OFFSET>,
            GetAllChangeUnitsPresentFlag::<Impl, OFFSET>,
            GetFilterForgottenKnowledge::<Impl, OFFSET>,
            GetFilteredReplicaLearnedKnowledge::<Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledge::<Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge::<Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Impl, OFFSET>,
        )
    }
}
pub trait ISyncChangeWithPrerequisiteImpl: Sized {
    fn GetPrerequisiteKnowledge();
    fn GetLearnedKnowledgeWithPrerequisite();
}
impl ::windows::core::RuntimeName for ISyncChangeWithPrerequisite {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncChangeWithPrerequisite";
}
impl ISyncChangeWithPrerequisiteVtbl {
    pub const fn new<Impl: ISyncChangeWithPrerequisiteImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncChangeWithPrerequisiteVtbl {
        unsafe extern "system" fn GetPrerequisiteKnowledge<Impl: ISyncChangeWithPrerequisiteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprerequisiteknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrerequisiteKnowledge(::core::mem::transmute_copy(&ppprerequisiteknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Impl: ISyncChangeWithPrerequisiteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: ::windows::core::RawPtr, pplearnedknowledgewithprerequisite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeWithPrerequisite(&*(&pdestinationknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplearnedknowledgewithprerequisite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncChangeWithPrerequisite>, base.5, GetPrerequisiteKnowledge::<Impl, OFFSET>, GetLearnedKnowledgeWithPrerequisite::<Impl, OFFSET>)
    }
}
pub trait ISyncConstraintCallbackImpl: Sized {
    fn OnConstraintConflict();
}
impl ::windows::core::RuntimeName for ISyncConstraintCallback {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncConstraintCallback";
}
impl ISyncConstraintCallbackVtbl {
    pub const fn new<Impl: ISyncConstraintCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncConstraintCallbackVtbl {
        unsafe extern "system" fn OnConstraintConflict<Impl: ISyncConstraintCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconflict: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnConstraintConflict(&*(&pconflict as *const <IConstraintConflict as ::windows::core::Abi>::Abi as *const <IConstraintConflict as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncConstraintCallback>, base.5, OnConstraintConflict::<Impl, OFFSET>)
    }
}
pub trait ISyncDataConverterImpl: Sized {
    fn ConvertDataRetrieverFromProviderFormat();
    fn ConvertDataRetrieverToProviderFormat();
    fn ConvertDataFromProviderFormat();
    fn ConvertDataToProviderFormat();
}
impl ::windows::core::RuntimeName for ISyncDataConverter {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncDataConverter";
}
impl ISyncDataConverterVtbl {
    pub const fn new<Impl: ISyncDataConverterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncDataConverterVtbl {
        unsafe extern "system" fn ConvertDataRetrieverFromProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertDataRetrieverFromProviderFormat(&*(&punkdataretrieverin as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&penumsyncchanges as *const <IEnumSyncChanges as ::windows::core::Abi>::Abi as *const <IEnumSyncChanges as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunkdataout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataRetrieverToProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: ::windows::core::RawPtr, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertDataRetrieverToProviderFormat(&*(&punkdataretrieverin as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&penumsyncchanges as *const <IEnumSyncChanges as ::windows::core::Abi>::Abi as *const <IEnumSyncChanges as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunkdataout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataFromProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertDataFromProviderFormat(&*(&pdatacontext as *const <ILoadChangeContext as ::windows::core::Abi>::Abi as *const <ILoadChangeContext as ::windows::core::DefaultType>::DefaultType), &*(&punkdatain as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunkdataout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataToProviderFormat<Impl: ISyncDataConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatacontext: ::windows::core::RawPtr, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertDataToProviderFormat(&*(&pdatacontext as *const <ILoadChangeContext as ::windows::core::Abi>::Abi as *const <ILoadChangeContext as ::windows::core::DefaultType>::DefaultType), &*(&punkdataout as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunkdataout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncDataConverter>, base.5, ConvertDataRetrieverFromProviderFormat::<Impl, OFFSET>, ConvertDataRetrieverToProviderFormat::<Impl, OFFSET>, ConvertDataFromProviderFormat::<Impl, OFFSET>, ConvertDataToProviderFormat::<Impl, OFFSET>)
    }
}
pub trait ISyncFilterImpl: Sized {
    fn IsIdentical();
    fn Serialize();
}
impl ::windows::core::RuntimeName for ISyncFilter {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncFilter";
}
impl ISyncFilterVtbl {
    pub const fn new<Impl: ISyncFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncFilterVtbl {
        unsafe extern "system" fn IsIdentical<Impl: ISyncFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsIdentical(&*(&psyncfilter as *const <ISyncFilter as ::windows::core::Abi>::Abi as *const <ISyncFilter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ISyncFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Serialize(pbsyncfilter, pcbsyncfilter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncFilter>, base.5, IsIdentical::<Impl, OFFSET>, Serialize::<Impl, OFFSET>)
    }
}
pub trait ISyncFilterDeserializerImpl: Sized {
    fn DeserializeSyncFilter();
}
impl ::windows::core::RuntimeName for ISyncFilterDeserializer {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncFilterDeserializer";
}
impl ISyncFilterDeserializerVtbl {
    pub const fn new<Impl: ISyncFilterDeserializerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncFilterDeserializerVtbl {
        unsafe extern "system" fn DeserializeSyncFilter<Impl: ISyncFilterDeserializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeserializeSyncFilter(pbsyncfilter, dwcbsyncfilter, ::core::mem::transmute_copy(&ppisyncfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncFilterDeserializer>, base.5, DeserializeSyncFilter::<Impl, OFFSET>)
    }
}
pub trait ISyncFilterInfoImpl: Sized {
    fn Serialize();
}
impl ::windows::core::RuntimeName for ISyncFilterInfo {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncFilterInfo";
}
impl ISyncFilterInfoVtbl {
    pub const fn new<Impl: ISyncFilterInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncFilterInfoVtbl {
        unsafe extern "system" fn Serialize<Impl: ISyncFilterInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Serialize(pbbuffer, pcbbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncFilterInfo>, base.5, Serialize::<Impl, OFFSET>)
    }
}
pub trait ISyncFilterInfo2Impl: Sized + ISyncFilterInfoImpl {
    fn GetFlags();
}
impl ::windows::core::RuntimeName for ISyncFilterInfo2 {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncFilterInfo2";
}
impl ISyncFilterInfo2Vtbl {
    pub const fn new<Impl: ISyncFilterInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncFilterInfo2Vtbl {
        unsafe extern "system" fn GetFlags<Impl: ISyncFilterInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlags(pdwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncFilterInfo2>, base.5, GetFlags::<Impl, OFFSET>)
    }
}
pub trait ISyncFullEnumerationChangeImpl: Sized {
    fn GetLearnedKnowledgeAfterRecoveryComplete();
    fn GetLearnedForgottenKnowledge();
}
impl ::windows::core::RuntimeName for ISyncFullEnumerationChange {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncFullEnumerationChange";
}
impl ISyncFullEnumerationChangeVtbl {
    pub const fn new<Impl: ISyncFullEnumerationChangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncFullEnumerationChangeVtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Impl: ISyncFullEnumerationChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeAfterRecoveryComplete(::core::mem::transmute_copy(&pplearnedknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Impl: ISyncFullEnumerationChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedForgottenKnowledge(::core::mem::transmute_copy(&pplearnedforgottenknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncFullEnumerationChange>, base.5, GetLearnedKnowledgeAfterRecoveryComplete::<Impl, OFFSET>, GetLearnedForgottenKnowledge::<Impl, OFFSET>)
    }
}
pub trait ISyncFullEnumerationChangeBatchImpl: Sized + ISyncChangeBatchBaseImpl {
    fn GetLearnedKnowledgeAfterRecoveryComplete();
    fn GetClosedLowerBoundItemId();
    fn GetClosedUpperBoundItemId();
}
impl ::windows::core::RuntimeName for ISyncFullEnumerationChangeBatch {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncFullEnumerationChangeBatch";
}
impl ISyncFullEnumerationChangeBatchVtbl {
    pub const fn new<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncFullEnumerationChangeBatchVtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLearnedKnowledgeAfterRecoveryComplete(::core::mem::transmute_copy(&pplearnedknowledgeafterrecoverycomplete)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosedLowerBoundItemId<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClosedLowerBoundItemId(pbclosedlowerbounditemid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosedUpperBoundItemId<Impl: ISyncFullEnumerationChangeBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClosedUpperBoundItemId(pbclosedupperbounditemid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncFullEnumerationChangeBatch>, base.5, GetLearnedKnowledgeAfterRecoveryComplete::<Impl, OFFSET>, GetClosedLowerBoundItemId::<Impl, OFFSET>, GetClosedUpperBoundItemId::<Impl, OFFSET>)
    }
}
pub trait ISyncFullEnumerationChangeBatch2Impl: Sized + ISyncFullEnumerationChangeBatchImpl + ISyncChangeBatchBaseImpl {
    fn AddMergeTombstoneMetadataToGroup();
}
impl ::windows::core::RuntimeName for ISyncFullEnumerationChangeBatch2 {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncFullEnumerationChangeBatch2";
}
impl ISyncFullEnumerationChangeBatch2Vtbl {
    pub const fn new<Impl: ISyncFullEnumerationChangeBatch2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncFullEnumerationChangeBatch2Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Impl: ISyncFullEnumerationChangeBatch2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddMergeTombstoneMetadataToGroup(pbownerreplicaid, pbwinneritemid, pbitemid, &*(&pchangeversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType), &*(&pcreationversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType), dwworkforchange, ::core::mem::transmute_copy(&ppchangebuilder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncFullEnumerationChangeBatch2>, base.5, AddMergeTombstoneMetadataToGroup::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ISyncKnowledge {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncKnowledge";
}
impl ISyncKnowledgeVtbl {
    pub const fn new<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncKnowledgeVtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOwnerReplicaId(pbreplicaid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Serialize(&*(&fserializereplicakeymap as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), pbknowledge, pcbknowledge) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalTickCount<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulltickcount: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLocalTickCount(ulltickcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsChange<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainsChange(pbversionownerreplicaid, pgiditemid, &*(&psyncversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainsChangeUnit(pbversionownerreplicaid, pbitemid, pbchangeunitid, &*(&psyncversion as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopeVector<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScopeVector(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppunk as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReplicaKeyMap<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreplicakeymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReplicaKeyMap(::core::mem::transmute_copy(&ppreplicakeymap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclonedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppclonedknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertVersion<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledgein: ::windows::core::RawPtr, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConvertVersion(&*(&pknowledgein as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), pbcurrentownerid, &*(&pversionin as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType), pbnewownerid, pcbidsize, &*(&pversionout as *const <SYNC_VERSION as ::windows::core::Abi>::Abi as *const <SYNC_VERSION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapRemoteToLocal<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, premoteknowledge: ::windows::core::RawPtr, ppmappedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapRemoteToLocal(&*(&premoteknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmappedknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Union<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Union(&*(&pknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoItem<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectOntoItem(pbitemid, ::core::mem::transmute_copy(&ppknowledgeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectOntoChangeUnit(pbitemid, pbchangeunitid, ::core::mem::transmute_copy(&ppknowledgeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoRange<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectOntoRange(&*(&psrngsyncrange as *const <SYNC_RANGE as ::windows::core::Abi>::Abi as *const <SYNC_RANGE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppknowledgeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcludeItem<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExcludeItem(pbitemid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcludeChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExcludeChangeUnit(pbitemid, pbchangeunitid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsKnowledge<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainsKnowledge(&*(&pknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindMinTickCountForReplica<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindMinTickCountForReplica(pbreplicaid, pullreplicatickcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeExceptions<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRangeExceptions(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppunk as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSingleItemExceptions<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSingleItemExceptions(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppunk as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitExceptions<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChangeUnitExceptions(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppunk as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClockVectorForItem<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindClockVectorForItem(pbitemid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppunk as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClockVectorForChangeUnit<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindClockVectorForChangeUnit(pbitemid, pbchangeunitid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppunk as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: ISyncKnowledgeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVersion(pdwversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISyncKnowledge>,
            base.5,
            GetOwnerReplicaId::<Impl, OFFSET>,
            Serialize::<Impl, OFFSET>,
            SetLocalTickCount::<Impl, OFFSET>,
            ContainsChange::<Impl, OFFSET>,
            ContainsChangeUnit::<Impl, OFFSET>,
            GetScopeVector::<Impl, OFFSET>,
            GetReplicaKeyMap::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            ConvertVersion::<Impl, OFFSET>,
            MapRemoteToLocal::<Impl, OFFSET>,
            Union::<Impl, OFFSET>,
            ProjectOntoItem::<Impl, OFFSET>,
            ProjectOntoChangeUnit::<Impl, OFFSET>,
            ProjectOntoRange::<Impl, OFFSET>,
            ExcludeItem::<Impl, OFFSET>,
            ExcludeChangeUnit::<Impl, OFFSET>,
            ContainsKnowledge::<Impl, OFFSET>,
            FindMinTickCountForReplica::<Impl, OFFSET>,
            GetRangeExceptions::<Impl, OFFSET>,
            GetSingleItemExceptions::<Impl, OFFSET>,
            GetChangeUnitExceptions::<Impl, OFFSET>,
            FindClockVectorForItem::<Impl, OFFSET>,
            FindClockVectorForChangeUnit::<Impl, OFFSET>,
            GetVersion::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for ISyncKnowledge2 {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncKnowledge2";
}
impl ISyncKnowledge2Vtbl {
    pub const fn new<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncKnowledge2Vtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIdParameters(&*(&pidparameters as *const <ID_PARAMETERS as ::windows::core::Abi>::Abi as *const <ID_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoColumnSet<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectOntoColumnSet(ppcolumns, count, ::core::mem::transmute_copy(&ppiknowledgeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerializeWithOptions<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SerializeWithOptions(targetformatversion, dwflags, pbbuffer, pdwserializedsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLowestUncontainedId<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisyncknowledge: ::windows::core::RawPtr, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLowestUncontainedId(&*(&pisyncknowledge as *const <ISyncKnowledge2 as ::windows::core::Abi>::Abi as *const <ISyncKnowledge2 as ::windows::core::DefaultType>::DefaultType), pbitemid, pcbitemidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInspector<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInspector(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppiinspector as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinimumSupportedVersion<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMinimumSupportedVersion(pversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatistics(which, pvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsKnowledgeForItem<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainsKnowledgeForItem(&*(&pknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), pbitemid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsKnowledgeForChangeUnit<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: ::windows::core::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainsKnowledgeForChangeUnit(&*(&pknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), pbitemid, pbchangeunitid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoKnowledgeWithPrerequisite<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: ::windows::core::RawPtr, ptemplateknowledge: ::windows::core::RawPtr, ppprojectedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectOntoKnowledgeWithPrerequisite(&*(&pprerequisiteknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), &*(&ptemplateknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprojectedknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complement<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr, ppcomplementedknowledge: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Complement(&*(&psyncknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcomplementedknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntersectsWithKnowledge<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IntersectsWithKnowledge(&*(&psyncknowledge as *const <ISyncKnowledge as ::windows::core::Abi>::Abi as *const <ISyncKnowledge as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKnowledgeCookie<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKnowledgeCookie(::core::mem::transmute_copy(&ppknowledgecookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareToKnowledgeCookie<Impl: ISyncKnowledge2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompareToKnowledgeCookie(&*(&pknowledgecookie as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), presult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISyncKnowledge2>,
            base.5,
            GetIdParameters::<Impl, OFFSET>,
            ProjectOntoColumnSet::<Impl, OFFSET>,
            SerializeWithOptions::<Impl, OFFSET>,
            GetLowestUncontainedId::<Impl, OFFSET>,
            GetInspector::<Impl, OFFSET>,
            GetMinimumSupportedVersion::<Impl, OFFSET>,
            GetStatistics::<Impl, OFFSET>,
            ContainsKnowledgeForItem::<Impl, OFFSET>,
            ContainsKnowledgeForChangeUnit::<Impl, OFFSET>,
            ProjectOntoKnowledgeWithPrerequisite::<Impl, OFFSET>,
            Complement::<Impl, OFFSET>,
            IntersectsWithKnowledge::<Impl, OFFSET>,
            GetKnowledgeCookie::<Impl, OFFSET>,
            CompareToKnowledgeCookie::<Impl, OFFSET>,
        )
    }
}
pub trait ISyncMergeTombstoneChangeImpl: Sized {
    fn GetWinnerItemId();
}
impl ::windows::core::RuntimeName for ISyncMergeTombstoneChange {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncMergeTombstoneChange";
}
impl ISyncMergeTombstoneChangeVtbl {
    pub const fn new<Impl: ISyncMergeTombstoneChangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncMergeTombstoneChangeVtbl {
        unsafe extern "system" fn GetWinnerItemId<Impl: ISyncMergeTombstoneChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWinnerItemId(pbwinneritemid, pcbidsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncMergeTombstoneChange>, base.5, GetWinnerItemId::<Impl, OFFSET>)
    }
}
pub trait ISyncProviderImpl: Sized {
    fn GetIdParameters();
}
impl ::windows::core::RuntimeName for ISyncProvider {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncProvider";
}
impl ISyncProviderVtbl {
    pub const fn new<Impl: ISyncProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncProviderVtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISyncProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIdParameters(&*(&pidparameters as *const <ID_PARAMETERS as ::windows::core::Abi>::Abi as *const <ID_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncProvider>, base.5, GetIdParameters::<Impl, OFFSET>)
    }
}
pub trait ISyncProviderConfigUIImpl: Sized {
    fn Init();
    fn GetRegisteredProperties();
    fn CreateAndRegisterNewSyncProvider();
    fn ModifySyncProvider();
}
impl ::windows::core::RuntimeName for ISyncProviderConfigUI {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncProviderConfigUI";
}
impl ISyncProviderConfigUIVtbl {
    pub const fn new<Impl: ISyncProviderConfigUIImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncProviderConfigUIVtbl {
        unsafe extern "system" fn Init<Impl: ISyncProviderConfigUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Init(
                &*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pguidcontenttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pconfigurationproperties as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredProperties<Impl: ISyncProviderConfigUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconfiguiproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRegisteredProperties(::core::mem::transmute_copy(&ppconfiguiproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndRegisterNewSyncProvider<Impl: ISyncProviderConfigUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAndRegisterNewSyncProvider(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&punkcontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppproviderinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifySyncProvider<Impl: ISyncProviderConfigUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModifySyncProvider(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&punkcontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pproviderinfo as *const <ISyncProviderInfo as ::windows::core::Abi>::Abi as *const <ISyncProviderInfo as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncProviderConfigUI>, base.5, Init::<Impl, OFFSET>, GetRegisteredProperties::<Impl, OFFSET>, CreateAndRegisterNewSyncProvider::<Impl, OFFSET>, ModifySyncProvider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderConfigUIInfoImpl: Sized + IPropertyStoreImpl {
    fn GetSyncProviderConfigUI();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::RuntimeName for ISyncProviderConfigUIInfo {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncProviderConfigUIInfo";
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderConfigUIInfoVtbl {
    pub const fn new<Impl: ISyncProviderConfigUIInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncProviderConfigUIInfoVtbl {
        unsafe extern "system" fn GetSyncProviderConfigUI<Impl: ISyncProviderConfigUIInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUI(dwclscontext, ::core::mem::transmute_copy(&ppsyncproviderconfigui)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncProviderConfigUIInfo>, base.5, GetSyncProviderConfigUI::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderInfoImpl: Sized + IPropertyStoreImpl {
    fn GetSyncProvider();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::RuntimeName for ISyncProviderInfo {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncProviderInfo";
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderInfoVtbl {
    pub const fn new<Impl: ISyncProviderInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncProviderInfoVtbl {
        unsafe extern "system" fn GetSyncProvider<Impl: ISyncProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncProvider(dwclscontext, ::core::mem::transmute_copy(&ppsyncprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncProviderInfo>, base.5, GetSyncProvider::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ISyncProviderRegistration {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncProviderRegistration";
}
impl ISyncProviderRegistrationVtbl {
    pub const fn new<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncProviderRegistrationVtbl {
        unsafe extern "system" fn CreateSyncProviderConfigUIRegistrationInstance<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSyncProviderConfigUIRegistrationInstance(&*(&pconfiguiconfig as *const <SyncProviderConfigUIConfiguration as ::windows::core::Abi>::Abi as *const <SyncProviderConfigUIConfiguration as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppconfiguiinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProviderConfigUI<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterSyncProviderConfigUI(&*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSyncProviderConfigUIs<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateSyncProviderConfigUIs(&*(&pguidcontenttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwsupportedarchitecture, ::core::mem::transmute_copy(&ppenumsyncproviderconfiguiinfos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSyncProviderRegistrationInstance<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSyncProviderRegistrationInstance(&*(&pproviderconfiguration as *const <SyncProviderConfiguration as ::windows::core::Abi>::Abi as *const <SyncProviderConfiguration as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppproviderinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProvider<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterSyncProvider(&*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfoforProvider<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidproviderinstanceid: *const ::windows::core::GUID, ppproviderconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUIInfoforProvider(&*(&pguidproviderinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppproviderconfiguiinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSyncProviders<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateSyncProviders(&*(&pguidcontenttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwstateflagstofiltermask, dwstateflagstofilter, &*(&refproviderclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwsupportedarchitecture, ::core::mem::transmute_copy(&ppenumsyncproviderinfos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderInfo<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppproviderinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderInfo(&*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppproviderinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderFromInstanceId<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppsyncprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderFromInstanceId(&*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwclscontext, ::core::mem::transmute_copy(&ppsyncprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfo<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppconfiguiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUIInfo(&*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppconfiguiinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIFromInstanceId<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppconfigui: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderConfigUIFromInstanceId(&*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwclscontext, ::core::mem::transmute_copy(&ppconfigui)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderState<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pdwstateflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderState(&*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwstateflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncProviderState<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSyncProviderState(&*(&pguidinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwstateflagsmask, dwstateflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterForEvent<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterForEvent(&*(&phevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeEvent<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevokeEvent(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChange<Impl: ISyncProviderRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChange(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISyncProviderRegistration>,
            base.5,
            CreateSyncProviderConfigUIRegistrationInstance::<Impl, OFFSET>,
            UnregisterSyncProviderConfigUI::<Impl, OFFSET>,
            EnumerateSyncProviderConfigUIs::<Impl, OFFSET>,
            CreateSyncProviderRegistrationInstance::<Impl, OFFSET>,
            UnregisterSyncProvider::<Impl, OFFSET>,
            GetSyncProviderConfigUIInfoforProvider::<Impl, OFFSET>,
            EnumerateSyncProviders::<Impl, OFFSET>,
            GetSyncProviderInfo::<Impl, OFFSET>,
            GetSyncProviderFromInstanceId::<Impl, OFFSET>,
            GetSyncProviderConfigUIInfo::<Impl, OFFSET>,
            GetSyncProviderConfigUIFromInstanceId::<Impl, OFFSET>,
            GetSyncProviderState::<Impl, OFFSET>,
            SetSyncProviderState::<Impl, OFFSET>,
            RegisterForEvent::<Impl, OFFSET>,
            RevokeEvent::<Impl, OFFSET>,
            GetChange::<Impl, OFFSET>,
        )
    }
}
pub trait ISyncRegistrationChangeImpl: Sized {
    fn GetEvent();
    fn GetInstanceId();
}
impl ::windows::core::RuntimeName for ISyncRegistrationChange {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncRegistrationChange";
}
impl ISyncRegistrationChangeVtbl {
    pub const fn new<Impl: ISyncRegistrationChangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncRegistrationChangeVtbl {
        unsafe extern "system" fn GetEvent<Impl: ISyncRegistrationChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEvent(::core::mem::transmute_copy(&psreevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Impl: ISyncRegistrationChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInstanceId(::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncRegistrationChange>, base.5, GetEvent::<Impl, OFFSET>, GetInstanceId::<Impl, OFFSET>)
    }
}
pub trait ISyncSessionExtendedErrorInfoImpl: Sized {
    fn GetSyncProviderWithError();
}
impl ::windows::core::RuntimeName for ISyncSessionExtendedErrorInfo {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncSessionExtendedErrorInfo";
}
impl ISyncSessionExtendedErrorInfoVtbl {
    pub const fn new<Impl: ISyncSessionExtendedErrorInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncSessionExtendedErrorInfoVtbl {
        unsafe extern "system" fn GetSyncProviderWithError<Impl: ISyncSessionExtendedErrorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproviderwitherror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSyncProviderWithError(::core::mem::transmute_copy(&ppproviderwitherror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncSessionExtendedErrorInfo>, base.5, GetSyncProviderWithError::<Impl, OFFSET>)
    }
}
pub trait ISyncSessionStateImpl: Sized {
    fn IsCanceled();
    fn GetInfoForChangeApplication();
    fn LoadInfoFromChangeApplication();
    fn GetForgottenKnowledgeRecoveryRangeStart();
    fn GetForgottenKnowledgeRecoveryRangeEnd();
    fn SetForgottenKnowledgeRecoveryRange();
    fn OnProgress();
}
impl ::windows::core::RuntimeName for ISyncSessionState {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncSessionState";
}
impl ISyncSessionStateVtbl {
    pub const fn new<Impl: ISyncSessionStateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncSessionStateVtbl {
        unsafe extern "system" fn IsCanceled<Impl: ISyncSessionStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCanceled(&*(&pfiscanceled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfoForChangeApplication<Impl: ISyncSessionStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInfoForChangeApplication(pbchangeapplierinfo, pcbchangeapplierinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadInfoFromChangeApplication<Impl: ISyncSessionStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadInfoFromChangeApplication(pbchangeapplierinfo, cbchangeapplierinfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeStart<Impl: ISyncSessionStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForgottenKnowledgeRecoveryRangeStart(pbrangestart, pcbrangestart) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeEnd<Impl: ISyncSessionStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForgottenKnowledgeRecoveryRangeEnd(pbrangeend, pcbrangeend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForgottenKnowledgeRecoveryRange<Impl: ISyncSessionStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *const SYNC_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetForgottenKnowledgeRecoveryRange(&*(&prange as *const <SYNC_RANGE as ::windows::core::Abi>::Abi as *const <SYNC_RANGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProgress<Impl: ISyncSessionStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnProgress(provider, syncstage, dwcompletedwork, dwtotalwork) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncSessionState>, base.5, IsCanceled::<Impl, OFFSET>, GetInfoForChangeApplication::<Impl, OFFSET>, LoadInfoFromChangeApplication::<Impl, OFFSET>, GetForgottenKnowledgeRecoveryRangeStart::<Impl, OFFSET>, GetForgottenKnowledgeRecoveryRangeEnd::<Impl, OFFSET>, SetForgottenKnowledgeRecoveryRange::<Impl, OFFSET>, OnProgress::<Impl, OFFSET>)
    }
}
pub trait ISyncSessionState2Impl: Sized + ISyncSessionStateImpl {
    fn SetProviderWithError();
    fn GetSessionErrorStatus();
}
impl ::windows::core::RuntimeName for ISyncSessionState2 {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISyncSessionState2";
}
impl ISyncSessionState2Vtbl {
    pub const fn new<Impl: ISyncSessionState2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISyncSessionState2Vtbl {
        unsafe extern "system" fn SetProviderWithError<Impl: ISyncSessionState2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fself: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProviderWithError(&*(&fself as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionErrorStatus<Impl: ISyncSessionState2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSessionErrorStatus(phrsessionerror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISyncSessionState2>, base.5, SetProviderWithError::<Impl, OFFSET>, GetSessionErrorStatus::<Impl, OFFSET>)
    }
}
pub trait ISynchronousDataRetrieverImpl: Sized {
    fn GetIdParameters();
    fn LoadChangeData();
}
impl ::windows::core::RuntimeName for ISynchronousDataRetriever {
    const NAME: &'static str = "Windows.Win32.System.WindowsSync.ISynchronousDataRetriever";
}
impl ISynchronousDataRetrieverVtbl {
    pub const fn new<Impl: ISynchronousDataRetrieverImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISynchronousDataRetrieverVtbl {
        unsafe extern "system" fn GetIdParameters<Impl: ISynchronousDataRetrieverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIdParameters(&*(&pidparameters as *const <ID_PARAMETERS as ::windows::core::Abi>::Abi as *const <ID_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadChangeData<Impl: ISynchronousDataRetrieverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploadchangecontext: ::windows::core::RawPtr, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadChangeData(&*(&ploadchangecontext as *const <ILoadChangeContext as ::windows::core::Abi>::Abi as *const <ILoadChangeContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunkdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISynchronousDataRetriever>, base.5, GetIdParameters::<Impl, OFFSET>, LoadChangeData::<Impl, OFFSET>)
    }
}
