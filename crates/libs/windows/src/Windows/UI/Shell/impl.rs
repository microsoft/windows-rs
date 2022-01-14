pub trait IAdaptiveCard_Impl: Sized {
    fn ToJson(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAdaptiveCard {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCard";
}
impl IAdaptiveCard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveCard_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveCard_Vtbl {
        unsafe extern "system" fn ToJson<Impl: IAdaptiveCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveCard, BASE_OFFSET>(), ToJson: ToJson::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveCard as ::windows::core::Interface>::IID
    }
}
pub trait IAdaptiveCardBuilderStatics_Impl: Sized {
    fn CreateAdaptiveCardFromJson(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IAdaptiveCard>;
}
impl ::windows::core::RuntimeName for IAdaptiveCardBuilderStatics {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCardBuilderStatics";
}
impl IAdaptiveCardBuilderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveCardBuilderStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveCardBuilderStatics_Vtbl {
        unsafe extern "system" fn CreateAdaptiveCardFromJson<Impl: IAdaptiveCardBuilderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAdaptiveCardFromJson(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveCardBuilderStatics, BASE_OFFSET>(),
            CreateAdaptiveCardFromJson: CreateAdaptiveCardFromJson::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveCardBuilderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISecurityAppManager_Impl: Sized {
    fn Register(&mut self, kind: SecurityAppKind, displayname: &::windows::core::HSTRING, detailsuri: &::core::option::Option<super::super::Foundation::Uri>, registerperuser: bool) -> ::windows::core::Result<::windows::core::GUID>;
    fn Unregister(&mut self, kind: SecurityAppKind, guidregistration: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UpdateState(&mut self, kind: SecurityAppKind, guidregistration: &::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecurityAppManager {
    const NAME: &'static str = "Windows.UI.Shell.ISecurityAppManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISecurityAppManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityAppManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityAppManager_Vtbl {
        unsafe extern "system" fn Register<Impl: ISecurityAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, detailsuri: ::windows::core::RawPtr, registerperuser: bool, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register(kind, &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&detailsuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), registerperuser) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unregister<Impl: ISecurityAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister(kind, &*(&guidregistration as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateState<Impl: ISecurityAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateState(kind, &*(&guidregistration as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), state, substatus, &*(&detailsuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecurityAppManager, BASE_OFFSET>(),
            Register: Register::<Impl, IMPL_OFFSET>,
            Unregister: Unregister::<Impl, IMPL_OFFSET>,
            UpdateState: UpdateState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityAppManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareWindowCommandEventArgs_Impl: Sized {
    fn WindowId(&mut self) -> ::windows::core::Result<super::WindowId>;
    fn Command(&mut self) -> ::windows::core::Result<ShareWindowCommand>;
    fn SetCommand(&mut self, value: ShareWindowCommand) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareWindowCommandEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.IShareWindowCommandEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IShareWindowCommandEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareWindowCommandEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareWindowCommandEventArgs_Vtbl {
        unsafe extern "system" fn WindowId<Impl: IShareWindowCommandEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::WindowId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Command<Impl: IShareWindowCommandEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ShareWindowCommand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Command() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommand<Impl: IShareWindowCommandEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ShareWindowCommand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommand(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShareWindowCommandEventArgs, BASE_OFFSET>(),
            WindowId: WindowId::<Impl, IMPL_OFFSET>,
            Command: Command::<Impl, IMPL_OFFSET>,
            SetCommand: SetCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareWindowCommandEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IShareWindowCommandSource_Impl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn ReportCommandChanged(&mut self) -> ::windows::core::Result<()>;
    fn CommandRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CommandInvoked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandInvoked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShareWindowCommandSource {
    const NAME: &'static str = "Windows.UI.Shell.IShareWindowCommandSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IShareWindowCommandSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareWindowCommandSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareWindowCommandSource_Vtbl {
        unsafe extern "system" fn Start<Impl: IShareWindowCommandSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IShareWindowCommandSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn ReportCommandChanged<Impl: IShareWindowCommandSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCommandChanged().into()
        }
        unsafe extern "system" fn CommandRequested<Impl: IShareWindowCommandSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCommandRequested<Impl: IShareWindowCommandSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCommandRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CommandInvoked<Impl: IShareWindowCommandSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandInvoked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCommandInvoked<Impl: IShareWindowCommandSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCommandInvoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShareWindowCommandSource, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            ReportCommandChanged: ReportCommandChanged::<Impl, IMPL_OFFSET>,
            CommandRequested: CommandRequested::<Impl, IMPL_OFFSET>,
            RemoveCommandRequested: RemoveCommandRequested::<Impl, IMPL_OFFSET>,
            CommandInvoked: CommandInvoked::<Impl, IMPL_OFFSET>,
            RemoveCommandInvoked: RemoveCommandInvoked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareWindowCommandSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareWindowCommandSourceStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<ShareWindowCommandSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareWindowCommandSourceStatics {
    const NAME: &'static str = "Windows.UI.Shell.IShareWindowCommandSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IShareWindowCommandSourceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareWindowCommandSourceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareWindowCommandSourceStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IShareWindowCommandSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShareWindowCommandSourceStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareWindowCommandSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITaskbarManager_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPinningAllowed(&mut self) -> ::windows::core::Result<bool>;
    fn IsCurrentAppPinnedAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn IsAppListEntryPinnedAsync(&mut self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinCurrentAppAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinAppListEntryAsync(&mut self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITaskbarManager {
    const NAME: &'static str = "Windows.UI.Shell.ITaskbarManager";
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "implement_exclusive"))]
impl ITaskbarManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskbarManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskbarManager_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: ITaskbarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPinningAllowed<Impl: ITaskbarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinningAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentAppPinnedAsync<Impl: ITaskbarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentAppPinnedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAppListEntryPinnedAsync<Impl: ITaskbarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAppListEntryPinnedAsync(&*(&applistentry as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinCurrentAppAsync<Impl: ITaskbarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPinCurrentAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinAppListEntryAsync<Impl: ITaskbarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPinAppListEntryAsync(&*(&applistentry as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITaskbarManager, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            IsPinningAllowed: IsPinningAllowed::<Impl, IMPL_OFFSET>,
            IsCurrentAppPinnedAsync: IsCurrentAppPinnedAsync::<Impl, IMPL_OFFSET>,
            IsAppListEntryPinnedAsync: IsAppListEntryPinnedAsync::<Impl, IMPL_OFFSET>,
            RequestPinCurrentAppAsync: RequestPinCurrentAppAsync::<Impl, IMPL_OFFSET>,
            RequestPinAppListEntryAsync: RequestPinAppListEntryAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskbarManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "UI_StartScreen", feature = "implement_exclusive"))]
pub trait ITaskbarManager2_Impl: Sized + ITaskbarManager_Impl {
    fn IsSecondaryTilePinnedAsync(&mut self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinSecondaryTileAsync(&mut self, secondarytile: &::core::option::Option<super::StartScreen::SecondaryTile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryUnpinSecondaryTileAsync(&mut self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "UI_StartScreen", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITaskbarManager2 {
    const NAME: &'static str = "Windows.UI.Shell.ITaskbarManager2";
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "UI_StartScreen", feature = "implement_exclusive"))]
impl ITaskbarManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskbarManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskbarManager2_Vtbl {
        unsafe extern "system" fn IsSecondaryTilePinnedAsync<Impl: ITaskbarManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecondaryTilePinnedAsync(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinSecondaryTileAsync<Impl: ITaskbarManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, secondarytile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPinSecondaryTileAsync(&*(&secondarytile as *const <super::StartScreen::SecondaryTile as ::windows::core::Abi>::Abi as *const <super::StartScreen::SecondaryTile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUnpinSecondaryTileAsync<Impl: ITaskbarManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUnpinSecondaryTileAsync(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITaskbarManager2, BASE_OFFSET>(),
            IsSecondaryTilePinnedAsync: IsSecondaryTilePinnedAsync::<Impl, IMPL_OFFSET>,
            RequestPinSecondaryTileAsync: RequestPinSecondaryTileAsync::<Impl, IMPL_OFFSET>,
            TryUnpinSecondaryTileAsync: TryUnpinSecondaryTileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskbarManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITaskbarManagerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<TaskbarManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITaskbarManagerStatics {
    const NAME: &'static str = "Windows.UI.Shell.ITaskbarManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITaskbarManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskbarManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskbarManagerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ITaskbarManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITaskbarManagerStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskbarManagerStatics as ::windows::core::Interface>::IID
    }
}
