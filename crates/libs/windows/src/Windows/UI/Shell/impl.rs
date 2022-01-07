pub trait IAdaptiveCardImpl: Sized {
    fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAdaptiveCard {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCard";
}
impl IAdaptiveCardVtbl {
    pub const fn new<Impl: IAdaptiveCardImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdaptiveCardVtbl {
        unsafe extern "system" fn ToJson<Impl: IAdaptiveCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ToJson() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdaptiveCard>, base.5, ToJson::<Impl, OFFSET>)
    }
}
pub trait IAdaptiveCardBuilderStaticsImpl: Sized {
    fn CreateAdaptiveCardFromJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IAdaptiveCard>;
}
impl ::windows::core::RuntimeName for IAdaptiveCardBuilderStatics {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCardBuilderStatics";
}
impl IAdaptiveCardBuilderStaticsVtbl {
    pub const fn new<Impl: IAdaptiveCardBuilderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdaptiveCardBuilderStaticsVtbl {
        unsafe extern "system" fn CreateAdaptiveCardFromJson<Impl: IAdaptiveCardBuilderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAdaptiveCardFromJson(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdaptiveCardBuilderStatics>, base.5, CreateAdaptiveCardFromJson::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecurityAppManagerImpl: Sized {
    fn Register(&self, kind: SecurityAppKind, displayname: &::windows::core::HSTRING, detailsuri: &::core::option::Option<super::super::Foundation::Uri>, registerperuser: bool) -> ::windows::core::Result<::windows::core::GUID>;
    fn Unregister(&self, kind: SecurityAppKind, guidregistration: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UpdateState(&self, kind: SecurityAppKind, guidregistration: &::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISecurityAppManager {
    const NAME: &'static str = "Windows.UI.Shell.ISecurityAppManager";
}
#[cfg(feature = "implement_exclusive")]
impl ISecurityAppManagerVtbl {
    pub const fn new<Impl: ISecurityAppManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISecurityAppManagerVtbl {
        unsafe extern "system" fn Register<Impl: ISecurityAppManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, detailsuri: ::windows::core::RawPtr, registerperuser: bool, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Register(kind, &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&detailsuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), registerperuser) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unregister<Impl: ISecurityAppManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Unregister(kind, &*(&guidregistration as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateState<Impl: ISecurityAppManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows::core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateState(kind, &*(&guidregistration as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), state, substatus, &*(&detailsuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISecurityAppManager>, base.5, Register::<Impl, OFFSET>, Unregister::<Impl, OFFSET>, UpdateState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareWindowCommandEventArgsImpl: Sized {
    fn WindowId(&self) -> ::windows::core::Result<super::WindowId>;
    fn Command(&self) -> ::windows::core::Result<ShareWindowCommand>;
    fn SetCommand(&self, value: ShareWindowCommand) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareWindowCommandEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.IShareWindowCommandEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IShareWindowCommandEventArgsVtbl {
    pub const fn new<Impl: IShareWindowCommandEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShareWindowCommandEventArgsVtbl {
        unsafe extern "system" fn WindowId<Impl: IShareWindowCommandEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::WindowId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WindowId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Command<Impl: IShareWindowCommandEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ShareWindowCommand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Command() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommand<Impl: IShareWindowCommandEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ShareWindowCommand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCommand(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShareWindowCommandEventArgs>, base.5, WindowId::<Impl, OFFSET>, Command::<Impl, OFFSET>, SetCommand::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareWindowCommandSourceImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn ReportCommandChanged(&self) -> ::windows::core::Result<()>;
    fn CommandRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CommandInvoked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandInvoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareWindowCommandSource {
    const NAME: &'static str = "Windows.UI.Shell.IShareWindowCommandSource";
}
#[cfg(feature = "implement_exclusive")]
impl IShareWindowCommandSourceVtbl {
    pub const fn new<Impl: IShareWindowCommandSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShareWindowCommandSourceVtbl {
        unsafe extern "system" fn Start<Impl: IShareWindowCommandSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IShareWindowCommandSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn ReportCommandChanged<Impl: IShareWindowCommandSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCommandChanged().into()
        }
        unsafe extern "system" fn CommandRequested<Impl: IShareWindowCommandSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommandRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCommandRequested<Impl: IShareWindowCommandSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCommandRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CommandInvoked<Impl: IShareWindowCommandSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommandInvoked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCommandInvoked<Impl: IShareWindowCommandSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCommandInvoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShareWindowCommandSource>, base.5, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, ReportCommandChanged::<Impl, OFFSET>, CommandRequested::<Impl, OFFSET>, RemoveCommandRequested::<Impl, OFFSET>, CommandInvoked::<Impl, OFFSET>, RemoveCommandInvoked::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareWindowCommandSourceStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ShareWindowCommandSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShareWindowCommandSourceStatics {
    const NAME: &'static str = "Windows.UI.Shell.IShareWindowCommandSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IShareWindowCommandSourceStaticsVtbl {
    pub const fn new<Impl: IShareWindowCommandSourceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShareWindowCommandSourceStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IShareWindowCommandSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShareWindowCommandSourceStatics>, base.5, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITaskbarManagerImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPinningAllowed(&self) -> ::windows::core::Result<bool>;
    fn IsCurrentAppPinnedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn IsAppListEntryPinnedAsync(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinCurrentAppAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinAppListEntryAsync(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITaskbarManager {
    const NAME: &'static str = "Windows.UI.Shell.ITaskbarManager";
}
#[cfg(feature = "implement_exclusive")]
impl ITaskbarManagerVtbl {
    pub const fn new<Impl: ITaskbarManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITaskbarManagerVtbl {
        unsafe extern "system" fn IsSupported<Impl: ITaskbarManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinningAllowed<Impl: ITaskbarManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPinningAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentAppPinnedAsync<Impl: ITaskbarManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCurrentAppPinnedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAppListEntryPinnedAsync<Impl: ITaskbarManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAppListEntryPinnedAsync(&*(&applistentry as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinCurrentAppAsync<Impl: ITaskbarManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestPinCurrentAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinAppListEntryAsync<Impl: ITaskbarManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestPinAppListEntryAsync(&*(&applistentry as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITaskbarManager>, base.5, IsSupported::<Impl, OFFSET>, IsPinningAllowed::<Impl, OFFSET>, IsCurrentAppPinnedAsync::<Impl, OFFSET>, IsAppListEntryPinnedAsync::<Impl, OFFSET>, RequestPinCurrentAppAsync::<Impl, OFFSET>, RequestPinAppListEntryAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITaskbarManager2Impl: Sized + ITaskbarManagerImpl {
    fn IsSecondaryTilePinnedAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinSecondaryTileAsync(&self, secondarytile: &::core::option::Option<super::StartScreen::SecondaryTile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryUnpinSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITaskbarManager2 {
    const NAME: &'static str = "Windows.UI.Shell.ITaskbarManager2";
}
#[cfg(feature = "implement_exclusive")]
impl ITaskbarManager2Vtbl {
    pub const fn new<Impl: ITaskbarManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITaskbarManager2Vtbl {
        unsafe extern "system" fn IsSecondaryTilePinnedAsync<Impl: ITaskbarManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSecondaryTilePinnedAsync(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPinSecondaryTileAsync<Impl: ITaskbarManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, secondarytile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestPinSecondaryTileAsync(&*(&secondarytile as *const <super::StartScreen::SecondaryTile as ::windows::core::Abi>::Abi as *const <super::StartScreen::SecondaryTile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUnpinSecondaryTileAsync<Impl: ITaskbarManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryUnpinSecondaryTileAsync(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITaskbarManager2>, base.5, IsSecondaryTilePinnedAsync::<Impl, OFFSET>, RequestPinSecondaryTileAsync::<Impl, OFFSET>, TryUnpinSecondaryTileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITaskbarManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<TaskbarManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITaskbarManagerStatics {
    const NAME: &'static str = "Windows.UI.Shell.ITaskbarManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITaskbarManagerStaticsVtbl {
    pub const fn new<Impl: ITaskbarManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITaskbarManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ITaskbarManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITaskbarManagerStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
