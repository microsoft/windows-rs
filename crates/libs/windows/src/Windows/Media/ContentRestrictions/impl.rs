#[cfg(feature = "implement_exclusive")]
pub trait IContentRestrictionsBrowsePolicyImpl: Sized {
    fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxBrowsableAgeRating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PreferredAgeRating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentRestrictionsBrowsePolicy {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IContentRestrictionsBrowsePolicy";
}
#[cfg(feature = "implement_exclusive")]
impl IContentRestrictionsBrowsePolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentRestrictionsBrowsePolicyImpl, const OFFSET: isize>() -> IContentRestrictionsBrowsePolicyVtbl {
        unsafe extern "system" fn GeographicRegion<Impl: IContentRestrictionsBrowsePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeographicRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxBrowsableAgeRating<Impl: IContentRestrictionsBrowsePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBrowsableAgeRating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredAgeRating<Impl: IContentRestrictionsBrowsePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAgeRating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContentRestrictionsBrowsePolicy>, ::windows::core::GetTrustLevel, GeographicRegion::<Impl, OFFSET>, MaxBrowsableAgeRating::<Impl, OFFSET>, PreferredAgeRating::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentDescriptionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Image(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetImage(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Category(&self) -> ::windows::core::Result<RatedContentCategory>;
    fn SetCategory(&self, value: RatedContentCategory) -> ::windows::core::Result<()>;
    fn Ratings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SetRatings(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatedContentDescription {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IRatedContentDescription";
}
#[cfg(feature = "implement_exclusive")]
impl IRatedContentDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatedContentDescriptionImpl, const OFFSET: isize>() -> IRatedContentDescriptionVtbl {
        unsafe extern "system" fn Id<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Category<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RatedContentCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RatedContentCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCategory(value).into()
        }
        unsafe extern "system" fn Ratings<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ratings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRatings<Impl: IRatedContentDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRatings(&*(&value as *const <super::super::Foundation::Collections::IVector<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRatedContentDescription>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            SetId::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            SetTitle::<Impl, OFFSET>,
            Image::<Impl, OFFSET>,
            SetImage::<Impl, OFFSET>,
            Category::<Impl, OFFSET>,
            SetCategory::<Impl, OFFSET>,
            Ratings::<Impl, OFFSET>,
            SetRatings::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentDescriptionFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING, title: &::windows::core::HSTRING, category: RatedContentCategory) -> ::windows::core::Result<RatedContentDescription>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatedContentDescriptionFactory {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IRatedContentDescriptionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRatedContentDescriptionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatedContentDescriptionFactoryImpl, const OFFSET: isize>() -> IRatedContentDescriptionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IRatedContentDescriptionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: RatedContentCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&title as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRatedContentDescriptionFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentRestrictionsImpl: Sized {
    fn GetBrowsePolicyAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>>;
    fn GetRestrictionLevelAsync(&self, ratedcontentdescription: &::core::option::Option<RatedContentDescription>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentAccessRestrictionLevel>>;
    fn RequestContentAccessAsync(&self, ratedcontentdescription: &::core::option::Option<RatedContentDescription>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RestrictionsChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRestrictionsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatedContentRestrictions {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IRatedContentRestrictions";
}
#[cfg(feature = "implement_exclusive")]
impl IRatedContentRestrictionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatedContentRestrictionsImpl, const OFFSET: isize>() -> IRatedContentRestrictionsVtbl {
        unsafe extern "system" fn GetBrowsePolicyAsync<Impl: IRatedContentRestrictionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBrowsePolicyAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictionLevelAsync<Impl: IRatedContentRestrictionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratedcontentdescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictionLevelAsync(&*(&ratedcontentdescription as *const <RatedContentDescription as ::windows::core::Abi>::Abi as *const <RatedContentDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestContentAccessAsync<Impl: IRatedContentRestrictionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratedcontentdescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestContentAccessAsync(&*(&ratedcontentdescription as *const <RatedContentDescription as ::windows::core::Abi>::Abi as *const <RatedContentDescription as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestrictionsChanged<Impl: IRatedContentRestrictionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestrictionsChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRestrictionsChanged<Impl: IRatedContentRestrictionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRestrictionsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRatedContentRestrictions>, ::windows::core::GetTrustLevel, GetBrowsePolicyAsync::<Impl, OFFSET>, GetRestrictionLevelAsync::<Impl, OFFSET>, RequestContentAccessAsync::<Impl, OFFSET>, RestrictionsChanged::<Impl, OFFSET>, RemoveRestrictionsChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentRestrictionsFactoryImpl: Sized {
    fn CreateWithMaxAgeRating(&self, maxagerating: u32) -> ::windows::core::Result<RatedContentRestrictions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatedContentRestrictionsFactory {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IRatedContentRestrictionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRatedContentRestrictionsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatedContentRestrictionsFactoryImpl, const OFFSET: isize>() -> IRatedContentRestrictionsFactoryVtbl {
        unsafe extern "system" fn CreateWithMaxAgeRating<Impl: IRatedContentRestrictionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxagerating: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithMaxAgeRating(maxagerating) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRatedContentRestrictionsFactory>, ::windows::core::GetTrustLevel, CreateWithMaxAgeRating::<Impl, OFFSET>)
    }
}
