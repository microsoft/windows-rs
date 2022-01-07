#[cfg(feature = "implement_exclusive")]
pub trait ILockApplicationHostImpl: Sized {
    fn RequestUnlock(&self) -> ::windows::core::Result<()>;
    fn Unlocking(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockApplicationHost, LockScreenUnlockingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnlocking(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockApplicationHost {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockApplicationHost";
}
#[cfg(feature = "implement_exclusive")]
impl ILockApplicationHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockApplicationHostImpl, const OFFSET: isize>() -> ILockApplicationHostVtbl {
        unsafe extern "system" fn RequestUnlock<Impl: ILockApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestUnlock().into()
        }
        unsafe extern "system" fn Unlocking<Impl: ILockApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unlocking(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LockApplicationHost, LockScreenUnlockingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LockApplicationHost, LockScreenUnlockingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnlocking<Impl: ILockApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUnlocking(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILockApplicationHost>, ::windows::core::GetTrustLevel, RequestUnlock::<Impl, OFFSET>, Unlocking::<Impl, OFFSET>, RemoveUnlocking::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockApplicationHostStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<LockApplicationHost>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockApplicationHostStatics {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockApplicationHostStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILockApplicationHostStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockApplicationHostStaticsImpl, const OFFSET: isize>() -> ILockApplicationHostStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ILockApplicationHostStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILockApplicationHostStatics>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenBadgeImpl: Sized {
    fn Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn Glyph(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn Number(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn AutomationName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchApp(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockScreenBadge {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockScreenBadge";
}
#[cfg(feature = "implement_exclusive")]
impl ILockScreenBadgeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenBadgeImpl, const OFFSET: isize>() -> ILockScreenBadgeVtbl {
        unsafe extern "system" fn Logo<Impl: ILockScreenBadgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Glyph<Impl: ILockScreenBadgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Glyph() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Number<Impl: ILockScreenBadgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Number() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutomationName<Impl: ILockScreenBadgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchApp<Impl: ILockScreenBadgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LaunchApp().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILockScreenBadge>, ::windows::core::GetTrustLevel, Logo::<Impl, OFFSET>, Glyph::<Impl, OFFSET>, Number::<Impl, OFFSET>, AutomationName::<Impl, OFFSET>, LaunchApp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenInfoImpl: Sized {
    fn LockScreenImageChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLockScreenImageChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LockScreenImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn BadgesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBadgesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Badges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<LockScreenBadge>>;
    fn DetailTextChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDetailTextChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DetailText(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn AlarmIconChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAlarmIconChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AlarmIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockScreenInfo {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockScreenInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ILockScreenInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenInfoImpl, const OFFSET: isize>() -> ILockScreenInfoVtbl {
        unsafe extern "system" fn LockScreenImageChanged<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockScreenImageChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLockScreenImageChanged<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLockScreenImageChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LockScreenImage<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockScreenImage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BadgesChanged<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BadgesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBadgesChanged<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBadgesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Badges<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Badges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailTextChanged<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailTextChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDetailTextChanged<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDetailTextChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DetailText<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlarmIconChanged<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlarmIconChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAlarmIconChanged<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAlarmIconChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlarmIcon<Impl: ILockScreenInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlarmIcon() {
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
            ::windows::core::GetRuntimeClassName::<ILockScreenInfo>,
            ::windows::core::GetTrustLevel,
            LockScreenImageChanged::<Impl, OFFSET>,
            RemoveLockScreenImageChanged::<Impl, OFFSET>,
            LockScreenImage::<Impl, OFFSET>,
            BadgesChanged::<Impl, OFFSET>,
            RemoveBadgesChanged::<Impl, OFFSET>,
            Badges::<Impl, OFFSET>,
            DetailTextChanged::<Impl, OFFSET>,
            RemoveDetailTextChanged::<Impl, OFFSET>,
            DetailText::<Impl, OFFSET>,
            AlarmIconChanged::<Impl, OFFSET>,
            RemoveAlarmIconChanged::<Impl, OFFSET>,
            AlarmIcon::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenUnlockingDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockScreenUnlockingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockScreenUnlockingDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl ILockScreenUnlockingDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenUnlockingDeferralImpl, const OFFSET: isize>() -> ILockScreenUnlockingDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: ILockScreenUnlockingDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILockScreenUnlockingDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenUnlockingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<LockScreenUnlockingDeferral>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockScreenUnlockingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockScreenUnlockingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILockScreenUnlockingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenUnlockingEventArgsImpl, const OFFSET: isize>() -> ILockScreenUnlockingEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: ILockScreenUnlockingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Deadline<Impl: ILockScreenUnlockingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILockScreenUnlockingEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, OFFSET>, Deadline::<Impl, OFFSET>)
    }
}
