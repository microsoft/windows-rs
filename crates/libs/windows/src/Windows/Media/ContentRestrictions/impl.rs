#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContentRestrictionsBrowsePolicy_Impl: Sized {
    fn GeographicRegion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxBrowsableAgeRating(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PreferredAgeRating(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentRestrictionsBrowsePolicy {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IContentRestrictionsBrowsePolicy";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContentRestrictionsBrowsePolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentRestrictionsBrowsePolicy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentRestrictionsBrowsePolicy_Vtbl {
        unsafe extern "system" fn GeographicRegion<Impl: IContentRestrictionsBrowsePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxBrowsableAgeRating<Impl: IContentRestrictionsBrowsePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PreferredAgeRating<Impl: IContentRestrictionsBrowsePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentRestrictionsBrowsePolicy, BASE_OFFSET>(),
            GeographicRegion: GeographicRegion::<Impl, IMPL_OFFSET>,
            MaxBrowsableAgeRating: MaxBrowsableAgeRating::<Impl, IMPL_OFFSET>,
            PreferredAgeRating: PreferredAgeRating::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentRestrictionsBrowsePolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRatedContentDescription_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Image(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetImage(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Category(&mut self) -> ::windows::core::Result<RatedContentCategory>;
    fn SetCategory(&mut self, value: RatedContentCategory) -> ::windows::core::Result<()>;
    fn Ratings(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SetRatings(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRatedContentDescription {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IRatedContentDescription";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRatedContentDescription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatedContentDescription_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRatedContentDescription_Vtbl {
        unsafe extern "system" fn Id<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetImage<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Category<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RatedContentCategory) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCategory<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RatedContentCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCategory(value).into()
        }
        unsafe extern "system" fn Ratings<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRatings<Impl: IRatedContentDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRatings(&*(&value as *const <super::super::Foundation::Collections::IVector<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRatedContentDescription, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            SetImage: SetImage::<Impl, IMPL_OFFSET>,
            Category: Category::<Impl, IMPL_OFFSET>,
            SetCategory: SetCategory::<Impl, IMPL_OFFSET>,
            Ratings: Ratings::<Impl, IMPL_OFFSET>,
            SetRatings: SetRatings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRatedContentDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentDescriptionFactory_Impl: Sized {
    fn Create(&mut self, id: &::windows::core::HSTRING, title: &::windows::core::HSTRING, category: RatedContentCategory) -> ::windows::core::Result<RatedContentDescription>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatedContentDescriptionFactory {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IRatedContentDescriptionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRatedContentDescriptionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatedContentDescriptionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRatedContentDescriptionFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IRatedContentDescriptionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: RatedContentCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRatedContentDescriptionFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRatedContentDescriptionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRatedContentRestrictions_Impl: Sized {
    fn GetBrowsePolicyAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>>;
    fn GetRestrictionLevelAsync(&mut self, ratedcontentdescription: &::core::option::Option<RatedContentDescription>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentAccessRestrictionLevel>>;
    fn RequestContentAccessAsync(&mut self, ratedcontentdescription: &::core::option::Option<RatedContentDescription>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RestrictionsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRestrictionsChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRatedContentRestrictions {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IRatedContentRestrictions";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRatedContentRestrictions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatedContentRestrictions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRatedContentRestrictions_Vtbl {
        unsafe extern "system" fn GetBrowsePolicyAsync<Impl: IRatedContentRestrictions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRestrictionLevelAsync<Impl: IRatedContentRestrictions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratedcontentdescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestContentAccessAsync<Impl: IRatedContentRestrictions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ratedcontentdescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RestrictionsChanged<Impl: IRatedContentRestrictions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRestrictionsChanged<Impl: IRatedContentRestrictions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRestrictionsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRatedContentRestrictions, BASE_OFFSET>(),
            GetBrowsePolicyAsync: GetBrowsePolicyAsync::<Impl, IMPL_OFFSET>,
            GetRestrictionLevelAsync: GetRestrictionLevelAsync::<Impl, IMPL_OFFSET>,
            RequestContentAccessAsync: RequestContentAccessAsync::<Impl, IMPL_OFFSET>,
            RestrictionsChanged: RestrictionsChanged::<Impl, IMPL_OFFSET>,
            RemoveRestrictionsChanged: RemoveRestrictionsChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRatedContentRestrictions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentRestrictionsFactory_Impl: Sized {
    fn CreateWithMaxAgeRating(&mut self, maxagerating: u32) -> ::windows::core::Result<RatedContentRestrictions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatedContentRestrictionsFactory {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.IRatedContentRestrictionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRatedContentRestrictionsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatedContentRestrictionsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRatedContentRestrictionsFactory_Vtbl {
        unsafe extern "system" fn CreateWithMaxAgeRating<Impl: IRatedContentRestrictionsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxagerating: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRatedContentRestrictionsFactory, BASE_OFFSET>(),
            CreateWithMaxAgeRating: CreateWithMaxAgeRating::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRatedContentRestrictionsFactory as ::windows::core::Interface>::IID
    }
}
