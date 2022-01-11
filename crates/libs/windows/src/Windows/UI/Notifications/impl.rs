#[cfg(feature = "Foundation_Collections")]
pub trait IAdaptiveNotificationContentImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AdaptiveNotificationContentKind>;
    fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IAdaptiveNotificationContent {
    const NAME: &'static str = "Windows.UI.Notifications.IAdaptiveNotificationContent";
}
#[cfg(feature = "Foundation_Collections")]
impl IAdaptiveNotificationContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveNotificationContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveNotificationContentVtbl {
        unsafe extern "system" fn Kind<Impl: IAdaptiveNotificationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveNotificationContentKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hints<Impl: IAdaptiveNotificationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveNotificationContent, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            Hints: Hints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveNotificationContent as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveNotificationTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveNotificationTextVtbl {
        unsafe extern "system" fn Text<Impl: IAdaptiveNotificationTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: IAdaptiveNotificationTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: IAdaptiveNotificationTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IAdaptiveNotificationTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveNotificationText, BASE_OFFSET>(),
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveNotificationText as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBadgeNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBadgeNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeNotification";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IBadgeNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBadgeNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBadgeNotificationVtbl {
        unsafe extern "system" fn Content<Impl: IBadgeNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpirationTime<Impl: IBadgeNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: IBadgeNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBadgeNotification, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            SetExpirationTime: SetExpirationTime::<Impl, IMPL_OFFSET>,
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBadgeNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IBadgeNotificationFactoryImpl: Sized {
    fn CreateBadgeNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<BadgeNotification>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBadgeNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeNotificationFactory";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IBadgeNotificationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBadgeNotificationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBadgeNotificationFactoryVtbl {
        unsafe extern "system" fn CreateBadgeNotification<Impl: IBadgeNotificationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBadgeNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBadgeNotificationFactory, BASE_OFFSET>(),
            CreateBadgeNotification: CreateBadgeNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBadgeNotificationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IBadgeUpdateManagerForUserImpl: Sized {
    fn CreateBadgeUpdaterForApplication(&self) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBadgeUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeUpdateManagerForUser";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IBadgeUpdateManagerForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBadgeUpdateManagerForUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBadgeUpdateManagerForUserVtbl {
        unsafe extern "system" fn CreateBadgeUpdaterForApplication<Impl: IBadgeUpdateManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForApplicationWithId<Impl: IBadgeUpdateManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForSecondaryTile<Impl: IBadgeUpdateManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IBadgeUpdateManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBadgeUpdateManagerForUser, BASE_OFFSET>(),
            CreateBadgeUpdaterForApplication: CreateBadgeUpdaterForApplication::<Impl, IMPL_OFFSET>,
            CreateBadgeUpdaterForApplicationWithId: CreateBadgeUpdaterForApplicationWithId::<Impl, IMPL_OFFSET>,
            CreateBadgeUpdaterForSecondaryTile: CreateBadgeUpdaterForSecondaryTile::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBadgeUpdateManagerForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IBadgeUpdateManagerStaticsImpl: Sized {
    fn CreateBadgeUpdaterForApplication(&self) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn CreateBadgeUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<BadgeUpdater>;
    fn GetTemplateContent(&self, r#type: BadgeTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBadgeUpdateManagerStatics {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeUpdateManagerStatics";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IBadgeUpdateManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBadgeUpdateManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBadgeUpdateManagerStaticsVtbl {
        unsafe extern "system" fn CreateBadgeUpdaterForApplication<Impl: IBadgeUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForApplicationWithId<Impl: IBadgeUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForSecondaryTile<Impl: IBadgeUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplateContent<Impl: IBadgeUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: BadgeTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemplateContent(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBadgeUpdateManagerStatics, BASE_OFFSET>(),
            CreateBadgeUpdaterForApplication: CreateBadgeUpdaterForApplication::<Impl, IMPL_OFFSET>,
            CreateBadgeUpdaterForApplicationWithId: CreateBadgeUpdaterForApplicationWithId::<Impl, IMPL_OFFSET>,
            CreateBadgeUpdaterForSecondaryTile: CreateBadgeUpdaterForSecondaryTile::<Impl, IMPL_OFFSET>,
            GetTemplateContent: GetTemplateContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBadgeUpdateManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IBadgeUpdateManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<BadgeUpdateManagerForUser>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBadgeUpdateManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeUpdateManagerStatics2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IBadgeUpdateManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBadgeUpdateManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBadgeUpdateManagerStatics2Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IBadgeUpdateManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBadgeUpdateManagerStatics2, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBadgeUpdateManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBadgeUpdaterImpl: Sized {
    fn Update(&self, notification: &::core::option::Option<BadgeNotification>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdate(&self, badgecontent: &::core::option::Option<super::super::Foundation::Uri>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateAtTime(&self, badgecontent: &::core::option::Option<super::super::Foundation::Uri>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBadgeUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.IBadgeUpdater";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBadgeUpdaterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBadgeUpdaterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBadgeUpdaterVtbl {
        unsafe extern "system" fn Update<Impl: IBadgeUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(&*(&notification as *const <BadgeNotification as ::windows::core::Abi>::Abi as *const <BadgeNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: IBadgeUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn StartPeriodicUpdate<Impl: IBadgeUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, badgecontent: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdate(&*(&badgecontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StartPeriodicUpdateAtTime<Impl: IBadgeUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, badgecontent: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdateAtTime(&*(&badgecontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StopPeriodicUpdate<Impl: IBadgeUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopPeriodicUpdate().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBadgeUpdater, BASE_OFFSET>(),
            Update: Update::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            StartPeriodicUpdate: StartPeriodicUpdate::<Impl, IMPL_OFFSET>,
            StartPeriodicUpdateAtTime: StartPeriodicUpdateAtTime::<Impl, IMPL_OFFSET>,
            StopPeriodicUpdate: StopPeriodicUpdate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBadgeUpdater as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownAdaptiveNotificationHintsStaticsVtbl {
        unsafe extern "system" fn Style<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Wrap<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wrap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLines<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinLines<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinLines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextStacking<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextStacking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Align<Impl: IKnownAdaptiveNotificationHintsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Align() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownAdaptiveNotificationHintsStatics, BASE_OFFSET>(),
            Style: Style::<Impl, IMPL_OFFSET>,
            Wrap: Wrap::<Impl, IMPL_OFFSET>,
            MaxLines: MaxLines::<Impl, IMPL_OFFSET>,
            MinLines: MinLines::<Impl, IMPL_OFFSET>,
            TextStacking: TextStacking::<Impl, IMPL_OFFSET>,
            Align: Align::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownAdaptiveNotificationHintsStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownAdaptiveNotificationTextStylesStaticsVtbl {
        unsafe extern "system" fn Caption<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Caption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Base<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Base() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subtitle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subtitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Subheader<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subheader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Header<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Header() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleNumeral<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TitleNumeral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubheaderNumeral<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubheaderNumeral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderNumeral<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderNumeral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptionSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptionSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodySubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodySubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BaseSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubtitleSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubtitleSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TitleSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubheaderSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubheaderSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubheaderNumeralSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubheaderNumeralSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderNumeralSubtle<Impl: IKnownAdaptiveNotificationTextStylesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderNumeralSubtle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownAdaptiveNotificationTextStylesStatics, BASE_OFFSET>(),
            Caption: Caption::<Impl, IMPL_OFFSET>,
            Body: Body::<Impl, IMPL_OFFSET>,
            Base: Base::<Impl, IMPL_OFFSET>,
            Subtitle: Subtitle::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Subheader: Subheader::<Impl, IMPL_OFFSET>,
            Header: Header::<Impl, IMPL_OFFSET>,
            TitleNumeral: TitleNumeral::<Impl, IMPL_OFFSET>,
            SubheaderNumeral: SubheaderNumeral::<Impl, IMPL_OFFSET>,
            HeaderNumeral: HeaderNumeral::<Impl, IMPL_OFFSET>,
            CaptionSubtle: CaptionSubtle::<Impl, IMPL_OFFSET>,
            BodySubtle: BodySubtle::<Impl, IMPL_OFFSET>,
            BaseSubtle: BaseSubtle::<Impl, IMPL_OFFSET>,
            SubtitleSubtle: SubtitleSubtle::<Impl, IMPL_OFFSET>,
            TitleSubtle: TitleSubtle::<Impl, IMPL_OFFSET>,
            SubheaderSubtle: SubheaderSubtle::<Impl, IMPL_OFFSET>,
            SubheaderNumeralSubtle: SubheaderNumeralSubtle::<Impl, IMPL_OFFSET>,
            HeaderSubtle: HeaderSubtle::<Impl, IMPL_OFFSET>,
            HeaderNumeralSubtle: HeaderNumeralSubtle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownAdaptiveNotificationTextStylesStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownNotificationBindingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownNotificationBindingsStaticsVtbl {
        unsafe extern "system" fn ToastGeneric<Impl: IKnownNotificationBindingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToastGeneric() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownNotificationBindingsStatics, BASE_OFFSET>(),
            ToastGeneric: ToastGeneric::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownNotificationBindingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INotificationImpl: Sized {
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Visual(&self) -> ::windows::core::Result<NotificationVisual>;
    fn SetVisual(&self, value: &::core::option::Option<NotificationVisual>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INotification {
    const NAME: &'static str = "Windows.UI.Notifications.INotification";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationVtbl {
        unsafe extern "system" fn ExpirationTime<Impl: INotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpirationTime<Impl: INotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Visual<Impl: INotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visual() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisual<Impl: INotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisual(&*(&value as *const <NotificationVisual as ::windows::core::Abi>::Abi as *const <NotificationVisual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotification, BASE_OFFSET>(),
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
            SetExpirationTime: SetExpirationTime::<Impl, IMPL_OFFSET>,
            Visual: Visual::<Impl, IMPL_OFFSET>,
            SetVisual: SetVisual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INotificationBindingImpl: Sized {
    fn Template(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTemplate(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn GetTextElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdaptiveNotificationText>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INotificationBinding {
    const NAME: &'static str = "Windows.UI.Notifications.INotificationBinding";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INotificationBindingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationBindingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationBindingVtbl {
        unsafe extern "system" fn Template<Impl: INotificationBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemplate<Impl: INotificationBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTemplate(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: INotificationBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: INotificationBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hints<Impl: INotificationBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTextElements<Impl: INotificationBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTextElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotificationBinding, BASE_OFFSET>(),
            Template: Template::<Impl, IMPL_OFFSET>,
            SetTemplate: SetTemplate::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
            Hints: Hints::<Impl, IMPL_OFFSET>,
            GetTextElements: GetTextElements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotificationBinding as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INotificationDataImpl: Sized {
    fn Values(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn SequenceNumber(&self) -> ::windows::core::Result<u32>;
    fn SetSequenceNumber(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INotificationData {
    const NAME: &'static str = "Windows.UI.Notifications.INotificationData";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INotificationDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationDataVtbl {
        unsafe extern "system" fn Values<Impl: INotificationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SequenceNumber<Impl: INotificationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SequenceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSequenceNumber<Impl: INotificationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSequenceNumber(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotificationData, BASE_OFFSET>(),
            Values: Values::<Impl, IMPL_OFFSET>,
            SequenceNumber: SequenceNumber::<Impl, IMPL_OFFSET>,
            SetSequenceNumber: SetSequenceNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotificationData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INotificationDataFactoryImpl: Sized {
    fn CreateNotificationDataWithValuesAndSequenceNumber(&self, initialvalues: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>, sequencenumber: u32) -> ::windows::core::Result<NotificationData>;
    fn CreateNotificationDataWithValues(&self, initialvalues: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<NotificationData>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INotificationDataFactory {
    const NAME: &'static str = "Windows.UI.Notifications.INotificationDataFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INotificationDataFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationDataFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationDataFactoryVtbl {
        unsafe extern "system" fn CreateNotificationDataWithValuesAndSequenceNumber<Impl: INotificationDataFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalues: ::windows::core::RawPtr, sequencenumber: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNotificationDataWithValuesAndSequenceNumber(&*(&initialvalues as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType), sequencenumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNotificationDataWithValues<Impl: INotificationDataFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialvalues: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNotificationDataWithValues(&*(&initialvalues as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotificationDataFactory, BASE_OFFSET>(),
            CreateNotificationDataWithValuesAndSequenceNumber: CreateNotificationDataWithValuesAndSequenceNumber::<Impl, IMPL_OFFSET>,
            CreateNotificationDataWithValues: CreateNotificationDataWithValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotificationDataFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INotificationVisualImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Bindings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<NotificationBinding>>;
    fn GetBinding(&self, templatename: &::windows::core::HSTRING) -> ::windows::core::Result<NotificationBinding>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INotificationVisual {
    const NAME: &'static str = "Windows.UI.Notifications.INotificationVisual";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INotificationVisualVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationVisualImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationVisualVtbl {
        unsafe extern "system" fn Language<Impl: INotificationVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: INotificationVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bindings<Impl: INotificationVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bindings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBinding<Impl: INotificationVisualImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, templatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBinding(&*(&templatename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INotificationVisual, BASE_OFFSET>(),
            Language: Language::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
            Bindings: Bindings::<Impl, IMPL_OFFSET>,
            GetBinding: GetBinding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotificationVisual as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScheduledTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledTileNotification";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IScheduledTileNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledTileNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledTileNotificationVtbl {
        unsafe extern "system" fn Content<Impl: IScheduledTileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeliveryTime<Impl: IScheduledTileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeliveryTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: IScheduledTileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: IScheduledTileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTag<Impl: IScheduledTileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IScheduledTileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IScheduledTileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IScheduledTileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScheduledTileNotification, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            DeliveryTime: DeliveryTime::<Impl, IMPL_OFFSET>,
            SetExpirationTime: SetExpirationTime::<Impl, IMPL_OFFSET>,
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledTileNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScheduledTileNotificationFactoryImpl: Sized {
    fn CreateScheduledTileNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>, deliverytime: &super::super::Foundation::DateTime) -> ::windows::core::Result<ScheduledTileNotification>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScheduledTileNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledTileNotificationFactory";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IScheduledTileNotificationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledTileNotificationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledTileNotificationFactoryVtbl {
        unsafe extern "system" fn CreateScheduledTileNotification<Impl: IScheduledTileNotificationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, deliverytime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScheduledTileNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType), &*(&deliverytime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScheduledTileNotificationFactory, BASE_OFFSET>(),
            CreateScheduledTileNotification: CreateScheduledTileNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledTileNotificationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScheduledToastNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SnoozeInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn MaximumSnoozeCount(&self) -> ::windows::core::Result<u32>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScheduledToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotification";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IScheduledToastNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledToastNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledToastNotificationVtbl {
        unsafe extern "system" fn Content<Impl: IScheduledToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeliveryTime<Impl: IScheduledToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeliveryTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SnoozeInterval<Impl: IScheduledToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnoozeInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumSnoozeCount<Impl: IScheduledToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumSnoozeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IScheduledToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IScheduledToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScheduledToastNotification, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            DeliveryTime: DeliveryTime::<Impl, IMPL_OFFSET>,
            SnoozeInterval: SnoozeInterval::<Impl, IMPL_OFFSET>,
            MaximumSnoozeCount: MaximumSnoozeCount::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledToastNotification as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledToastNotification2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledToastNotification2Vtbl {
        unsafe extern "system" fn SetTag<Impl: IScheduledToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IScheduledToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Impl: IScheduledToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroup(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Group<Impl: IScheduledToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuppressPopup<Impl: IScheduledToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuppressPopup(value).into()
        }
        unsafe extern "system" fn SuppressPopup<Impl: IScheduledToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressPopup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScheduledToastNotification2, BASE_OFFSET>(),
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            SetGroup: SetGroup::<Impl, IMPL_OFFSET>,
            Group: Group::<Impl, IMPL_OFFSET>,
            SetSuppressPopup: SetSuppressPopup::<Impl, IMPL_OFFSET>,
            SuppressPopup: SuppressPopup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledToastNotification2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledToastNotification3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledToastNotification3Vtbl {
        unsafe extern "system" fn NotificationMirroring<Impl: IScheduledToastNotification3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationMirroring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMirroring<Impl: IScheduledToastNotification3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationMirroring(value).into()
        }
        unsafe extern "system" fn RemoteId<Impl: IScheduledToastNotification3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRemoteId<Impl: IScheduledToastNotification3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScheduledToastNotification3, BASE_OFFSET>(),
            NotificationMirroring: NotificationMirroring::<Impl, IMPL_OFFSET>,
            SetNotificationMirroring: SetNotificationMirroring::<Impl, IMPL_OFFSET>,
            RemoteId: RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId: SetRemoteId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledToastNotification3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScheduledToastNotification4Impl: Sized {
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScheduledToastNotification4 {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotification4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScheduledToastNotification4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledToastNotification4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledToastNotification4Vtbl {
        unsafe extern "system" fn ExpirationTime<Impl: IScheduledToastNotification4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpirationTime<Impl: IScheduledToastNotification4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScheduledToastNotification4, BASE_OFFSET>(),
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
            SetExpirationTime: SetExpirationTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledToastNotification4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScheduledToastNotificationFactoryImpl: Sized {
    fn CreateScheduledToastNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>, deliverytime: &super::super::Foundation::DateTime) -> ::windows::core::Result<ScheduledToastNotification>;
    fn CreateScheduledToastNotificationRecurring(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>, deliverytime: &super::super::Foundation::DateTime, snoozeinterval: &super::super::Foundation::TimeSpan, maximumsnoozecount: u32) -> ::windows::core::Result<ScheduledToastNotification>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScheduledToastNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotificationFactory";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IScheduledToastNotificationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledToastNotificationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledToastNotificationFactoryVtbl {
        unsafe extern "system" fn CreateScheduledToastNotification<Impl: IScheduledToastNotificationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, deliverytime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScheduledToastNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType), &*(&deliverytime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScheduledToastNotificationRecurring<Impl: IScheduledToastNotificationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, deliverytime: super::super::Foundation::DateTime, snoozeinterval: super::super::Foundation::TimeSpan, maximumsnoozecount: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScheduledToastNotificationFactory, BASE_OFFSET>(),
            CreateScheduledToastNotification: CreateScheduledToastNotification::<Impl, IMPL_OFFSET>,
            CreateScheduledToastNotificationRecurring: CreateScheduledToastNotificationRecurring::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledToastNotificationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScheduledToastNotificationShowingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn ScheduledToastNotification(&self) -> ::windows::core::Result<ScheduledToastNotification>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScheduledToastNotificationShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.IScheduledToastNotificationShowingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScheduledToastNotificationShowingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledToastNotificationShowingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledToastNotificationShowingEventArgsVtbl {
        unsafe extern "system" fn Cancel<Impl: IScheduledToastNotificationShowingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: IScheduledToastNotificationShowingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn ScheduledToastNotification<Impl: IScheduledToastNotificationShowingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduledToastNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IScheduledToastNotificationShowingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScheduledToastNotificationShowingEventArgs, BASE_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            SetCancel: SetCancel::<Impl, IMPL_OFFSET>,
            ScheduledToastNotification: ScheduledToastNotification::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledToastNotificationShowingEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShownTileNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShownTileNotificationVtbl {
        unsafe extern "system" fn Arguments<Impl: IShownTileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IShownTileNotification, BASE_OFFSET>(), Arguments: Arguments::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShownTileNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITileFlyoutNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileFlyoutNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ITileFlyoutNotification";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ITileFlyoutNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileFlyoutNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileFlyoutNotificationVtbl {
        unsafe extern "system" fn Content<Impl: ITileFlyoutNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpirationTime<Impl: ITileFlyoutNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: ITileFlyoutNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileFlyoutNotification, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            SetExpirationTime: SetExpirationTime::<Impl, IMPL_OFFSET>,
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileFlyoutNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait ITileFlyoutNotificationFactoryImpl: Sized {
    fn CreateTileFlyoutNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<TileFlyoutNotification>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileFlyoutNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.ITileFlyoutNotificationFactory";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ITileFlyoutNotificationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileFlyoutNotificationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileFlyoutNotificationFactoryVtbl {
        unsafe extern "system" fn CreateTileFlyoutNotification<Impl: ITileFlyoutNotificationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileFlyoutNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileFlyoutNotificationFactory, BASE_OFFSET>(),
            CreateTileFlyoutNotification: CreateTileFlyoutNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileFlyoutNotificationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait ITileFlyoutUpdateManagerStaticsImpl: Sized {
    fn CreateTileFlyoutUpdaterForApplication(&self) -> ::windows::core::Result<TileFlyoutUpdater>;
    fn CreateTileFlyoutUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileFlyoutUpdater>;
    fn CreateTileFlyoutUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileFlyoutUpdater>;
    fn GetTemplateContent(&self, r#type: TileFlyoutTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileFlyoutUpdateManagerStatics {
    const NAME: &'static str = "Windows.UI.Notifications.ITileFlyoutUpdateManagerStatics";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ITileFlyoutUpdateManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileFlyoutUpdateManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileFlyoutUpdateManagerStaticsVtbl {
        unsafe extern "system" fn CreateTileFlyoutUpdaterForApplication<Impl: ITileFlyoutUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileFlyoutUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileFlyoutUpdaterForApplicationWithId<Impl: ITileFlyoutUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileFlyoutUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileFlyoutUpdaterForSecondaryTile<Impl: ITileFlyoutUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileFlyoutUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplateContent<Impl: ITileFlyoutUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TileFlyoutTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemplateContent(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileFlyoutUpdateManagerStatics, BASE_OFFSET>(),
            CreateTileFlyoutUpdaterForApplication: CreateTileFlyoutUpdaterForApplication::<Impl, IMPL_OFFSET>,
            CreateTileFlyoutUpdaterForApplicationWithId: CreateTileFlyoutUpdaterForApplicationWithId::<Impl, IMPL_OFFSET>,
            CreateTileFlyoutUpdaterForSecondaryTile: CreateTileFlyoutUpdaterForSecondaryTile::<Impl, IMPL_OFFSET>,
            GetTemplateContent: GetTemplateContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileFlyoutUpdateManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITileFlyoutUpdaterImpl: Sized {
    fn Update(&self, notification: &::core::option::Option<TileFlyoutNotification>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdate(&self, tileflyoutcontent: &::core::option::Option<super::super::Foundation::Uri>, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StartPeriodicUpdateAtTime(&self, tileflyoutcontent: &::core::option::Option<super::super::Foundation::Uri>, starttime: &super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::Result<()>;
    fn StopPeriodicUpdate(&self) -> ::windows::core::Result<()>;
    fn Setting(&self) -> ::windows::core::Result<NotificationSetting>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileFlyoutUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.ITileFlyoutUpdater";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITileFlyoutUpdaterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileFlyoutUpdaterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileFlyoutUpdaterVtbl {
        unsafe extern "system" fn Update<Impl: ITileFlyoutUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(&*(&notification as *const <TileFlyoutNotification as ::windows::core::Abi>::Abi as *const <TileFlyoutNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITileFlyoutUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn StartPeriodicUpdate<Impl: ITileFlyoutUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileflyoutcontent: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdate(&*(&tileflyoutcontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StartPeriodicUpdateAtTime<Impl: ITileFlyoutUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileflyoutcontent: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdateAtTime(&*(&tileflyoutcontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StopPeriodicUpdate<Impl: ITileFlyoutUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopPeriodicUpdate().into()
        }
        unsafe extern "system" fn Setting<Impl: ITileFlyoutUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileFlyoutUpdater, BASE_OFFSET>(),
            Update: Update::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            StartPeriodicUpdate: StartPeriodicUpdate::<Impl, IMPL_OFFSET>,
            StartPeriodicUpdateAtTime: StartPeriodicUpdateAtTime::<Impl, IMPL_OFFSET>,
            StopPeriodicUpdate: StopPeriodicUpdate::<Impl, IMPL_OFFSET>,
            Setting: Setting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileFlyoutUpdater as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITileNotificationImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetExpirationTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ITileNotification";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ITileNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileNotificationVtbl {
        unsafe extern "system" fn Content<Impl: ITileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpirationTime<Impl: ITileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: ITileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTag<Impl: ITileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: ITileNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileNotification, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            SetExpirationTime: SetExpirationTime::<Impl, IMPL_OFFSET>,
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait ITileNotificationFactoryImpl: Sized {
    fn CreateTileNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<TileNotification>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.ITileNotificationFactory";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ITileNotificationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileNotificationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileNotificationFactoryVtbl {
        unsafe extern "system" fn CreateTileNotification<Impl: ITileNotificationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileNotificationFactory, BASE_OFFSET>(),
            CreateTileNotification: CreateTileNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileNotificationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait ITileUpdateManagerForUserImpl: Sized {
    fn CreateTileUpdaterForApplication(&self) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.ITileUpdateManagerForUser";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ITileUpdateManagerForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileUpdateManagerForUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileUpdateManagerForUserVtbl {
        unsafe extern "system" fn CreateTileUpdaterForApplication<Impl: ITileUpdateManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForApplicationWithId<Impl: ITileUpdateManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForSecondaryTile<Impl: ITileUpdateManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: ITileUpdateManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileUpdateManagerForUser, BASE_OFFSET>(),
            CreateTileUpdaterForApplication: CreateTileUpdaterForApplication::<Impl, IMPL_OFFSET>,
            CreateTileUpdaterForApplicationWithId: CreateTileUpdaterForApplicationWithId::<Impl, IMPL_OFFSET>,
            CreateTileUpdaterForSecondaryTile: CreateTileUpdaterForSecondaryTile::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileUpdateManagerForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait ITileUpdateManagerStaticsImpl: Sized {
    fn CreateTileUpdaterForApplication(&self) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForApplicationWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn CreateTileUpdaterForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<TileUpdater>;
    fn GetTemplateContent(&self, r#type: TileTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileUpdateManagerStatics {
    const NAME: &'static str = "Windows.UI.Notifications.ITileUpdateManagerStatics";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ITileUpdateManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileUpdateManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileUpdateManagerStaticsVtbl {
        unsafe extern "system" fn CreateTileUpdaterForApplication<Impl: ITileUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForApplication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForApplicationWithId<Impl: ITileUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForApplicationWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForSecondaryTile<Impl: ITileUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplateContent<Impl: ITileUpdateManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TileTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemplateContent(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileUpdateManagerStatics, BASE_OFFSET>(),
            CreateTileUpdaterForApplication: CreateTileUpdaterForApplication::<Impl, IMPL_OFFSET>,
            CreateTileUpdaterForApplicationWithId: CreateTileUpdaterForApplicationWithId::<Impl, IMPL_OFFSET>,
            CreateTileUpdaterForSecondaryTile: CreateTileUpdaterForSecondaryTile::<Impl, IMPL_OFFSET>,
            GetTemplateContent: GetTemplateContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileUpdateManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait ITileUpdateManagerStatics2Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<TileUpdateManagerForUser>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileUpdateManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Notifications.ITileUpdateManagerStatics2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ITileUpdateManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileUpdateManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileUpdateManagerStatics2Vtbl {
        unsafe extern "system" fn GetForUser<Impl: ITileUpdateManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITileUpdateManagerStatics2, BASE_OFFSET>(), GetForUser: GetForUser::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileUpdateManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.ITileUpdater";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITileUpdaterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileUpdaterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileUpdaterVtbl {
        unsafe extern "system" fn Update<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(&*(&notification as *const <TileNotification as ::windows::core::Abi>::Abi as *const <TileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn EnableNotificationQueue<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotificationQueue(enable).into()
        }
        unsafe extern "system" fn Setting<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSchedule<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledtile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToSchedule(&*(&scheduledtile as *const <ScheduledTileNotification as ::windows::core::Abi>::Abi as *const <ScheduledTileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveFromSchedule<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledtile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromSchedule(&*(&scheduledtile as *const <ScheduledTileNotification as ::windows::core::Abi>::Abi as *const <ScheduledTileNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetScheduledTileNotifications<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScheduledTileNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartPeriodicUpdate<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilecontent: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdate(&*(&tilecontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StartPeriodicUpdateAtTime<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilecontent: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdateAtTime(&*(&tilecontent as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StopPeriodicUpdate<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopPeriodicUpdate().into()
        }
        unsafe extern "system" fn StartPeriodicUpdateBatch<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilecontents: ::windows::core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartPeriodicUpdateBatch(&*(&tilecontents as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType), requestedinterval).into()
        }
        unsafe extern "system" fn StartPeriodicUpdateBatchAtTime<Impl: ITileUpdaterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tilecontents: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StartPeriodicUpdateBatchAtTime(&*(&tilecontents as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType), &*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), requestedinterval)
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileUpdater, BASE_OFFSET>(),
            Update: Update::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            EnableNotificationQueue: EnableNotificationQueue::<Impl, IMPL_OFFSET>,
            Setting: Setting::<Impl, IMPL_OFFSET>,
            AddToSchedule: AddToSchedule::<Impl, IMPL_OFFSET>,
            RemoveFromSchedule: RemoveFromSchedule::<Impl, IMPL_OFFSET>,
            GetScheduledTileNotifications: GetScheduledTileNotifications::<Impl, IMPL_OFFSET>,
            StartPeriodicUpdate: StartPeriodicUpdate::<Impl, IMPL_OFFSET>,
            StartPeriodicUpdateAtTime: StartPeriodicUpdateAtTime::<Impl, IMPL_OFFSET>,
            StopPeriodicUpdate: StopPeriodicUpdate::<Impl, IMPL_OFFSET>,
            StartPeriodicUpdateBatch: StartPeriodicUpdateBatch::<Impl, IMPL_OFFSET>,
            StartPeriodicUpdateBatchAtTime: StartPeriodicUpdateBatchAtTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileUpdater as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileUpdater2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileUpdater2Vtbl {
        unsafe extern "system" fn EnableNotificationQueueForSquare150x150<Impl: ITileUpdater2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotificationQueueForSquare150x150(enable).into()
        }
        unsafe extern "system" fn EnableNotificationQueueForWide310x150<Impl: ITileUpdater2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotificationQueueForWide310x150(enable).into()
        }
        unsafe extern "system" fn EnableNotificationQueueForSquare310x310<Impl: ITileUpdater2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotificationQueueForSquare310x310(enable).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileUpdater2, BASE_OFFSET>(),
            EnableNotificationQueueForSquare150x150: EnableNotificationQueueForSquare150x150::<Impl, IMPL_OFFSET>,
            EnableNotificationQueueForWide310x150: EnableNotificationQueueForWide310x150::<Impl, IMPL_OFFSET>,
            EnableNotificationQueueForSquare310x310: EnableNotificationQueueForSquare310x310::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileUpdater2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastActivatedEventArgsVtbl {
        unsafe extern "system" fn Arguments<Impl: IToastActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToastActivatedEventArgs, BASE_OFFSET>(), Arguments: Arguments::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IToastActivatedEventArgs2Impl: Sized {
    fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastActivatedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastActivatedEventArgs2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IToastActivatedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastActivatedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastActivatedEventArgs2Vtbl {
        unsafe extern "system" fn UserInput<Impl: IToastActivatedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToastActivatedEventArgs2, BASE_OFFSET>(), UserInput: UserInput::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastActivatedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IToastCollectionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LaunchArgs(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLaunchArgs(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Icon(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetIcon(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastCollection {
    const NAME: &'static str = "Windows.UI.Notifications.IToastCollection";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IToastCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastCollectionVtbl {
        unsafe extern "system" fn Id<Impl: IToastCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IToastCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IToastCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LaunchArgs<Impl: IToastCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchArgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLaunchArgs<Impl: IToastCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLaunchArgs(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Icon<Impl: IToastCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Icon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcon<Impl: IToastCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIcon(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastCollection, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            LaunchArgs: LaunchArgs::<Impl, IMPL_OFFSET>,
            SetLaunchArgs: SetLaunchArgs::<Impl, IMPL_OFFSET>,
            Icon: Icon::<Impl, IMPL_OFFSET>,
            SetIcon: SetIcon::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IToastCollectionFactoryImpl: Sized {
    fn CreateInstance(&self, collectionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, launchargs: &::windows::core::HSTRING, iconuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<ToastCollection>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastCollectionFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IToastCollectionFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IToastCollectionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastCollectionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastCollectionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IToastCollectionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, launchargs: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastCollectionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastCollectionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
pub trait IToastCollectionManagerImpl: Sized {
    fn SaveToastCollectionAsync(&self, collection: &::core::option::Option<ToastCollection>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FindAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ToastCollection>>>;
    fn GetToastCollectionAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastCollection>>;
    fn RemoveToastCollectionAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RemoveAllToastCollectionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastCollectionManager {
    const NAME: &'static str = "Windows.UI.Notifications.IToastCollectionManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl IToastCollectionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastCollectionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastCollectionManagerVtbl {
        unsafe extern "system" fn SaveToastCollectionAsync<Impl: IToastCollectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToastCollectionAsync(&*(&collection as *const <ToastCollection as ::windows::core::Abi>::Abi as *const <ToastCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllToastCollectionsAsync<Impl: IToastCollectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllToastCollectionsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToastCollectionAsync<Impl: IToastCollectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetToastCollectionAsync(&*(&collectionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveToastCollectionAsync<Impl: IToastCollectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveToastCollectionAsync(&*(&collectionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllToastCollectionsAsync<Impl: IToastCollectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAllToastCollectionsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IToastCollectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppId<Impl: IToastCollectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastCollectionManager, BASE_OFFSET>(),
            SaveToastCollectionAsync: SaveToastCollectionAsync::<Impl, IMPL_OFFSET>,
            FindAllToastCollectionsAsync: FindAllToastCollectionsAsync::<Impl, IMPL_OFFSET>,
            GetToastCollectionAsync: GetToastCollectionAsync::<Impl, IMPL_OFFSET>,
            RemoveToastCollectionAsync: RemoveToastCollectionAsync::<Impl, IMPL_OFFSET>,
            RemoveAllToastCollectionsAsync: RemoveAllToastCollectionsAsync::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
            AppId: AppId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastCollectionManager as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastDismissedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastDismissedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IToastDismissedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ToastDismissalReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToastDismissedEventArgs, BASE_OFFSET>(), Reason: Reason::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastDismissedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastFailedEventArgsVtbl {
        unsafe extern "system" fn ErrorCode<Impl: IToastFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToastFailedEventArgs, BASE_OFFSET>(), ErrorCode: ErrorCode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotification";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IToastNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationVtbl {
        unsafe extern "system" fn Content<Impl: IToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpirationTime<Impl: IToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: IToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Dismissed<Impl: IToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dismissed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDismissed<Impl: IToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDismissed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Activated<Impl: IToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: IToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Failed<Impl: IToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Failed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFailed<Impl: IToastNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotification, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            SetExpirationTime: SetExpirationTime::<Impl, IMPL_OFFSET>,
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
            Dismissed: Dismissed::<Impl, IMPL_OFFSET>,
            RemoveDismissed: RemoveDismissed::<Impl, IMPL_OFFSET>,
            Activated: Activated::<Impl, IMPL_OFFSET>,
            RemoveActivated: RemoveActivated::<Impl, IMPL_OFFSET>,
            Failed: Failed::<Impl, IMPL_OFFSET>,
            RemoveFailed: RemoveFailed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotification as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotification2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotification2Vtbl {
        unsafe extern "system" fn SetTag<Impl: IToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Impl: IToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroup(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Group<Impl: IToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuppressPopup<Impl: IToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuppressPopup(value).into()
        }
        unsafe extern "system" fn SuppressPopup<Impl: IToastNotification2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressPopup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotification2, BASE_OFFSET>(),
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            SetGroup: SetGroup::<Impl, IMPL_OFFSET>,
            Group: Group::<Impl, IMPL_OFFSET>,
            SetSuppressPopup: SetSuppressPopup::<Impl, IMPL_OFFSET>,
            SuppressPopup: SuppressPopup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotification2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotification3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotification3Vtbl {
        unsafe extern "system" fn NotificationMirroring<Impl: IToastNotification3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationMirroring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMirroring<Impl: IToastNotification3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationMirroring(value).into()
        }
        unsafe extern "system" fn RemoteId<Impl: IToastNotification3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRemoteId<Impl: IToastNotification3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotification3, BASE_OFFSET>(),
            NotificationMirroring: NotificationMirroring::<Impl, IMPL_OFFSET>,
            SetNotificationMirroring: SetNotificationMirroring::<Impl, IMPL_OFFSET>,
            RemoteId: RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId: SetRemoteId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotification3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotification4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotification4Vtbl {
        unsafe extern "system" fn Data<Impl: IToastNotification4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IToastNotification4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <NotificationData as ::windows::core::Abi>::Abi as *const <NotificationData as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Priority<Impl: IToastNotification4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ToastNotificationPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IToastNotification4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ToastNotificationPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotification4, BASE_OFFSET>(),
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotification4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotification6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotification6Vtbl {
        unsafe extern "system" fn ExpiresOnReboot<Impl: IToastNotification6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpiresOnReboot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpiresOnReboot<Impl: IToastNotification6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpiresOnReboot(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotification6, BASE_OFFSET>(),
            ExpiresOnReboot: ExpiresOnReboot::<Impl, IMPL_OFFSET>,
            SetExpiresOnReboot: SetExpiresOnReboot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotification6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IToastNotificationActionTriggerDetailImpl: Sized {
    fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotificationActionTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationActionTriggerDetail";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IToastNotificationActionTriggerDetailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationActionTriggerDetailImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationActionTriggerDetailVtbl {
        unsafe extern "system" fn Argument<Impl: IToastNotificationActionTriggerDetailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Argument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserInput<Impl: IToastNotificationActionTriggerDetailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationActionTriggerDetail, BASE_OFFSET>(),
            Argument: Argument::<Impl, IMPL_OFFSET>,
            UserInput: UserInput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationActionTriggerDetail as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IToastNotificationFactoryImpl: Sized {
    fn CreateToastNotification(&self, content: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<ToastNotification>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotificationFactory {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationFactory";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IToastNotificationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationFactoryVtbl {
        unsafe extern "system" fn CreateToastNotification<Impl: IToastNotificationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateToastNotification(&*(&content as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationFactory, BASE_OFFSET>(),
            CreateToastNotification: CreateToastNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationHistoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationHistoryVtbl {
        unsafe extern "system" fn RemoveGroup<Impl: IToastNotificationHistoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGroup(&*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveGroupWithId<Impl: IToastNotificationHistoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGroupWithId(&*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveGroupedTagWithId<Impl: IToastNotificationHistoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RemoveGroupedTagWithId(
                    &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RemoveGroupedTag<Impl: IToastNotificationHistoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGroupedTag(&*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: IToastNotificationHistoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clear<Impl: IToastNotificationHistoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn ClearWithId<Impl: IToastNotificationHistoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationHistory, BASE_OFFSET>(),
            RemoveGroup: RemoveGroup::<Impl, IMPL_OFFSET>,
            RemoveGroupWithId: RemoveGroupWithId::<Impl, IMPL_OFFSET>,
            RemoveGroupedTagWithId: RemoveGroupedTagWithId::<Impl, IMPL_OFFSET>,
            RemoveGroupedTag: RemoveGroupedTag::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            ClearWithId: ClearWithId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationHistory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IToastNotificationHistory2Impl: Sized {
    fn GetHistory(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>>;
    fn GetHistoryWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotificationHistory2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationHistory2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IToastNotificationHistory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationHistory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationHistory2Vtbl {
        unsafe extern "system" fn GetHistory<Impl: IToastNotificationHistory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHistory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHistoryWithId<Impl: IToastNotificationHistory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHistoryWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationHistory2, BASE_OFFSET>(),
            GetHistory: GetHistory::<Impl, IMPL_OFFSET>,
            GetHistoryWithId: GetHistoryWithId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationHistory2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationHistoryChangedTriggerDetailImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationHistoryChangedTriggerDetailVtbl {
        unsafe extern "system" fn ChangeType<Impl: IToastNotificationHistoryChangedTriggerDetailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ToastHistoryChangedType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationHistoryChangedTriggerDetail, BASE_OFFSET>(),
            ChangeType: ChangeType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationHistoryChangedTriggerDetail as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationHistoryChangedTriggerDetail2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationHistoryChangedTriggerDetail2Vtbl {
        unsafe extern "system" fn CollectionId<Impl: IToastNotificationHistoryChangedTriggerDetail2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollectionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationHistoryChangedTriggerDetail2, BASE_OFFSET>(),
            CollectionId: CollectionId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationHistoryChangedTriggerDetail2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IToastNotificationManagerForUserImpl: Sized {
    fn CreateToastNotifier(&self) -> ::windows::core::Result<ToastNotifier>;
    fn CreateToastNotifierWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotifier>;
    fn History(&self) -> ::windows::core::Result<ToastNotificationHistory>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotificationManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerForUser";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IToastNotificationManagerForUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationManagerForUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationManagerForUserVtbl {
        unsafe extern "system" fn CreateToastNotifier<Impl: IToastNotificationManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateToastNotifierWithId<Impl: IToastNotificationManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifierWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn History<Impl: IToastNotificationManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).History() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IToastNotificationManagerForUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationManagerForUser, BASE_OFFSET>(),
            CreateToastNotifier: CreateToastNotifier::<Impl, IMPL_OFFSET>,
            CreateToastNotifierWithId: CreateToastNotifierWithId::<Impl, IMPL_OFFSET>,
            History: History::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationManagerForUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IToastNotificationManagerForUser2Impl: Sized {
    fn GetToastNotifierForToastCollectionIdAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotifier>>;
    fn GetHistoryForToastCollectionIdAsync(&self, collectionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ToastNotificationHistory>>;
    fn GetToastCollectionManager(&self) -> ::windows::core::Result<ToastCollectionManager>;
    fn GetToastCollectionManagerWithAppId(&self, appid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastCollectionManager>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotificationManagerForUser2 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerForUser2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IToastNotificationManagerForUser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationManagerForUser2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationManagerForUser2Vtbl {
        unsafe extern "system" fn GetToastNotifierForToastCollectionIdAsync<Impl: IToastNotificationManagerForUser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetToastNotifierForToastCollectionIdAsync(&*(&collectionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHistoryForToastCollectionIdAsync<Impl: IToastNotificationManagerForUser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHistoryForToastCollectionIdAsync(&*(&collectionid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToastCollectionManager<Impl: IToastNotificationManagerForUser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetToastCollectionManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetToastCollectionManagerWithAppId<Impl: IToastNotificationManagerForUser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetToastCollectionManagerWithAppId(&*(&appid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationManagerForUser2, BASE_OFFSET>(),
            GetToastNotifierForToastCollectionIdAsync: GetToastNotifierForToastCollectionIdAsync::<Impl, IMPL_OFFSET>,
            GetHistoryForToastCollectionIdAsync: GetHistoryForToastCollectionIdAsync::<Impl, IMPL_OFFSET>,
            GetToastCollectionManager: GetToastCollectionManager::<Impl, IMPL_OFFSET>,
            GetToastCollectionManagerWithAppId: GetToastCollectionManagerWithAppId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationManagerForUser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
pub trait IToastNotificationManagerStaticsImpl: Sized {
    fn CreateToastNotifier(&self) -> ::windows::core::Result<ToastNotifier>;
    fn CreateToastNotifierWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotifier>;
    fn GetTemplateContent(&self, r#type: ToastTemplateType) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotificationManagerStatics {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerStatics";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "implement_exclusive"))]
impl IToastNotificationManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationManagerStaticsVtbl {
        unsafe extern "system" fn CreateToastNotifier<Impl: IToastNotificationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateToastNotifierWithId<Impl: IToastNotificationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifierWithId(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplateContent<Impl: IToastNotificationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ToastTemplateType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemplateContent(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationManagerStatics, BASE_OFFSET>(),
            CreateToastNotifier: CreateToastNotifier::<Impl, IMPL_OFFSET>,
            CreateToastNotifierWithId: CreateToastNotifierWithId::<Impl, IMPL_OFFSET>,
            GetTemplateContent: GetTemplateContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationManagerStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationManagerStatics2Vtbl {
        unsafe extern "system" fn History<Impl: IToastNotificationManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).History() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationManagerStatics2, BASE_OFFSET>(),
            History: History::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IToastNotificationManagerStatics4Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<ToastNotificationManagerForUser>;
    fn ConfigureNotificationMirroring(&self, value: NotificationMirroring) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotificationManagerStatics4 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotificationManagerStatics4";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IToastNotificationManagerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationManagerStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationManagerStatics4Vtbl {
        unsafe extern "system" fn GetForUser<Impl: IToastNotificationManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConfigureNotificationMirroring<Impl: IToastNotificationManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureNotificationMirroring(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationManagerStatics4, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
            ConfigureNotificationMirroring: ConfigureNotificationMirroring::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationManagerStatics4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationManagerStatics5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationManagerStatics5Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IToastNotificationManagerStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationManagerStatics5, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationManagerStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IToastNotifierImpl: Sized {
    fn Show(&self, notification: &::core::option::Option<ToastNotification>) -> ::windows::core::Result<()>;
    fn Hide(&self, notification: &::core::option::Option<ToastNotification>) -> ::windows::core::Result<()>;
    fn Setting(&self) -> ::windows::core::Result<NotificationSetting>;
    fn AddToSchedule(&self, scheduledtoast: &::core::option::Option<ScheduledToastNotification>) -> ::windows::core::Result<()>;
    fn RemoveFromSchedule(&self, scheduledtoast: &::core::option::Option<ScheduledToastNotification>) -> ::windows::core::Result<()>;
    fn GetScheduledToastNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ScheduledToastNotification>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotifier {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotifier";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IToastNotifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotifierVtbl {
        unsafe extern "system" fn Show<Impl: IToastNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(&*(&notification as *const <ToastNotification as ::windows::core::Abi>::Abi as *const <ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hide<Impl: IToastNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hide(&*(&notification as *const <ToastNotification as ::windows::core::Abi>::Abi as *const <ToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Setting<Impl: IToastNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSchedule<Impl: IToastNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledtoast: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToSchedule(&*(&scheduledtoast as *const <ScheduledToastNotification as ::windows::core::Abi>::Abi as *const <ScheduledToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveFromSchedule<Impl: IToastNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduledtoast: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromSchedule(&*(&scheduledtoast as *const <ScheduledToastNotification as ::windows::core::Abi>::Abi as *const <ScheduledToastNotification as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetScheduledToastNotifications<Impl: IToastNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScheduledToastNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotifier, BASE_OFFSET>(),
            Show: Show::<Impl, IMPL_OFFSET>,
            Hide: Hide::<Impl, IMPL_OFFSET>,
            Setting: Setting::<Impl, IMPL_OFFSET>,
            AddToSchedule: AddToSchedule::<Impl, IMPL_OFFSET>,
            RemoveFromSchedule: RemoveFromSchedule::<Impl, IMPL_OFFSET>,
            GetScheduledToastNotifications: GetScheduledToastNotifications::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotifier as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotifier2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotifier2Vtbl {
        unsafe extern "system" fn UpdateWithTagAndGroup<Impl: IToastNotifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut NotificationUpdateResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateWithTagAndGroup(&*(&data as *const <NotificationData as ::windows::core::Abi>::Abi as *const <NotificationData as ::windows::core::DefaultType>::DefaultType), &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&group as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateWithTag<Impl: IToastNotifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut NotificationUpdateResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateWithTag(&*(&data as *const <NotificationData as ::windows::core::Abi>::Abi as *const <NotificationData as ::windows::core::DefaultType>::DefaultType), &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotifier2, BASE_OFFSET>(),
            UpdateWithTagAndGroup: UpdateWithTagAndGroup::<Impl, IMPL_OFFSET>,
            UpdateWithTag: UpdateWithTag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotifier2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IToastNotifier3Impl: Sized {
    fn ScheduledToastNotificationShowing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScheduledToastNotificationShowing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToastNotifier3 {
    const NAME: &'static str = "Windows.UI.Notifications.IToastNotifier3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IToastNotifier3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotifier3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotifier3Vtbl {
        unsafe extern "system" fn ScheduledToastNotificationShowing<Impl: IToastNotifier3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduledToastNotificationShowing(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScheduledToastNotificationShowing<Impl: IToastNotifier3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScheduledToastNotificationShowing(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotifier3, BASE_OFFSET>(),
            ScheduledToastNotificationShowing: ScheduledToastNotificationShowing::<Impl, IMPL_OFFSET>,
            RemoveScheduledToastNotificationShowing: RemoveScheduledToastNotificationShowing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotifier3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserNotificationImpl: Sized {
    fn Notification(&self) -> ::windows::core::Result<Notification>;
    fn AppInfo(&self) -> ::windows::core::Result<super::super::ApplicationModel::AppInfo>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn CreationTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserNotification {
    const NAME: &'static str = "Windows.UI.Notifications.IUserNotification";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl IUserNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserNotificationVtbl {
        unsafe extern "system" fn Notification<Impl: IUserNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppInfo<Impl: IUserNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IUserNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreationTime<Impl: IUserNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserNotification, BASE_OFFSET>(),
            Notification: Notification::<Impl, IMPL_OFFSET>,
            AppInfo: AppInfo::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            CreationTime: CreationTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserNotification as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserNotificationChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserNotificationChangedEventArgsVtbl {
        unsafe extern "system" fn ChangeKind<Impl: IUserNotificationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserNotificationChangedKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserNotificationId<Impl: IUserNotificationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserNotificationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserNotificationChangedEventArgs, BASE_OFFSET>(),
            ChangeKind: ChangeKind::<Impl, IMPL_OFFSET>,
            UserNotificationId: UserNotificationId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserNotificationChangedEventArgs as ::windows::core::Interface>::IID
    }
}
