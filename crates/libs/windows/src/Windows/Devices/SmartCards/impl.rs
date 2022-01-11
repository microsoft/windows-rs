#[cfg(feature = "implement_exclusive")]
pub trait ICardAddedEventArgsImpl: Sized {
    fn SmartCard(&self) -> ::windows::core::Result<SmartCard>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICardAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.ICardAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICardAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICardAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICardAddedEventArgsVtbl {
        unsafe extern "system" fn SmartCard<Impl: ICardAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartCard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICardAddedEventArgs, BASE_OFFSET>(), SmartCard: SmartCard::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICardAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICardRemovedEventArgsImpl: Sized {
    fn SmartCard(&self) -> ::windows::core::Result<SmartCard>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICardRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.ICardRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICardRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICardRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICardRemovedEventArgsVtbl {
        unsafe extern "system" fn SmartCard<Impl: ICardRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartCard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICardRemovedEventArgs, BASE_OFFSET>(), SmartCard: SmartCard::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICardRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IKnownSmartCardAppletIdsImpl: Sized {
    fn PaymentSystemEnvironment(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ProximityPaymentSystemEnvironment(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownSmartCardAppletIds {
    const NAME: &'static str = "Windows.Devices.SmartCards.IKnownSmartCardAppletIds";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IKnownSmartCardAppletIdsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownSmartCardAppletIdsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownSmartCardAppletIdsVtbl {
        unsafe extern "system" fn PaymentSystemEnvironment<Impl: IKnownSmartCardAppletIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PaymentSystemEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProximityPaymentSystemEnvironment<Impl: IKnownSmartCardAppletIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProximityPaymentSystemEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownSmartCardAppletIds, BASE_OFFSET>(),
            PaymentSystemEnvironment: PaymentSystemEnvironment::<Impl, IMPL_OFFSET>,
            ProximityPaymentSystemEnvironment: ProximityPaymentSystemEnvironment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownSmartCardAppletIds as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardImpl: Sized {
    fn Reader(&self) -> ::windows::core::Result<SmartCardReader>;
    fn GetStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardStatus>>;
    fn GetAnswerToResetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCard {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCard";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardVtbl {
        unsafe extern "system" fn Reader<Impl: ISmartCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusAsync<Impl: ISmartCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnswerToResetAsync<Impl: ISmartCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnswerToResetAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCard, BASE_OFFSET>(),
            Reader: Reader::<Impl, IMPL_OFFSET>,
            GetStatusAsync: GetStatusAsync::<Impl, IMPL_OFFSET>,
            GetAnswerToResetAsync: GetAnswerToResetAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCard as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardAppletIdGroupImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppletIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>;
    fn SmartCardEmulationCategory(&self) -> ::windows::core::Result<SmartCardEmulationCategory>;
    fn SetSmartCardEmulationCategory(&self, value: SmartCardEmulationCategory) -> ::windows::core::Result<()>;
    fn SmartCardEmulationType(&self) -> ::windows::core::Result<SmartCardEmulationType>;
    fn SetSmartCardEmulationType(&self, value: SmartCardEmulationType) -> ::windows::core::Result<()>;
    fn AutomaticEnablement(&self) -> ::windows::core::Result<bool>;
    fn SetAutomaticEnablement(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardAppletIdGroup {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAppletIdGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardAppletIdGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAppletIdGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAppletIdGroupVtbl {
        unsafe extern "system" fn DisplayName<Impl: ISmartCardAppletIdGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: ISmartCardAppletIdGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppletIds<Impl: ISmartCardAppletIdGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppletIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmartCardEmulationCategory<Impl: ISmartCardAppletIdGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulationCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartCardEmulationCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmartCardEmulationCategory<Impl: ISmartCardAppletIdGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmartCardEmulationCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmartCardEmulationCategory(value).into()
        }
        unsafe extern "system" fn SmartCardEmulationType<Impl: ISmartCardAppletIdGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartCardEmulationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmartCardEmulationType<Impl: ISmartCardAppletIdGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmartCardEmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmartCardEmulationType(value).into()
        }
        unsafe extern "system" fn AutomaticEnablement<Impl: ISmartCardAppletIdGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomaticEnablement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomaticEnablement<Impl: ISmartCardAppletIdGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomaticEnablement(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAppletIdGroup, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            AppletIds: AppletIds::<Impl, IMPL_OFFSET>,
            SmartCardEmulationCategory: SmartCardEmulationCategory::<Impl, IMPL_OFFSET>,
            SetSmartCardEmulationCategory: SetSmartCardEmulationCategory::<Impl, IMPL_OFFSET>,
            SmartCardEmulationType: SmartCardEmulationType::<Impl, IMPL_OFFSET>,
            SetSmartCardEmulationType: SetSmartCardEmulationType::<Impl, IMPL_OFFSET>,
            AutomaticEnablement: AutomaticEnablement::<Impl, IMPL_OFFSET>,
            SetAutomaticEnablement: SetAutomaticEnablement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAppletIdGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardAppletIdGroup2Impl: Sized {
    fn Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogo(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn SecureUserAuthenticationRequired(&self) -> ::windows::core::Result<bool>;
    fn SetSecureUserAuthenticationRequired(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardAppletIdGroup2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAppletIdGroup2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardAppletIdGroup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAppletIdGroup2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAppletIdGroup2Vtbl {
        unsafe extern "system" fn Logo<Impl: ISmartCardAppletIdGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLogo<Impl: ISmartCardAppletIdGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogo(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: ISmartCardAppletIdGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: ISmartCardAppletIdGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: ISmartCardAppletIdGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecureUserAuthenticationRequired<Impl: ISmartCardAppletIdGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecureUserAuthenticationRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecureUserAuthenticationRequired<Impl: ISmartCardAppletIdGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecureUserAuthenticationRequired(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAppletIdGroup2, BASE_OFFSET>(),
            Logo: Logo::<Impl, IMPL_OFFSET>,
            SetLogo: SetLogo::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            SecureUserAuthenticationRequired: SecureUserAuthenticationRequired::<Impl, IMPL_OFFSET>,
            SetSecureUserAuthenticationRequired: SetSecureUserAuthenticationRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAppletIdGroup2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardAppletIdGroupFactoryImpl: Sized {
    fn Create(&self, displayname: &::windows::core::HSTRING, appletids: &::core::option::Option<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType) -> ::windows::core::Result<SmartCardAppletIdGroup>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardAppletIdGroupFactory {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAppletIdGroupFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardAppletIdGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAppletIdGroupFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAppletIdGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISmartCardAppletIdGroupFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appletids: ::windows::core::RawPtr, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&appletids as *const <super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer> as ::windows::core::DefaultType>::DefaultType),
                emulationcategory,
                emulationtype,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAppletIdGroupFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAppletIdGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISmartCardAppletIdGroupRegistrationImpl: Sized {
    fn ActivationPolicy(&self) -> ::windows::core::Result<SmartCardAppletIdGroupActivationPolicy>;
    fn AppletIdGroup(&self) -> ::windows::core::Result<SmartCardAppletIdGroup>;
    fn RequestActivationPolicyChangeAsync(&self, policy: SmartCardAppletIdGroupActivationPolicy) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardActivationPolicyChangeResult>>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetAutomaticResponseApdusAsync(&self, apdus: &::core::option::Option<super::super::Foundation::Collections::IIterable<SmartCardAutomaticResponseApdu>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardAppletIdGroupRegistration {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAppletIdGroupRegistration";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISmartCardAppletIdGroupRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAppletIdGroupRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAppletIdGroupRegistrationVtbl {
        unsafe extern "system" fn ActivationPolicy<Impl: ISmartCardAppletIdGroupRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardAppletIdGroupActivationPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivationPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppletIdGroup<Impl: ISmartCardAppletIdGroupRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppletIdGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestActivationPolicyChangeAsync<Impl: ISmartCardAppletIdGroupRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: SmartCardAppletIdGroupActivationPolicy, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestActivationPolicyChangeAsync(policy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISmartCardAppletIdGroupRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutomaticResponseApdusAsync<Impl: ISmartCardAppletIdGroupRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apdus: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutomaticResponseApdusAsync(&*(&apdus as *const <super::super::Foundation::Collections::IIterable<SmartCardAutomaticResponseApdu> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SmartCardAutomaticResponseApdu> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAppletIdGroupRegistration, BASE_OFFSET>(),
            ActivationPolicy: ActivationPolicy::<Impl, IMPL_OFFSET>,
            AppletIdGroup: AppletIdGroup::<Impl, IMPL_OFFSET>,
            RequestActivationPolicyChangeAsync: RequestActivationPolicyChangeAsync::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            SetAutomaticResponseApdusAsync: SetAutomaticResponseApdusAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAppletIdGroupRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISmartCardAppletIdGroupRegistration2Impl: Sized {
    fn SmartCardReaderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPropertiesAsync(&self, props: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardAppletIdGroupRegistration2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAppletIdGroupRegistration2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISmartCardAppletIdGroupRegistration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAppletIdGroupRegistration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAppletIdGroupRegistration2Vtbl {
        unsafe extern "system" fn SmartCardReaderId<Impl: ISmartCardAppletIdGroupRegistration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartCardReaderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertiesAsync<Impl: ISmartCardAppletIdGroupRegistration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertiesAsync(&*(&props as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAppletIdGroupRegistration2, BASE_OFFSET>(),
            SmartCardReaderId: SmartCardReaderId::<Impl, IMPL_OFFSET>,
            SetPropertiesAsync: SetPropertiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAppletIdGroupRegistration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAppletIdGroupStaticsImpl: Sized {
    fn MaxAppletIds(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardAppletIdGroupStatics {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAppletIdGroupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardAppletIdGroupStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAppletIdGroupStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAppletIdGroupStaticsVtbl {
        unsafe extern "system" fn MaxAppletIds<Impl: ISmartCardAppletIdGroupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAppletIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAppletIdGroupStatics, BASE_OFFSET>(),
            MaxAppletIds: MaxAppletIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAppletIdGroupStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardAutomaticResponseApduImpl: Sized {
    fn CommandApdu(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetCommandApdu(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CommandApduBitMask(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetCommandApduBitMask(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ShouldMatchLength(&self) -> ::windows::core::Result<bool>;
    fn SetShouldMatchLength(&self, value: bool) -> ::windows::core::Result<()>;
    fn AppletId(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetAppletId(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ResponseApdu(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetResponseApdu(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardAutomaticResponseApdu {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAutomaticResponseApdu";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardAutomaticResponseApduVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAutomaticResponseApduImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAutomaticResponseApduVtbl {
        unsafe extern "system" fn CommandApdu<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandApdu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommandApdu<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommandApdu(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CommandApduBitMask<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandApduBitMask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommandApduBitMask<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommandApduBitMask(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShouldMatchLength<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldMatchLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldMatchLength<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldMatchLength(value).into()
        }
        unsafe extern "system" fn AppletId<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppletId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppletId<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppletId(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResponseApdu<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseApdu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResponseApdu<Impl: ISmartCardAutomaticResponseApduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResponseApdu(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAutomaticResponseApdu, BASE_OFFSET>(),
            CommandApdu: CommandApdu::<Impl, IMPL_OFFSET>,
            SetCommandApdu: SetCommandApdu::<Impl, IMPL_OFFSET>,
            CommandApduBitMask: CommandApduBitMask::<Impl, IMPL_OFFSET>,
            SetCommandApduBitMask: SetCommandApduBitMask::<Impl, IMPL_OFFSET>,
            ShouldMatchLength: ShouldMatchLength::<Impl, IMPL_OFFSET>,
            SetShouldMatchLength: SetShouldMatchLength::<Impl, IMPL_OFFSET>,
            AppletId: AppletId::<Impl, IMPL_OFFSET>,
            SetAppletId: SetAppletId::<Impl, IMPL_OFFSET>,
            ResponseApdu: ResponseApdu::<Impl, IMPL_OFFSET>,
            SetResponseApdu: SetResponseApdu::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAutomaticResponseApdu as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardAutomaticResponseApdu2Impl: Sized {
    fn InputState(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetInputState(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn OutputState(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetOutputState(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardAutomaticResponseApdu2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAutomaticResponseApdu2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardAutomaticResponseApdu2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAutomaticResponseApdu2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAutomaticResponseApdu2Vtbl {
        unsafe extern "system" fn InputState<Impl: ISmartCardAutomaticResponseApdu2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputState<Impl: ISmartCardAutomaticResponseApdu2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputState(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutputState<Impl: ISmartCardAutomaticResponseApdu2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputState<Impl: ISmartCardAutomaticResponseApdu2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputState(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAutomaticResponseApdu2, BASE_OFFSET>(),
            InputState: InputState::<Impl, IMPL_OFFSET>,
            SetInputState: SetInputState::<Impl, IMPL_OFFSET>,
            OutputState: OutputState::<Impl, IMPL_OFFSET>,
            SetOutputState: SetOutputState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAutomaticResponseApdu2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAutomaticResponseApdu3Impl: Sized {
    fn AllowWhenCryptogramGeneratorNotPrepared(&self) -> ::windows::core::Result<bool>;
    fn SetAllowWhenCryptogramGeneratorNotPrepared(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardAutomaticResponseApdu3 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAutomaticResponseApdu3";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardAutomaticResponseApdu3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAutomaticResponseApdu3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAutomaticResponseApdu3Vtbl {
        unsafe extern "system" fn AllowWhenCryptogramGeneratorNotPrepared<Impl: ISmartCardAutomaticResponseApdu3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowWhenCryptogramGeneratorNotPrepared() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowWhenCryptogramGeneratorNotPrepared<Impl: ISmartCardAutomaticResponseApdu3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowWhenCryptogramGeneratorNotPrepared(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAutomaticResponseApdu3, BASE_OFFSET>(),
            AllowWhenCryptogramGeneratorNotPrepared: AllowWhenCryptogramGeneratorNotPrepared::<Impl, IMPL_OFFSET>,
            SetAllowWhenCryptogramGeneratorNotPrepared: SetAllowWhenCryptogramGeneratorNotPrepared::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAutomaticResponseApdu3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardAutomaticResponseApduFactoryImpl: Sized {
    fn Create(&self, commandapdu: &::core::option::Option<super::super::Storage::Streams::IBuffer>, responseapdu: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<SmartCardAutomaticResponseApdu>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardAutomaticResponseApduFactory {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardAutomaticResponseApduFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardAutomaticResponseApduFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardAutomaticResponseApduFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardAutomaticResponseApduFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISmartCardAutomaticResponseApduFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandapdu: ::windows::core::RawPtr, responseapdu: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&commandapdu as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&responseapdu as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardAutomaticResponseApduFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardAutomaticResponseApduFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardChallengeContextImpl: Sized + IClosableImpl {
    fn Challenge(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn VerifyResponseAsync(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ProvisionAsync(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>, formatcard: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ProvisionAsyncWithNewCardId(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>, formatcard: bool, newcardid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ChangeAdministrativeKeyAsync(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>, newadministrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardChallengeContext {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardChallengeContext";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardChallengeContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardChallengeContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardChallengeContextVtbl {
        unsafe extern "system" fn Challenge<Impl: ISmartCardChallengeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Challenge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyResponseAsync<Impl: ISmartCardChallengeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyResponseAsync(&*(&response as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvisionAsync<Impl: ISmartCardChallengeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, formatcard: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProvisionAsync(&*(&response as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), formatcard) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvisionAsyncWithNewCardId<Impl: ISmartCardChallengeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, formatcard: bool, newcardid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProvisionAsyncWithNewCardId(&*(&response as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), formatcard, &*(&newcardid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeAdministrativeKeyAsync<Impl: ISmartCardChallengeContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, newadministrativekey: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeAdministrativeKeyAsync(&*(&response as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&newadministrativekey as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardChallengeContext, BASE_OFFSET>(),
            Challenge: Challenge::<Impl, IMPL_OFFSET>,
            VerifyResponseAsync: VerifyResponseAsync::<Impl, IMPL_OFFSET>,
            ProvisionAsync: ProvisionAsync::<Impl, IMPL_OFFSET>,
            ProvisionAsyncWithNewCardId: ProvisionAsyncWithNewCardId::<Impl, IMPL_OFFSET>,
            ChangeAdministrativeKeyAsync: ChangeAdministrativeKeyAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardChallengeContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardConnectImpl: Sized {
    fn ConnectAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardConnection>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardConnect {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardConnect";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardConnectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardConnectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardConnectVtbl {
        unsafe extern "system" fn ConnectAsync<Impl: ISmartCardConnectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardConnect, BASE_OFFSET>(), ConnectAsync: ConnectAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardConnect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardConnectionImpl: Sized + IClosableImpl {
    fn TransmitAsync(&self, command: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardConnection {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardConnection";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardConnectionVtbl {
        unsafe extern "system" fn TransmitAsync<Impl: ISmartCardConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmitAsync(&*(&command as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardConnection, BASE_OFFSET>(), TransmitAsync: TransmitAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Core", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramGeneratorImpl: Sized {
    fn SupportedCryptogramMaterialTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialType>>;
    fn SupportedCryptogramAlgorithms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>;
    fn SupportedCryptogramMaterialPackageFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageFormat>>;
    fn SupportedCryptogramMaterialPackageConfirmationResponseFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>>;
    fn SupportedSmartCardCryptogramStorageKeyCapabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCapabilities>>;
    fn DeleteCryptogramMaterialStorageKeyAsync(&self, storagekeyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn CreateCryptogramMaterialStorageKeyAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: &::windows::core::HSTRING, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn RequestCryptogramMaterialStorageKeyInfoAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: &::windows::core::HSTRING, format: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramStorageKeyInfo>>;
    fn ImportCryptogramMaterialPackageAsync(&self, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: &::windows::core::HSTRING, materialpackagename: &::windows::core::HSTRING, cryptogrammaterialpackage: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn TryProvePossessionOfCryptogramMaterialPackageAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: &::windows::core::HSTRING, materialname: &::windows::core::HSTRING, challenge: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramMaterialPossessionProof>>;
    fn RequestUnlockCryptogramMaterialForUseAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn DeleteCryptogramMaterialPackageAsync(&self, materialpackagename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Core", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramGenerator {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramGenerator";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Core", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardCryptogramGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramGeneratorVtbl {
        unsafe extern "system" fn SupportedCryptogramMaterialTypes<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCryptogramMaterialTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCryptogramAlgorithms<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCryptogramAlgorithms() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCryptogramMaterialPackageFormats<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCryptogramMaterialPackageFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCryptogramMaterialPackageConfirmationResponseFormats<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCryptogramMaterialPackageConfirmationResponseFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedSmartCardCryptogramStorageKeyCapabilities<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedSmartCardCryptogramStorageKeyCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCryptogramMaterialStorageKeyAsync<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagekeyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteCryptogramMaterialStorageKeyAsync(&*(&storagekeyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCryptogramMaterialStorageKeyAsync<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCryptogramMaterialStorageKeyAsync(promptingbehavior, &*(&storagekeyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), algorithm, capabilities) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCryptogramMaterialStorageKeyInfoAsync<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCryptogramMaterialStorageKeyInfoAsync(promptingbehavior, &*(&storagekeyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportCryptogramMaterialPackageAsync<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, materialpackagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, cryptogrammaterialpackage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportCryptogramMaterialPackageAsync(
                format,
                &*(&storagekeyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&materialpackagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&cryptogrammaterialpackage as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryProvePossessionOfCryptogramMaterialPackageAsync<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, materialname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, challenge: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryProvePossessionOfCryptogramMaterialPackageAsync(
                promptingbehavior,
                responseformat,
                &*(&materialpackagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&materialname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&challenge as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestUnlockCryptogramMaterialForUseAsync<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestUnlockCryptogramMaterialForUseAsync(promptingbehavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCryptogramMaterialPackageAsync<Impl: ISmartCardCryptogramGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, materialpackagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteCryptogramMaterialPackageAsync(&*(&materialpackagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramGenerator, BASE_OFFSET>(),
            SupportedCryptogramMaterialTypes: SupportedCryptogramMaterialTypes::<Impl, IMPL_OFFSET>,
            SupportedCryptogramAlgorithms: SupportedCryptogramAlgorithms::<Impl, IMPL_OFFSET>,
            SupportedCryptogramMaterialPackageFormats: SupportedCryptogramMaterialPackageFormats::<Impl, IMPL_OFFSET>,
            SupportedCryptogramMaterialPackageConfirmationResponseFormats: SupportedCryptogramMaterialPackageConfirmationResponseFormats::<Impl, IMPL_OFFSET>,
            SupportedSmartCardCryptogramStorageKeyCapabilities: SupportedSmartCardCryptogramStorageKeyCapabilities::<Impl, IMPL_OFFSET>,
            DeleteCryptogramMaterialStorageKeyAsync: DeleteCryptogramMaterialStorageKeyAsync::<Impl, IMPL_OFFSET>,
            CreateCryptogramMaterialStorageKeyAsync: CreateCryptogramMaterialStorageKeyAsync::<Impl, IMPL_OFFSET>,
            RequestCryptogramMaterialStorageKeyInfoAsync: RequestCryptogramMaterialStorageKeyInfoAsync::<Impl, IMPL_OFFSET>,
            ImportCryptogramMaterialPackageAsync: ImportCryptogramMaterialPackageAsync::<Impl, IMPL_OFFSET>,
            TryProvePossessionOfCryptogramMaterialPackageAsync: TryProvePossessionOfCryptogramMaterialPackageAsync::<Impl, IMPL_OFFSET>,
            RequestUnlockCryptogramMaterialForUseAsync: RequestUnlockCryptogramMaterialForUseAsync::<Impl, IMPL_OFFSET>,
            DeleteCryptogramMaterialPackageAsync: DeleteCryptogramMaterialPackageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramGenerator2Impl: Sized {
    fn ValidateRequestApduAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: &::core::option::Option<super::super::Storage::Streams::IBuffer>, cryptogramplacementsteps: &::core::option::Option<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn GetAllCryptogramStorageKeyCharacteristicsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult>>;
    fn GetAllCryptogramMaterialPackageCharacteristicsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>>;
    fn GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync(&self, storagekeyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>>;
    fn GetAllCryptogramMaterialCharacteristicsAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramGenerator2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramGenerator2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardCryptogramGenerator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramGenerator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramGenerator2Vtbl {
        unsafe extern "system" fn ValidateRequestApduAsync<Impl: ISmartCardCryptogramGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: ::windows::core::RawPtr, cryptogramplacementsteps: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateRequestApduAsync(
                promptingbehavior,
                &*(&apdutovalidate as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&cryptogramplacementsteps as *const <super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllCryptogramStorageKeyCharacteristicsAsync<Impl: ISmartCardCryptogramGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllCryptogramStorageKeyCharacteristicsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllCryptogramMaterialPackageCharacteristicsAsync<Impl: ISmartCardCryptogramGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllCryptogramMaterialPackageCharacteristicsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync<Impl: ISmartCardCryptogramGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagekeyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync(&*(&storagekeyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllCryptogramMaterialCharacteristicsAsync<Impl: ISmartCardCryptogramGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllCryptogramMaterialCharacteristicsAsync(promptingbehavior, &*(&materialpackagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramGenerator2, BASE_OFFSET>(),
            ValidateRequestApduAsync: ValidateRequestApduAsync::<Impl, IMPL_OFFSET>,
            GetAllCryptogramStorageKeyCharacteristicsAsync: GetAllCryptogramStorageKeyCharacteristicsAsync::<Impl, IMPL_OFFSET>,
            GetAllCryptogramMaterialPackageCharacteristicsAsync: GetAllCryptogramMaterialPackageCharacteristicsAsync::<Impl, IMPL_OFFSET>,
            GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync: GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync::<Impl, IMPL_OFFSET>,
            GetAllCryptogramMaterialCharacteristicsAsync: GetAllCryptogramMaterialCharacteristicsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramGenerator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramGeneratorStaticsImpl: Sized {
    fn GetSmartCardCryptogramGeneratorAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGenerator>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramGeneratorStatics {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramGeneratorStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardCryptogramGeneratorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramGeneratorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramGeneratorStaticsVtbl {
        unsafe extern "system" fn GetSmartCardCryptogramGeneratorAsync<Impl: ISmartCardCryptogramGeneratorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSmartCardCryptogramGeneratorAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramGeneratorStatics, BASE_OFFSET>(),
            GetSmartCardCryptogramGeneratorAsync: GetSmartCardCryptogramGeneratorAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramGeneratorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramGeneratorStatics2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardCryptogramGeneratorStatics2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramGeneratorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardCryptogramGeneratorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramGeneratorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramGeneratorStatics2Vtbl {
        unsafe extern "system" fn IsSupported<Impl: ISmartCardCryptogramGeneratorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramGeneratorStatics2, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramGeneratorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResultImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialCharacteristics>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResultVtbl {
        unsafe extern "system" fn OperationStatus<Impl: ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Impl: ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult, BASE_OFFSET>(),
            OperationStatus: OperationStatus::<Impl, IMPL_OFFSET>,
            Characteristics: Characteristics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResultImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageCharacteristics>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResultVtbl {
        unsafe extern "system" fn OperationStatus<Impl: ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Impl: ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult, BASE_OFFSET>(),
            OperationStatus: OperationStatus::<Impl, IMPL_OFFSET>,
            Characteristics: Characteristics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResultImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCharacteristics>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResultVtbl {
        unsafe extern "system" fn OperationStatus<Impl: ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Impl: ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult, BASE_OFFSET>(),
            OperationStatus: OperationStatus::<Impl, IMPL_OFFSET>,
            Characteristics: Characteristics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramMaterialCharacteristicsImpl: Sized {
    fn MaterialName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowedAlgorithms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>;
    fn AllowedProofOfPossessionAlgorithms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>>;
    fn AllowedValidations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>;
    fn MaterialType(&self) -> ::windows::core::Result<SmartCardCryptogramMaterialType>;
    fn ProtectionMethod(&self) -> ::windows::core::Result<SmartCardCryptogramMaterialProtectionMethod>;
    fn ProtectionVersion(&self) -> ::windows::core::Result<i32>;
    fn MaterialLength(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramMaterialCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramMaterialCharacteristics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISmartCardCryptogramMaterialCharacteristicsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramMaterialCharacteristicsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramMaterialCharacteristicsVtbl {
        unsafe extern "system" fn MaterialName<Impl: ISmartCardCryptogramMaterialCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowedAlgorithms<Impl: ISmartCardCryptogramMaterialCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedAlgorithms() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowedProofOfPossessionAlgorithms<Impl: ISmartCardCryptogramMaterialCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedProofOfPossessionAlgorithms() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowedValidations<Impl: ISmartCardCryptogramMaterialCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedValidations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaterialType<Impl: ISmartCardCryptogramMaterialCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramMaterialType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectionMethod<Impl: ISmartCardCryptogramMaterialCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramMaterialProtectionMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectionVersion<Impl: ISmartCardCryptogramMaterialCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaterialLength<Impl: ISmartCardCryptogramMaterialCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaterialLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramMaterialCharacteristics, BASE_OFFSET>(),
            MaterialName: MaterialName::<Impl, IMPL_OFFSET>,
            AllowedAlgorithms: AllowedAlgorithms::<Impl, IMPL_OFFSET>,
            AllowedProofOfPossessionAlgorithms: AllowedProofOfPossessionAlgorithms::<Impl, IMPL_OFFSET>,
            AllowedValidations: AllowedValidations::<Impl, IMPL_OFFSET>,
            MaterialType: MaterialType::<Impl, IMPL_OFFSET>,
            ProtectionMethod: ProtectionMethod::<Impl, IMPL_OFFSET>,
            ProtectionVersion: ProtectionVersion::<Impl, IMPL_OFFSET>,
            MaterialLength: MaterialLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramMaterialCharacteristics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramMaterialPackageCharacteristicsImpl: Sized {
    fn PackageName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageKeyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateImported(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PackageFormat(&self) -> ::windows::core::Result<SmartCardCryptogramMaterialPackageFormat>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramMaterialPackageCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramMaterialPackageCharacteristics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardCryptogramMaterialPackageCharacteristicsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramMaterialPackageCharacteristicsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramMaterialPackageCharacteristicsVtbl {
        unsafe extern "system" fn PackageName<Impl: ISmartCardCryptogramMaterialPackageCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageKeyName<Impl: ISmartCardCryptogramMaterialPackageCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageKeyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateImported<Impl: ISmartCardCryptogramMaterialPackageCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateImported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageFormat<Impl: ISmartCardCryptogramMaterialPackageCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramMaterialPackageFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramMaterialPackageCharacteristics, BASE_OFFSET>(),
            PackageName: PackageName::<Impl, IMPL_OFFSET>,
            StorageKeyName: StorageKeyName::<Impl, IMPL_OFFSET>,
            DateImported: DateImported::<Impl, IMPL_OFFSET>,
            PackageFormat: PackageFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramMaterialPackageCharacteristics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramMaterialPossessionProofImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn Proof(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramMaterialPossessionProof {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramMaterialPossessionProof";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardCryptogramMaterialPossessionProofVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramMaterialPossessionProofImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramMaterialPossessionProofVtbl {
        unsafe extern "system" fn OperationStatus<Impl: ISmartCardCryptogramMaterialPossessionProofImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Proof<Impl: ISmartCardCryptogramMaterialPossessionProofImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Proof() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramMaterialPossessionProof, BASE_OFFSET>(),
            OperationStatus: OperationStatus::<Impl, IMPL_OFFSET>,
            Proof: Proof::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramMaterialPossessionProof as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramPlacementStepImpl: Sized {
    fn Algorithm(&self) -> ::windows::core::Result<SmartCardCryptogramAlgorithm>;
    fn SetAlgorithm(&self, value: SmartCardCryptogramAlgorithm) -> ::windows::core::Result<()>;
    fn SourceData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetSourceData(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CryptogramMaterialPackageName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCryptogramMaterialPackageName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CryptogramMaterialName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCryptogramMaterialName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TemplateOffset(&self) -> ::windows::core::Result<i32>;
    fn SetTemplateOffset(&self, value: i32) -> ::windows::core::Result<()>;
    fn CryptogramOffset(&self) -> ::windows::core::Result<i32>;
    fn SetCryptogramOffset(&self, value: i32) -> ::windows::core::Result<()>;
    fn CryptogramLength(&self) -> ::windows::core::Result<i32>;
    fn SetCryptogramLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn CryptogramPlacementOptions(&self) -> ::windows::core::Result<SmartCardCryptogramPlacementOptions>;
    fn SetCryptogramPlacementOptions(&self, value: SmartCardCryptogramPlacementOptions) -> ::windows::core::Result<()>;
    fn ChainedOutputStep(&self) -> ::windows::core::Result<SmartCardCryptogramPlacementStep>;
    fn SetChainedOutputStep(&self, value: &::core::option::Option<SmartCardCryptogramPlacementStep>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramPlacementStep {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramPlacementStep";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardCryptogramPlacementStepVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramPlacementStepImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramPlacementStepVtbl {
        unsafe extern "system" fn Algorithm<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Algorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlgorithm<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmartCardCryptogramAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlgorithm(value).into()
        }
        unsafe extern "system" fn SourceData<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceData<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceData(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CryptogramMaterialPackageName<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CryptogramMaterialPackageName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCryptogramMaterialPackageName<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCryptogramMaterialPackageName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CryptogramMaterialName<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CryptogramMaterialName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCryptogramMaterialName<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCryptogramMaterialName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TemplateOffset<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemplateOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemplateOffset<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTemplateOffset(value).into()
        }
        unsafe extern "system" fn CryptogramOffset<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CryptogramOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCryptogramOffset<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCryptogramOffset(value).into()
        }
        unsafe extern "system" fn CryptogramLength<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CryptogramLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCryptogramLength<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCryptogramLength(value).into()
        }
        unsafe extern "system" fn CryptogramPlacementOptions<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramPlacementOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CryptogramPlacementOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCryptogramPlacementOptions<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmartCardCryptogramPlacementOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCryptogramPlacementOptions(value).into()
        }
        unsafe extern "system" fn ChainedOutputStep<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChainedOutputStep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChainedOutputStep<Impl: ISmartCardCryptogramPlacementStepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChainedOutputStep(&*(&value as *const <SmartCardCryptogramPlacementStep as ::windows::core::Abi>::Abi as *const <SmartCardCryptogramPlacementStep as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramPlacementStep, BASE_OFFSET>(),
            Algorithm: Algorithm::<Impl, IMPL_OFFSET>,
            SetAlgorithm: SetAlgorithm::<Impl, IMPL_OFFSET>,
            SourceData: SourceData::<Impl, IMPL_OFFSET>,
            SetSourceData: SetSourceData::<Impl, IMPL_OFFSET>,
            CryptogramMaterialPackageName: CryptogramMaterialPackageName::<Impl, IMPL_OFFSET>,
            SetCryptogramMaterialPackageName: SetCryptogramMaterialPackageName::<Impl, IMPL_OFFSET>,
            CryptogramMaterialName: CryptogramMaterialName::<Impl, IMPL_OFFSET>,
            SetCryptogramMaterialName: SetCryptogramMaterialName::<Impl, IMPL_OFFSET>,
            TemplateOffset: TemplateOffset::<Impl, IMPL_OFFSET>,
            SetTemplateOffset: SetTemplateOffset::<Impl, IMPL_OFFSET>,
            CryptogramOffset: CryptogramOffset::<Impl, IMPL_OFFSET>,
            SetCryptogramOffset: SetCryptogramOffset::<Impl, IMPL_OFFSET>,
            CryptogramLength: CryptogramLength::<Impl, IMPL_OFFSET>,
            SetCryptogramLength: SetCryptogramLength::<Impl, IMPL_OFFSET>,
            CryptogramPlacementOptions: CryptogramPlacementOptions::<Impl, IMPL_OFFSET>,
            SetCryptogramPlacementOptions: SetCryptogramPlacementOptions::<Impl, IMPL_OFFSET>,
            ChainedOutputStep: ChainedOutputStep::<Impl, IMPL_OFFSET>,
            SetChainedOutputStep: SetChainedOutputStep::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramPlacementStep as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramStorageKeyCharacteristicsImpl: Sized {
    fn StorageKeyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateCreated(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Algorithm(&self) -> ::windows::core::Result<SmartCardCryptogramStorageKeyAlgorithm>;
    fn Capabilities(&self) -> ::windows::core::Result<SmartCardCryptogramStorageKeyCapabilities>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramStorageKeyCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramStorageKeyCharacteristics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardCryptogramStorageKeyCharacteristicsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramStorageKeyCharacteristicsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramStorageKeyCharacteristicsVtbl {
        unsafe extern "system" fn StorageKeyName<Impl: ISmartCardCryptogramStorageKeyCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageKeyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateCreated<Impl: ISmartCardCryptogramStorageKeyCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateCreated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Algorithm<Impl: ISmartCardCryptogramStorageKeyCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramStorageKeyAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Algorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Impl: ISmartCardCryptogramStorageKeyCharacteristicsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramStorageKeyCharacteristics, BASE_OFFSET>(),
            StorageKeyName: StorageKeyName::<Impl, IMPL_OFFSET>,
            DateCreated: DateCreated::<Impl, IMPL_OFFSET>,
            Algorithm: Algorithm::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramStorageKeyCharacteristics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardCryptogramStorageKeyInfoImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn PublicKeyBlobType(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType>;
    fn PublicKey(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AttestationStatus(&self) -> ::windows::core::Result<SmartCardCryptographicKeyAttestationStatus>;
    fn Attestation(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AttestationCertificateChain(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Capabilities(&self) -> ::windows::core::Result<SmartCardCryptogramStorageKeyCapabilities>;
}
#[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardCryptogramStorageKeyInfo {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramStorageKeyInfo";
}
#[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardCryptogramStorageKeyInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramStorageKeyInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramStorageKeyInfoVtbl {
        unsafe extern "system" fn OperationStatus<Impl: ISmartCardCryptogramStorageKeyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramGeneratorOperationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicKeyBlobType<Impl: ISmartCardCryptogramStorageKeyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicKeyBlobType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicKey<Impl: ISmartCardCryptogramStorageKeyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttestationStatus<Impl: ISmartCardCryptogramStorageKeyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptographicKeyAttestationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttestationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attestation<Impl: ISmartCardCryptogramStorageKeyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attestation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttestationCertificateChain<Impl: ISmartCardCryptogramStorageKeyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttestationCertificateChain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Impl: ISmartCardCryptogramStorageKeyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardCryptogramStorageKeyCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramStorageKeyInfo, BASE_OFFSET>(),
            OperationStatus: OperationStatus::<Impl, IMPL_OFFSET>,
            PublicKeyBlobType: PublicKeyBlobType::<Impl, IMPL_OFFSET>,
            PublicKey: PublicKey::<Impl, IMPL_OFFSET>,
            AttestationStatus: AttestationStatus::<Impl, IMPL_OFFSET>,
            Attestation: Attestation::<Impl, IMPL_OFFSET>,
            AttestationCertificateChain: AttestationCertificateChain::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramStorageKeyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramStorageKeyInfo2Impl: Sized {
    fn OperationalRequirements(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardCryptogramStorageKeyInfo2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardCryptogramStorageKeyInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardCryptogramStorageKeyInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardCryptogramStorageKeyInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardCryptogramStorageKeyInfo2Vtbl {
        unsafe extern "system" fn OperationalRequirements<Impl: ISmartCardCryptogramStorageKeyInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationalRequirements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardCryptogramStorageKeyInfo2, BASE_OFFSET>(),
            OperationalRequirements: OperationalRequirements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardCryptogramStorageKeyInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorImpl: Sized {
    fn EnablementPolicy(&self) -> ::windows::core::Result<SmartCardEmulatorEnablementPolicy>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardEmulator {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulator";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardEmulatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulatorVtbl {
        unsafe extern "system" fn EnablementPolicy<Impl: ISmartCardEmulatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulatorEnablementPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnablementPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulator, BASE_OFFSET>(),
            EnablementPolicy: EnablementPolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardEmulator2Impl: Sized {
    fn ApduReceived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorApduReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveApduReceived(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionDeactivated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorConnectionDeactivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionDeactivated(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn IsHostCardEmulationSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardEmulator2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulator2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardEmulator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulator2Vtbl {
        unsafe extern "system" fn ApduReceived<Impl: ISmartCardEmulator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApduReceived(&*(&value as *const <super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorApduReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorApduReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveApduReceived<Impl: ISmartCardEmulator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveApduReceived(&*(&value as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectionDeactivated<Impl: ISmartCardEmulator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionDeactivated(&*(&value as *const <super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorConnectionDeactivatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorConnectionDeactivatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionDeactivated<Impl: ISmartCardEmulator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionDeactivated(&*(&value as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: ISmartCardEmulator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn IsHostCardEmulationSupported<Impl: ISmartCardEmulator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHostCardEmulationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulator2, BASE_OFFSET>(),
            ApduReceived: ApduReceived::<Impl, IMPL_OFFSET>,
            RemoveApduReceived: RemoveApduReceived::<Impl, IMPL_OFFSET>,
            ConnectionDeactivated: ConnectionDeactivated::<Impl, IMPL_OFFSET>,
            RemoveConnectionDeactivated: RemoveConnectionDeactivated::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            IsHostCardEmulationSupported: IsHostCardEmulationSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardEmulatorApduReceivedEventArgsImpl: Sized {
    fn CommandApdu(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ConnectionProperties(&self) -> ::windows::core::Result<SmartCardEmulatorConnectionProperties>;
    fn TryRespondAsync(&self, responseapdu: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn AutomaticResponseStatus(&self) -> ::windows::core::Result<SmartCardAutomaticResponseStatus>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardEmulatorApduReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulatorApduReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardEmulatorApduReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulatorApduReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulatorApduReceivedEventArgsVtbl {
        unsafe extern "system" fn CommandApdu<Impl: ISmartCardEmulatorApduReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandApdu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionProperties<Impl: ISmartCardEmulatorApduReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRespondAsync<Impl: ISmartCardEmulatorApduReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, responseapdu: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRespondAsync(&*(&responseapdu as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutomaticResponseStatus<Impl: ISmartCardEmulatorApduReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardAutomaticResponseStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomaticResponseStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulatorApduReceivedEventArgs, BASE_OFFSET>(),
            CommandApdu: CommandApdu::<Impl, IMPL_OFFSET>,
            ConnectionProperties: ConnectionProperties::<Impl, IMPL_OFFSET>,
            TryRespondAsync: TryRespondAsync::<Impl, IMPL_OFFSET>,
            AutomaticResponseStatus: AutomaticResponseStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulatorApduReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardEmulatorApduReceivedEventArgs2Impl: Sized {
    fn State(&self) -> ::windows::core::Result<u32>;
    fn TryRespondWithStateAsync(&self, responseapdu: &::core::option::Option<super::super::Storage::Streams::IBuffer>, nextstate: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardEmulatorApduReceivedEventArgs2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulatorApduReceivedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardEmulatorApduReceivedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulatorApduReceivedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulatorApduReceivedEventArgs2Vtbl {
        unsafe extern "system" fn State<Impl: ISmartCardEmulatorApduReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryRespondWithStateAsync<Impl: ISmartCardEmulatorApduReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, responseapdu: ::windows::core::RawPtr, nextstate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRespondWithStateAsync(&*(&responseapdu as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&nextstate as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulatorApduReceivedEventArgs2, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            TryRespondWithStateAsync: TryRespondWithStateAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulatorApduReceivedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardEmulatorApduReceivedEventArgsWithCryptogramsImpl: Sized {
    fn TryRespondWithCryptogramsAsync(&self, responsetemplate: &::core::option::Option<super::super::Storage::Streams::IBuffer>, cryptogramplacementsteps: &::core::option::Option<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn TryRespondWithCryptogramsAndStateAsync(&self, responsetemplate: &::core::option::Option<super::super::Storage::Streams::IBuffer>, cryptogramplacementsteps: &::core::option::Option<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>, nextstate: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulatorApduReceivedEventArgsWithCryptograms";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardEmulatorApduReceivedEventArgsWithCryptogramsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulatorApduReceivedEventArgsWithCryptogramsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulatorApduReceivedEventArgsWithCryptogramsVtbl {
        unsafe extern "system" fn TryRespondWithCryptogramsAsync<Impl: ISmartCardEmulatorApduReceivedEventArgsWithCryptogramsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, responsetemplate: ::windows::core::RawPtr, cryptogramplacementsteps: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRespondWithCryptogramsAsync(
                &*(&responsetemplate as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&cryptogramplacementsteps as *const <super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRespondWithCryptogramsAndStateAsync<Impl: ISmartCardEmulatorApduReceivedEventArgsWithCryptogramsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, responsetemplate: ::windows::core::RawPtr, cryptogramplacementsteps: ::windows::core::RawPtr, nextstate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRespondWithCryptogramsAndStateAsync(
                &*(&responsetemplate as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&cryptogramplacementsteps as *const <super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep> as ::windows::core::DefaultType>::DefaultType),
                &*(&nextstate as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulatorApduReceivedEventArgsWithCryptograms, BASE_OFFSET>(),
            TryRespondWithCryptogramsAsync: TryRespondWithCryptogramsAsync::<Impl, IMPL_OFFSET>,
            TryRespondWithCryptogramsAndStateAsync: TryRespondWithCryptogramsAndStateAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulatorApduReceivedEventArgsWithCryptograms as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorConnectionDeactivatedEventArgsImpl: Sized {
    fn ConnectionProperties(&self) -> ::windows::core::Result<SmartCardEmulatorConnectionProperties>;
    fn Reason(&self) -> ::windows::core::Result<SmartCardEmulatorConnectionDeactivatedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardEmulatorConnectionDeactivatedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulatorConnectionDeactivatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardEmulatorConnectionDeactivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulatorConnectionDeactivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulatorConnectionDeactivatedEventArgsVtbl {
        unsafe extern "system" fn ConnectionProperties<Impl: ISmartCardEmulatorConnectionDeactivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: ISmartCardEmulatorConnectionDeactivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulatorConnectionDeactivatedReason) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulatorConnectionDeactivatedEventArgs, BASE_OFFSET>(),
            ConnectionProperties: ConnectionProperties::<Impl, IMPL_OFFSET>,
            Reason: Reason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulatorConnectionDeactivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorConnectionPropertiesImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Source(&self) -> ::windows::core::Result<SmartCardEmulatorConnectionSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardEmulatorConnectionProperties {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulatorConnectionProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardEmulatorConnectionPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulatorConnectionPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulatorConnectionPropertiesVtbl {
        unsafe extern "system" fn Id<Impl: ISmartCardEmulatorConnectionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Source<Impl: ISmartCardEmulatorConnectionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardEmulatorConnectionSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulatorConnectionProperties, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulatorConnectionProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardEmulatorStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardEmulator>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardEmulatorStatics {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulatorStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardEmulatorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulatorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulatorStaticsVtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: ISmartCardEmulatorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulatorStatics, BASE_OFFSET>(),
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulatorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISmartCardEmulatorStatics2Impl: Sized {
    fn GetAppletIdGroupRegistrationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCardAppletIdGroupRegistration>>>;
    fn RegisterAppletIdGroupAsync(&self, appletidgroup: &::core::option::Option<SmartCardAppletIdGroup>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardAppletIdGroupRegistration>>;
    fn UnregisterAppletIdGroupAsync(&self, registration: &::core::option::Option<SmartCardAppletIdGroupRegistration>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MaxAppletIdGroupRegistrations(&self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardEmulatorStatics2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulatorStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISmartCardEmulatorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulatorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulatorStatics2Vtbl {
        unsafe extern "system" fn GetAppletIdGroupRegistrationsAsync<Impl: ISmartCardEmulatorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppletIdGroupRegistrationsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAppletIdGroupAsync<Impl: ISmartCardEmulatorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appletidgroup: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterAppletIdGroupAsync(&*(&appletidgroup as *const <SmartCardAppletIdGroup as ::windows::core::Abi>::Abi as *const <SmartCardAppletIdGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAppletIdGroupAsync<Impl: ISmartCardEmulatorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterAppletIdGroupAsync(&*(&registration as *const <SmartCardAppletIdGroupRegistration as ::windows::core::Abi>::Abi as *const <SmartCardAppletIdGroupRegistration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAppletIdGroupRegistrations<Impl: ISmartCardEmulatorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAppletIdGroupRegistrations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulatorStatics2, BASE_OFFSET>(),
            GetAppletIdGroupRegistrationsAsync: GetAppletIdGroupRegistrationsAsync::<Impl, IMPL_OFFSET>,
            RegisterAppletIdGroupAsync: RegisterAppletIdGroupAsync::<Impl, IMPL_OFFSET>,
            UnregisterAppletIdGroupAsync: UnregisterAppletIdGroupAsync::<Impl, IMPL_OFFSET>,
            MaxAppletIdGroupRegistrations: MaxAppletIdGroupRegistrations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulatorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorStatics3Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardEmulatorStatics3 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardEmulatorStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardEmulatorStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardEmulatorStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardEmulatorStatics3Vtbl {
        unsafe extern "system" fn IsSupported<Impl: ISmartCardEmulatorStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardEmulatorStatics3, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardEmulatorStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardPinPolicyImpl: Sized {
    fn MinLength(&self) -> ::windows::core::Result<u32>;
    fn SetMinLength(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxLength(&self) -> ::windows::core::Result<u32>;
    fn SetMaxLength(&self, value: u32) -> ::windows::core::Result<()>;
    fn UppercaseLetters(&self) -> ::windows::core::Result<SmartCardPinCharacterPolicyOption>;
    fn SetUppercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::Result<()>;
    fn LowercaseLetters(&self) -> ::windows::core::Result<SmartCardPinCharacterPolicyOption>;
    fn SetLowercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::Result<()>;
    fn Digits(&self) -> ::windows::core::Result<SmartCardPinCharacterPolicyOption>;
    fn SetDigits(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::Result<()>;
    fn SpecialCharacters(&self) -> ::windows::core::Result<SmartCardPinCharacterPolicyOption>;
    fn SetSpecialCharacters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardPinPolicy {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardPinPolicy";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardPinPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardPinPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardPinPolicyVtbl {
        unsafe extern "system" fn MinLength<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinLength<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinLength(value).into()
        }
        unsafe extern "system" fn MaxLength<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxLength<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxLength(value).into()
        }
        unsafe extern "system" fn UppercaseLetters<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UppercaseLetters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUppercaseLetters<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUppercaseLetters(value).into()
        }
        unsafe extern "system" fn LowercaseLetters<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowercaseLetters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowercaseLetters<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLowercaseLetters(value).into()
        }
        unsafe extern "system" fn Digits<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Digits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigits<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDigits(value).into()
        }
        unsafe extern "system" fn SpecialCharacters<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardPinCharacterPolicyOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpecialCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpecialCharacters<Impl: ISmartCardPinPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpecialCharacters(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardPinPolicy, BASE_OFFSET>(),
            MinLength: MinLength::<Impl, IMPL_OFFSET>,
            SetMinLength: SetMinLength::<Impl, IMPL_OFFSET>,
            MaxLength: MaxLength::<Impl, IMPL_OFFSET>,
            SetMaxLength: SetMaxLength::<Impl, IMPL_OFFSET>,
            UppercaseLetters: UppercaseLetters::<Impl, IMPL_OFFSET>,
            SetUppercaseLetters: SetUppercaseLetters::<Impl, IMPL_OFFSET>,
            LowercaseLetters: LowercaseLetters::<Impl, IMPL_OFFSET>,
            SetLowercaseLetters: SetLowercaseLetters::<Impl, IMPL_OFFSET>,
            Digits: Digits::<Impl, IMPL_OFFSET>,
            SetDigits: SetDigits::<Impl, IMPL_OFFSET>,
            SpecialCharacters: SpecialCharacters::<Impl, IMPL_OFFSET>,
            SetSpecialCharacters: SetSpecialCharacters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardPinPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardPinResetDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardPinResetDeferral {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardPinResetDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardPinResetDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardPinResetDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardPinResetDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: ISmartCardPinResetDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardPinResetDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardPinResetDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardPinResetRequestImpl: Sized {
    fn Challenge(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn GetDeferral(&self) -> ::windows::core::Result<SmartCardPinResetDeferral>;
    fn SetResponse(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardPinResetRequest {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardPinResetRequest";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardPinResetRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardPinResetRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardPinResetRequestVtbl {
        unsafe extern "system" fn Challenge<Impl: ISmartCardPinResetRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Challenge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Impl: ISmartCardPinResetRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: ISmartCardPinResetRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetResponse<Impl: ISmartCardPinResetRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResponse(&*(&response as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardPinResetRequest, BASE_OFFSET>(),
            Challenge: Challenge::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            SetResponse: SetResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardPinResetRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardProvisioningImpl: Sized {
    fn SmartCard(&self) -> ::windows::core::Result<SmartCard>;
    fn GetIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::GUID>>;
    fn GetNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetChallengeContextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardChallengeContext>>;
    fn RequestPinChangeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinResetAsync(&self, handler: &::core::option::Option<SmartCardPinResetHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardProvisioning {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardProvisioning";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardProvisioningVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardProvisioningImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardProvisioningVtbl {
        unsafe extern "system" fn SmartCard<Impl: ISmartCardProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartCard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdAsync<Impl: ISmartCardProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameAsync<Impl: ISmartCardProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChallengeContextAsync<Impl: ISmartCardProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChallengeContextAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinChangeAsync<Impl: ISmartCardProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPinChangeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinResetAsync<Impl: ISmartCardProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPinResetAsync(&*(&handler as *const <SmartCardPinResetHandler as ::windows::core::Abi>::Abi as *const <SmartCardPinResetHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardProvisioning, BASE_OFFSET>(),
            SmartCard: SmartCard::<Impl, IMPL_OFFSET>,
            GetIdAsync: GetIdAsync::<Impl, IMPL_OFFSET>,
            GetNameAsync: GetNameAsync::<Impl, IMPL_OFFSET>,
            GetChallengeContextAsync: GetChallengeContextAsync::<Impl, IMPL_OFFSET>,
            RequestPinChangeAsync: RequestPinChangeAsync::<Impl, IMPL_OFFSET>,
            RequestPinResetAsync: RequestPinResetAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardProvisioning as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardProvisioning2Impl: Sized {
    fn GetAuthorityKeyContainerNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardProvisioning2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardProvisioning2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardProvisioning2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardProvisioning2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardProvisioning2Vtbl {
        unsafe extern "system" fn GetAuthorityKeyContainerNameAsync<Impl: ISmartCardProvisioning2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthorityKeyContainerNameAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardProvisioning2, BASE_OFFSET>(),
            GetAuthorityKeyContainerNameAsync: GetAuthorityKeyContainerNameAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardProvisioning2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardProvisioningStaticsImpl: Sized {
    fn FromSmartCardAsync(&self, card: &::core::option::Option<SmartCard>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
    fn RequestVirtualSmartCardCreationAsync(&self, friendlyname: &::windows::core::HSTRING, administrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, pinpolicy: &::core::option::Option<SmartCardPinPolicy>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
    fn RequestVirtualSmartCardCreationAsyncWithCardId(&self, friendlyname: &::windows::core::HSTRING, administrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, pinpolicy: &::core::option::Option<SmartCardPinPolicy>, cardid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
    fn RequestVirtualSmartCardDeletionAsync(&self, card: &::core::option::Option<SmartCard>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardProvisioningStatics {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardProvisioningStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardProvisioningStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardProvisioningStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardProvisioningStaticsVtbl {
        unsafe extern "system" fn FromSmartCardAsync<Impl: ISmartCardProvisioningStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, card: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromSmartCardAsync(&*(&card as *const <SmartCard as ::windows::core::Abi>::Abi as *const <SmartCard as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestVirtualSmartCardCreationAsync<Impl: ISmartCardProvisioningStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, administrativekey: ::windows::core::RawPtr, pinpolicy: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestVirtualSmartCardCreationAsync(
                &*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&administrativekey as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&pinpolicy as *const <SmartCardPinPolicy as ::windows::core::Abi>::Abi as *const <SmartCardPinPolicy as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestVirtualSmartCardCreationAsyncWithCardId<Impl: ISmartCardProvisioningStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, administrativekey: ::windows::core::RawPtr, pinpolicy: ::windows::core::RawPtr, cardid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestVirtualSmartCardCreationAsyncWithCardId(
                &*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&administrativekey as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&pinpolicy as *const <SmartCardPinPolicy as ::windows::core::Abi>::Abi as *const <SmartCardPinPolicy as ::windows::core::DefaultType>::DefaultType),
                &*(&cardid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestVirtualSmartCardDeletionAsync<Impl: ISmartCardProvisioningStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, card: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestVirtualSmartCardDeletionAsync(&*(&card as *const <SmartCard as ::windows::core::Abi>::Abi as *const <SmartCard as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardProvisioningStatics, BASE_OFFSET>(),
            FromSmartCardAsync: FromSmartCardAsync::<Impl, IMPL_OFFSET>,
            RequestVirtualSmartCardCreationAsync: RequestVirtualSmartCardCreationAsync::<Impl, IMPL_OFFSET>,
            RequestVirtualSmartCardCreationAsyncWithCardId: RequestVirtualSmartCardCreationAsyncWithCardId::<Impl, IMPL_OFFSET>,
            RequestVirtualSmartCardDeletionAsync: RequestVirtualSmartCardDeletionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardProvisioningStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardProvisioningStatics2Impl: Sized {
    fn RequestAttestedVirtualSmartCardCreationAsync(&self, friendlyname: &::windows::core::HSTRING, administrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, pinpolicy: &::core::option::Option<SmartCardPinPolicy>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
    fn RequestAttestedVirtualSmartCardCreationAsyncWithCardId(&self, friendlyname: &::windows::core::HSTRING, administrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, pinpolicy: &::core::option::Option<SmartCardPinPolicy>, cardid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardProvisioningStatics2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardProvisioningStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardProvisioningStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardProvisioningStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardProvisioningStatics2Vtbl {
        unsafe extern "system" fn RequestAttestedVirtualSmartCardCreationAsync<Impl: ISmartCardProvisioningStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, administrativekey: ::windows::core::RawPtr, pinpolicy: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAttestedVirtualSmartCardCreationAsync(
                &*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&administrativekey as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&pinpolicy as *const <SmartCardPinPolicy as ::windows::core::Abi>::Abi as *const <SmartCardPinPolicy as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAttestedVirtualSmartCardCreationAsyncWithCardId<Impl: ISmartCardProvisioningStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, administrativekey: ::windows::core::RawPtr, pinpolicy: ::windows::core::RawPtr, cardid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAttestedVirtualSmartCardCreationAsyncWithCardId(
                &*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&administrativekey as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&pinpolicy as *const <SmartCardPinPolicy as ::windows::core::Abi>::Abi as *const <SmartCardPinPolicy as ::windows::core::DefaultType>::DefaultType),
                &*(&cardid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardProvisioningStatics2, BASE_OFFSET>(),
            RequestAttestedVirtualSmartCardCreationAsync: RequestAttestedVirtualSmartCardCreationAsync::<Impl, IMPL_OFFSET>,
            RequestAttestedVirtualSmartCardCreationAsyncWithCardId: RequestAttestedVirtualSmartCardCreationAsyncWithCardId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardProvisioningStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardReaderImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<SmartCardReaderKind>;
    fn GetStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardReaderStatus>>;
    fn FindAllCardsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCard>>>;
    fn CardAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmartCardReader, CardAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCardAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CardRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmartCardReader, CardRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCardRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardReader {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardReaderVtbl {
        unsafe extern "system" fn DeviceId<Impl: ISmartCardReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISmartCardReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: ISmartCardReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardReaderKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetStatusAsync<Impl: ISmartCardReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllCardsAsync<Impl: ISmartCardReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllCardsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardAdded<Impl: ISmartCardReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SmartCardReader, CardAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SmartCardReader, CardAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCardAdded<Impl: ISmartCardReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCardAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CardRemoved<Impl: ISmartCardReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SmartCardReader, CardRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SmartCardReader, CardRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCardRemoved<Impl: ISmartCardReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCardRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardReader, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            GetStatusAsync: GetStatusAsync::<Impl, IMPL_OFFSET>,
            FindAllCardsAsync: FindAllCardsAsync::<Impl, IMPL_OFFSET>,
            CardAdded: CardAdded::<Impl, IMPL_OFFSET>,
            RemoveCardAdded: RemoveCardAdded::<Impl, IMPL_OFFSET>,
            CardRemoved: CardRemoved::<Impl, IMPL_OFFSET>,
            RemoveCardRemoved: RemoveCardRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardReaderStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithKind(&self, kind: SmartCardReaderKind) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardReader>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardReaderStatics {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardReaderStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardReaderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardReaderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardReaderStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ISmartCardReaderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorWithKind<Impl: ISmartCardReaderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: SmartCardReaderKind, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorWithKind(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ISmartCardReaderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardReaderStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorWithKind: GetDeviceSelectorWithKind::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardReaderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISmartCardTriggerDetailsImpl: Sized {
    fn TriggerType(&self) -> ::windows::core::Result<SmartCardTriggerType>;
    fn SourceAppletId(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn TriggerData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardTriggerDetails {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardTriggerDetails";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISmartCardTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardTriggerDetailsVtbl {
        unsafe extern "system" fn TriggerType<Impl: ISmartCardTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmartCardTriggerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceAppletId<Impl: ISmartCardTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceAppletId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TriggerData<Impl: ISmartCardTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardTriggerDetails, BASE_OFFSET>(),
            TriggerType: TriggerType::<Impl, IMPL_OFFSET>,
            SourceAppletId: SourceAppletId::<Impl, IMPL_OFFSET>,
            TriggerData: TriggerData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardTriggerDetails2Impl: Sized {
    fn Emulator(&self) -> ::windows::core::Result<SmartCardEmulator>;
    fn TryLaunchCurrentAppAsync(&self, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryLaunchCurrentAppWithBehaviorAsync(&self, arguments: &::windows::core::HSTRING, behavior: SmartCardLaunchBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardTriggerDetails2 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardTriggerDetails2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISmartCardTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardTriggerDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardTriggerDetails2Vtbl {
        unsafe extern "system" fn Emulator<Impl: ISmartCardTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Emulator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryLaunchCurrentAppAsync<Impl: ISmartCardTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryLaunchCurrentAppAsync(&*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryLaunchCurrentAppWithBehaviorAsync<Impl: ISmartCardTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: SmartCardLaunchBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryLaunchCurrentAppWithBehaviorAsync(&*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), behavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardTriggerDetails2, BASE_OFFSET>(),
            Emulator: Emulator::<Impl, IMPL_OFFSET>,
            TryLaunchCurrentAppAsync: TryLaunchCurrentAppAsync::<Impl, IMPL_OFFSET>,
            TryLaunchCurrentAppWithBehaviorAsync: TryLaunchCurrentAppWithBehaviorAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardTriggerDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardTriggerDetails3Impl: Sized {
    fn SmartCard(&self) -> ::windows::core::Result<SmartCard>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardTriggerDetails3 {
    const NAME: &'static str = "Windows.Devices.SmartCards.ISmartCardTriggerDetails3";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardTriggerDetails3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardTriggerDetails3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardTriggerDetails3Vtbl {
        unsafe extern "system" fn SmartCard<Impl: ISmartCardTriggerDetails3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartCard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardTriggerDetails3, BASE_OFFSET>(), SmartCard: SmartCard::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardTriggerDetails3 as ::windows::core::Interface>::IID
    }
}
