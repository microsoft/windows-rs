#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILockApplicationHostImpl: Sized {
    fn RequestUnlock(&mut self) -> ::windows::core::Result<()>;
    fn Unlocking(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockApplicationHost, LockScreenUnlockingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnlocking(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockApplicationHost {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockApplicationHost";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILockApplicationHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockApplicationHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockApplicationHostVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILockApplicationHost, BASE_OFFSET>(),
            RequestUnlock: RequestUnlock::<Impl, IMPL_OFFSET>,
            Unlocking: Unlocking::<Impl, IMPL_OFFSET>,
            RemoveUnlocking: RemoveUnlocking::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockApplicationHost as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockApplicationHostStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<LockApplicationHost>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockApplicationHostStatics {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockApplicationHostStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILockApplicationHostStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockApplicationHostStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockApplicationHostStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILockApplicationHostStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockApplicationHostStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ILockScreenBadgeImpl: Sized {
    fn Logo(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn Glyph(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn Number(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn AutomationName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchApp(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockScreenBadge {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockScreenBadge";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ILockScreenBadgeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenBadgeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenBadgeVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenBadge, BASE_OFFSET>(),
            Logo: Logo::<Impl, IMPL_OFFSET>,
            Glyph: Glyph::<Impl, IMPL_OFFSET>,
            Number: Number::<Impl, IMPL_OFFSET>,
            AutomationName: AutomationName::<Impl, IMPL_OFFSET>,
            LaunchApp: LaunchApp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenBadge as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ILockScreenInfoImpl: Sized {
    fn LockScreenImageChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLockScreenImageChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LockScreenImage(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn BadgesChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBadgesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Badges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<LockScreenBadge>>;
    fn DetailTextChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDetailTextChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DetailText(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn AlarmIconChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAlarmIconChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AlarmIcon(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockScreenInfo {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockScreenInfo";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ILockScreenInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenInfo, BASE_OFFSET>(),
            LockScreenImageChanged: LockScreenImageChanged::<Impl, IMPL_OFFSET>,
            RemoveLockScreenImageChanged: RemoveLockScreenImageChanged::<Impl, IMPL_OFFSET>,
            LockScreenImage: LockScreenImage::<Impl, IMPL_OFFSET>,
            BadgesChanged: BadgesChanged::<Impl, IMPL_OFFSET>,
            RemoveBadgesChanged: RemoveBadgesChanged::<Impl, IMPL_OFFSET>,
            Badges: Badges::<Impl, IMPL_OFFSET>,
            DetailTextChanged: DetailTextChanged::<Impl, IMPL_OFFSET>,
            RemoveDetailTextChanged: RemoveDetailTextChanged::<Impl, IMPL_OFFSET>,
            DetailText: DetailText::<Impl, IMPL_OFFSET>,
            AlarmIconChanged: AlarmIconChanged::<Impl, IMPL_OFFSET>,
            RemoveAlarmIconChanged: RemoveAlarmIconChanged::<Impl, IMPL_OFFSET>,
            AlarmIcon: AlarmIcon::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenUnlockingDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILockScreenUnlockingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockScreenUnlockingDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl ILockScreenUnlockingDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenUnlockingDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenUnlockingDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: ILockScreenUnlockingDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenUnlockingDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenUnlockingDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILockScreenUnlockingEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<LockScreenUnlockingDeferral>;
    fn Deadline(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILockScreenUnlockingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.LockScreen.ILockScreenUnlockingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILockScreenUnlockingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILockScreenUnlockingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILockScreenUnlockingEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILockScreenUnlockingEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILockScreenUnlockingEventArgs as ::windows::core::Interface>::IID
    }
}
