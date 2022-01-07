pub trait IAdaptiveNotificationContentImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AdaptiveNotificationContentKind>;
    fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
impl ::windows::core::RuntimeName for IAdaptiveNotificationContent {
    const NAME: &'static str = "Windows.UI.Notifications.IAdaptiveNotificationContent";
}
impl IAdaptiveNotificationContentVtbl {
    pub const fn new<Impl: IAdaptiveNotificationContentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdaptiveNotificationContentVtbl {
        unsafe extern "system" fn Kind<Impl: IAdaptiveNotificationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveNotificationContentKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hints<Impl: IAdaptiveNotificationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdaptiveNotificationContent>, base.5, Kind::<Impl, OFFSET>, Hints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveNotificationTextImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveNotificationText {
    const NAME: &'static str = "Windows.UI.Notifications.IAdaptiveNotificationText";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveNotificationTextVtbl {
    pub const fn new<Impl: IAdaptiveNotificationTextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdaptiveNotificationTextVtbl {
        unsafe extern "system" fn Text<Impl: IAdaptiveNotificationTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: IAdaptiveNotificationTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: IAdaptiveNotificationTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IAdaptiveNotificationTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdaptiveNotificationText>, base.5, Text::<Impl, OFFSET>, SetText::<Impl, OFFSET>, Language::<Impl, OFFSET>, SetLanguage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBadgeNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeNotification";
}
#[cfg(feature = "implement_exclusive")]
impl IBadgeNotificationVtbl {
    pub const fn new<Impl: IBadgeNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBadgeNotificationVtbl {
        unsafe extern "system" fn Content<Impl: IBadgeNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: IBadgeNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: IBadgeNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBadgeNotification>, base.5, Content::<Impl, OFFSET>, SetExpirationTime::<Impl, OFFSET>, ExpirationTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeNotificationFactoryImpl: Sized {
    fn CreateBadgeNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<BadgeNotification>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBadgeNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeNotificationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBadgeNotificationFactoryVtbl {
    pub const fn new<Impl: IBadgeNotificationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBadgeNotificationFactoryVtbl {
        unsafe extern "system" fn CreateBadgeNotification<Impl: IBadgeNotificationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBadgeNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBadgeNotificationFactory>, base.5, CreateBadgeNotification::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeUpdateManagerForUserImpl: Sized {
    fn CreateBadgeUpdaterForApplication(&self) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBadgeUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeUpdateManagerForUser";
}
#[cfg(feature = "implement_exclusive")]
impl IBadgeUpdateManagerForUserVtbl {
    pub const fn new<Impl: IBadgeUpdateManagerForUserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBadgeUpdateManagerForUserVtbl {
        unsafe extern "system" fn CreateBadgeUpdaterForApplication<Impl: IBadgeUpdateManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForApplicationWithId<Impl: IBadgeUpdateManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForSecondaryTile<Impl: IBadgeUpdateManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IBadgeUpdateManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBadgeUpdateManagerForUser>, base.5, CreateBadgeUpdaterForApplication::<Impl, OFFSET>, CreateBadgeUpdaterForApplicationWithId::<Impl, OFFSET>, CreateBadgeUpdaterForSecondaryTile::<Impl, OFFSET>, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeUpdateManagerStaticsImpl: Sized {
    fn CreateBadgeUpdaterForApplication(&self) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn GetTemplateContent(&self, r#type: BadgeTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBadgeUpdateManagerStatics {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeUpdateManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBadgeUpdateManagerStaticsVtbl {
    pub const fn new<Impl: IBadgeUpdateManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBadgeUpdateManagerStaticsVtbl {
        unsafe extern "system" fn CreateBadgeUpdaterForApplication<Impl: IBadgeUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForApplicationWithId<Impl: IBadgeUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForSecondaryTile<Impl: IBadgeUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplateContent<Impl: IBadgeUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: BadgeTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTemplateContent(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBadgeUpdateManagerStatics>, base.5, CreateBadgeUpdaterForApplication::<Impl, OFFSET>, CreateBadgeUpdaterForApplicationWithId::<Impl, OFFSET>, CreateBadgeUpdaterForSecondaryTile::<Impl, OFFSET>, GetTemplateContent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeUpdateManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<BadgeUpdateManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBadgeUpdateManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeUpdateManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBadgeUpdateManagerStatics2Vtbl {
    pub const fn new<Impl: IBadgeUpdateManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBadgeUpdateManagerStatics2Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IBadgeUpdateManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBadgeUpdateManagerStatics2>, base.5, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBadgeUpdaterImpl: Sized {
    fn Update(&self, notification: &::core::option::Option<BadgeNotification>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdate(&self, badgecontent: &::core::option::Option<super::super::Foundation::Uri>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateAtTime(&self, badgecontent: &::core::option::Option<super::super::Foundation::Uri>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBadgeUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeUpdater";
}
#[cfg(feature = "implement_exclusive")]
impl IBadgeUpdaterVtbl {
    pub const fn new<Impl: IBadgeUpdaterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBadgeUpdaterVtbl {
        unsafe extern "system" fn Update<Impl: IBadgeUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Update(&*(&notification as *const <BadgeNotification as ::windows::core::Abi>::Abi as *const <BadgeNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: IBadgeUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn StartPeriodicUpdate<Impl: IBadgeUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, badgecontent: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdate(&*(&badgecontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StartPeriodicUpdateAtTime<Impl: IBadgeUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, badgecontent: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdateAtTime(&*(&badgecontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StopPeriodicUpdate<Impl: IBadgeUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopPeriodicUpdate().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBadgeUpdater>, base.5, Update::<Impl, OFFSET>, Clear::<Impl, OFFSET>, StartPeriodicUpdate::<Impl, OFFSET>, StartPeriodicUpdateAtTime::<Impl, OFFSET>, StopPeriodicUpdate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownAdaptiveNotificationHintsStaticsImpl: Sized {
    fn Style(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wrap(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxLines(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinLines(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TextStacking(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Align(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownAdaptiveNotificationHintsStatics {
    const NAME: &'static str = "Windows.UI.Notifications.IKnownAdaptiveNotificationHintsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownAdaptiveNotificationHintsStaticsVtbl {
    pub const fn new<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnownAdaptiveNotificationHintsStaticsVtbl {
        unsafe extern "system" fn Style<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Style() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wrap<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Wrap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLines<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinLines<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextStacking<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextStacking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Align<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Align() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKnownAdaptiveNotificationHintsStatics>, base.5, Style::<Impl, OFFSET>, Wrap::<Impl, OFFSET>, MaxLines::<Impl, OFFSET>, MinLines::<Impl, OFFSET>, TextStacking::<Impl, OFFSET>, Align::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownAdaptiveNotificationTextStylesStaticsImpl: Sized {
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Base(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subheader(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Header(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TitleNumeral(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubheaderNumeral(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HeaderNumeral(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CaptionSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BodySubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BaseSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubtitleSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TitleSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubheaderSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubheaderNumeralSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HeaderSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HeaderNumeralSubtle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownAdaptiveNotificationTextStylesStatics {
    const NAME: &'static str = "Windows.UI.Notifications.IKnownAdaptiveNotificationTextStylesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownAdaptiveNotificationTextStylesStaticsVtbl {
    pub const fn new<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnownAdaptiveNotificationTextStylesStaticsVtbl {
        unsafe extern "system" fn Caption<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Caption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Base<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Base() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subtitle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subtitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subheader<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subheader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Header<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Header() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleNumeral<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TitleNumeral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubheaderNumeral<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubheaderNumeral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderNumeral<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeaderNumeral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptionSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CaptionSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodySubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BodySubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BaseSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BaseSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubtitleSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubtitleSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TitleSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubheaderSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubheaderSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubheaderNumeralSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubheaderNumeralSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeaderSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderNumeralSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeaderNumeralSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IKnownAdaptiveNotificationTextStylesStatics>,
            base.5,
            Caption::<Impl, OFFSET>,
            Body::<Impl, OFFSET>,
            Base::<Impl, OFFSET>,
            Subtitle::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            Subheader::<Impl, OFFSET>,
            Header::<Impl, OFFSET>,
            TitleNumeral::<Impl, OFFSET>,
            SubheaderNumeral::<Impl, OFFSET>,
            HeaderNumeral::<Impl, OFFSET>,
            CaptionSubtle::<Impl, OFFSET>,
            BodySubtle::<Impl, OFFSET>,
            BaseSubtle::<Impl, OFFSET>,
            SubtitleSubtle::<Impl, OFFSET>,
            TitleSubtle::<Impl, OFFSET>,
            SubheaderSubtle::<Impl, OFFSET>,
            SubheaderNumeralSubtle::<Impl, OFFSET>,
            HeaderSubtle::<Impl, OFFSET>,
            HeaderNumeralSubtle::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownNotificationBindingsStaticsImpl: Sized {
    fn ToastGeneric(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownNotificationBindingsStatics {
    const NAME: &'static str = "Windows.UI.Notifications.IKnownNotificationBindingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownNotificationBindingsStaticsVtbl {
    pub const fn new<Impl: IKnownNotificationBindingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnownNotificationBindingsStaticsVtbl {
        unsafe extern "system" fn ToastGeneric<Impl: IKnownNotificationBindingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToastGeneric() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKnownNotificationBindingsStatics>, base.5, ToastGeneric::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationImpl: Sized {
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Visual(&self) -> ::windows::core::Result<NotificationVisual>;
    fn SetVisual(&self, value: &::core::option::Option<NotificationVisual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotification {
    const NAME: &'static str = "Windows.UI.Notifications.INotification";
}
#[cfg(feature = "implement_exclusive")]
impl INotificationVtbl {
    pub const fn new<Impl: INotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotificationVtbl {
        unsafe extern "system" fn ExpirationTime<Impl: INotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: INotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Visual<Impl: INotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisual<Impl: INotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVisual(&*(&value as *const <NotificationVisual as ::windows::core::Abi>::Abi as *const <NotificationVisual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotification>, base.5, ExpirationTime::<Impl, OFFSET>, SetExpirationTime::<Impl, OFFSET>, Visual::<Impl, OFFSET>, SetVisual::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationBindingImpl: Sized {
    fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTemplate(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn GetTextElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdaptiveNotificationText>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotificationBinding {
    const NAME: &'static str = "Windows.UI.Notifications.INotificationBinding";
}
#[cfg(feature = "implement_exclusive")]
impl INotificationBindingVtbl {
    pub const fn new<Impl: INotificationBindingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotificationBindingVtbl {
        unsafe extern "system" fn Template<Impl: INotificationBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemplate<Impl: INotificationBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTemplate(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: INotificationBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: INotificationBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hints<Impl: INotificationBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextElements<Impl: INotificationBindingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTextElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotificationBinding>, base.5, Template::<Impl, OFFSET>, SetTemplate::<Impl, OFFSET>, Language::<Impl, OFFSET>, SetLanguage::<Impl, OFFSET>, Hints::<Impl, OFFSET>, GetTextElements::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationDataImpl: Sized {
    fn Values(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn SequenceNumber(&self) -> ::windows::core::Result<u32>;
    fn SetSequenceNumber(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotificationData {
    const NAME: &'static str = "Windows.UI.Notifications.INotificationData";
}
#[cfg(feature = "implement_exclusive")]
impl INotificationDataVtbl {
    pub const fn new<Impl: INotificationDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotificationDataVtbl {
        unsafe extern "system" fn Values<Impl: INotificationDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SequenceNumber<Impl: INotificationDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SequenceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSequenceNumber<Impl: INotificationDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSequenceNumber(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotificationData>, base.5, Values::<Impl, OFFSET>, SequenceNumber::<Impl, OFFSET>, SetSequenceNumber::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationDataFactoryImpl: Sized {
    fn CreateNotificationDataWithValuesAndSequenceNumber(&self, initialvalues: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>, sequencenumber: u32) -> ::windows::core::Result<NotificationData>;
    fn CreateNotificationDataWithValues(&self, initialvalues: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<NotificationData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotificationDataFactory {
    const NAME: &'static str = "Windows.UI.Notifications.INotificationDataFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INotificationDataFactoryVtbl {
    pub const fn new<Impl: INotificationDataFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotificationDataFactoryVtbl {
        unsafe extern "system" fn CreateNotificationDataWithValuesAndSequenceNumber<Impl: INotificationDataFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalues: ::windows::core::RawPtr, sequencenumber: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNotificationDataWithValuesAndSequenceNumber(&*(&initialvalues as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType), sequencenumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNotificationDataWithValues<Impl: INotificationDataFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalues: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNotificationDataWithValues(&*(&initialvalues as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotificationDataFactory>, base.5, CreateNotificationDataWithValuesAndSequenceNumber::<Impl, OFFSET>, CreateNotificationDataWithValues::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INotificationVisualImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Bindings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<NotificationBinding>>;
    fn GetBinding(&self, templatename: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationBinding>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INotificationVisual {
    const NAME: &'static str = "Windows.UI.Notifications.INotificationVisual";
}
#[cfg(feature = "implement_exclusive")]
impl INotificationVisualVtbl {
    pub const fn new<Impl: INotificationVisualImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INotificationVisualVtbl {
        unsafe extern "system" fn Language<Impl: INotificationVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: INotificationVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bindings<Impl: INotificationVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bindings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBinding<Impl: INotificationVisualImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, templatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBinding(&*(&templatename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INotificationVisual>, base.5, Language::<Impl, OFFSET>, SetLanguage::<Impl, OFFSET>, Bindings::<Impl, OFFSET>, GetBinding::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledTileNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScheduledTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledTileNotification";
}
#[cfg(feature = "implement_exclusive")]
impl IScheduledTileNotificationVtbl {
    pub const fn new<Impl: IScheduledTileNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScheduledTileNotificationVtbl {
        unsafe extern "system" fn Content<Impl: IScheduledTileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeliveryTime<Impl: IScheduledTileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeliveryTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: IScheduledTileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: IScheduledTileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IScheduledTileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IScheduledTileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IScheduledTileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IScheduledTileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScheduledTileNotification>, base.5, Content::<Impl, OFFSET>, DeliveryTime::<Impl, OFFSET>, SetExpirationTime::<Impl, OFFSET>, ExpirationTime::<Impl, OFFSET>, SetTag::<Impl, OFFSET>, Tag::<Impl, OFFSET>, SetId::<Impl, OFFSET>, Id::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledTileNotificationFactoryImpl: Sized {
    fn CreateScheduledTileNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>, deliverytime: &super::super::Foundation::DateTime) -> ::windows::core::Result<ScheduledTileNotification>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScheduledTileNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledTileNotificationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IScheduledTileNotificationFactoryVtbl {
    pub const fn new<Impl: IScheduledTileNotificationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScheduledTileNotificationFactoryVtbl {
        unsafe extern "system" fn CreateScheduledTileNotification<Impl: IScheduledTileNotificationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, deliverytime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateScheduledTileNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType), &*(&deliverytime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScheduledTileNotificationFactory>, base.5, CreateScheduledTileNotification::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SnoozeInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn MaximumSnoozeCount(&self) -> ::windows::core::Result<u32>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScheduledToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotification";
}
#[cfg(feature = "implement_exclusive")]
impl IScheduledToastNotificationVtbl {
    pub const fn new<Impl: IScheduledToastNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScheduledToastNotificationVtbl {
        unsafe extern "system" fn Content<Impl: IScheduledToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeliveryTime<Impl: IScheduledToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeliveryTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnoozeInterval<Impl: IScheduledToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SnoozeInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumSnoozeCount<Impl: IScheduledToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaximumSnoozeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IScheduledToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IScheduledToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScheduledToastNotification>, base.5, Content::<Impl, OFFSET>, DeliveryTime::<Impl, OFFSET>, SnoozeInterval::<Impl, OFFSET>, MaximumSnoozeCount::<Impl, OFFSET>, SetId::<Impl, OFFSET>, Id::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotification2Impl: Sized {
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSuppressPopup(&self, value: bool) -> ::windows::core::Result<()>;
    fn SuppressPopup(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScheduledToastNotification2 {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotification2";
}
#[cfg(feature = "implement_exclusive")]
impl IScheduledToastNotification2Vtbl {
    pub const fn new<Impl: IScheduledToastNotification2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScheduledToastNotification2Vtbl {
        unsafe extern "system" fn SetTag<Impl: IScheduledToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IScheduledToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Impl: IScheduledToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGroup(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Group<Impl: IScheduledToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuppressPopup<Impl: IScheduledToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSuppressPopup(value).into()
        }
        unsafe extern "system" fn SuppressPopup<Impl: IScheduledToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuppressPopup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScheduledToastNotification2>, base.5, SetTag::<Impl, OFFSET>, Tag::<Impl, OFFSET>, SetGroup::<Impl, OFFSET>, Group::<Impl, OFFSET>, SetSuppressPopup::<Impl, OFFSET>, SuppressPopup::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotification3Impl: Sized {
    fn NotificationMirroring(&self) -> ::windows::core::Result<NotificationMirroring>;
    fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScheduledToastNotification3 {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotification3";
}
#[cfg(feature = "implement_exclusive")]
impl IScheduledToastNotification3Vtbl {
    pub const fn new<Impl: IScheduledToastNotification3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScheduledToastNotification3Vtbl {
        unsafe extern "system" fn NotificationMirroring<Impl: IScheduledToastNotification3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotificationMirroring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMirroring<Impl: IScheduledToastNotification3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNotificationMirroring(value).into()
        }
        unsafe extern "system" fn RemoteId<Impl: IScheduledToastNotification3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteId<Impl: IScheduledToastNotification3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScheduledToastNotification3>, base.5, NotificationMirroring::<Impl, OFFSET>, SetNotificationMirroring::<Impl, OFFSET>, RemoteId::<Impl, OFFSET>, SetRemoteId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotification4Impl: Sized {
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScheduledToastNotification4 {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotification4";
}
#[cfg(feature = "implement_exclusive")]
impl IScheduledToastNotification4Vtbl {
    pub const fn new<Impl: IScheduledToastNotification4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScheduledToastNotification4Vtbl {
        unsafe extern "system" fn ExpirationTime<Impl: IScheduledToastNotification4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: IScheduledToastNotification4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScheduledToastNotification4>, base.5, ExpirationTime::<Impl, OFFSET>, SetExpirationTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotificationFactoryImpl: Sized {
    fn CreateScheduledToastNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>, deliverytime: &super::super::Foundation::DateTime) -> ::windows::core::Result<ScheduledToastNotification>;
    fn CreateScheduledToastNotificationRecurring(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>, deliverytime: &super::super::Foundation::DateTime, snoozeinterval: &super::super::Foundation::TimeSpan, maximumsnoozecount: u32) -> ::windows::core::Result<ScheduledToastNotification>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScheduledToastNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotificationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IScheduledToastNotificationFactoryVtbl {
    pub const fn new<Impl: IScheduledToastNotificationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScheduledToastNotificationFactoryVtbl {
        unsafe extern "system" fn CreateScheduledToastNotification<Impl: IScheduledToastNotificationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, deliverytime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateScheduledToastNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType), &*(&deliverytime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScheduledToastNotificationRecurring<Impl: IScheduledToastNotificationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, deliverytime: super::super::Foundation::DateTime, snoozeinterval: super::super::Foundation::TimeSpan, maximumsnoozecount: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateScheduledToastNotificationRecurring(
                &*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType),
                &*(&deliverytime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&snoozeinterval as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                maximumsnoozecount,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScheduledToastNotificationFactory>, base.5, CreateScheduledToastNotification::<Impl, OFFSET>, CreateScheduledToastNotificationRecurring::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScheduledToastNotificationShowingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn ScheduledToastNotification(&self) -> ::windows::core::Result<ScheduledToastNotification>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScheduledToastNotificationShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotificationShowingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IScheduledToastNotificationShowingEventArgsVtbl {
    pub const fn new<Impl: IScheduledToastNotificationShowingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScheduledToastNotificationShowingEventArgsVtbl {
        unsafe extern "system" fn Cancel<Impl: IScheduledToastNotificationShowingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancel<Impl: IScheduledToastNotificationShowingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn ScheduledToastNotification<Impl: IScheduledToastNotificationShowingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScheduledToastNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IScheduledToastNotificationShowingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScheduledToastNotificationShowingEventArgs>, base.5, Cancel::<Impl, OFFSET>, SetCancel::<Impl, OFFSET>, ScheduledToastNotification::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShownTileNotificationImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShownTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IShownTileNotification";
}
#[cfg(feature = "implement_exclusive")]
impl IShownTileNotificationVtbl {
    pub const fn new<Impl: IShownTileNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShownTileNotificationVtbl {
        unsafe extern "system" fn Arguments<Impl: IShownTileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShownTileNotification>, base.5, Arguments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileFlyoutNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileFlyoutNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ITileFlyoutNotification";
}
#[cfg(feature = "implement_exclusive")]
impl ITileFlyoutNotificationVtbl {
    pub const fn new<Impl: ITileFlyoutNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileFlyoutNotificationVtbl {
        unsafe extern "system" fn Content<Impl: ITileFlyoutNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: ITileFlyoutNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: ITileFlyoutNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileFlyoutNotification>, base.5, Content::<Impl, OFFSET>, SetExpirationTime::<Impl, OFFSET>, ExpirationTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileFlyoutNotificationFactoryImpl: Sized {
    fn CreateTileFlyoutNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<TileFlyoutNotification>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileFlyoutNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.ITileFlyoutNotificationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITileFlyoutNotificationFactoryVtbl {
    pub const fn new<Impl: ITileFlyoutNotificationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileFlyoutNotificationFactoryVtbl {
        unsafe extern "system" fn CreateTileFlyoutNotification<Impl: ITileFlyoutNotificationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileFlyoutNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileFlyoutNotificationFactory>, base.5, CreateTileFlyoutNotification::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileFlyoutUpdateManagerStaticsImpl: Sized {
    fn CreateTileFlyoutUpdaterForApplication(&self) -> ::windows::core::Result<TileFlyoutUpdater>;
    fn CreateTileFlyoutUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileFlyoutUpdater>;
    fn CreateTileFlyoutUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileFlyoutUpdater>;
    fn GetTemplateContent(&self, r#type: TileFlyoutTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileFlyoutUpdateManagerStatics {
    const NAME: &'static str = "Windows.UI.Notifications.ITileFlyoutUpdateManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITileFlyoutUpdateManagerStaticsVtbl {
    pub const fn new<Impl: ITileFlyoutUpdateManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileFlyoutUpdateManagerStaticsVtbl {
        unsafe extern "system" fn CreateTileFlyoutUpdaterForApplication<Impl: ITileFlyoutUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileFlyoutUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileFlyoutUpdaterForApplicationWithId<Impl: ITileFlyoutUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileFlyoutUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileFlyoutUpdaterForSecondaryTile<Impl: ITileFlyoutUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileFlyoutUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplateContent<Impl: ITileFlyoutUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: TileFlyoutTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTemplateContent(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileFlyoutUpdateManagerStatics>, base.5, CreateTileFlyoutUpdaterForApplication::<Impl, OFFSET>, CreateTileFlyoutUpdaterForApplicationWithId::<Impl, OFFSET>, CreateTileFlyoutUpdaterForSecondaryTile::<Impl, OFFSET>, GetTemplateContent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileFlyoutUpdaterImpl: Sized {
    fn Update(&self, notification: &::core::option::Option<TileFlyoutNotification>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdate(&self, tileflyoutcontent: &::core::option::Option<super::super::Foundation::Uri>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateAtTime(&self, tileflyoutcontent: &::core::option::Option<super::super::Foundation::Uri>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()>;
    fn Setting(&self) -> ::windows::core::Result<NotificationSetting>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileFlyoutUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.ITileFlyoutUpdater";
}
#[cfg(feature = "implement_exclusive")]
impl ITileFlyoutUpdaterVtbl {
    pub const fn new<Impl: ITileFlyoutUpdaterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileFlyoutUpdaterVtbl {
        unsafe extern "system" fn Update<Impl: ITileFlyoutUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Update(&*(&notification as *const <TileFlyoutNotification as ::windows::core::Abi>::Abi as *const <TileFlyoutNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITileFlyoutUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn StartPeriodicUpdate<Impl: ITileFlyoutUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileflyoutcontent: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdate(&*(&tileflyoutcontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StartPeriodicUpdateAtTime<Impl: ITileFlyoutUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileflyoutcontent: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdateAtTime(&*(&tileflyoutcontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StopPeriodicUpdate<Impl: ITileFlyoutUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopPeriodicUpdate().into()
        }
        unsafe extern "system" fn Setting<Impl: ITileFlyoutUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileFlyoutUpdater>, base.5, Update::<Impl, OFFSET>, Clear::<Impl, OFFSET>, StartPeriodicUpdate::<Impl, OFFSET>, StartPeriodicUpdateAtTime::<Impl, OFFSET>, StopPeriodicUpdate::<Impl, OFFSET>, Setting::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ITileNotification";
}
#[cfg(feature = "implement_exclusive")]
impl ITileNotificationVtbl {
    pub const fn new<Impl: ITileNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileNotificationVtbl {
        unsafe extern "system" fn Content<Impl: ITileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: ITileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: ITileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: ITileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: ITileNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileNotification>, base.5, Content::<Impl, OFFSET>, SetExpirationTime::<Impl, OFFSET>, ExpirationTime::<Impl, OFFSET>, SetTag::<Impl, OFFSET>, Tag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileNotificationFactoryImpl: Sized {
    fn CreateTileNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<TileNotification>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.ITileNotificationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITileNotificationFactoryVtbl {
    pub const fn new<Impl: ITileNotificationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileNotificationFactoryVtbl {
        unsafe extern "system" fn CreateTileNotification<Impl: ITileNotificationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileNotificationFactory>, base.5, CreateTileNotification::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdateManagerForUserImpl: Sized {
    fn CreateTileUpdaterForApplication(&self) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.ITileUpdateManagerForUser";
}
#[cfg(feature = "implement_exclusive")]
impl ITileUpdateManagerForUserVtbl {
    pub const fn new<Impl: ITileUpdateManagerForUserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileUpdateManagerForUserVtbl {
        unsafe extern "system" fn CreateTileUpdaterForApplication<Impl: ITileUpdateManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForApplicationWithId<Impl: ITileUpdateManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForSecondaryTile<Impl: ITileUpdateManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: ITileUpdateManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileUpdateManagerForUser>, base.5, CreateTileUpdaterForApplication::<Impl, OFFSET>, CreateTileUpdaterForApplicationWithId::<Impl, OFFSET>, CreateTileUpdaterForSecondaryTile::<Impl, OFFSET>, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdateManagerStaticsImpl: Sized {
    fn CreateTileUpdaterForApplication(&self) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn GetTemplateContent(&self, r#type: TileTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileUpdateManagerStatics {
    const NAME: &'static str = "Windows.UI.Notifications.ITileUpdateManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITileUpdateManagerStaticsVtbl {
    pub const fn new<Impl: ITileUpdateManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileUpdateManagerStaticsVtbl {
        unsafe extern "system" fn CreateTileUpdaterForApplication<Impl: ITileUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForApplicationWithId<Impl: ITileUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForSecondaryTile<Impl: ITileUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplateContent<Impl: ITileUpdateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: TileTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTemplateContent(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileUpdateManagerStatics>, base.5, CreateTileUpdaterForApplication::<Impl, OFFSET>, CreateTileUpdaterForApplicationWithId::<Impl, OFFSET>, CreateTileUpdaterForSecondaryTile::<Impl, OFFSET>, GetTemplateContent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdateManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<TileUpdateManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileUpdateManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Notifications.ITileUpdateManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ITileUpdateManagerStatics2Vtbl {
    pub const fn new<Impl: ITileUpdateManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileUpdateManagerStatics2Vtbl {
        unsafe extern "system" fn GetForUser<Impl: ITileUpdateManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileUpdateManagerStatics2>, base.5, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdaterImpl: Sized {
    fn Update(&self, notification: &::core::option::Option<TileNotification>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn EnableNotificationQueue(&self, enable: bool) -> ::windows::core::Result<()>;
    fn Setting(&self) -> ::windows::core::Result<NotificationSetting>;
    fn AddToSchedule(&self, scheduledtile: &::core::option::Option<ScheduledTileNotification>) -> ::windows::core::Result<()>;
    fn RemoveFromSchedule(&self, scheduledtile: &::core::option::Option<ScheduledTileNotification>) -> ::windows::core::Result<()>;
    fn GetScheduledTileNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ScheduledTileNotification>>;
    fn StartPeriodicUpdate(&self, tilecontent: &::core::option::Option<super::super::Foundation::Uri>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateAtTime(&self, tilecontent: &::core::option::Option<super::super::Foundation::Uri>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateBatch(&self, tilecontents: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateBatchAtTime(&self, tilecontents: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.ITileUpdater";
}
#[cfg(feature = "implement_exclusive")]
impl ITileUpdaterVtbl {
    pub const fn new<Impl: ITileUpdaterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileUpdaterVtbl {
        unsafe extern "system" fn Update<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Update(&*(&notification as *const <TileNotification as ::windows::core::Abi>::Abi as *const <TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn EnableNotificationQueue<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnableNotificationQueue(enable).into()
        }
        unsafe extern "system" fn Setting<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSchedule<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheduledtile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddToSchedule(&*(&scheduledtile as *const <ScheduledTileNotification as ::windows::core::Abi>::Abi as *const <ScheduledTileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveFromSchedule<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheduledtile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFromSchedule(&*(&scheduledtile as *const <ScheduledTileNotification as ::windows::core::Abi>::Abi as *const <ScheduledTileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetScheduledTileNotifications<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScheduledTileNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartPeriodicUpdate<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tilecontent: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdate(&*(&tilecontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StartPeriodicUpdateAtTime<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tilecontent: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdateAtTime(&*(&tilecontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StopPeriodicUpdate<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopPeriodicUpdate().into()
        }
        unsafe extern "system" fn StartPeriodicUpdateBatch<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tilecontents: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdateBatch(&*(&tilecontents as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StartPeriodicUpdateBatchAtTime<Impl: ITileUpdaterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tilecontents: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .StartPeriodicUpdateBatchAtTime(&*(&tilecontents as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), requestedinterval)
                .into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITileUpdater>,
            base.5,
            Update::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
            EnableNotificationQueue::<Impl, OFFSET>,
            Setting::<Impl, OFFSET>,
            AddToSchedule::<Impl, OFFSET>,
            RemoveFromSchedule::<Impl, OFFSET>,
            GetScheduledTileNotifications::<Impl, OFFSET>,
            StartPeriodicUpdate::<Impl, OFFSET>,
            StartPeriodicUpdateAtTime::<Impl, OFFSET>,
            StopPeriodicUpdate::<Impl, OFFSET>,
            StartPeriodicUpdateBatch::<Impl, OFFSET>,
            StartPeriodicUpdateBatchAtTime::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileUpdater2Impl: Sized {
    fn EnableNotificationQueueForSquare150x150(&self, enable: bool) -> ::windows::core::Result<()>;
    fn EnableNotificationQueueForWide310x150(&self, enable: bool) -> ::windows::core::Result<()>;
    fn EnableNotificationQueueForSquare310x310(&self, enable: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileUpdater2 {
    const NAME: &'static str = "Windows.UI.Notifications.ITileUpdater2";
}
#[cfg(feature = "implement_exclusive")]
impl ITileUpdater2Vtbl {
    pub const fn new<Impl: ITileUpdater2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITileUpdater2Vtbl {
        unsafe extern "system" fn EnableNotificationQueueForSquare150x150<Impl: ITileUpdater2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnableNotificationQueueForSquare150x150(enable).into()
        }
        unsafe extern "system" fn EnableNotificationQueueForWide310x150<Impl: ITileUpdater2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnableNotificationQueueForWide310x150(enable).into()
        }
        unsafe extern "system" fn EnableNotificationQueueForSquare310x310<Impl: ITileUpdater2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnableNotificationQueueForSquare310x310(enable).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITileUpdater2>, base.5, EnableNotificationQueueForSquare150x150::<Impl, OFFSET>, EnableNotificationQueueForWide310x150::<Impl, OFFSET>, EnableNotificationQueueForSquare310x310::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastActivatedEventArgsImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.IToastActivatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IToastActivatedEventArgsVtbl {
    pub const fn new<Impl: IToastActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastActivatedEventArgsVtbl {
        unsafe extern "system" fn Arguments<Impl: IToastActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastActivatedEventArgs>, base.5, Arguments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastActivatedEventArgs2Impl: Sized {
    fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastActivatedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastActivatedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IToastActivatedEventArgs2Vtbl {
    pub const fn new<Impl: IToastActivatedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastActivatedEventArgs2Vtbl {
        unsafe extern "system" fn UserInput<Impl: IToastActivatedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastActivatedEventArgs2>, base.5, UserInput::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastCollectionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LaunchArgs(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLaunchArgs(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Icon(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetIcon(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastCollection {
    const NAME: &'static str = "Windows.UI.Notifications.IToastCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IToastCollectionVtbl {
    pub const fn new<Impl: IToastCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastCollectionVtbl {
        unsafe extern "system" fn Id<Impl: IToastCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IToastCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IToastCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LaunchArgs<Impl: IToastCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchArgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLaunchArgs<Impl: IToastCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLaunchArgs(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Icon<Impl: IToastCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Icon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcon<Impl: IToastCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIcon(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastCollection>, base.5, Id::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, SetDisplayName::<Impl, OFFSET>, LaunchArgs::<Impl, OFFSET>, SetLaunchArgs::<Impl, OFFSET>, Icon::<Impl, OFFSET>, SetIcon::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastCollectionFactoryImpl: Sized {
    fn CreateInstance(&self, collectionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, launchargs: &::windows::core::HSTRING, iconuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<ToastCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastCollectionFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IToastCollectionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToastCollectionFactoryVtbl {
    pub const fn new<Impl: IToastCollectionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastCollectionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IToastCollectionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, launchargs: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(
                &*(&collectionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&launchargs as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&iconuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastCollectionFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastCollectionManagerImpl: Sized {
    fn SaveToastCollectionAsync(&self, collection: &::core::option::Option<ToastCollection>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FindAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ToastCollection>>>;
    fn GetToastCollectionAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastCollection>>;
    fn RemoveToastCollectionAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RemoveAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastCollectionManager {
    const NAME: &'static str = "Windows.UI.Notifications.IToastCollectionManager";
}
#[cfg(feature = "implement_exclusive")]
impl IToastCollectionManagerVtbl {
    pub const fn new<Impl: IToastCollectionManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastCollectionManagerVtbl {
        unsafe extern "system" fn SaveToastCollectionAsync<Impl: IToastCollectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collection: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SaveToastCollectionAsync(&*(&collection as *const <ToastCollection as ::windows::core::Abi>::Abi as *const <ToastCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllToastCollectionsAsync<Impl: IToastCollectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllToastCollectionsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToastCollectionAsync<Impl: IToastCollectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetToastCollectionAsync(&*(&collectionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveToastCollectionAsync<Impl: IToastCollectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveToastCollectionAsync(&*(&collectionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllToastCollectionsAsync<Impl: IToastCollectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAllToastCollectionsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IToastCollectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: IToastCollectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastCollectionManager>, base.5, SaveToastCollectionAsync::<Impl, OFFSET>, FindAllToastCollectionsAsync::<Impl, OFFSET>, GetToastCollectionAsync::<Impl, OFFSET>, RemoveToastCollectionAsync::<Impl, OFFSET>, RemoveAllToastCollectionsAsync::<Impl, OFFSET>, User::<Impl, OFFSET>, AppId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastDismissedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<ToastDismissalReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.IToastDismissedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IToastDismissedEventArgsVtbl {
    pub const fn new<Impl: IToastDismissedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastDismissedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IToastDismissedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ToastDismissalReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastDismissedEventArgs>, base.5, Reason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastFailedEventArgsImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.IToastFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IToastFailedEventArgsVtbl {
    pub const fn new<Impl: IToastFailedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastFailedEventArgsVtbl {
        unsafe extern "system" fn ErrorCode<Impl: IToastFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastFailedEventArgs>, base.5, ErrorCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn Dismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDismissed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Activated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ToastNotification, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Failed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotification";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationVtbl {
    pub const fn new<Impl: IToastNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationVtbl {
        unsafe extern "system" fn Content<Impl: IToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: IToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: IToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dismissed<Impl: IToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dismissed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDismissed<Impl: IToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDismissed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Activated<Impl: IToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: IToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Failed<Impl: IToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Failed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFailed<Impl: IToastNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotification>, base.5, Content::<Impl, OFFSET>, SetExpirationTime::<Impl, OFFSET>, ExpirationTime::<Impl, OFFSET>, Dismissed::<Impl, OFFSET>, RemoveDismissed::<Impl, OFFSET>, Activated::<Impl, OFFSET>, RemoveActivated::<Impl, OFFSET>, Failed::<Impl, OFFSET>, RemoveFailed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotification2Impl: Sized {
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSuppressPopup(&self, value: bool) -> ::windows::core::Result<()>;
    fn SuppressPopup(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotification2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotification2";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotification2Vtbl {
    pub const fn new<Impl: IToastNotification2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotification2Vtbl {
        unsafe extern "system" fn SetTag<Impl: IToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Impl: IToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGroup(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Group<Impl: IToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuppressPopup<Impl: IToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSuppressPopup(value).into()
        }
        unsafe extern "system" fn SuppressPopup<Impl: IToastNotification2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuppressPopup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotification2>, base.5, SetTag::<Impl, OFFSET>, Tag::<Impl, OFFSET>, SetGroup::<Impl, OFFSET>, Group::<Impl, OFFSET>, SetSuppressPopup::<Impl, OFFSET>, SuppressPopup::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotification3Impl: Sized {
    fn NotificationMirroring(&self) -> ::windows::core::Result<NotificationMirroring>;
    fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotification3 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotification3";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotification3Vtbl {
    pub const fn new<Impl: IToastNotification3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotification3Vtbl {
        unsafe extern "system" fn NotificationMirroring<Impl: IToastNotification3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotificationMirroring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMirroring<Impl: IToastNotification3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNotificationMirroring(value).into()
        }
        unsafe extern "system" fn RemoteId<Impl: IToastNotification3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteId<Impl: IToastNotification3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotification3>, base.5, NotificationMirroring::<Impl, OFFSET>, SetNotificationMirroring::<Impl, OFFSET>, RemoteId::<Impl, OFFSET>, SetRemoteId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotification4Impl: Sized {
    fn Data(&self) -> ::windows::core::Result<NotificationData>;
    fn SetData(&self, value: &::core::option::Option<NotificationData>) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<ToastNotificationPriority>;
    fn SetPriority(&self, value: ToastNotificationPriority) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotification4 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotification4";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotification4Vtbl {
    pub const fn new<Impl: IToastNotification4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotification4Vtbl {
        unsafe extern "system" fn Data<Impl: IToastNotification4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IToastNotification4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <NotificationData as ::windows::core::Abi>::Abi as *const <NotificationData as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Priority<Impl: IToastNotification4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ToastNotificationPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IToastNotification4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ToastNotificationPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPriority(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotification4>, base.5, Data::<Impl, OFFSET>, SetData::<Impl, OFFSET>, Priority::<Impl, OFFSET>, SetPriority::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotification6Impl: Sized {
    fn ExpiresOnReboot(&self) -> ::windows::core::Result<bool>;
    fn SetExpiresOnReboot(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotification6 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotification6";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotification6Vtbl {
    pub const fn new<Impl: IToastNotification6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotification6Vtbl {
        unsafe extern "system" fn ExpiresOnReboot<Impl: IToastNotification6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpiresOnReboot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpiresOnReboot<Impl: IToastNotification6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExpiresOnReboot(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotification6>, base.5, ExpiresOnReboot::<Impl, OFFSET>, SetExpiresOnReboot::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationActionTriggerDetailImpl: Sized {
    fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationActionTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationActionTriggerDetail";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationActionTriggerDetailVtbl {
    pub const fn new<Impl: IToastNotificationActionTriggerDetailImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationActionTriggerDetailVtbl {
        unsafe extern "system" fn Argument<Impl: IToastNotificationActionTriggerDetailImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Argument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserInput<Impl: IToastNotificationActionTriggerDetailImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationActionTriggerDetail>, base.5, Argument::<Impl, OFFSET>, UserInput::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationFactoryImpl: Sized {
    fn CreateToastNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<ToastNotification>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationFactoryVtbl {
    pub const fn new<Impl: IToastNotificationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationFactoryVtbl {
        unsafe extern "system" fn CreateToastNotification<Impl: IToastNotificationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateToastNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationFactory>, base.5, CreateToastNotification::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistoryImpl: Sized {
    fn RemoveGroup(&self, group: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveGroupWithId(&self, group: &::windows::core::HSTRING, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveGroupedTagWithId(&self, tag: &::windows::core::HSTRING, group: &::windows::core::HSTRING, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveGroupedTag(&self, tag: &::windows::core::HSTRING, group: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Remove(&self, tag: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn ClearWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationHistory {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationHistory";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationHistoryVtbl {
    pub const fn new<Impl: IToastNotificationHistoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationHistoryVtbl {
        unsafe extern "system" fn RemoveGroup<Impl: IToastNotificationHistoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGroup(&*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveGroupWithId<Impl: IToastNotificationHistoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGroupWithId(&*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveGroupedTagWithId<Impl: IToastNotificationHistoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RemoveGroupedTagWithId(
                    &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RemoveGroupedTag<Impl: IToastNotificationHistoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGroupedTag(&*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: IToastNotificationHistoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Remove(&*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: IToastNotificationHistoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ClearWithId<Impl: IToastNotificationHistoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ClearWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationHistory>, base.5, RemoveGroup::<Impl, OFFSET>, RemoveGroupWithId::<Impl, OFFSET>, RemoveGroupedTagWithId::<Impl, OFFSET>, RemoveGroupedTag::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Clear::<Impl, OFFSET>, ClearWithId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistory2Impl: Sized {
    fn GetHistory(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>>;
    fn GetHistoryWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationHistory2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationHistory2";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationHistory2Vtbl {
    pub const fn new<Impl: IToastNotificationHistory2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationHistory2Vtbl {
        unsafe extern "system" fn GetHistory<Impl: IToastNotificationHistory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHistoryWithId<Impl: IToastNotificationHistory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHistoryWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationHistory2>, base.5, GetHistory::<Impl, OFFSET>, GetHistoryWithId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistoryChangedTriggerDetailImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<ToastHistoryChangedType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationHistoryChangedTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationHistoryChangedTriggerDetailVtbl {
    pub const fn new<Impl: IToastNotificationHistoryChangedTriggerDetailImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationHistoryChangedTriggerDetailVtbl {
        unsafe extern "system" fn ChangeType<Impl: IToastNotificationHistoryChangedTriggerDetailImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ToastHistoryChangedType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationHistoryChangedTriggerDetail>, base.5, ChangeType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistoryChangedTriggerDetail2Impl: Sized {
    fn CollectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationHistoryChangedTriggerDetail2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail2";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationHistoryChangedTriggerDetail2Vtbl {
    pub const fn new<Impl: IToastNotificationHistoryChangedTriggerDetail2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationHistoryChangedTriggerDetail2Vtbl {
        unsafe extern "system" fn CollectionId<Impl: IToastNotificationHistoryChangedTriggerDetail2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CollectionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationHistoryChangedTriggerDetail2>, base.5, CollectionId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerForUserImpl: Sized {
    fn CreateToastNotifier(&self) -> ::windows::core::Result<ToastNotifier>;
    fn CreateToastNotifierWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotifier>;
    fn History(&self) -> ::windows::core::Result<ToastNotificationHistory>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerForUser";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationManagerForUserVtbl {
    pub const fn new<Impl: IToastNotificationManagerForUserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationManagerForUserVtbl {
        unsafe extern "system" fn CreateToastNotifier<Impl: IToastNotificationManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateToastNotifierWithId<Impl: IToastNotificationManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifierWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn History<Impl: IToastNotificationManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).History() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IToastNotificationManagerForUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationManagerForUser>, base.5, CreateToastNotifier::<Impl, OFFSET>, CreateToastNotifierWithId::<Impl, OFFSET>, History::<Impl, OFFSET>, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerForUser2Impl: Sized {
    fn GetToastNotifierForToastCollectionIdAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotifier>>;
    fn GetHistoryForToastCollectionIdAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotificationHistory>>;
    fn GetToastCollectionManager(&self) -> ::windows::core::Result<ToastCollectionManager>;
    fn GetToastCollectionManagerWithAppId(&self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastCollectionManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationManagerForUser2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerForUser2";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationManagerForUser2Vtbl {
    pub const fn new<Impl: IToastNotificationManagerForUser2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationManagerForUser2Vtbl {
        unsafe extern "system" fn GetToastNotifierForToastCollectionIdAsync<Impl: IToastNotificationManagerForUser2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetToastNotifierForToastCollectionIdAsync(&*(&collectionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHistoryForToastCollectionIdAsync<Impl: IToastNotificationManagerForUser2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHistoryForToastCollectionIdAsync(&*(&collectionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToastCollectionManager<Impl: IToastNotificationManagerForUser2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetToastCollectionManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToastCollectionManagerWithAppId<Impl: IToastNotificationManagerForUser2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetToastCollectionManagerWithAppId(&*(&appid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationManagerForUser2>, base.5, GetToastNotifierForToastCollectionIdAsync::<Impl, OFFSET>, GetHistoryForToastCollectionIdAsync::<Impl, OFFSET>, GetToastCollectionManager::<Impl, OFFSET>, GetToastCollectionManagerWithAppId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerStaticsImpl: Sized {
    fn CreateToastNotifier(&self) -> ::windows::core::Result<ToastNotifier>;
    fn CreateToastNotifierWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotifier>;
    fn GetTemplateContent(&self, r#type: ToastTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationManagerStatics {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationManagerStaticsVtbl {
    pub const fn new<Impl: IToastNotificationManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationManagerStaticsVtbl {
        unsafe extern "system" fn CreateToastNotifier<Impl: IToastNotificationManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateToastNotifierWithId<Impl: IToastNotificationManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifierWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplateContent<Impl: IToastNotificationManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: ToastTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTemplateContent(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationManagerStatics>, base.5, CreateToastNotifier::<Impl, OFFSET>, CreateToastNotifierWithId::<Impl, OFFSET>, GetTemplateContent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerStatics2Impl: Sized {
    fn History(&self) -> ::windows::core::Result<ToastNotificationHistory>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationManagerStatics2Vtbl {
    pub const fn new<Impl: IToastNotificationManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationManagerStatics2Vtbl {
        unsafe extern "system" fn History<Impl: IToastNotificationManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).History() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationManagerStatics2>, base.5, History::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerStatics4Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<ToastNotificationManagerForUser>;
    fn ConfigureNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationManagerStatics4 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationManagerStatics4Vtbl {
    pub const fn new<Impl: IToastNotificationManagerStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationManagerStatics4Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IToastNotificationManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureNotificationMirroring<Impl: IToastNotificationManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ConfigureNotificationMirroring(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationManagerStatics4>, base.5, GetForUser::<Impl, OFFSET>, ConfigureNotificationMirroring::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationManagerStatics5Impl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<ToastNotificationManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationManagerStatics5 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationManagerStatics5Vtbl {
    pub const fn new<Impl: IToastNotificationManagerStatics5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationManagerStatics5Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IToastNotificationManagerStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationManagerStatics5>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotifierImpl: Sized {
    fn Show(&self, notification: &::core::option::Option<ToastNotification>) -> ::windows::core::Result<()>;
    fn Hide(&self, notification: &::core::option::Option<ToastNotification>) -> ::windows::core::Result<()>;
    fn Setting(&self) -> ::windows::core::Result<NotificationSetting>;
    fn AddToSchedule(&self, scheduledtoast: &::core::option::Option<ScheduledToastNotification>) -> ::windows::core::Result<()>;
    fn RemoveFromSchedule(&self, scheduledtoast: &::core::option::Option<ScheduledToastNotification>) -> ::windows::core::Result<()>;
    fn GetScheduledToastNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ScheduledToastNotification>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotifier {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotifier";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotifierVtbl {
    pub const fn new<Impl: IToastNotifierImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotifierVtbl {
        unsafe extern "system" fn Show<Impl: IToastNotifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Show(&*(&notification as *const <ToastNotification as ::windows::core::Abi>::Abi as *const <ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hide<Impl: IToastNotifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Hide(&*(&notification as *const <ToastNotification as ::windows::core::Abi>::Abi as *const <ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Setting<Impl: IToastNotifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSchedule<Impl: IToastNotifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheduledtoast: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddToSchedule(&*(&scheduledtoast as *const <ScheduledToastNotification as ::windows::core::Abi>::Abi as *const <ScheduledToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveFromSchedule<Impl: IToastNotifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheduledtoast: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFromSchedule(&*(&scheduledtoast as *const <ScheduledToastNotification as ::windows::core::Abi>::Abi as *const <ScheduledToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetScheduledToastNotifications<Impl: IToastNotifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScheduledToastNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotifier>, base.5, Show::<Impl, OFFSET>, Hide::<Impl, OFFSET>, Setting::<Impl, OFFSET>, AddToSchedule::<Impl, OFFSET>, RemoveFromSchedule::<Impl, OFFSET>, GetScheduledToastNotifications::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotifier2Impl: Sized {
    fn UpdateWithTagAndGroup(&self, data: &::core::option::Option<NotificationData>, tag: &::windows::core::HSTRING, group: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationUpdateResult>;
    fn UpdateWithTag(&self, data: &::core::option::Option<NotificationData>, tag: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationUpdateResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotifier2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotifier2";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotifier2Vtbl {
    pub const fn new<Impl: IToastNotifier2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotifier2Vtbl {
        unsafe extern "system" fn UpdateWithTagAndGroup<Impl: IToastNotifier2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut NotificationUpdateResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateWithTagAndGroup(&*(&data as *const <NotificationData as ::windows::core::Abi>::Abi as *const <NotificationData as ::windows::core::DefaultType>::DefaultType), &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateWithTag<Impl: IToastNotifier2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut NotificationUpdateResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateWithTag(&*(&data as *const <NotificationData as ::windows::core::Abi>::Abi as *const <NotificationData as ::windows::core::DefaultType>::DefaultType), &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotifier2>, base.5, UpdateWithTagAndGroup::<Impl, OFFSET>, UpdateWithTag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotifier3Impl: Sized {
    fn ScheduledToastNotificationShowing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScheduledToastNotificationShowing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotifier3 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotifier3";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotifier3Vtbl {
    pub const fn new<Impl: IToastNotifier3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotifier3Vtbl {
        unsafe extern "system" fn ScheduledToastNotificationShowing<Impl: IToastNotifier3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScheduledToastNotificationShowing(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScheduledToastNotificationShowing<Impl: IToastNotifier3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveScheduledToastNotificationShowing(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotifier3>, base.5, ScheduledToastNotificationShowing::<Impl, OFFSET>, RemoveScheduledToastNotificationShowing::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserNotificationImpl: Sized {
    fn Notification(&self) -> ::windows::core::Result<Notification>;
    fn AppInfo(&self) -> ::windows::core::Result<super::super::ApplicationModel::AppInfo>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn CreationTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IUserNotification";
}
#[cfg(feature = "implement_exclusive")]
impl IUserNotificationVtbl {
    pub const fn new<Impl: IUserNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserNotificationVtbl {
        unsafe extern "system" fn Notification<Impl: IUserNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Notification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppInfo<Impl: IUserNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IUserNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Impl: IUserNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserNotification>, base.5, Notification::<Impl, OFFSET>, AppInfo::<Impl, OFFSET>, Id::<Impl, OFFSET>, CreationTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserNotificationChangedEventArgsImpl: Sized {
    fn ChangeKind(&self) -> ::windows::core::Result<UserNotificationChangedKind>;
    fn UserNotificationId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserNotificationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.IUserNotificationChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUserNotificationChangedEventArgsVtbl {
    pub const fn new<Impl: IUserNotificationChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserNotificationChangedEventArgsVtbl {
        unsafe extern "system" fn ChangeKind<Impl: IUserNotificationChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserNotificationChangedKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangeKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserNotificationId<Impl: IUserNotificationChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserNotificationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserNotificationChangedEventArgs>, base.5, ChangeKind::<Impl, OFFSET>, UserNotificationId::<Impl, OFFSET>)
    }
}
