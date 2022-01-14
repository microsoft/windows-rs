#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivity_Impl: Sized {
    fn State(&mut self) -> ::windows::core::Result<UserActivityState>;
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VisualElements(&mut self) -> ::windows::core::Result<UserActivityVisualElements>;
    fn ContentUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FallbackUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetFallbackUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ActivationUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetActivationUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentInfo(&mut self) -> ::windows::core::Result<IUserActivityContentInfo>;
    fn SetContentInfo(&mut self, value: &::core::option::Option<IUserActivityContentInfo>) -> ::windows::core::Result<()>;
    fn SaveAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateSession(&mut self) -> ::windows::core::Result<UserActivitySession>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivity {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivity";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivity_Vtbl {
        unsafe extern "system" fn State<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserActivityState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActivityId<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VisualElements<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentUri<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentUri<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentType<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentType<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FallbackUri<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFallbackUri<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFallbackUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActivationUri<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetActivationUri<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentInfo<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentInfo<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentInfo(&*(&value as *const <IUserActivityContentInfo as ::windows::core::Abi>::Abi as *const <IUserActivityContentInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SaveAsync<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateSession<Impl: IUserActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivity, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            VisualElements: VisualElements::<Impl, IMPL_OFFSET>,
            ContentUri: ContentUri::<Impl, IMPL_OFFSET>,
            SetContentUri: SetContentUri::<Impl, IMPL_OFFSET>,
            ContentType: ContentType::<Impl, IMPL_OFFSET>,
            SetContentType: SetContentType::<Impl, IMPL_OFFSET>,
            FallbackUri: FallbackUri::<Impl, IMPL_OFFSET>,
            SetFallbackUri: SetFallbackUri::<Impl, IMPL_OFFSET>,
            ActivationUri: ActivationUri::<Impl, IMPL_OFFSET>,
            SetActivationUri: SetActivationUri::<Impl, IMPL_OFFSET>,
            ContentInfo: ContentInfo::<Impl, IMPL_OFFSET>,
            SetContentInfo: SetContentInfo::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
            CreateSession: CreateSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivity2_Impl: Sized {
    fn ToJson(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivity2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivity2";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivity2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivity2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivity2_Vtbl {
        unsafe extern "system" fn ToJson<Impl: IUserActivity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivity2, BASE_OFFSET>(), ToJson: ToJson::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivity2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivity3_Impl: Sized {
    fn IsRoamable(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRoamable(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivity3 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivity3";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivity3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivity3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivity3_Vtbl {
        unsafe extern "system" fn IsRoamable<Impl: IUserActivity3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsRoamable<Impl: IUserActivity3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRoamable(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivity3, BASE_OFFSET>(),
            IsRoamable: IsRoamable::<Impl, IMPL_OFFSET>,
            SetIsRoamable: SetIsRoamable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivity3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityAttribution_Impl: Sized {
    fn IconUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetIconUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AlternateText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAlternateText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddImageQuery(&mut self) -> ::windows::core::Result<bool>;
    fn SetAddImageQuery(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityAttribution {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityAttribution";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityAttribution_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityAttribution_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityAttribution_Vtbl {
        unsafe extern "system" fn IconUri<Impl: IUserActivityAttribution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIconUri<Impl: IUserActivityAttribution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlternateText<Impl: IUserActivityAttribution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAlternateText<Impl: IUserActivityAttribution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlternateText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddImageQuery<Impl: IUserActivityAttribution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAddImageQuery<Impl: IUserActivityAttribution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddImageQuery(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityAttribution, BASE_OFFSET>(),
            IconUri: IconUri::<Impl, IMPL_OFFSET>,
            SetIconUri: SetIconUri::<Impl, IMPL_OFFSET>,
            AlternateText: AlternateText::<Impl, IMPL_OFFSET>,
            SetAlternateText: SetAlternateText::<Impl, IMPL_OFFSET>,
            AddImageQuery: AddImageQuery::<Impl, IMPL_OFFSET>,
            SetAddImageQuery: SetAddImageQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityAttribution as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityAttributionFactory_Impl: Sized {
    fn CreateWithUri(&mut self, iconuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<UserActivityAttribution>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityAttributionFactory {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityAttributionFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityAttributionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityAttributionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityAttributionFactory_Vtbl {
        unsafe extern "system" fn CreateWithUri<Impl: IUserActivityAttributionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iconuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityAttributionFactory, BASE_OFFSET>(),
            CreateWithUri: CreateWithUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityAttributionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityChannel_Impl: Sized {
    fn GetOrCreateUserActivityAsync(&mut self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserActivity>>;
    fn DeleteActivityAsync(&mut self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAllActivitiesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityChannel {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannel";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannel_Vtbl {
        unsafe extern "system" fn GetOrCreateUserActivityAsync<Impl: IUserActivityChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteActivityAsync<Impl: IUserActivityChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteAllActivitiesAsync<Impl: IUserActivityChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityChannel, BASE_OFFSET>(),
            GetOrCreateUserActivityAsync: GetOrCreateUserActivityAsync::<Impl, IMPL_OFFSET>,
            DeleteActivityAsync: DeleteActivityAsync::<Impl, IMPL_OFFSET>,
            DeleteAllActivitiesAsync: DeleteAllActivitiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserActivityChannel2_Impl: Sized {
    fn GetRecentUserActivitiesAsync(&mut self, maxuniqueactivities: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>;
    fn GetSessionHistoryItemsForUserActivityAsync(&mut self, activityid: &::windows::core::HSTRING, starttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityChannel2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannel2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserActivityChannel2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannel2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannel2_Vtbl {
        unsafe extern "system" fn GetRecentUserActivitiesAsync<Impl: IUserActivityChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxuniqueactivities: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSessionHistoryItemsForUserActivityAsync<Impl: IUserActivityChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, starttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityChannel2, BASE_OFFSET>(),
            GetRecentUserActivitiesAsync: GetRecentUserActivitiesAsync::<Impl, IMPL_OFFSET>,
            GetSessionHistoryItemsForUserActivityAsync: GetSessionHistoryItemsForUserActivityAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannel2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityChannelStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<UserActivityChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityChannelStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannelStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityChannelStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannelStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannelStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IUserActivityChannelStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityChannelStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannelStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IUserActivityChannelStatics2_Impl: Sized {
    fn DisableAutoSessionCreation(&mut self) -> ::windows::core::Result<()>;
    fn TryGetForWebAccount(&mut self, account: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<UserActivityChannel>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityChannelStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannelStatics2";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IUserActivityChannelStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannelStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannelStatics2_Vtbl {
        unsafe extern "system" fn DisableAutoSessionCreation<Impl: IUserActivityChannelStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableAutoSessionCreation().into()
        }
        unsafe extern "system" fn TryGetForWebAccount<Impl: IUserActivityChannelStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityChannelStatics2, BASE_OFFSET>(),
            DisableAutoSessionCreation: DisableAutoSessionCreation::<Impl, IMPL_OFFSET>,
            TryGetForWebAccount: TryGetForWebAccount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannelStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IUserActivityChannelStatics3_Impl: Sized {
    fn GetForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<UserActivityChannel>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityChannelStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityChannelStatics3";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IUserActivityChannelStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityChannelStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityChannelStatics3_Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IUserActivityChannelStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityChannelStatics3, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityChannelStatics3 as ::windows::core::Interface>::IID
    }
}
pub trait IUserActivityContentInfo_Impl: Sized {
    fn ToJson(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IUserActivityContentInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityContentInfo";
}
impl IUserActivityContentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityContentInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityContentInfo_Vtbl {
        unsafe extern "system" fn ToJson<Impl: IUserActivityContentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityContentInfo, BASE_OFFSET>(), ToJson: ToJson::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityContentInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityContentInfoStatics_Impl: Sized {
    fn FromJson(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivityContentInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityContentInfoStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityContentInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityContentInfoStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityContentInfoStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityContentInfoStatics_Vtbl {
        unsafe extern "system" fn FromJson<Impl: IUserActivityContentInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityContentInfoStatics, BASE_OFFSET>(),
            FromJson: FromJson::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityContentInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityFactory_Impl: Sized {
    fn CreateWithActivityId(&mut self, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityFactory {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityFactory_Vtbl {
        unsafe extern "system" fn CreateWithActivityId<Impl: IUserActivityFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityFactory, BASE_OFFSET>(),
            CreateWithActivityId: CreateWithActivityId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityRequest_Impl: Sized {
    fn SetUserActivity(&mut self, activity: &::core::option::Option<UserActivity>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityRequest_Vtbl {
        unsafe extern "system" fn SetUserActivity<Impl: IUserActivityRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activity: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserActivity(&*(&activity as *const <UserActivity as ::windows::core::Abi>::Abi as *const <UserActivity as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityRequest, BASE_OFFSET>(),
            SetUserActivity: SetUserActivity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityRequestManager_Impl: Sized {
    fn UserActivityRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UserActivityRequestManager, UserActivityRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserActivityRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityRequestManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityRequestManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityRequestManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityRequestManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityRequestManager_Vtbl {
        unsafe extern "system" fn UserActivityRequested<Impl: IUserActivityRequestManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUserActivityRequested<Impl: IUserActivityRequestManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserActivityRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityRequestManager, BASE_OFFSET>(),
            UserActivityRequested: UserActivityRequested::<Impl, IMPL_OFFSET>,
            RemoveUserActivityRequested: RemoveUserActivityRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequestManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityRequestManagerStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<UserActivityRequestManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityRequestManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityRequestManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityRequestManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityRequestManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityRequestManagerStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IUserActivityRequestManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityRequestManagerStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequestManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivityRequestedEventArgs_Impl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<UserActivityRequest>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivityRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Request<Impl: IUserActivityRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IUserActivityRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityRequestedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivitySession_Impl: Sized {
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivitySession {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivitySession";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivitySession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivitySession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivitySession_Vtbl {
        unsafe extern "system" fn ActivityId<Impl: IUserActivitySession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivitySession, BASE_OFFSET>(), ActivityId: ActivityId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivitySession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserActivitySessionHistoryItem_Impl: Sized {
    fn UserActivity(&mut self) -> ::windows::core::Result<UserActivity>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn EndTime(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivitySessionHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivitySessionHistoryItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserActivitySessionHistoryItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivitySessionHistoryItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivitySessionHistoryItem_Vtbl {
        unsafe extern "system" fn UserActivity<Impl: IUserActivitySessionHistoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartTime<Impl: IUserActivitySessionHistoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndTime<Impl: IUserActivitySessionHistoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivitySessionHistoryItem, BASE_OFFSET>(),
            UserActivity: UserActivity::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            EndTime: EndTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivitySessionHistoryItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserActivityStatics_Impl: Sized {
    fn TryParseFromJson(&mut self, json: &::windows::core::HSTRING) -> ::windows::core::Result<UserActivity>;
    fn TryParseFromJsonArray(&mut self, json: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<UserActivity>>;
    fn ToJsonArray(&mut self, activities: &::core::option::Option<super::super::Foundation::Collections::IIterable<UserActivity>>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserActivityStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityStatics_Vtbl {
        unsafe extern "system" fn TryParseFromJson<Impl: IUserActivityStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryParseFromJsonArray<Impl: IUserActivityStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, json: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ToJsonArray<Impl: IUserActivityStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activities: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityStatics, BASE_OFFSET>(),
            TryParseFromJson: TryParseFromJson::<Impl, IMPL_OFFSET>,
            TryParseFromJsonArray: TryParseFromJsonArray::<Impl, IMPL_OFFSET>,
            ToJsonArray: ToJsonArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "UI_Shell", feature = "implement_exclusive"))]
pub trait IUserActivityVisualElements_Impl: Sized {
    fn DisplayText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackgroundColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Attribution(&mut self) -> ::windows::core::Result<UserActivityAttribution>;
    fn SetAttribution(&mut self, value: &::core::option::Option<UserActivityAttribution>) -> ::windows::core::Result<()>;
    fn SetContent(&mut self, value: &::core::option::Option<super::super::UI::Shell::IAdaptiveCard>) -> ::windows::core::Result<()>;
    fn Content(&mut self) -> ::windows::core::Result<super::super::UI::Shell::IAdaptiveCard>;
}
#[cfg(all(feature = "UI", feature = "UI_Shell", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserActivityVisualElements {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityVisualElements";
}
#[cfg(all(feature = "UI", feature = "UI_Shell", feature = "implement_exclusive"))]
impl IUserActivityVisualElements_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityVisualElements_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityVisualElements_Vtbl {
        unsafe extern "system" fn DisplayText<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayText<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBackgroundColor<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Attribution<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAttribution<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttribution(&*(&value as *const <UserActivityAttribution as ::windows::core::Abi>::Abi as *const <UserActivityAttribution as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContent<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(&*(&value as *const <super::super::UI::Shell::IAdaptiveCard as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::IAdaptiveCard as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Content<Impl: IUserActivityVisualElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityVisualElements, BASE_OFFSET>(),
            DisplayText: DisplayText::<Impl, IMPL_OFFSET>,
            SetDisplayText: SetDisplayText::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            Attribution: Attribution::<Impl, IMPL_OFFSET>,
            SetAttribution: SetAttribution::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityVisualElements as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserActivityVisualElements2_Impl: Sized {
    fn AttributionDisplayText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAttributionDisplayText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserActivityVisualElements2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityVisualElements2";
}
#[cfg(feature = "implement_exclusive")]
impl IUserActivityVisualElements2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityVisualElements2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityVisualElements2_Vtbl {
        unsafe extern "system" fn AttributionDisplayText<Impl: IUserActivityVisualElements2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAttributionDisplayText<Impl: IUserActivityVisualElements2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttributionDisplayText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityVisualElements2, BASE_OFFSET>(),
            AttributionDisplayText: AttributionDisplayText::<Impl, IMPL_OFFSET>,
            SetAttributionDisplayText: SetAttributionDisplayText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityVisualElements2 as ::windows::core::Interface>::IID
    }
}
