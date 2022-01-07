#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialDashboardItemUpdaterImpl: Sized {
    fn OwnerRemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Content(&self) -> ::windows::core::Result<super::SocialFeedContent>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetTimestamp(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::SocialItemThumbnail>) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::SocialItemThumbnail>;
    fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn TargetUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetTargetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialDashboardItemUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.ISocialDashboardItemUpdater";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialDashboardItemUpdaterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>() -> ISocialDashboardItemUpdaterVtbl {
        unsafe extern "system" fn OwnerRemoteId<Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OwnerRemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimestamp<Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimestamp(&*(&value as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetThumbnail<Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::SocialItemThumbnail as ::windows::core::Abi>::Abi as *const <super::SocialItemThumbnail as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitAsync<Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetUri<Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetUri<Impl: ISocialDashboardItemUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISocialDashboardItemUpdater>,
            ::windows::core::GetTrustLevel,
            OwnerRemoteId::<Impl, OFFSET>,
            Content::<Impl, OFFSET>,
            Timestamp::<Impl, OFFSET>,
            SetTimestamp::<Impl, OFFSET>,
            SetThumbnail::<Impl, OFFSET>,
            Thumbnail::<Impl, OFFSET>,
            CommitAsync::<Impl, OFFSET>,
            TargetUri::<Impl, OFFSET>,
            SetTargetUri::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedUpdaterImpl: Sized {
    fn OwnerRemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<super::SocialFeedKind>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::SocialFeedItem>>;
    fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialFeedUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.ISocialFeedUpdater";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialFeedUpdaterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialFeedUpdaterImpl, const OFFSET: isize>() -> ISocialFeedUpdaterVtbl {
        unsafe extern "system" fn OwnerRemoteId<Impl: ISocialFeedUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OwnerRemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: ISocialFeedUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::SocialFeedKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Impl: ISocialFeedUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitAsync<Impl: ISocialFeedUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISocialFeedUpdater>, ::windows::core::GetTrustLevel, OwnerRemoteId::<Impl, OFFSET>, Kind::<Impl, OFFSET>, Items::<Impl, OFFSET>, CommitAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialInfoProviderManagerStaticsImpl: Sized {
    fn CreateSocialFeedUpdaterAsync(&self, kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SocialFeedUpdater>>;
    fn CreateDashboardItemUpdaterAsync(&self, ownerremoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SocialDashboardItemUpdater>>;
    fn UpdateBadgeCountValue(&self, itemremoteid: &::windows::core::HSTRING, newcount: i32) -> ::windows::core::Result<()>;
    fn ReportNewContentAvailable(&self, contactremoteid: &::windows::core::HSTRING, kind: super::SocialFeedKind) -> ::windows::core::Result<()>;
    fn ProvisionAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn DeprovisionAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialInfoProviderManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.ISocialInfoProviderManagerStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialInfoProviderManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialInfoProviderManagerStaticsImpl, const OFFSET: isize>() -> ISocialInfoProviderManagerStaticsVtbl {
        unsafe extern "system" fn CreateSocialFeedUpdaterAsync<Impl: ISocialInfoProviderManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSocialFeedUpdaterAsync(kind, mode, &*(&ownerremoteid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDashboardItemUpdaterAsync<Impl: ISocialInfoProviderManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ownerremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDashboardItemUpdaterAsync(&*(&ownerremoteid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateBadgeCountValue<Impl: ISocialInfoProviderManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateBadgeCountValue(&*(&itemremoteid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), newcount).into()
        }
        unsafe extern "system" fn ReportNewContentAvailable<Impl: ISocialInfoProviderManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, kind: super::SocialFeedKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportNewContentAvailable(&*(&contactremoteid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), kind).into()
        }
        unsafe extern "system" fn ProvisionAsync<Impl: ISocialInfoProviderManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProvisionAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeprovisionAsync<Impl: ISocialInfoProviderManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeprovisionAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISocialInfoProviderManagerStatics>,
            ::windows::core::GetTrustLevel,
            CreateSocialFeedUpdaterAsync::<Impl, OFFSET>,
            CreateDashboardItemUpdaterAsync::<Impl, OFFSET>,
            UpdateBadgeCountValue::<Impl, OFFSET>,
            ReportNewContentAvailable::<Impl, OFFSET>,
            ProvisionAsync::<Impl, OFFSET>,
            DeprovisionAsync::<Impl, OFFSET>,
        )
    }
}
