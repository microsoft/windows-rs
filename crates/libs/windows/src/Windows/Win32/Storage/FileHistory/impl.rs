#[cfg(feature = "Win32_Foundation")]
pub trait IFhConfigMgrImpl: Sized {
    fn LoadConfiguration();
    fn CreateDefaultConfiguration();
    fn SaveConfiguration();
    fn AddRemoveExcludeRule();
    fn GetIncludeExcludeRules();
    fn GetLocalPolicy();
    fn SetLocalPolicy();
    fn GetBackupStatus();
    fn SetBackupStatus();
    fn GetDefaultTarget();
    fn ValidateTarget();
    fn ProvisionAndSetNewTarget();
    fn ChangeDefaultTargetRecommendation();
    fn QueryProtectionStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IFhConfigMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFhConfigMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFhConfigMgrVtbl {
        unsafe extern "system" fn LoadConfiguration<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDefaultConfiguration<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveConfiguration<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRemoveExcludeRule<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIncludeExcludeRules<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalPolicy<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalPolicy<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackupStatus<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backupstatus: *mut FH_BACKUP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackupStatus<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backupstatus: FH_BACKUP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultTarget<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaulttarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateTarget<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProvisionAndSetNewTarget<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangeDefaultTargetRecommendation<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recommend: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryProtectionStatus<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protectionstate: *mut u32, protecteduntiltime: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            LoadConfiguration::<Impl, IMPL_OFFSET>,
            CreateDefaultConfiguration::<Impl, IMPL_OFFSET>,
            SaveConfiguration::<Impl, IMPL_OFFSET>,
            AddRemoveExcludeRule::<Impl, IMPL_OFFSET>,
            GetIncludeExcludeRules::<Impl, IMPL_OFFSET>,
            GetLocalPolicy::<Impl, IMPL_OFFSET>,
            SetLocalPolicy::<Impl, IMPL_OFFSET>,
            GetBackupStatus::<Impl, IMPL_OFFSET>,
            SetBackupStatus::<Impl, IMPL_OFFSET>,
            GetDefaultTarget::<Impl, IMPL_OFFSET>,
            ValidateTarget::<Impl, IMPL_OFFSET>,
            ProvisionAndSetNewTarget::<Impl, IMPL_OFFSET>,
            ChangeDefaultTargetRecommendation::<Impl, IMPL_OFFSET>,
            QueryProtectionStatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFhConfigMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFhReassociationImpl: Sized {
    fn ValidateTarget();
    fn ScanTargetForConfigurations();
    fn GetConfigurationDetails();
    fn SelectConfiguration();
    fn PerformReassociation();
}
#[cfg(feature = "Win32_Foundation")]
impl IFhReassociationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFhReassociationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFhReassociationVtbl {
        unsafe extern "system" fn ValidateTarget<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScanTargetForConfigurations<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfigurationDetails<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, username: *mut super::super::Foundation::BSTR, pcname: *mut super::super::Foundation::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectConfiguration<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PerformReassociation<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ValidateTarget::<Impl, IMPL_OFFSET>, ScanTargetForConfigurations::<Impl, IMPL_OFFSET>, GetConfigurationDetails::<Impl, IMPL_OFFSET>, SelectConfiguration::<Impl, IMPL_OFFSET>, PerformReassociation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFhReassociation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFhScopeIteratorImpl: Sized {
    fn MoveToNextItem();
    fn GetItem();
}
#[cfg(feature = "Win32_Foundation")]
impl IFhScopeIteratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFhScopeIteratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFhScopeIteratorVtbl {
        unsafe extern "system" fn MoveToNextItem<Impl: IFhScopeIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItem<Impl: IFhScopeIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MoveToNextItem::<Impl, IMPL_OFFSET>, GetItem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFhScopeIterator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFhTargetImpl: Sized {
    fn GetStringProperty();
    fn GetNumericalProperty();
}
#[cfg(feature = "Win32_Foundation")]
impl IFhTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFhTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFhTargetVtbl {
        unsafe extern "system" fn GetStringProperty<Impl: IFhTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumericalProperty<Impl: IFhTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetStringProperty::<Impl, IMPL_OFFSET>, GetNumericalProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFhTarget as ::windows::core::Interface>::IID
    }
}
