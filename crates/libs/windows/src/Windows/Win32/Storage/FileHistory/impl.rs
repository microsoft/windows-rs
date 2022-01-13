#[cfg(feature = "Win32_Foundation")]
pub trait IFhConfigMgrImpl: Sized {
    fn LoadConfiguration(&mut self) -> ::windows::core::Result<()>;
    fn CreateDefaultConfiguration(&mut self, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SaveConfiguration(&mut self) -> ::windows::core::Result<()>;
    fn AddRemoveExcludeRule(&mut self, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetIncludeExcludeRules(&mut self, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY) -> ::windows::core::Result<IFhScopeIterator>;
    fn GetLocalPolicy(&mut self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> ::windows::core::Result<u64>;
    fn SetLocalPolicy(&mut self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows::core::Result<()>;
    fn GetBackupStatus(&mut self) -> ::windows::core::Result<FH_BACKUP_STATUS>;
    fn SetBackupStatus(&mut self, backupstatus: FH_BACKUP_STATUS) -> ::windows::core::Result<()>;
    fn GetDefaultTarget(&mut self) -> ::windows::core::Result<IFhTarget>;
    fn ValidateTarget(&mut self, targeturl: super::super::Foundation::BSTR) -> ::windows::core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ProvisionAndSetNewTarget(&mut self, targeturl: super::super::Foundation::BSTR, targetname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangeDefaultTargetRecommendation(&mut self, recommend: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn QueryProtectionStatus(&mut self, protectionstate: *mut u32, protecteduntiltime: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFhConfigMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFhConfigMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFhConfigMgrVtbl {
        unsafe extern "system" fn LoadConfiguration<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadConfiguration().into()
        }
        unsafe extern "system" fn CreateDefaultConfiguration<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDefaultConfiguration(::core::mem::transmute_copy(&overwriteifexists)).into()
        }
        unsafe extern "system" fn SaveConfiguration<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveConfiguration().into()
        }
        unsafe extern "system" fn AddRemoveExcludeRule<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRemoveExcludeRule(::core::mem::transmute_copy(&add), ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn GetIncludeExcludeRules<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIncludeExcludeRules(::core::mem::transmute_copy(&include), ::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    *iterator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalPolicy<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalPolicy(::core::mem::transmute_copy(&localpolicytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *policyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalPolicy<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalPolicy(::core::mem::transmute_copy(&localpolicytype), ::core::mem::transmute_copy(&policyvalue)).into()
        }
        unsafe extern "system" fn GetBackupStatus<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backupstatus: *mut FH_BACKUP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackupStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *backupstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupStatus<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backupstatus: FH_BACKUP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackupStatus(::core::mem::transmute_copy(&backupstatus)).into()
        }
        unsafe extern "system" fn GetDefaultTarget<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaulttarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *defaulttarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateTarget<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateTarget(::core::mem::transmute_copy(&targeturl)) {
                ::core::result::Result::Ok(ok__) => {
                    *validationresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvisionAndSetNewTarget<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProvisionAndSetNewTarget(::core::mem::transmute_copy(&targeturl), ::core::mem::transmute_copy(&targetname)).into()
        }
        unsafe extern "system" fn ChangeDefaultTargetRecommendation<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recommend: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeDefaultTargetRecommendation(::core::mem::transmute_copy(&recommend)).into()
        }
        unsafe extern "system" fn QueryProtectionStatus<Impl: IFhConfigMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protectionstate: *mut u32, protecteduntiltime: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryProtectionStatus(::core::mem::transmute_copy(&protectionstate), ::core::mem::transmute_copy(&protecteduntiltime)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LoadConfiguration: LoadConfiguration::<Impl, IMPL_OFFSET>,
            CreateDefaultConfiguration: CreateDefaultConfiguration::<Impl, IMPL_OFFSET>,
            SaveConfiguration: SaveConfiguration::<Impl, IMPL_OFFSET>,
            AddRemoveExcludeRule: AddRemoveExcludeRule::<Impl, IMPL_OFFSET>,
            GetIncludeExcludeRules: GetIncludeExcludeRules::<Impl, IMPL_OFFSET>,
            GetLocalPolicy: GetLocalPolicy::<Impl, IMPL_OFFSET>,
            SetLocalPolicy: SetLocalPolicy::<Impl, IMPL_OFFSET>,
            GetBackupStatus: GetBackupStatus::<Impl, IMPL_OFFSET>,
            SetBackupStatus: SetBackupStatus::<Impl, IMPL_OFFSET>,
            GetDefaultTarget: GetDefaultTarget::<Impl, IMPL_OFFSET>,
            ValidateTarget: ValidateTarget::<Impl, IMPL_OFFSET>,
            ProvisionAndSetNewTarget: ProvisionAndSetNewTarget::<Impl, IMPL_OFFSET>,
            ChangeDefaultTargetRecommendation: ChangeDefaultTargetRecommendation::<Impl, IMPL_OFFSET>,
            QueryProtectionStatus: QueryProtectionStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFhConfigMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFhReassociationImpl: Sized {
    fn ValidateTarget(&mut self, targeturl: super::super::Foundation::BSTR) -> ::windows::core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ScanTargetForConfigurations(&mut self, targeturl: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetConfigurationDetails(&mut self, index: u32, username: *mut super::super::Foundation::BSTR, pcname: *mut super::super::Foundation::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn SelectConfiguration(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn PerformReassociation(&mut self, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFhReassociationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFhReassociationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFhReassociationVtbl {
        unsafe extern "system" fn ValidateTarget<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateTarget(::core::mem::transmute_copy(&targeturl)) {
                ::core::result::Result::Ok(ok__) => {
                    *validationresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanTargetForConfigurations<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScanTargetForConfigurations(::core::mem::transmute_copy(&targeturl)).into()
        }
        unsafe extern "system" fn GetConfigurationDetails<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, username: *mut super::super::Foundation::BSTR, pcname: *mut super::super::Foundation::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConfigurationDetails(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&pcname), ::core::mem::transmute_copy(&backuptime)).into()
        }
        unsafe extern "system" fn SelectConfiguration<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectConfiguration(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn PerformReassociation<Impl: IFhReassociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PerformReassociation(::core::mem::transmute_copy(&overwriteifexists)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ValidateTarget: ValidateTarget::<Impl, IMPL_OFFSET>,
            ScanTargetForConfigurations: ScanTargetForConfigurations::<Impl, IMPL_OFFSET>,
            GetConfigurationDetails: GetConfigurationDetails::<Impl, IMPL_OFFSET>,
            SelectConfiguration: SelectConfiguration::<Impl, IMPL_OFFSET>,
            PerformReassociation: PerformReassociation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFhReassociation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFhScopeIteratorImpl: Sized {
    fn MoveToNextItem(&mut self) -> ::windows::core::Result<()>;
    fn GetItem(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFhScopeIteratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFhScopeIteratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFhScopeIteratorVtbl {
        unsafe extern "system" fn MoveToNextItem<Impl: IFhScopeIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveToNextItem().into()
        }
        unsafe extern "system" fn GetItem<Impl: IFhScopeIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem() {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MoveToNextItem: MoveToNextItem::<Impl, IMPL_OFFSET>,
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFhScopeIterator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFhTargetImpl: Sized {
    fn GetStringProperty(&mut self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetNumericalProperty(&mut self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFhTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFhTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFhTargetVtbl {
        unsafe extern "system" fn GetStringProperty<Impl: IFhTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringProperty(::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericalProperty<Impl: IFhTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericalProperty(::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStringProperty: GetStringProperty::<Impl, IMPL_OFFSET>,
            GetNumericalProperty: GetNumericalProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFhTarget as ::windows::core::Interface>::IID
    }
}
