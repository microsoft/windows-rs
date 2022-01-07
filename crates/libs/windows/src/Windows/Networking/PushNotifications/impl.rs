#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn PushNotificationReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePushNotificationReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannel {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannel";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelImpl, const OFFSET: isize>() -> IPushNotificationChannelVtbl {
        unsafe extern "system" fn Uri<Impl: IPushNotificationChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationTime<Impl: IPushNotificationChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IPushNotificationChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn PushNotificationReceived<Impl: IPushNotificationChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushNotificationReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePushNotificationReceived<Impl: IPushNotificationChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePushNotificationReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPushNotificationChannel>, ::windows::core::GetTrustLevel, Uri::<Impl, OFFSET>, ExpirationTime::<Impl, OFFSET>, Close::<Impl, OFFSET>, PushNotificationReceived::<Impl, OFFSET>, RemovePushNotificationReceived::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerForUserImpl: Sized {
    fn CreatePushNotificationChannelForApplicationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForApplicationAsyncWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerForUser {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelManagerForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerForUserImpl, const OFFSET: isize>() -> IPushNotificationChannelManagerForUserVtbl {
        unsafe extern "system" fn CreatePushNotificationChannelForApplicationAsync<Impl: IPushNotificationChannelManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePushNotificationChannelForApplicationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePushNotificationChannelForApplicationAsyncWithId<Impl: IPushNotificationChannelManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePushNotificationChannelForApplicationAsyncWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePushNotificationChannelForSecondaryTileAsync<Impl: IPushNotificationChannelManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePushNotificationChannelForSecondaryTileAsync(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IPushNotificationChannelManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPushNotificationChannelManagerForUser>,
            ::windows::core::GetTrustLevel,
            CreatePushNotificationChannelForApplicationAsync::<Impl, OFFSET>,
            CreatePushNotificationChannelForApplicationAsyncWithId::<Impl, OFFSET>,
            CreatePushNotificationChannelForSecondaryTileAsync::<Impl, OFFSET>,
            User::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerForUser2Impl: Sized {
    fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync(&self, appserverkey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, channelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId(&self, appserverkey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, channelid: &::windows::core::HSTRING, appid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerForUser2 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser2";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelManagerForUser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerForUser2Impl, const OFFSET: isize>() -> IPushNotificationChannelManagerForUser2Vtbl {
        unsafe extern "system" fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync<Impl: IPushNotificationChannelManagerForUser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appserverkey: ::windows::core::RawPtr, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync(&*(&appserverkey as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&channelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId<Impl: IPushNotificationChannelManagerForUser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appserverkey: ::windows::core::RawPtr, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId(
                &*(&appserverkey as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&channelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&appid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPushNotificationChannelManagerForUser2>, ::windows::core::GetTrustLevel, CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync::<Impl, OFFSET>, CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerStaticsImpl: Sized {
    fn CreatePushNotificationChannelForApplicationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForApplicationAsyncWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerStatics {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerStaticsImpl, const OFFSET: isize>() -> IPushNotificationChannelManagerStaticsVtbl {
        unsafe extern "system" fn CreatePushNotificationChannelForApplicationAsync<Impl: IPushNotificationChannelManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePushNotificationChannelForApplicationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePushNotificationChannelForApplicationAsyncWithId<Impl: IPushNotificationChannelManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePushNotificationChannelForApplicationAsyncWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePushNotificationChannelForSecondaryTileAsync<Impl: IPushNotificationChannelManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePushNotificationChannelForSecondaryTileAsync(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IPushNotificationChannelManagerStatics>,
            ::windows::core::GetTrustLevel,
            CreatePushNotificationChannelForApplicationAsync::<Impl, OFFSET>,
            CreatePushNotificationChannelForApplicationAsyncWithId::<Impl, OFFSET>,
            CreatePushNotificationChannelForSecondaryTileAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<PushNotificationChannelManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerStatics2 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerStatics2Impl, const OFFSET: isize>() -> IPushNotificationChannelManagerStatics2Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IPushNotificationChannelManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPushNotificationChannelManagerStatics2>, ::windows::core::GetTrustLevel, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerStatics3Impl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PushNotificationChannelManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerStatics3 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerStatics3Impl, const OFFSET: isize>() -> IPushNotificationChannelManagerStatics3Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IPushNotificationChannelManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPushNotificationChannelManagerStatics3>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerStatics4Impl: Sized {
    fn ChannelsRevoked(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<PushNotificationChannelsRevokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChannelsRevoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerStatics4 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelManagerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerStatics4Impl, const OFFSET: isize>() -> IPushNotificationChannelManagerStatics4Vtbl {
        unsafe extern "system" fn ChannelsRevoked<Impl: IPushNotificationChannelManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelsRevoked(&*(&handler as *const <super::super::Foundation::EventHandler<PushNotificationChannelsRevokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<PushNotificationChannelsRevokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChannelsRevoked<Impl: IPushNotificationChannelManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChannelsRevoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPushNotificationChannelManagerStatics4>, ::windows::core::GetTrustLevel, ChannelsRevoked::<Impl, OFFSET>, RemoveChannelsRevoked::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelsRevokedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannelsRevokedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelsRevokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelsRevokedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelsRevokedEventArgsImpl, const OFFSET: isize>() -> IPushNotificationChannelsRevokedEventArgsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPushNotificationChannelsRevokedEventArgs>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationReceivedEventArgsImpl: Sized {
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn NotificationType(&self) -> ::windows::core::Result<PushNotificationType>;
    fn ToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn TileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn BadgeNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::BadgeNotification>;
    fn RawNotification(&self) -> ::windows::core::Result<RawNotification>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationReceivedEventArgsImpl, const OFFSET: isize>() -> IPushNotificationReceivedEventArgsVtbl {
        unsafe extern "system" fn SetCancel<Impl: IPushNotificationReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn Cancel<Impl: IPushNotificationReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotificationType<Impl: IPushNotificationReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PushNotificationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToastNotification<Impl: IPushNotificationReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToastNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileNotification<Impl: IPushNotificationReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BadgeNotification<Impl: IPushNotificationReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BadgeNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawNotification<Impl: IPushNotificationReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawNotification() {
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
            ::windows::core::GetRuntimeClassName::<IPushNotificationReceivedEventArgs>,
            ::windows::core::GetTrustLevel,
            SetCancel::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
            NotificationType::<Impl, OFFSET>,
            ToastNotification::<Impl, OFFSET>,
            TileNotification::<Impl, OFFSET>,
            BadgeNotification::<Impl, OFFSET>,
            RawNotification::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRawNotification {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IRawNotification";
}
#[cfg(feature = "implement_exclusive")]
impl IRawNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawNotificationImpl, const OFFSET: isize>() -> IRawNotificationVtbl {
        unsafe extern "system" fn Content<Impl: IRawNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawNotification>, ::windows::core::GetTrustLevel, Content::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawNotification2Impl: Sized {
    fn Headers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn ChannelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRawNotification2 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IRawNotification2";
}
#[cfg(feature = "implement_exclusive")]
impl IRawNotification2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawNotification2Impl, const OFFSET: isize>() -> IRawNotification2Vtbl {
        unsafe extern "system" fn Headers<Impl: IRawNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Headers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelId<Impl: IRawNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawNotification2>, ::windows::core::GetTrustLevel, Headers::<Impl, OFFSET>, ChannelId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawNotification3Impl: Sized {
    fn ContentBytes(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRawNotification3 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IRawNotification3";
}
#[cfg(feature = "implement_exclusive")]
impl IRawNotification3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawNotification3Impl, const OFFSET: isize>() -> IRawNotification3Vtbl {
        unsafe extern "system" fn ContentBytes<Impl: IRawNotification3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawNotification3>, ::windows::core::GetTrustLevel, ContentBytes::<Impl, OFFSET>)
    }
}
