#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserNotificationListenerImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserNotificationListenerAccessStatus>>;
    fn GetAccessStatus(&self) -> ::windows::core::Result<UserNotificationListenerAccessStatus>;
    fn NotificationChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<UserNotificationListener, super::UserNotificationChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNotificationChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetNotificationsAsync(&self, kinds: super::NotificationKinds) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::UserNotification>>>;
    fn GetNotification(&self, notificationid: u32) -> ::windows::core::Result<super::UserNotification>;
    fn ClearNotifications(&self) -> ::windows::core::Result<()>;
    fn RemoveNotification(&self, notificationid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserNotificationListener {
    const NAME: &'static str = "Windows.UI.Notifications.Management.IUserNotificationListener";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserNotificationListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserNotificationListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserNotificationListenerVtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IUserNotificationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessStatus<Impl: IUserNotificationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserNotificationListenerAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotificationChanged<Impl: IUserNotificationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<UserNotificationListener, super::UserNotificationChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<UserNotificationListener, super::UserNotificationChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNotificationChanged<Impl: IUserNotificationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNotificationChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetNotificationsAsync<Impl: IUserNotificationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kinds: super::NotificationKinds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotificationsAsync(kinds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNotification<Impl: IUserNotificationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotification(notificationid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearNotifications<Impl: IUserNotificationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearNotifications().into()
        }
        unsafe extern "system" fn RemoveNotification<Impl: IUserNotificationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNotification(notificationid).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUserNotificationListener>,
            ::windows::core::GetTrustLevel,
            RequestAccessAsync::<Impl, IMPL_OFFSET>,
            GetAccessStatus::<Impl, IMPL_OFFSET>,
            NotificationChanged::<Impl, IMPL_OFFSET>,
            RemoveNotificationChanged::<Impl, IMPL_OFFSET>,
            GetNotificationsAsync::<Impl, IMPL_OFFSET>,
            GetNotification::<Impl, IMPL_OFFSET>,
            ClearNotifications::<Impl, IMPL_OFFSET>,
            RemoveNotification::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserNotificationListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserNotificationListenerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<UserNotificationListener>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserNotificationListenerStatics {
    const NAME: &'static str = "Windows.UI.Notifications.Management.IUserNotificationListenerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserNotificationListenerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserNotificationListenerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserNotificationListenerStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IUserNotificationListenerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserNotificationListenerStatics>, ::windows::core::GetTrustLevel, Current::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserNotificationListenerStatics as ::windows::core::Interface>::IID
    }
}
