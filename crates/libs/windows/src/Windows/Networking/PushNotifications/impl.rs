#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPushNotificationChannel_Impl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpirationTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn PushNotificationReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePushNotificationReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPushNotificationChannel {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannel";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPushNotificationChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationChannel_Vtbl {
        unsafe extern "system" fn Uri<Impl: IPushNotificationChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpirationTime<Impl: IPushNotificationChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Close<Impl: IPushNotificationChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn PushNotificationReceived<Impl: IPushNotificationChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePushNotificationReceived<Impl: IPushNotificationChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePushNotificationReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationChannel, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            PushNotificationReceived: PushNotificationReceived::<Impl, IMPL_OFFSET>,
            RemovePushNotificationReceived: RemovePushNotificationReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IPushNotificationChannelManagerForUser_Impl: Sized {
    fn CreatePushNotificationChannelForApplicationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForApplicationAsyncWithId(&mut self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForSecondaryTileAsync(&mut self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerForUser {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IPushNotificationChannelManagerForUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerForUser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationChannelManagerForUser_Vtbl {
        unsafe extern "system" fn CreatePushNotificationChannelForApplicationAsync<Impl: IPushNotificationChannelManagerForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePushNotificationChannelForApplicationAsyncWithId<Impl: IPushNotificationChannelManagerForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePushNotificationChannelForSecondaryTileAsync<Impl: IPushNotificationChannelManagerForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn User<Impl: IPushNotificationChannelManagerForUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationChannelManagerForUser, BASE_OFFSET>(),
            CreatePushNotificationChannelForApplicationAsync: CreatePushNotificationChannelForApplicationAsync::<Impl, IMPL_OFFSET>,
            CreatePushNotificationChannelForApplicationAsyncWithId: CreatePushNotificationChannelForApplicationAsyncWithId::<Impl, IMPL_OFFSET>,
            CreatePushNotificationChannelForSecondaryTileAsync: CreatePushNotificationChannelForSecondaryTileAsync::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationChannelManagerForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPushNotificationChannelManagerForUser2_Impl: Sized {
    fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync(&mut self, appserverkey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, channelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId(&mut self, appserverkey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, channelid: &::windows::core::HSTRING, appid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerForUser2 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerForUser2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPushNotificationChannelManagerForUser2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerForUser2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationChannelManagerForUser2_Vtbl {
        unsafe extern "system" fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync<Impl: IPushNotificationChannelManagerForUser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appserverkey: ::windows::core::RawPtr, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId<Impl: IPushNotificationChannelManagerForUser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appserverkey: ::windows::core::RawPtr, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationChannelManagerForUser2, BASE_OFFSET>(),
            CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync::<Impl, IMPL_OFFSET>,
            CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationChannelManagerForUser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPushNotificationChannelManagerStatics_Impl: Sized {
    fn CreatePushNotificationChannelForApplicationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForApplicationAsyncWithId(&mut self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
    fn CreatePushNotificationChannelForSecondaryTileAsync(&mut self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerStatics {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPushNotificationChannelManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationChannelManagerStatics_Vtbl {
        unsafe extern "system" fn CreatePushNotificationChannelForApplicationAsync<Impl: IPushNotificationChannelManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePushNotificationChannelForApplicationAsyncWithId<Impl: IPushNotificationChannelManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePushNotificationChannelForSecondaryTileAsync<Impl: IPushNotificationChannelManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationChannelManagerStatics, BASE_OFFSET>(),
            CreatePushNotificationChannelForApplicationAsync: CreatePushNotificationChannelForApplicationAsync::<Impl, IMPL_OFFSET>,
            CreatePushNotificationChannelForApplicationAsyncWithId: CreatePushNotificationChannelForApplicationAsyncWithId::<Impl, IMPL_OFFSET>,
            CreatePushNotificationChannelForSecondaryTileAsync: CreatePushNotificationChannelForSecondaryTileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationChannelManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IPushNotificationChannelManagerStatics2_Impl: Sized {
    fn GetForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<PushNotificationChannelManagerForUser>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerStatics2 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IPushNotificationChannelManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationChannelManagerStatics2_Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IPushNotificationChannelManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationChannelManagerStatics2, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationChannelManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelManagerStatics3_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<PushNotificationChannelManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerStatics3 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationChannelManagerStatics3_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IPushNotificationChannelManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationChannelManagerStatics3, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationChannelManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPushNotificationChannelManagerStatics4_Impl: Sized {
    fn ChannelsRevoked(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<PushNotificationChannelsRevokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChannelsRevoked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPushNotificationChannelManagerStatics4 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelManagerStatics4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPushNotificationChannelManagerStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelManagerStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationChannelManagerStatics4_Vtbl {
        unsafe extern "system" fn ChannelsRevoked<Impl: IPushNotificationChannelManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveChannelsRevoked<Impl: IPushNotificationChannelManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChannelsRevoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationChannelManagerStatics4, BASE_OFFSET>(),
            ChannelsRevoked: ChannelsRevoked::<Impl, IMPL_OFFSET>,
            RemoveChannelsRevoked: RemoveChannelsRevoked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationChannelManagerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationChannelsRevokedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationChannelsRevokedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationChannelsRevokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationChannelsRevokedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationChannelsRevokedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationChannelsRevokedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationChannelsRevokedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationChannelsRevokedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
pub trait IPushNotificationReceivedEventArgs_Impl: Sized {
    fn SetCancel(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<bool>;
    fn NotificationType(&mut self) -> ::windows::core::Result<PushNotificationType>;
    fn ToastNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn TileNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn BadgeNotification(&mut self) -> ::windows::core::Result<super::super::UI::Notifications::BadgeNotification>;
    fn RawNotification(&mut self) -> ::windows::core::Result<RawNotification>;
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPushNotificationReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IPushNotificationReceivedEventArgs";
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl IPushNotificationReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationReceivedEventArgs_Vtbl {
        unsafe extern "system" fn SetCancel<Impl: IPushNotificationReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn Cancel<Impl: IPushNotificationReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NotificationType<Impl: IPushNotificationReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PushNotificationType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ToastNotification<Impl: IPushNotificationReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TileNotification<Impl: IPushNotificationReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BadgeNotification<Impl: IPushNotificationReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RawNotification<Impl: IPushNotificationReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationReceivedEventArgs, BASE_OFFSET>(),
            SetCancel: SetCancel::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            NotificationType: NotificationType::<Impl, IMPL_OFFSET>,
            ToastNotification: ToastNotification::<Impl, IMPL_OFFSET>,
            TileNotification: TileNotification::<Impl, IMPL_OFFSET>,
            BadgeNotification: BadgeNotification::<Impl, IMPL_OFFSET>,
            RawNotification: RawNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRawNotification_Impl: Sized {
    fn Content(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRawNotification {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IRawNotification";
}
#[cfg(feature = "implement_exclusive")]
impl IRawNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawNotification_Vtbl {
        unsafe extern "system" fn Content<Impl: IRawNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRawNotification, BASE_OFFSET>(), Content: Content::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRawNotification2_Impl: Sized {
    fn Headers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn ChannelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRawNotification2 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IRawNotification2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRawNotification2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawNotification2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawNotification2_Vtbl {
        unsafe extern "system" fn Headers<Impl: IRawNotification2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChannelId<Impl: IRawNotification2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRawNotification2, BASE_OFFSET>(),
            Headers: Headers::<Impl, IMPL_OFFSET>,
            ChannelId: ChannelId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawNotification2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRawNotification3_Impl: Sized {
    fn ContentBytes(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRawNotification3 {
    const NAME: &'static str = "Windows.Networking.PushNotifications.IRawNotification3";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRawNotification3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawNotification3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawNotification3_Vtbl {
        unsafe extern "system" fn ContentBytes<Impl: IRawNotification3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRawNotification3, BASE_OFFSET>(), ContentBytes: ContentBytes::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawNotification3 as ::windows::core::Interface>::IID
    }
}
