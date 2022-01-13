#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedChildItemImpl: Sized {
    fn Author(&mut self) -> ::windows::core::Result<SocialUserInfo>;
    fn PrimaryContent(&mut self) -> ::windows::core::Result<SocialFeedContent>;
    fn SecondaryContent(&mut self) -> ::windows::core::Result<SocialFeedContent>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetTimestamp(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn TargetUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Thumbnails(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SocialItemThumbnail>>;
    fn SharedItem(&mut self) -> ::windows::core::Result<SocialFeedSharedItem>;
    fn SetSharedItem(&mut self, value: &::core::option::Option<SocialFeedSharedItem>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialFeedChildItem {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.ISocialFeedChildItem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialFeedChildItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialFeedChildItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocialFeedChildItemVtbl {
        unsafe extern "system" fn Author<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimaryContent<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecondaryContent<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecondaryContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTimestamp<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimestamp(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetUri<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetUri<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnails<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharedItem<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharedItem<Impl: ISocialFeedChildItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharedItem(&*(&value as *const <SocialFeedSharedItem as ::windows::core::Abi>::Abi as *const <SocialFeedSharedItem as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocialFeedChildItem, BASE_OFFSET>(),
            Author: Author::<Impl, IMPL_OFFSET>,
            PrimaryContent: PrimaryContent::<Impl, IMPL_OFFSET>,
            SecondaryContent: SecondaryContent::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            SetTimestamp: SetTimestamp::<Impl, IMPL_OFFSET>,
            TargetUri: TargetUri::<Impl, IMPL_OFFSET>,
            SetTargetUri: SetTargetUri::<Impl, IMPL_OFFSET>,
            Thumbnails: Thumbnails::<Impl, IMPL_OFFSET>,
            SharedItem: SharedItem::<Impl, IMPL_OFFSET>,
            SetSharedItem: SetSharedItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocialFeedChildItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedContentImpl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialFeedContent {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.ISocialFeedContent";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialFeedContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialFeedContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocialFeedContentVtbl {
        unsafe extern "system" fn Title<Impl: ISocialFeedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: ISocialFeedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Message<Impl: ISocialFeedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessage<Impl: ISocialFeedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetUri<Impl: ISocialFeedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetUri<Impl: ISocialFeedContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocialFeedContent, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            SetMessage: SetMessage::<Impl, IMPL_OFFSET>,
            TargetUri: TargetUri::<Impl, IMPL_OFFSET>,
            SetTargetUri: SetTargetUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocialFeedContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedItemImpl: Sized {
    fn Author(&mut self) -> ::windows::core::Result<SocialUserInfo>;
    fn PrimaryContent(&mut self) -> ::windows::core::Result<SocialFeedContent>;
    fn SecondaryContent(&mut self) -> ::windows::core::Result<SocialFeedContent>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetTimestamp(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn TargetUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Thumbnails(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SocialItemThumbnail>>;
    fn SharedItem(&mut self) -> ::windows::core::Result<SocialFeedSharedItem>;
    fn SetSharedItem(&mut self, value: &::core::option::Option<SocialFeedSharedItem>) -> ::windows::core::Result<()>;
    fn BadgeStyle(&mut self) -> ::windows::core::Result<SocialItemBadgeStyle>;
    fn SetBadgeStyle(&mut self, value: SocialItemBadgeStyle) -> ::windows::core::Result<()>;
    fn BadgeCountValue(&mut self) -> ::windows::core::Result<i32>;
    fn SetBadgeCountValue(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn RemoteId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ChildItem(&mut self) -> ::windows::core::Result<SocialFeedChildItem>;
    fn SetChildItem(&mut self, value: &::core::option::Option<SocialFeedChildItem>) -> ::windows::core::Result<()>;
    fn Style(&mut self) -> ::windows::core::Result<SocialFeedItemStyle>;
    fn SetStyle(&mut self, value: SocialFeedItemStyle) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialFeedItem {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.ISocialFeedItem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialFeedItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialFeedItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocialFeedItemVtbl {
        unsafe extern "system" fn Author<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimaryContent<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecondaryContent<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecondaryContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTimestamp<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimestamp(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetUri<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetUri<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnails<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharedItem<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSharedItem<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSharedItem(&*(&value as *const <SocialFeedSharedItem as ::windows::core::Abi>::Abi as *const <SocialFeedSharedItem as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BadgeStyle<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocialItemBadgeStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BadgeStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBadgeStyle<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocialItemBadgeStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBadgeStyle(value).into()
        }
        unsafe extern "system" fn BadgeCountValue<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BadgeCountValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBadgeCountValue<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBadgeCountValue(value).into()
        }
        unsafe extern "system" fn RemoteId<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteId<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChildItem<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChildItem<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChildItem(&*(&value as *const <SocialFeedChildItem as ::windows::core::Abi>::Abi as *const <SocialFeedChildItem as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Style<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocialFeedItemStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Style() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: ISocialFeedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocialFeedItemStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyle(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocialFeedItem, BASE_OFFSET>(),
            Author: Author::<Impl, IMPL_OFFSET>,
            PrimaryContent: PrimaryContent::<Impl, IMPL_OFFSET>,
            SecondaryContent: SecondaryContent::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            SetTimestamp: SetTimestamp::<Impl, IMPL_OFFSET>,
            TargetUri: TargetUri::<Impl, IMPL_OFFSET>,
            SetTargetUri: SetTargetUri::<Impl, IMPL_OFFSET>,
            Thumbnails: Thumbnails::<Impl, IMPL_OFFSET>,
            SharedItem: SharedItem::<Impl, IMPL_OFFSET>,
            SetSharedItem: SetSharedItem::<Impl, IMPL_OFFSET>,
            BadgeStyle: BadgeStyle::<Impl, IMPL_OFFSET>,
            SetBadgeStyle: SetBadgeStyle::<Impl, IMPL_OFFSET>,
            BadgeCountValue: BadgeCountValue::<Impl, IMPL_OFFSET>,
            SetBadgeCountValue: SetBadgeCountValue::<Impl, IMPL_OFFSET>,
            RemoteId: RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId: SetRemoteId::<Impl, IMPL_OFFSET>,
            ChildItem: ChildItem::<Impl, IMPL_OFFSET>,
            SetChildItem: SetChildItem::<Impl, IMPL_OFFSET>,
            Style: Style::<Impl, IMPL_OFFSET>,
            SetStyle: SetStyle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocialFeedItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedSharedItemImpl: Sized {
    fn OriginalSource(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetOriginalSource(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Content(&mut self) -> ::windows::core::Result<SocialFeedContent>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetTimestamp(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn TargetUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetThumbnail(&mut self, value: &::core::option::Option<SocialItemThumbnail>) -> ::windows::core::Result<()>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<SocialItemThumbnail>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialFeedSharedItem {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.ISocialFeedSharedItem";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialFeedSharedItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialFeedSharedItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocialFeedSharedItemVtbl {
        unsafe extern "system" fn OriginalSource<Impl: ISocialFeedSharedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginalSource<Impl: ISocialFeedSharedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOriginalSource(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Content<Impl: ISocialFeedSharedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Timestamp<Impl: ISocialFeedSharedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTimestamp<Impl: ISocialFeedSharedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimestamp(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetUri<Impl: ISocialFeedSharedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetUri<Impl: ISocialFeedSharedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetThumbnail<Impl: ISocialFeedSharedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <SocialItemThumbnail as ::windows::core::Abi>::Abi as *const <SocialItemThumbnail as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: ISocialFeedSharedItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocialFeedSharedItem, BASE_OFFSET>(),
            OriginalSource: OriginalSource::<Impl, IMPL_OFFSET>,
            SetOriginalSource: SetOriginalSource::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            SetTimestamp: SetTimestamp::<Impl, IMPL_OFFSET>,
            TargetUri: TargetUri::<Impl, IMPL_OFFSET>,
            SetTargetUri: SetTargetUri::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocialFeedSharedItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialItemThumbnailImpl: Sized {
    fn TargetUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ImageUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetImageUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn BitmapSize(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetBitmapSize(&mut self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
    fn SetImageAsync(&mut self, image: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialItemThumbnail {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.ISocialItemThumbnail";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialItemThumbnailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialItemThumbnailImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocialItemThumbnailVtbl {
        unsafe extern "system" fn TargetUri<Impl: ISocialItemThumbnailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetUri<Impl: ISocialItemThumbnailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImageUri<Impl: ISocialItemThumbnailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageUri<Impl: ISocialItemThumbnailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImageUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BitmapSize<Impl: ISocialItemThumbnailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitmapSize<Impl: ISocialItemThumbnailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitmapSize(&*(&value as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::BitmapSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetImageAsync<Impl: ISocialItemThumbnailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, image: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetImageAsync(&*(&image as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocialItemThumbnail, BASE_OFFSET>(),
            TargetUri: TargetUri::<Impl, IMPL_OFFSET>,
            SetTargetUri: SetTargetUri::<Impl, IMPL_OFFSET>,
            ImageUri: ImageUri::<Impl, IMPL_OFFSET>,
            SetImageUri: SetImageUri::<Impl, IMPL_OFFSET>,
            BitmapSize: BitmapSize::<Impl, IMPL_OFFSET>,
            SetBitmapSize: SetBitmapSize::<Impl, IMPL_OFFSET>,
            SetImageAsync: SetImageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocialItemThumbnail as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialUserInfoImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoteId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocialUserInfo {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.ISocialUserInfo";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ISocialUserInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocialUserInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocialUserInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: ISocialUserInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: ISocialUserInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserName<Impl: ISocialUserInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: ISocialUserInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteId<Impl: ISocialUserInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteId<Impl: ISocialUserInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TargetUri<Impl: ISocialUserInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetUri<Impl: ISocialUserInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocialUserInfo, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            UserName: UserName::<Impl, IMPL_OFFSET>,
            SetUserName: SetUserName::<Impl, IMPL_OFFSET>,
            RemoteId: RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId: SetRemoteId::<Impl, IMPL_OFFSET>,
            TargetUri: TargetUri::<Impl, IMPL_OFFSET>,
            SetTargetUri: SetTargetUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocialUserInfo as ::windows::core::Interface>::IID
    }
}
