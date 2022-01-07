#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountPartnerAccountInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Priority(&self) -> ::windows::core::Result<u32>;
    fn AccountKind(&self) -> ::windows::core::Result<UserDataAccountProviderPartnerAccountKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataAccountPartnerAccountInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountPartnerAccountInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataAccountPartnerAccountInfoVtbl {
    pub const fn new<Impl: IUserDataAccountPartnerAccountInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataAccountPartnerAccountInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: IUserDataAccountPartnerAccountInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IUserDataAccountPartnerAccountInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountKind<Impl: IUserDataAccountPartnerAccountInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderPartnerAccountKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccountKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataAccountPartnerAccountInfo>, base.5, DisplayName::<Impl, OFFSET>, Priority::<Impl, OFFSET>, AccountKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountProviderAddAccountOperationImpl: Sized + IUserDataAccountProviderOperationImpl {
    fn ContentKinds(&self) -> ::windows::core::Result<super::UserDataAccountContentKinds>;
    fn PartnerAccountInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>>;
    fn ReportCompleted(&self, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderAddAccountOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataAccountProviderAddAccountOperationVtbl {
    pub const fn new<Impl: IUserDataAccountProviderAddAccountOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataAccountProviderAddAccountOperationVtbl {
        unsafe extern "system" fn ContentKinds<Impl: IUserDataAccountProviderAddAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::UserDataAccountContentKinds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentKinds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartnerAccountInfos<Impl: IUserDataAccountProviderAddAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PartnerAccountInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IUserDataAccountProviderAddAccountOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCompleted(&*(&userdataaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataAccountProviderAddAccountOperation>, base.5, ContentKinds::<Impl, OFFSET>, PartnerAccountInfos::<Impl, OFFSET>, ReportCompleted::<Impl, OFFSET>)
    }
}
pub trait IUserDataAccountProviderOperationImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind>;
}
impl ::windows::core::RuntimeName for IUserDataAccountProviderOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation";
}
impl IUserDataAccountProviderOperationVtbl {
    pub const fn new<Impl: IUserDataAccountProviderOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataAccountProviderOperationVtbl {
        unsafe extern "system" fn Kind<Impl: IUserDataAccountProviderOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderOperationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataAccountProviderOperation>, base.5, Kind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountProviderResolveErrorsOperationImpl: Sized + IUserDataAccountProviderOperationImpl {
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataAccountProviderResolveErrorsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderResolveErrorsOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataAccountProviderResolveErrorsOperationVtbl {
    pub const fn new<Impl: IUserDataAccountProviderResolveErrorsOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataAccountProviderResolveErrorsOperationVtbl {
        unsafe extern "system" fn UserDataAccountId<Impl: IUserDataAccountProviderResolveErrorsOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserDataAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IUserDataAccountProviderResolveErrorsOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataAccountProviderResolveErrorsOperation>, base.5, UserDataAccountId::<Impl, OFFSET>, ReportCompleted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountProviderSettingsOperationImpl: Sized + IUserDataAccountProviderOperationImpl {
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataAccountProviderSettingsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderSettingsOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataAccountProviderSettingsOperationVtbl {
    pub const fn new<Impl: IUserDataAccountProviderSettingsOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDataAccountProviderSettingsOperationVtbl {
        unsafe extern "system" fn UserDataAccountId<Impl: IUserDataAccountProviderSettingsOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserDataAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompleted<Impl: IUserDataAccountProviderSettingsOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCompleted().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDataAccountProviderSettingsOperation>, base.5, UserDataAccountId::<Impl, OFFSET>, ReportCompleted::<Impl, OFFSET>)
    }
}
