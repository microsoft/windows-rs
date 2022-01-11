#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityImpl: Sized {
    fn State(&self) -> ::windows::core::Result<UserActivityState>;
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VisualElements(&self) -> ::windows::core::Result<UserActivityVisualElements>;
    fn ContentUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetFallbackUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ActivationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetActivationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentInfo(&self) -> ::windows::core::Result<IUserActivityContentInfo>;
    fn SetContentInfo(&self, value: &::core::option::Option<IUserActivityContentInfo>) -> ::windows::core::Result<()>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateSession(&self) -> ::windows::core::Result<UserActivitySession>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivity {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivity";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityVtbl {
        unsafe extern "system" fn State<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserActivityState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisualElements<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisualElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentUri<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentUri<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentType<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FallbackUri<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFallbackUri<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFallbackUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActivationUri<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivationUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivationUri<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentInfo<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentInfo<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentInfo(&*(&value as *const <IUserActivityContentInfo as ::windows::core::Abi>::Abi as *const <IUserActivityContentInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SaveAsync<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Impl: IUserActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUserActivity>,
            ::windows::core::GetTrustLevel,
            State::<Impl, IMPL_OFFSET>,
            ActivityId::<Impl, IMPL_OFFSET>,
            VisualElements::<Impl, IMPL_OFFSET>,
            ContentUri::<Impl, IMPL_OFFSET>,
            SetContentUri::<Impl, IMPL_OFFSET>,
            ContentType::<Impl, IMPL_OFFSET>,
            SetContentType::<Impl, IMPL_OFFSET>,
            FallbackUri::<Impl, IMPL_OFFSET>,
            SetFallbackUri::<Impl, IMPL_OFFSET>,
            ActivationUri::<Impl, IMPL_OFFSET>,
            SetActivationUri::<Impl, IMPL_OFFSET>,
            ContentInfo::<Impl, IMPL_OFFSET>,
            SetContentInfo::<Impl, IMPL_OFFSET>,
            SaveAsync::<Impl, IMPL_OFFSET>,
            CreateSession::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivity2Impl: Sized {
    fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivity2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivity2";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivity2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivity2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivity2Vtbl {
        unsafe extern "system" fn ToJson<Impl: IUserActivity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToJson() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivity2>, ::windows::core::GetTrustLevel, ToJson::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivity2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivity3Impl: Sized {
    fn IsRoamable(&self) -> ::windows::core::Result<bool>;
    fn SetIsRoamable(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivity3 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivity3";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivity3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivity3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivity3Vtbl {
        unsafe extern "system" fn IsRoamable<Impl: IUserActivity3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoamable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRoamable<Impl: IUserActivity3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRoamable(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivity3>, ::windows::core::GetTrustLevel, IsRoamable::<Impl, IMPL_OFFSET>, SetIsRoamable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivity3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityAttributionImpl: Sized {
    fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetIconUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlternateText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddImageQuery(&self) -> ::windows::core::Result<bool>;
    fn SetAddImageQuery(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityAttribution {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityAttribution";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityAttributionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityAttributionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityAttributionVtbl {
        unsafe extern "system" fn IconUri<Impl: IUserActivityAttributionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIconUri<Impl: IUserActivityAttributionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlternateText<Impl: IUserActivityAttributionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlternateText<Impl: IUserActivityAttributionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlternateText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddImageQuery<Impl: IUserActivityAttributionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddImageQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddImageQuery<Impl: IUserActivityAttributionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddImageQuery(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUserActivityAttribution>,
            ::windows::core::GetTrustLevel,
            IconUri::<Impl, IMPL_OFFSET>,
            SetIconUri::<Impl, IMPL_OFFSET>,
            AlternateText::<Impl, IMPL_OFFSET>,
            SetAlternateText::<Impl, IMPL_OFFSET>,
            AddImageQuery::<Impl, IMPL_OFFSET>,
            SetAddImageQuery::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityAttribution as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityAttributionFactoryImpl: Sized {
    fn CreateWithUri(&self, iconuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<UserActivityAttribution>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityAttributionFactory {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityAttributionFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityAttributionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityAttributionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityAttributionFactoryVtbl {
        unsafe extern "system" fn CreateWithUri<Impl: IUserActivityAttributionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iconuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithUri(&*(&iconuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityAttributionFactory>, ::windows::core::GetTrustLevel, CreateWithUri::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityAttributionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityChannelImpl: Sized {
    fn GetOrCreateUserActivityAsync(&self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserActivity>>;
    fn DeleteActivityAsync(&self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAllActivitiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityChannel {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannel";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannelVtbl {
        unsafe extern "system" fn GetOrCreateUserActivityAsync<Impl: IUserActivityChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOrCreateUserActivityAsync(&*(&activityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteActivityAsync<Impl: IUserActivityChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteActivityAsync(&*(&activityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAllActivitiesAsync<Impl: IUserActivityChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAllActivitiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityChannel>, ::windows::core::GetTrustLevel, GetOrCreateUserActivityAsync::<Impl, IMPL_OFFSET>, DeleteActivityAsync::<Impl, IMPL_OFFSET>, DeleteAllActivitiesAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserActivityChannel2Impl: Sized {
    fn GetRecentUserActivitiesAsync(&self, maxuniqueactivities: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>;
    fn GetSessionHistoryItemsForUserActivityAsync(&self, activityid: &::windows::core::HSTRING, starttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityChannel2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannel2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserActivityChannel2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannel2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannel2Vtbl {
        unsafe extern "system" fn GetRecentUserActivitiesAsync<Impl: IUserActivityChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxuniqueactivities: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecentUserActivitiesAsync(maxuniqueactivities) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionHistoryItemsForUserActivityAsync<Impl: IUserActivityChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, starttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionHistoryItemsForUserActivityAsync(&*(&activityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityChannel2>, ::windows::core::GetTrustLevel, GetRecentUserActivitiesAsync::<Impl, IMPL_OFFSET>, GetSessionHistoryItemsForUserActivityAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannel2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityChannelStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<UserActivityChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityChannelStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannelStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityChannelStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannelStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannelStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IUserActivityChannelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityChannelStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannelStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IUserActivityChannelStatics2Impl: Sized {
    fn DisableAutoSessionCreation(&self) -> ::windows::core::Result<()>;
    fn TryGetForWebAccount(&self, account: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<UserActivityChannel>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityChannelStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannelStatics2";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IUserActivityChannelStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannelStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannelStatics2Vtbl {
        unsafe extern "system" fn DisableAutoSessionCreation<Impl: IUserActivityChannelStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableAutoSessionCreation().into()
        }
        unsafe extern "system" fn TryGetForWebAccount<Impl: IUserActivityChannelStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetForWebAccount(&*(&account as *const <super::super::Security::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityChannelStatics2>, ::windows::core::GetTrustLevel, DisableAutoSessionCreation::<Impl, IMPL_OFFSET>, TryGetForWebAccount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannelStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IUserActivityChannelStatics3Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<UserActivityChannel>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityChannelStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannelStatics3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IUserActivityChannelStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannelStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannelStatics3Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IUserActivityChannelStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityChannelStatics3>, ::windows::core::GetTrustLevel, GetForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannelStatics3 as ::windows::core::Interface>::IID
    }
}
pub trait IUserActivityContentInfoImpl: Sized {
    fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IUserActivityContentInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityContentInfo";
}
impl IUserActivityContentInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityContentInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityContentInfoVtbl {
        unsafe extern "system" fn ToJson<Impl: IUserActivityContentInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToJson() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityContentInfo>, ::windows::core::GetTrustLevel, ToJson::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityContentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityContentInfoStaticsImpl: Sized {
    fn FromJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivityContentInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityContentInfoStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityContentInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityContentInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityContentInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityContentInfoStaticsVtbl {
        unsafe extern "system" fn FromJson<Impl: IUserActivityContentInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromJson(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityContentInfoStatics>, ::windows::core::GetTrustLevel, FromJson::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityContentInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityFactoryImpl: Sized {
    fn CreateWithActivityId(&self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityFactory {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityFactoryVtbl {
        unsafe extern "system" fn CreateWithActivityId<Impl: IUserActivityFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithActivityId(&*(&activityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityFactory>, ::windows::core::GetTrustLevel, CreateWithActivityId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityRequestImpl: Sized {
    fn SetUserActivity(&self, activity: &::core::option::Option<UserActivity>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityRequestVtbl {
        unsafe extern "system" fn SetUserActivity<Impl: IUserActivityRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activity: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserActivity(&*(&activity as *const <UserActivity as ::windows::core::Abi>::Abi as *const <UserActivity as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityRequest>, ::windows::core::GetTrustLevel, SetUserActivity::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityRequestManagerImpl: Sized {
    fn UserActivityRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UserActivityRequestManager, UserActivityRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserActivityRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityRequestManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityRequestManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityRequestManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityRequestManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityRequestManagerVtbl {
        unsafe extern "system" fn UserActivityRequested<Impl: IUserActivityRequestManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserActivityRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UserActivityRequestManager, UserActivityRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UserActivityRequestManager, UserActivityRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserActivityRequested<Impl: IUserActivityRequestManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserActivityRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityRequestManager>, ::windows::core::GetTrustLevel, UserActivityRequested::<Impl, IMPL_OFFSET>, RemoveUserActivityRequested::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequestManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityRequestManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<UserActivityRequestManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityRequestManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityRequestManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityRequestManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityRequestManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityRequestManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IUserActivityRequestManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityRequestManagerStatics>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequestManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<UserActivityRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IUserActivityRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IUserActivityRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityRequestedEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivitySessionImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivitySession {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivitySession";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivitySessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivitySessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivitySessionVtbl {
        unsafe extern "system" fn ActivityId<Impl: IUserActivitySessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivitySession>, ::windows::core::GetTrustLevel, ActivityId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivitySession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivitySessionHistoryItemImpl: Sized {
    fn UserActivity(&self) -> ::windows::core::Result<UserActivity>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivitySessionHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivitySessionHistoryItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivitySessionHistoryItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivitySessionHistoryItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivitySessionHistoryItemVtbl {
        unsafe extern "system" fn UserActivity<Impl: IUserActivitySessionHistoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserActivity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: IUserActivitySessionHistoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndTime<Impl: IUserActivitySessionHistoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivitySessionHistoryItem>, ::windows::core::GetTrustLevel, UserActivity::<Impl, IMPL_OFFSET>, StartTime::<Impl, IMPL_OFFSET>, EndTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivitySessionHistoryItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserActivityStaticsImpl: Sized {
    fn TryParseFromJson(&self, json: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity>;
    fn TryParseFromJsonArray(&self, json: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<UserActivity>>;
    fn ToJsonArray(&self, activities: &::core::option::Option<super::super::Foundation::Collections::IIterable<UserActivity>>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserActivityStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityStaticsVtbl {
        unsafe extern "system" fn TryParseFromJson<Impl: IUserActivityStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseFromJson(&*(&json as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParseFromJsonArray<Impl: IUserActivityStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseFromJsonArray(&*(&json as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToJsonArray<Impl: IUserActivityStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activities: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToJsonArray(&*(&activities as *const <super::super::Foundation::Collections::IIterable<UserActivity> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<UserActivity> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityStatics>, ::windows::core::GetTrustLevel, TryParseFromJson::<Impl, IMPL_OFFSET>, TryParseFromJsonArray::<Impl, IMPL_OFFSET>, ToJsonArray::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "UI_Shell", feature = "implement_exclusive"))]
pub trait IUserActivityVisualElementsImpl: Sized {
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Attribution(&self) -> ::windows::core::Result<UserActivityAttribution>;
    fn SetAttribution(&self, value: &::core::option::Option<UserActivityAttribution>) -> ::windows::core::Result<()>;
    fn SetContent(&self, value: &::core::option::Option<super::super::UI::Shell::IAdaptiveCard>) -> ::windows::core::Result<()>;
    fn Content(&self) -> ::windows::core::Result<super::super::UI::Shell::IAdaptiveCard>;
}
#[cfg(all(feature = "UI", feature = "UI_Shell", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityVisualElements {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityVisualElements";
}
#[cfg(all(feature = "UI", feature = "UI_Shell", feature = "implement_exclusive"))]
impl IUserActivityVisualElementsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityVisualElementsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityVisualElementsVtbl {
        unsafe extern "system" fn DisplayText<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayText<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Attribution<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attribution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttribution<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttribution(&*(&value as *const <UserActivityAttribution as ::windows::core::Abi>::Abi as *const <UserActivityAttribution as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContent<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(&*(&value as *const <super::super::UI::Shell::IAdaptiveCard as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::IAdaptiveCard as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Content<Impl: IUserActivityVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUserActivityVisualElements>,
            ::windows::core::GetTrustLevel,
            DisplayText::<Impl, IMPL_OFFSET>,
            SetDisplayText::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor::<Impl, IMPL_OFFSET>,
            Attribution::<Impl, IMPL_OFFSET>,
            SetAttribution::<Impl, IMPL_OFFSET>,
            SetContent::<Impl, IMPL_OFFSET>,
            Content::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityVisualElements as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityVisualElements2Impl: Sized {
    fn AttributionDisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAttributionDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityVisualElements2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityVisualElements2";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityVisualElements2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityVisualElements2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityVisualElements2Vtbl {
        unsafe extern "system" fn AttributionDisplayText<Impl: IUserActivityVisualElements2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributionDisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributionDisplayText<Impl: IUserActivityVisualElements2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttributionDisplayText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserActivityVisualElements2>, ::windows::core::GetTrustLevel, AttributionDisplayText::<Impl, IMPL_OFFSET>, SetAttributionDisplayText::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityVisualElements2 as ::windows::core::Interface>::IID
    }
}
