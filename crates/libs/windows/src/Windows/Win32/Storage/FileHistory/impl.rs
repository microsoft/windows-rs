#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFhConfigMgr_Impl: Sized {
    fn LoadConfiguration(&self) -> ::windows_core::Result<()>;
    fn CreateDefaultConfiguration(&self, overwriteifexists: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SaveConfiguration(&self) -> ::windows_core::Result<()>;
    fn AddRemoveExcludeRule(&self, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetIncludeExcludeRules(&self, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY) -> ::windows_core::Result<IFhScopeIterator>;
    fn GetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> ::windows_core::Result<u64>;
    fn SetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows_core::Result<()>;
    fn GetBackupStatus(&self) -> ::windows_core::Result<FH_BACKUP_STATUS>;
    fn SetBackupStatus(&self, backupstatus: FH_BACKUP_STATUS) -> ::windows_core::Result<()>;
    fn GetDefaultTarget(&self) -> ::windows_core::Result<IFhTarget>;
    fn ValidateTarget(&self, targeturl: &::windows_core::BSTR) -> ::windows_core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ProvisionAndSetNewTarget(&self, targeturl: &::windows_core::BSTR, targetname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ChangeDefaultTargetRecommendation(&self, recommend: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn QueryProtectionStatus(&self, protectionstate: *mut u32, protecteduntiltime: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFhConfigMgr {}
#[cfg(feature = "Win32_Foundation")]
impl IFhConfigMgr_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>() -> IFhConfigMgr_Vtbl {
        unsafe extern "system" fn LoadConfiguration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadConfiguration().into()
        }
        unsafe extern "system" fn CreateDefaultConfiguration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDefaultConfiguration(::core::mem::transmute_copy(&overwriteifexists)).into()
        }
        unsafe extern "system" fn SaveConfiguration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveConfiguration().into()
        }
        unsafe extern "system" fn AddRemoveExcludeRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRemoveExcludeRule(::core::mem::transmute_copy(&add), ::core::mem::transmute_copy(&category), ::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn GetIncludeExcludeRules<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIncludeExcludeRules(::core::mem::transmute_copy(&include), ::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iterator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalPolicy(::core::mem::transmute_copy(&localpolicytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(policyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalPolicy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocalPolicy(::core::mem::transmute_copy(&localpolicytype), ::core::mem::transmute_copy(&policyvalue)).into()
        }
        unsafe extern "system" fn GetBackupStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backupstatus: *mut FH_BACKUP_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackupStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(backupstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backupstatus: FH_BACKUP_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBackupStatus(::core::mem::transmute_copy(&backupstatus)).into()
        }
        unsafe extern "system" fn GetDefaultTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaulttarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultTarget() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaulttarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::std::mem::MaybeUninit<::windows_core::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValidateTarget(::core::mem::transmute(&targeturl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(validationresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvisionAndSetNewTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProvisionAndSetNewTarget(::core::mem::transmute(&targeturl), ::core::mem::transmute(&targetname)).into()
        }
        unsafe extern "system" fn ChangeDefaultTargetRecommendation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recommend: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeDefaultTargetRecommendation(::core::mem::transmute_copy(&recommend)).into()
        }
        unsafe extern "system" fn QueryProtectionStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protectionstate: *mut u32, protecteduntiltime: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryProtectionStatus(::core::mem::transmute_copy(&protectionstate), ::core::mem::transmute_copy(&protecteduntiltime)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadConfiguration: LoadConfiguration::<Identity, Impl, OFFSET>,
            CreateDefaultConfiguration: CreateDefaultConfiguration::<Identity, Impl, OFFSET>,
            SaveConfiguration: SaveConfiguration::<Identity, Impl, OFFSET>,
            AddRemoveExcludeRule: AddRemoveExcludeRule::<Identity, Impl, OFFSET>,
            GetIncludeExcludeRules: GetIncludeExcludeRules::<Identity, Impl, OFFSET>,
            GetLocalPolicy: GetLocalPolicy::<Identity, Impl, OFFSET>,
            SetLocalPolicy: SetLocalPolicy::<Identity, Impl, OFFSET>,
            GetBackupStatus: GetBackupStatus::<Identity, Impl, OFFSET>,
            SetBackupStatus: SetBackupStatus::<Identity, Impl, OFFSET>,
            GetDefaultTarget: GetDefaultTarget::<Identity, Impl, OFFSET>,
            ValidateTarget: ValidateTarget::<Identity, Impl, OFFSET>,
            ProvisionAndSetNewTarget: ProvisionAndSetNewTarget::<Identity, Impl, OFFSET>,
            ChangeDefaultTargetRecommendation: ChangeDefaultTargetRecommendation::<Identity, Impl, OFFSET>,
            QueryProtectionStatus: QueryProtectionStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFhConfigMgr as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFhReassociation_Impl: Sized {
    fn ValidateTarget(&self, targeturl: &::windows_core::BSTR) -> ::windows_core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ScanTargetForConfigurations(&self, targeturl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetConfigurationDetails(&self, index: u32, username: *mut ::windows_core::BSTR, pcname: *mut ::windows_core::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn SelectConfiguration(&self, index: u32) -> ::windows_core::Result<()>;
    fn PerformReassociation(&self, overwriteifexists: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IFhReassociation {}
#[cfg(feature = "Win32_Foundation")]
impl IFhReassociation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: isize>() -> IFhReassociation_Vtbl {
        unsafe extern "system" fn ValidateTarget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::std::mem::MaybeUninit<::windows_core::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValidateTarget(::core::mem::transmute(&targeturl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(validationresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanTargetForConfigurations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targeturl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScanTargetForConfigurations(::core::mem::transmute(&targeturl)).into()
        }
        unsafe extern "system" fn GetConfigurationDetails<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, username: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pcname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConfigurationDetails(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&pcname), ::core::mem::transmute_copy(&backuptime)).into()
        }
        unsafe extern "system" fn SelectConfiguration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectConfiguration(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn PerformReassociation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PerformReassociation(::core::mem::transmute_copy(&overwriteifexists)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ValidateTarget: ValidateTarget::<Identity, Impl, OFFSET>,
            ScanTargetForConfigurations: ScanTargetForConfigurations::<Identity, Impl, OFFSET>,
            GetConfigurationDetails: GetConfigurationDetails::<Identity, Impl, OFFSET>,
            SelectConfiguration: SelectConfiguration::<Identity, Impl, OFFSET>,
            PerformReassociation: PerformReassociation::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFhReassociation as ::windows_core::ComInterface>::IID
    }
}
pub trait IFhScopeIterator_Impl: Sized {
    fn MoveToNextItem(&self) -> ::windows_core::Result<()>;
    fn GetItem(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::RuntimeName for IFhScopeIterator {}
impl IFhScopeIterator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhScopeIterator_Impl, const OFFSET: isize>() -> IFhScopeIterator_Vtbl {
        unsafe extern "system" fn MoveToNextItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhScopeIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveToNextItem().into()
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhScopeIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveToNextItem: MoveToNextItem::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFhScopeIterator as ::windows_core::ComInterface>::IID
    }
}
pub trait IFhTarget_Impl: Sized {
    fn GetStringProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetNumericalProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows_core::Result<u64>;
}
impl ::windows_core::RuntimeName for IFhTarget {}
impl IFhTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhTarget_Impl, const OFFSET: isize>() -> IFhTarget_Vtbl {
        unsafe extern "system" fn GetStringProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringProperty(::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericalProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFhTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumericalProperty(::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStringProperty: GetStringProperty::<Identity, Impl, OFFSET>,
            GetNumericalProperty: GetNumericalProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFhTarget as ::windows_core::ComInterface>::IID
    }
}
