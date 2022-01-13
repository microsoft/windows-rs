#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialDashboardItemUpdaterImpl: Sized {
    fn OwnerRemoteId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Content(&mut self) -> ::windows::core::Result<super::SocialFeedContent>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetTimestamp(&mut self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetThumbnail(&mut self, value: &::core::option::Option<super::SocialItemThumbnail>) -> ::windows::core::Result<()>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::SocialItemThumbnail>;
    fn CommitAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn TargetUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetTargetUri(&mut self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialDashboardItemUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.ISocialDashboardItemUpdater";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialDashboardItemUpdaterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialDashboardItemUpdaterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocialDashboardItemUpdaterVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocialDashboardItemUpdater, BASE_OFFSET>(),
            OwnerRemoteId: OwnerRemoteId::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            SetTimestamp: SetTimestamp::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            CommitAsync: CommitAsync::<Impl, IMPL_OFFSET>,
            TargetUri: TargetUri::<Impl, IMPL_OFFSET>,
            SetTargetUri: SetTargetUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocialDashboardItemUpdater as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedUpdaterImpl: Sized {
    fn OwnerRemoteId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&mut self) -> ::windows::core::Result<super::SocialFeedKind>;
    fn Items(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::SocialFeedItem>>;
    fn CommitAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialFeedUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.ISocialFeedUpdater";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialFeedUpdaterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialFeedUpdaterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocialFeedUpdaterVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocialFeedUpdater, BASE_OFFSET>(),
            OwnerRemoteId: OwnerRemoteId::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            Items: Items::<Impl, IMPL_OFFSET>,
            CommitAsync: CommitAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocialFeedUpdater as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialInfoProviderManagerStaticsImpl: Sized {
    fn CreateSocialFeedUpdaterAsync(&mut self, kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SocialFeedUpdater>>;
    fn CreateDashboardItemUpdaterAsync(&mut self, ownerremoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SocialDashboardItemUpdater>>;
    fn UpdateBadgeCountValue(&mut self, itemremoteid: &::windows::core::HSTRING, newcount: i32) -> ::windows::core::Result<()>;
    fn ReportNewContentAvailable(&mut self, contactremoteid: &::windows::core::HSTRING, kind: super::SocialFeedKind) -> ::windows::core::Result<()>;
    fn ProvisionAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn DeprovisionAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialInfoProviderManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.ISocialInfoProviderManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialInfoProviderManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialInfoProviderManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocialInfoProviderManagerStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocialInfoProviderManagerStatics, BASE_OFFSET>(),
            CreateSocialFeedUpdaterAsync: CreateSocialFeedUpdaterAsync::<Impl, IMPL_OFFSET>,
            CreateDashboardItemUpdaterAsync: CreateDashboardItemUpdaterAsync::<Impl, IMPL_OFFSET>,
            UpdateBadgeCountValue: UpdateBadgeCountValue::<Impl, IMPL_OFFSET>,
            ReportNewContentAvailable: ReportNewContentAvailable::<Impl, IMPL_OFFSET>,
            ProvisionAsync: ProvisionAsync::<Impl, IMPL_OFFSET>,
            DeprovisionAsync: DeprovisionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocialInfoProviderManagerStatics as ::windows::core::Interface>::IID
    }
}
