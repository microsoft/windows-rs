#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IJumpListImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<JumpListItem>>;
    fn SystemGroupKind(&self) -> ::windows::core::Result<JumpListSystemGroupKind>;
    fn SetSystemGroupKind(&self, value: JumpListSystemGroupKind) -> ::windows::core::Result<()>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IJumpList {
    const NAME: &'static str = "Windows.UI.StartScreen.IJumpList";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IJumpListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListVtbl {
        unsafe extern "system" fn Items<Impl: IJumpListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemGroupKind<Impl: IJumpListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut JumpListSystemGroupKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemGroupKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemGroupKind<Impl: IJumpListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: JumpListSystemGroupKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemGroupKind(value).into()
        }
        unsafe extern "system" fn SaveAsync<Impl: IJumpListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJumpList, BASE_OFFSET>(),
            Items: Items::<Impl, IMPL_OFFSET>,
            SystemGroupKind: SystemGroupKind::<Impl, IMPL_OFFSET>,
            SetSystemGroupKind: SetSystemGroupKind::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJumpList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IJumpListItemImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<JumpListItemKind>;
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemovedByUser(&self) -> ::windows::core::Result<bool>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroupName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IJumpListItem {
    const NAME: &'static str = "Windows.UI.StartScreen.IJumpListItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IJumpListItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemVtbl {
        unsafe extern "system" fn Kind<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut JumpListItemKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Arguments<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovedByUser<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovedByUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GroupName<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupName<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Logo<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLogo<Impl: IJumpListItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJumpListItem, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            Arguments: Arguments::<Impl, IMPL_OFFSET>,
            RemovedByUser: RemovedByUser::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            GroupName: GroupName::<Impl, IMPL_OFFSET>,
            SetGroupName: SetGroupName::<Impl, IMPL_OFFSET>,
            Logo: Logo::<Impl, IMPL_OFFSET>,
            SetLogo: SetLogo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJumpListItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemStaticsImpl: Sized {
    fn CreateWithArguments(&self, arguments: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<JumpListItem>;
    fn CreateSeparator(&self) -> ::windows::core::Result<JumpListItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJumpListItemStatics {
    const NAME: &'static str = "Windows.UI.StartScreen.IJumpListItemStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJumpListItemStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemStaticsVtbl {
        unsafe extern "system" fn CreateWithArguments<Impl: IJumpListItemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithArguments(&*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSeparator<Impl: IJumpListItemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSeparator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJumpListItemStatics, BASE_OFFSET>(),
            CreateWithArguments: CreateWithArguments::<Impl, IMPL_OFFSET>,
            CreateSeparator: CreateSeparator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJumpListItemStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IJumpListStaticsImpl: Sized {
    fn LoadCurrentAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<JumpList>>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IJumpListStatics {
    const NAME: &'static str = "Windows.UI.StartScreen.IJumpListStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IJumpListStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListStaticsVtbl {
        unsafe extern "system" fn LoadCurrentAsync<Impl: IJumpListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadCurrentAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: IJumpListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJumpListStatics, BASE_OFFSET>(),
            LoadCurrentAsync: LoadCurrentAsync::<Impl, IMPL_OFFSET>,
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJumpListStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait ISecondaryTileImpl: Sized {
    fn SetTileId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetArguments(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetShortName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ShortName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSmallLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SmallLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetWideLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn WideLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetLockScreenBadgeLogo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn LockScreenBadgeLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetLockScreenDisplayBadgeAndTileText(&self, value: bool) -> ::windows::core::Result<()>;
    fn LockScreenDisplayBadgeAndTileText(&self) -> ::windows::core::Result<bool>;
    fn SetTileOptions(&self, value: TileOptions) -> ::windows::core::Result<()>;
    fn TileOptions(&self) -> ::windows::core::Result<TileOptions>;
    fn SetForegroundText(&self, value: ForegroundText) -> ::windows::core::Result<()>;
    fn ForegroundText(&self) -> ::windows::core::Result<ForegroundText>;
    fn SetBackgroundColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::Color>;
    fn RequestCreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestCreateAsyncWithPoint(&self, invocationpoint: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestCreateAsyncWithRect(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestCreateAsyncWithRectAndPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsyncWithPoint(&self, invocationpoint: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsyncWithRect(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsyncWithRectAndPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryTile {
    const NAME: &'static str = "Windows.UI.StartScreen.ISecondaryTile";
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ISecondaryTileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryTileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryTileVtbl {
        unsafe extern "system" fn SetTileId<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTileId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TileId<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArguments<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArguments(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Arguments<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShortName<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShortName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShortName<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShortName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLogo<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Logo<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSmallLogo<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmallLogo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SmallLogo<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmallLogo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWideLogo<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWideLogo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WideLogo<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WideLogo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLockScreenBadgeLogo<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLockScreenBadgeLogo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LockScreenBadgeLogo<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockScreenBadgeLogo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLockScreenDisplayBadgeAndTileText<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLockScreenDisplayBadgeAndTileText(value).into()
        }
        unsafe extern "system" fn LockScreenDisplayBadgeAndTileText<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockScreenDisplayBadgeAndTileText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTileOptions<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TileOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTileOptions(value).into()
        }
        unsafe extern "system" fn TileOptions<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TileOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TileOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundText<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ForegroundText) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundText(value).into()
        }
        unsafe extern "system" fn ForegroundText<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ForegroundText) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCreateAsync<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCreateAsyncWithPoint<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, invocationpoint: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCreateAsyncWithPoint(&*(&invocationpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCreateAsyncWithRect<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCreateAsyncWithRect(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCreateAsyncWithRectAndPlacement<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCreateAsyncWithRectAndPlacement(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDeleteAsync<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDeleteAsyncWithPoint<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, invocationpoint: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDeleteAsyncWithPoint(&*(&invocationpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDeleteAsyncWithRect<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDeleteAsyncWithRect(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDeleteAsyncWithRectAndPlacement<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDeleteAsyncWithRectAndPlacement(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAsync<Impl: ISecondaryTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryTile, BASE_OFFSET>(),
            SetTileId: SetTileId::<Impl, IMPL_OFFSET>,
            TileId: TileId::<Impl, IMPL_OFFSET>,
            SetArguments: SetArguments::<Impl, IMPL_OFFSET>,
            Arguments: Arguments::<Impl, IMPL_OFFSET>,
            SetShortName: SetShortName::<Impl, IMPL_OFFSET>,
            ShortName: ShortName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetLogo: SetLogo::<Impl, IMPL_OFFSET>,
            Logo: Logo::<Impl, IMPL_OFFSET>,
            SetSmallLogo: SetSmallLogo::<Impl, IMPL_OFFSET>,
            SmallLogo: SmallLogo::<Impl, IMPL_OFFSET>,
            SetWideLogo: SetWideLogo::<Impl, IMPL_OFFSET>,
            WideLogo: WideLogo::<Impl, IMPL_OFFSET>,
            SetLockScreenBadgeLogo: SetLockScreenBadgeLogo::<Impl, IMPL_OFFSET>,
            LockScreenBadgeLogo: LockScreenBadgeLogo::<Impl, IMPL_OFFSET>,
            SetLockScreenDisplayBadgeAndTileText: SetLockScreenDisplayBadgeAndTileText::<Impl, IMPL_OFFSET>,
            LockScreenDisplayBadgeAndTileText: LockScreenDisplayBadgeAndTileText::<Impl, IMPL_OFFSET>,
            SetTileOptions: SetTileOptions::<Impl, IMPL_OFFSET>,
            TileOptions: TileOptions::<Impl, IMPL_OFFSET>,
            SetForegroundText: SetForegroundText::<Impl, IMPL_OFFSET>,
            ForegroundText: ForegroundText::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            RequestCreateAsync: RequestCreateAsync::<Impl, IMPL_OFFSET>,
            RequestCreateAsyncWithPoint: RequestCreateAsyncWithPoint::<Impl, IMPL_OFFSET>,
            RequestCreateAsyncWithRect: RequestCreateAsyncWithRect::<Impl, IMPL_OFFSET>,
            RequestCreateAsyncWithRectAndPlacement: RequestCreateAsyncWithRectAndPlacement::<Impl, IMPL_OFFSET>,
            RequestDeleteAsync: RequestDeleteAsync::<Impl, IMPL_OFFSET>,
            RequestDeleteAsyncWithPoint: RequestDeleteAsyncWithPoint::<Impl, IMPL_OFFSET>,
            RequestDeleteAsyncWithRect: RequestDeleteAsyncWithRect::<Impl, IMPL_OFFSET>,
            RequestDeleteAsyncWithRectAndPlacement: RequestDeleteAsyncWithRectAndPlacement::<Impl, IMPL_OFFSET>,
            UpdateAsync: UpdateAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryTile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait ISecondaryTile2Impl: Sized + ISecondaryTileImpl {
    fn SetPhoneticName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PhoneticName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VisualElements(&self) -> ::windows::core::Result<SecondaryTileVisualElements>;
    fn SetRoamingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RoamingEnabled(&self) -> ::windows::core::Result<bool>;
    fn VisualElementsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SecondaryTile, VisualElementsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisualElementsRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryTile2 {
    const NAME: &'static str = "Windows.UI.StartScreen.ISecondaryTile2";
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ISecondaryTile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryTile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryTile2Vtbl {
        unsafe extern "system" fn SetPhoneticName<Impl: ISecondaryTile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhoneticName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PhoneticName<Impl: ISecondaryTile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneticName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisualElements<Impl: ISecondaryTile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisualElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoamingEnabled<Impl: ISecondaryTile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoamingEnabled(value).into()
        }
        unsafe extern "system" fn RoamingEnabled<Impl: ISecondaryTile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoamingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisualElementsRequested<Impl: ISecondaryTile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisualElementsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SecondaryTile, VisualElementsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SecondaryTile, VisualElementsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisualElementsRequested<Impl: ISecondaryTile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVisualElementsRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryTile2, BASE_OFFSET>(),
            SetPhoneticName: SetPhoneticName::<Impl, IMPL_OFFSET>,
            PhoneticName: PhoneticName::<Impl, IMPL_OFFSET>,
            VisualElements: VisualElements::<Impl, IMPL_OFFSET>,
            SetRoamingEnabled: SetRoamingEnabled::<Impl, IMPL_OFFSET>,
            RoamingEnabled: RoamingEnabled::<Impl, IMPL_OFFSET>,
            VisualElementsRequested: VisualElementsRequested::<Impl, IMPL_OFFSET>,
            RemoveVisualElementsRequested: RemoveVisualElementsRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryTile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISecondaryTileFactoryImpl: Sized {
    fn CreateTile(&self, tileid: &::windows::core::HSTRING, shortname: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, tileoptions: TileOptions, logoreference: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<SecondaryTile>;
    fn CreateWideTile(&self, tileid: &::windows::core::HSTRING, shortname: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, tileoptions: TileOptions, logoreference: &::core::option::Option<super::super::Foundation::Uri>, widelogoreference: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<SecondaryTile>;
    fn CreateWithId(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<SecondaryTile>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryTileFactory {
    const NAME: &'static str = "Windows.UI.StartScreen.ISecondaryTileFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISecondaryTileFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryTileFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryTileFactoryVtbl {
        unsafe extern "system" fn CreateTile<Impl: ISecondaryTileFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, shortname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tileoptions: TileOptions, logoreference: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTile(
                &*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&shortname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                tileoptions,
                &*(&logoreference as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWideTile<Impl: ISecondaryTileFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, shortname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tileoptions: TileOptions, logoreference: ::windows::core::RawPtr, widelogoreference: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWideTile(
                &*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&shortname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                tileoptions,
                &*(&logoreference as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&widelogoreference as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithId<Impl: ISecondaryTileFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithId(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryTileFactory, BASE_OFFSET>(),
            CreateTile: CreateTile::<Impl, IMPL_OFFSET>,
            CreateWideTile: CreateWideTile::<Impl, IMPL_OFFSET>,
            CreateWithId: CreateWithId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryTileFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISecondaryTileFactory2Impl: Sized + ISecondaryTileFactoryImpl {
    fn CreateMinimalTile(&self, tileid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, square150x150logo: &::core::option::Option<super::super::Foundation::Uri>, desiredsize: TileSize) -> ::windows::core::Result<SecondaryTile>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryTileFactory2 {
    const NAME: &'static str = "Windows.UI.StartScreen.ISecondaryTileFactory2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISecondaryTileFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryTileFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryTileFactory2Vtbl {
        unsafe extern "system" fn CreateMinimalTile<Impl: ISecondaryTileFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, square150x150logo: ::windows::core::RawPtr, desiredsize: TileSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMinimalTile(
                &*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&square150x150logo as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                desiredsize,
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryTileFactory2, BASE_OFFSET>(),
            CreateMinimalTile: CreateMinimalTile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryTileFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISecondaryTileStaticsImpl: Sized {
    fn Exists(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>;
    fn FindAllForApplicationAsync(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>;
    fn FindAllForPackageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryTileStatics {
    const NAME: &'static str = "Windows.UI.StartScreen.ISecondaryTileStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISecondaryTileStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryTileStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryTileStaticsVtbl {
        unsafe extern "system" fn Exists<Impl: ISecondaryTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exists(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: ISecondaryTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllForApplicationAsync<Impl: ISecondaryTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllForApplicationAsync(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllForPackageAsync<Impl: ISecondaryTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllForPackageAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryTileStatics, BASE_OFFSET>(),
            Exists: Exists::<Impl, IMPL_OFFSET>,
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FindAllForApplicationAsync: FindAllForApplicationAsync::<Impl, IMPL_OFFSET>,
            FindAllForPackageAsync: FindAllForPackageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryTileStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISecondaryTileVisualElementsImpl: Sized {
    fn SetSquare30x30Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSquare70x70Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square70x70Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSquare150x150Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square150x150Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetWide310x150Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Wide310x150Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSquare310x310Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square310x310Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetForegroundText(&self, value: ForegroundText) -> ::windows::core::Result<()>;
    fn ForegroundText(&self) -> ::windows::core::Result<ForegroundText>;
    fn SetBackgroundColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetShowNameOnSquare150x150Logo(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowNameOnSquare150x150Logo(&self) -> ::windows::core::Result<bool>;
    fn SetShowNameOnWide310x150Logo(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowNameOnWide310x150Logo(&self) -> ::windows::core::Result<bool>;
    fn SetShowNameOnSquare310x310Logo(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowNameOnSquare310x310Logo(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryTileVisualElements {
    const NAME: &'static str = "Windows.UI.StartScreen.ISecondaryTileVisualElements";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISecondaryTileVisualElementsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryTileVisualElementsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryTileVisualElementsVtbl {
        unsafe extern "system" fn SetSquare30x30Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSquare30x30Logo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Square30x30Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Square30x30Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSquare70x70Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSquare70x70Logo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Square70x70Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Square70x70Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSquare150x150Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSquare150x150Logo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Square150x150Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Square150x150Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWide310x150Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWide310x150Logo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Wide310x150Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wide310x150Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSquare310x310Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSquare310x310Logo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Square310x310Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Square310x310Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundText<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ForegroundText) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundText(value).into()
        }
        unsafe extern "system" fn ForegroundText<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ForegroundText) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowNameOnSquare150x150Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowNameOnSquare150x150Logo(value).into()
        }
        unsafe extern "system" fn ShowNameOnSquare150x150Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowNameOnSquare150x150Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowNameOnWide310x150Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowNameOnWide310x150Logo(value).into()
        }
        unsafe extern "system" fn ShowNameOnWide310x150Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowNameOnWide310x150Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowNameOnSquare310x310Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowNameOnSquare310x310Logo(value).into()
        }
        unsafe extern "system" fn ShowNameOnSquare310x310Logo<Impl: ISecondaryTileVisualElementsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowNameOnSquare310x310Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryTileVisualElements, BASE_OFFSET>(),
            SetSquare30x30Logo: SetSquare30x30Logo::<Impl, IMPL_OFFSET>,
            Square30x30Logo: Square30x30Logo::<Impl, IMPL_OFFSET>,
            SetSquare70x70Logo: SetSquare70x70Logo::<Impl, IMPL_OFFSET>,
            Square70x70Logo: Square70x70Logo::<Impl, IMPL_OFFSET>,
            SetSquare150x150Logo: SetSquare150x150Logo::<Impl, IMPL_OFFSET>,
            Square150x150Logo: Square150x150Logo::<Impl, IMPL_OFFSET>,
            SetWide310x150Logo: SetWide310x150Logo::<Impl, IMPL_OFFSET>,
            Wide310x150Logo: Wide310x150Logo::<Impl, IMPL_OFFSET>,
            SetSquare310x310Logo: SetSquare310x310Logo::<Impl, IMPL_OFFSET>,
            Square310x310Logo: Square310x310Logo::<Impl, IMPL_OFFSET>,
            SetForegroundText: SetForegroundText::<Impl, IMPL_OFFSET>,
            ForegroundText: ForegroundText::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetShowNameOnSquare150x150Logo: SetShowNameOnSquare150x150Logo::<Impl, IMPL_OFFSET>,
            ShowNameOnSquare150x150Logo: ShowNameOnSquare150x150Logo::<Impl, IMPL_OFFSET>,
            SetShowNameOnWide310x150Logo: SetShowNameOnWide310x150Logo::<Impl, IMPL_OFFSET>,
            ShowNameOnWide310x150Logo: ShowNameOnWide310x150Logo::<Impl, IMPL_OFFSET>,
            SetShowNameOnSquare310x310Logo: SetShowNameOnSquare310x310Logo::<Impl, IMPL_OFFSET>,
            ShowNameOnSquare310x310Logo: ShowNameOnSquare310x310Logo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryTileVisualElements as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISecondaryTileVisualElements2Impl: Sized {
    fn SetSquare71x71Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square71x71Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryTileVisualElements2 {
    const NAME: &'static str = "Windows.UI.StartScreen.ISecondaryTileVisualElements2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISecondaryTileVisualElements2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryTileVisualElements2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryTileVisualElements2Vtbl {
        unsafe extern "system" fn SetSquare71x71Logo<Impl: ISecondaryTileVisualElements2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSquare71x71Logo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Square71x71Logo<Impl: ISecondaryTileVisualElements2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Square71x71Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryTileVisualElements2, BASE_OFFSET>(),
            SetSquare71x71Logo: SetSquare71x71Logo::<Impl, IMPL_OFFSET>,
            Square71x71Logo: Square71x71Logo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryTileVisualElements2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISecondaryTileVisualElements3Impl: Sized {
    fn SetSquare44x44Logo(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Square44x44Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryTileVisualElements3 {
    const NAME: &'static str = "Windows.UI.StartScreen.ISecondaryTileVisualElements3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISecondaryTileVisualElements3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryTileVisualElements3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryTileVisualElements3Vtbl {
        unsafe extern "system" fn SetSquare44x44Logo<Impl: ISecondaryTileVisualElements3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSquare44x44Logo(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Square44x44Logo<Impl: ISecondaryTileVisualElements3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Square44x44Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryTileVisualElements3, BASE_OFFSET>(),
            SetSquare44x44Logo: SetSquare44x44Logo::<Impl, IMPL_OFFSET>,
            Square44x44Logo: Square44x44Logo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryTileVisualElements3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISecondaryTileVisualElements4Impl: Sized {
    fn MixedRealityModel(&self) -> ::windows::core::Result<TileMixedRealityModel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISecondaryTileVisualElements4 {
    const NAME: &'static str = "Windows.UI.StartScreen.ISecondaryTileVisualElements4";
}
#[cfg(feature = "implement_exclusive")]
impl ISecondaryTileVisualElements4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryTileVisualElements4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryTileVisualElements4Vtbl {
        unsafe extern "system" fn MixedRealityModel<Impl: ISecondaryTileVisualElements4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MixedRealityModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryTileVisualElements4, BASE_OFFSET>(),
            MixedRealityModel: MixedRealityModel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryTileVisualElements4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IStartScreenManagerImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn SupportsAppListEntry(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<bool>;
    fn ContainsAppListEntryAsync(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestAddAppListEntryAsync(&self, applistentry: &::core::option::Option<super::super::ApplicationModel::Core::AppListEntry>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStartScreenManager {
    const NAME: &'static str = "Windows.UI.StartScreen.IStartScreenManager";
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IStartScreenManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStartScreenManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStartScreenManagerVtbl {
        unsafe extern "system" fn User<Impl: IStartScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportsAppListEntry<Impl: IStartScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applistentry: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsAppListEntry(&*(&applistentry as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsAppListEntryAsync<Impl: IStartScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainsAppListEntryAsync(&*(&applistentry as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAddAppListEntryAsync<Impl: IStartScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applistentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAddAppListEntryAsync(&*(&applistentry as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Core::AppListEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStartScreenManager, BASE_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            SupportsAppListEntry: SupportsAppListEntry::<Impl, IMPL_OFFSET>,
            ContainsAppListEntryAsync: ContainsAppListEntryAsync::<Impl, IMPL_OFFSET>,
            RequestAddAppListEntryAsync: RequestAddAppListEntryAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStartScreenManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IStartScreenManager2Impl: Sized + IStartScreenManagerImpl {
    fn ContainsSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryRemoveSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStartScreenManager2 {
    const NAME: &'static str = "Windows.UI.StartScreen.IStartScreenManager2";
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IStartScreenManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStartScreenManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStartScreenManager2Vtbl {
        unsafe extern "system" fn ContainsSecondaryTileAsync<Impl: IStartScreenManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainsSecondaryTileAsync(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRemoveSecondaryTileAsync<Impl: IStartScreenManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRemoveSecondaryTileAsync(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStartScreenManager2, BASE_OFFSET>(),
            ContainsSecondaryTileAsync: ContainsSecondaryTileAsync::<Impl, IMPL_OFFSET>,
            TryRemoveSecondaryTileAsync: TryRemoveSecondaryTileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStartScreenManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IStartScreenManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<StartScreenManager>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<StartScreenManager>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStartScreenManagerStatics {
    const NAME: &'static str = "Windows.UI.StartScreen.IStartScreenManagerStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IStartScreenManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStartScreenManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStartScreenManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IStartScreenManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: IStartScreenManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStartScreenManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStartScreenManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait ITileMixedRealityModelImpl: Sized {
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetBoundingBox(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>) -> ::windows::core::Result<()>;
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITileMixedRealityModel {
    const NAME: &'static str = "Windows.UI.StartScreen.ITileMixedRealityModel";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ITileMixedRealityModelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileMixedRealityModelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileMixedRealityModelVtbl {
        unsafe extern "system" fn SetUri<Impl: ITileMixedRealityModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Uri<Impl: ITileMixedRealityModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBoundingBox<Impl: ITileMixedRealityModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoundingBox(&*(&value as *const <super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BoundingBox<Impl: ITileMixedRealityModelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileMixedRealityModel, BASE_OFFSET>(),
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            SetBoundingBox: SetBoundingBox::<Impl, IMPL_OFFSET>,
            BoundingBox: BoundingBox::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileMixedRealityModel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITileMixedRealityModel2Impl: Sized {
    fn SetActivationBehavior(&self, value: TileMixedRealityModelActivationBehavior) -> ::windows::core::Result<()>;
    fn ActivationBehavior(&self) -> ::windows::core::Result<TileMixedRealityModelActivationBehavior>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITileMixedRealityModel2 {
    const NAME: &'static str = "Windows.UI.StartScreen.ITileMixedRealityModel2";
}
#[cfg(feature = "implement_exclusive")]
impl ITileMixedRealityModel2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITileMixedRealityModel2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITileMixedRealityModel2Vtbl {
        unsafe extern "system" fn SetActivationBehavior<Impl: ITileMixedRealityModel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TileMixedRealityModelActivationBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivationBehavior(value).into()
        }
        unsafe extern "system" fn ActivationBehavior<Impl: ITileMixedRealityModel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TileMixedRealityModelActivationBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITileMixedRealityModel2, BASE_OFFSET>(),
            SetActivationBehavior: SetActivationBehavior::<Impl, IMPL_OFFSET>,
            ActivationBehavior: ActivationBehavior::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITileMixedRealityModel2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVisualElementsRequestImpl: Sized {
    fn VisualElements(&self) -> ::windows::core::Result<SecondaryTileVisualElements>;
    fn AlternateVisualElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SecondaryTileVisualElements>>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn GetDeferral(&self) -> ::windows::core::Result<VisualElementsRequestDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualElementsRequest {
    const NAME: &'static str = "Windows.UI.StartScreen.IVisualElementsRequest";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVisualElementsRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElementsRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualElementsRequestVtbl {
        unsafe extern "system" fn VisualElements<Impl: IVisualElementsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisualElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateVisualElements<Impl: IVisualElementsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateVisualElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Impl: IVisualElementsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IVisualElementsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElementsRequest, BASE_OFFSET>(),
            VisualElements: VisualElements::<Impl, IMPL_OFFSET>,
            AlternateVisualElements: AlternateVisualElements::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElementsRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualElementsRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualElementsRequestDeferral {
    const NAME: &'static str = "Windows.UI.StartScreen.IVisualElementsRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualElementsRequestDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElementsRequestDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualElementsRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IVisualElementsRequestDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElementsRequestDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElementsRequestDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualElementsRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<VisualElementsRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualElementsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.StartScreen.IVisualElementsRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualElementsRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualElementsRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualElementsRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IVisualElementsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualElementsRequestedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualElementsRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
