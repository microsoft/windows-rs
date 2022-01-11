#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IOnlineIdAuthenticatorImpl: Sized {
    fn AuthenticateUserAsync(&self, request: &::core::option::Option<OnlineIdServiceTicketRequest>) -> ::windows::core::Result<UserAuthenticationOperation>;
    fn AuthenticateUserAsyncAdvanced(&self, requests: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<OnlineIdServiceTicketRequest>>, credentialprompttype: CredentialPromptType) -> ::windows::core::Result<UserAuthenticationOperation>;
    fn SignOutUserAsync(&self) -> ::windows::core::Result<SignOutUserOperation>;
    fn SetApplicationId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CanSignOut(&self) -> ::windows::core::Result<bool>;
    fn AuthenticatedSafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOnlineIdAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.IOnlineIdAuthenticator";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IOnlineIdAuthenticatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOnlineIdAuthenticatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOnlineIdAuthenticatorVtbl {
        unsafe extern "system" fn AuthenticateUserAsync<Impl: IOnlineIdAuthenticatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateUserAsync(&*(&request as *const <OnlineIdServiceTicketRequest as ::windows::core::Abi>::Abi as *const <OnlineIdServiceTicketRequest as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateUserAsyncAdvanced<Impl: IOnlineIdAuthenticatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requests: ::windows::core::RawPtr, credentialprompttype: CredentialPromptType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateUserAsyncAdvanced(&*(&requests as *const <super::super::super::Foundation::Collections::IIterable<OnlineIdServiceTicketRequest> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<OnlineIdServiceTicketRequest> as ::windows::core::DefaultType>::DefaultType), credentialprompttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignOutUserAsync<Impl: IOnlineIdAuthenticatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignOutUserAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationId<Impl: IOnlineIdAuthenticatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ApplicationId<Impl: IOnlineIdAuthenticatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanSignOut<Impl: IOnlineIdAuthenticatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSignOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticatedSafeCustomerId<Impl: IOnlineIdAuthenticatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticatedSafeCustomerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOnlineIdAuthenticator, BASE_OFFSET>(),
            AuthenticateUserAsync: AuthenticateUserAsync::<Impl, IMPL_OFFSET>,
            AuthenticateUserAsyncAdvanced: AuthenticateUserAsyncAdvanced::<Impl, IMPL_OFFSET>,
            SignOutUserAsync: SignOutUserAsync::<Impl, IMPL_OFFSET>,
            SetApplicationId: SetApplicationId::<Impl, IMPL_OFFSET>,
            ApplicationId: ApplicationId::<Impl, IMPL_OFFSET>,
            CanSignOut: CanSignOut::<Impl, IMPL_OFFSET>,
            AuthenticatedSafeCustomerId: AuthenticatedSafeCustomerId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOnlineIdAuthenticator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdServiceTicketImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Request(&self) -> ::windows::core::Result<OnlineIdServiceTicketRequest>;
    fn ErrorCode(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOnlineIdServiceTicket {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicket";
}
#[cfg(feature = "implement_exclusive")]
impl IOnlineIdServiceTicketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOnlineIdServiceTicketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOnlineIdServiceTicketVtbl {
        unsafe extern "system" fn Value<Impl: IOnlineIdServiceTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: IOnlineIdServiceTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IOnlineIdServiceTicketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOnlineIdServiceTicket, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            Request: Request::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOnlineIdServiceTicket as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdServiceTicketRequestImpl: Sized {
    fn Service(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Policy(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOnlineIdServiceTicketRequest {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IOnlineIdServiceTicketRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOnlineIdServiceTicketRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOnlineIdServiceTicketRequestVtbl {
        unsafe extern "system" fn Service<Impl: IOnlineIdServiceTicketRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Service() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Policy<Impl: IOnlineIdServiceTicketRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Policy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOnlineIdServiceTicketRequest, BASE_OFFSET>(),
            Service: Service::<Impl, IMPL_OFFSET>,
            Policy: Policy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOnlineIdServiceTicketRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdServiceTicketRequestFactoryImpl: Sized {
    fn CreateOnlineIdServiceTicketRequest(&self, service: &::windows::core::HSTRING, policy: &::windows::core::HSTRING) -> ::windows::core::Result<OnlineIdServiceTicketRequest>;
    fn CreateOnlineIdServiceTicketRequestAdvanced(&self, service: &::windows::core::HSTRING) -> ::windows::core::Result<OnlineIdServiceTicketRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOnlineIdServiceTicketRequestFactory {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.IOnlineIdServiceTicketRequestFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IOnlineIdServiceTicketRequestFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOnlineIdServiceTicketRequestFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOnlineIdServiceTicketRequestFactoryVtbl {
        unsafe extern "system" fn CreateOnlineIdServiceTicketRequest<Impl: IOnlineIdServiceTicketRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, service: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOnlineIdServiceTicketRequest(&*(&service as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&policy as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOnlineIdServiceTicketRequestAdvanced<Impl: IOnlineIdServiceTicketRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, service: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOnlineIdServiceTicketRequestAdvanced(&*(&service as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOnlineIdServiceTicketRequestFactory, BASE_OFFSET>(),
            CreateOnlineIdServiceTicketRequest: CreateOnlineIdServiceTicketRequest::<Impl, IMPL_OFFSET>,
            CreateOnlineIdServiceTicketRequestAdvanced: CreateOnlineIdServiceTicketRequestAdvanced::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOnlineIdServiceTicketRequestFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IOnlineIdSystemAuthenticatorForUserImpl: Sized {
    fn GetTicketAsync(&self, request: &::core::option::Option<OnlineIdServiceTicketRequest>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>>;
    fn SetApplicationId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn User(&self) -> ::windows::core::Result<super::super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOnlineIdSystemAuthenticatorForUser {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorForUser";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IOnlineIdSystemAuthenticatorForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOnlineIdSystemAuthenticatorForUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOnlineIdSystemAuthenticatorForUserVtbl {
        unsafe extern "system" fn GetTicketAsync<Impl: IOnlineIdSystemAuthenticatorForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTicketAsync(&*(&request as *const <OnlineIdServiceTicketRequest as ::windows::core::Abi>::Abi as *const <OnlineIdServiceTicketRequest as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationId<Impl: IOnlineIdSystemAuthenticatorForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ApplicationId<Impl: IOnlineIdSystemAuthenticatorForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IOnlineIdSystemAuthenticatorForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOnlineIdSystemAuthenticatorForUser, BASE_OFFSET>(),
            GetTicketAsync: GetTicketAsync::<Impl, IMPL_OFFSET>,
            SetApplicationId: SetApplicationId::<Impl, IMPL_OFFSET>,
            ApplicationId: ApplicationId::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOnlineIdSystemAuthenticatorForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IOnlineIdSystemAuthenticatorStaticsImpl: Sized {
    fn Default(&self) -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOnlineIdSystemAuthenticatorStatics {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.IOnlineIdSystemAuthenticatorStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IOnlineIdSystemAuthenticatorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOnlineIdSystemAuthenticatorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOnlineIdSystemAuthenticatorStaticsVtbl {
        unsafe extern "system" fn Default<Impl: IOnlineIdSystemAuthenticatorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Default() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IOnlineIdSystemAuthenticatorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOnlineIdSystemAuthenticatorStatics, BASE_OFFSET>(),
            Default: Default::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOnlineIdSystemAuthenticatorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdSystemIdentityImpl: Sized {
    fn Ticket(&self) -> ::windows::core::Result<OnlineIdServiceTicket>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOnlineIdSystemIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.IOnlineIdSystemIdentity";
}
#[cfg(feature = "implement_exclusive")]
impl IOnlineIdSystemIdentityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOnlineIdSystemIdentityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOnlineIdSystemIdentityVtbl {
        unsafe extern "system" fn Ticket<Impl: IOnlineIdSystemIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ticket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IOnlineIdSystemIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOnlineIdSystemIdentity, BASE_OFFSET>(),
            Ticket: Ticket::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOnlineIdSystemIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdSystemTicketResultImpl: Sized {
    fn Identity(&self) -> ::windows::core::Result<OnlineIdSystemIdentity>;
    fn Status(&self) -> ::windows::core::Result<OnlineIdSystemTicketStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOnlineIdSystemTicketResult {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.IOnlineIdSystemTicketResult";
}
#[cfg(feature = "implement_exclusive")]
impl IOnlineIdSystemTicketResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOnlineIdSystemTicketResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOnlineIdSystemTicketResultVtbl {
        unsafe extern "system" fn Identity<Impl: IOnlineIdSystemTicketResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IOnlineIdSystemTicketResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut OnlineIdSystemTicketStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IOnlineIdSystemTicketResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOnlineIdSystemTicketResult, BASE_OFFSET>(),
            Identity: Identity::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOnlineIdSystemTicketResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserIdentityImpl: Sized {
    fn Tickets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignInName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsBetaAccount(&self) -> ::windows::core::Result<bool>;
    fn IsConfirmedPC(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.IUserIdentity";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserIdentityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserIdentityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserIdentityVtbl {
        unsafe extern "system" fn Tickets<Impl: IUserIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tickets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IUserIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SafeCustomerId<Impl: IUserIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SafeCustomerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignInName<Impl: IUserIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignInName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstName<Impl: IUserIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastName<Impl: IUserIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBetaAccount<Impl: IUserIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBetaAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConfirmedPC<Impl: IUserIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConfirmedPC() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserIdentity, BASE_OFFSET>(),
            Tickets: Tickets::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            SafeCustomerId: SafeCustomerId::<Impl, IMPL_OFFSET>,
            SignInName: SignInName::<Impl, IMPL_OFFSET>,
            FirstName: FirstName::<Impl, IMPL_OFFSET>,
            LastName: LastName::<Impl, IMPL_OFFSET>,
            IsBetaAccount: IsBetaAccount::<Impl, IMPL_OFFSET>,
            IsConfirmedPC: IsConfirmedPC::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserIdentity as ::windows::core::Interface>::IID
    }
}
