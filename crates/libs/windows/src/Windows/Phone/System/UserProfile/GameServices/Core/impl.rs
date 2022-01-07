#[cfg(feature = "implement_exclusive")]
pub trait IGameServiceImpl: Sized {
    fn ServiceUri(&self) -> ::windows::core::Result<super::super::super::super::super::Foundation::Uri>;
    fn GetGamerProfileAsync(&self) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>;
    fn GetInstalledGameItemsAsync(&self) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>>;
    fn GetPartnerTokenAsync(&self, audienceuri: &::core::option::Option<super::super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetPrivilegesAsync(&self) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GrantAchievement(&self, achievementid: u32) -> ::windows::core::Result<()>;
    fn GrantAvatarAward(&self, avatarawardid: u32) -> ::windows::core::Result<()>;
    fn PostResult(&self, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: &::core::option::Option<super::super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameService {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.IGameService";
}
#[cfg(feature = "implement_exclusive")]
impl IGameServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameServiceImpl, const OFFSET: isize>() -> IGameServiceVtbl {
        unsafe extern "system" fn ServiceUri<Impl: IGameServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetGamerProfileAsync<Impl: IGameServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetInstalledGameItemsAsync<Impl: IGameServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPartnerTokenAsync<Impl: IGameServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audienceuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPrivilegesAsync<Impl: IGameServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GrantAchievement<Impl: IGameServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, achievementid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GrantAchievement(achievementid).into()
        }
        unsafe extern "system" fn GrantAvatarAward<Impl: IGameServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avatarawardid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GrantAvatarAward(avatarawardid).into()
        }
        unsafe extern "system" fn PostResult<Impl: IGameServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostResult(gamevariant, scorekind, scorevalue, gameoutcome, &*(&buffer as *const <super::super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGameService>,
            ::windows::core::GetTrustLevel,
            ServiceUri::<Impl, OFFSET>,
            GetGamerProfileAsync::<Impl, OFFSET>,
            GetInstalledGameItemsAsync::<Impl, OFFSET>,
            GetPartnerTokenAsync::<Impl, OFFSET>,
            GetPrivilegesAsync::<Impl, OFFSET>,
            GrantAchievement::<Impl, OFFSET>,
            GrantAvatarAward::<Impl, OFFSET>,
            PostResult::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameService2Impl: Sized {
    fn NotifyPartnerTokenExpired(&self, audienceuri: &::core::option::Option<super::super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn GetAuthenticationStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameService2 {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.IGameService2";
}
#[cfg(feature = "implement_exclusive")]
impl IGameService2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameService2Impl, const OFFSET: isize>() -> IGameService2Vtbl {
        unsafe extern "system" fn NotifyPartnerTokenExpired<Impl: IGameService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audienceuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyPartnerTokenExpired(&*(&audienceuri as *const <super::super::super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAuthenticationStatus<Impl: IGameService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameService2>, ::windows::core::GetTrustLevel, NotifyPartnerTokenExpired::<Impl, OFFSET>, GetAuthenticationStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameServicePropertyCollectionImpl: Sized {
    fn GetPropertyAsync(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameServicePropertyCollection {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.IGameServicePropertyCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IGameServicePropertyCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameServicePropertyCollectionImpl, const OFFSET: isize>() -> IGameServicePropertyCollectionVtbl {
        unsafe extern "system" fn GetPropertyAsync<Impl: IGameServicePropertyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameServicePropertyCollection>, ::windows::core::GetTrustLevel, GetPropertyAsync::<Impl, OFFSET>)
    }
}
