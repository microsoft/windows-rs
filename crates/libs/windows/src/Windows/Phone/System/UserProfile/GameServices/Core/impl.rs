#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGameService_Impl: Sized {
    fn ServiceUri(&mut self) -> ::windows::core::Result<super::super::super::super::super::Foundation::Uri>;
    fn GetGamerProfileAsync(&mut self) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>;
    fn GetInstalledGameItemsAsync(&mut self) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>;
    fn GetPartnerTokenAsync(&mut self, audienceuri: &::core::option::Option<super::super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetPrivilegesAsync(&mut self) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GrantAchievement(&mut self, achievementid: u32) -> ::windows::core::Result<()>;
    fn GrantAvatarAward(&mut self, avatarawardid: u32) -> ::windows::core::Result<()>;
    fn PostResult(&mut self, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: &::core::option::Option<super::super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameService {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.IGameService";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGameService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameService_Vtbl {
        unsafe extern "system" fn ServiceUri<Impl: IGameService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamerProfileAsync<Impl: IGameService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamerProfileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstalledGameItemsAsync<Impl: IGameService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstalledGameItemsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartnerTokenAsync<Impl: IGameService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audienceuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartnerTokenAsync(&*(&audienceuri as *const <super::super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivilegesAsync<Impl: IGameService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivilegesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrantAchievement<Impl: IGameService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, achievementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GrantAchievement(achievementid).into()
        }
        unsafe extern "system" fn GrantAvatarAward<Impl: IGameService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avatarawardid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GrantAvatarAward(avatarawardid).into()
        }
        unsafe extern "system" fn PostResult<Impl: IGameService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostResult(gamevariant, scorekind, scorevalue, gameoutcome, &*(&buffer as *const <super::super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameService, BASE_OFFSET>(),
            ServiceUri: ServiceUri::<Impl, IMPL_OFFSET>,
            GetGamerProfileAsync: GetGamerProfileAsync::<Impl, IMPL_OFFSET>,
            GetInstalledGameItemsAsync: GetInstalledGameItemsAsync::<Impl, IMPL_OFFSET>,
            GetPartnerTokenAsync: GetPartnerTokenAsync::<Impl, IMPL_OFFSET>,
            GetPrivilegesAsync: GetPrivilegesAsync::<Impl, IMPL_OFFSET>,
            GrantAchievement: GrantAchievement::<Impl, IMPL_OFFSET>,
            GrantAvatarAward: GrantAvatarAward::<Impl, IMPL_OFFSET>,
            PostResult: PostResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameService2_Impl: Sized {
    fn NotifyPartnerTokenExpired(&mut self, audienceuri: &::core::option::Option<super::super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn GetAuthenticationStatus(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameService2 {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.IGameService2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameService2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameService2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameService2_Vtbl {
        unsafe extern "system" fn NotifyPartnerTokenExpired<Impl: IGameService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audienceuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyPartnerTokenExpired(&*(&audienceuri as *const <super::super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAuthenticationStatus<Impl: IGameService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthenticationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameService2, BASE_OFFSET>(),
            NotifyPartnerTokenExpired: NotifyPartnerTokenExpired::<Impl, IMPL_OFFSET>,
            GetAuthenticationStatus: GetAuthenticationStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameService2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameServicePropertyCollection_Impl: Sized {
    fn GetPropertyAsync(&mut self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameServicePropertyCollection {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.IGameServicePropertyCollection";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameServicePropertyCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameServicePropertyCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameServicePropertyCollection_Vtbl {
        unsafe extern "system" fn GetPropertyAsync<Impl: IGameServicePropertyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyAsync(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameServicePropertyCollection, BASE_OFFSET>(),
            GetPropertyAsync: GetPropertyAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameServicePropertyCollection as ::windows::core::Interface>::IID
    }
}
