#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IGameListEntry_Impl: Sized {
    fn DisplayInfo(&mut self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppDisplayInfo>;
    fn LaunchAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn Category(&mut self) -> ::windows::core::Result<GameListCategory>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SetCategoryAsync(&mut self, value: GameListCategory) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IGameListEntry {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListEntry";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections"))]
impl IGameListEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameListEntry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameListEntry_Vtbl {
        unsafe extern "system" fn DisplayInfo<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchAsync<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameListCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCategoryAsync<Impl: IGameListEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GameListCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCategoryAsync(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameListEntry, BASE_OFFSET>(),
            DisplayInfo: DisplayInfo::<Impl, IMPL_OFFSET>,
            LaunchAsync: LaunchAsync::<Impl, IMPL_OFFSET>,
            Category: Category::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            SetCategoryAsync: SetCategoryAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameListEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IGameListEntry2_Impl: Sized + IGameListEntry_Impl {
    fn LaunchableState(&mut self) -> ::windows::core::Result<GameListEntryLaunchableState>;
    fn LauncherExecutable(&mut self) -> ::windows::core::Result<super::super::super::Storage::IStorageFile>;
    fn LaunchParameters(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLauncherExecutableFileAsync(&mut self, executablefile: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn SetLauncherExecutableFileWithParamsAsync(&mut self, executablefile: &::core::option::Option<super::super::super::Storage::IStorageFile>, launchparams: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn TitleId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitleIdAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn GameModeConfiguration(&mut self) -> ::windows::core::Result<GameModeConfiguration>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameListEntry2 {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListEntry2";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IGameListEntry2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameListEntry2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameListEntry2_Vtbl {
        unsafe extern "system" fn LaunchableState<Impl: IGameListEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameListEntryLaunchableState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchableState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LauncherExecutable<Impl: IGameListEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LauncherExecutable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchParameters<Impl: IGameListEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLauncherExecutableFileAsync<Impl: IGameListEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLauncherExecutableFileAsync(&*(&executablefile as *const <super::super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLauncherExecutableFileWithParamsAsync<Impl: IGameListEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablefile: ::windows::core::RawPtr, launchparams: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLauncherExecutableFileWithParamsAsync(&*(&executablefile as *const <super::super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&launchparams as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleId<Impl: IGameListEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TitleId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitleIdAsync<Impl: IGameListEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTitleIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GameModeConfiguration<Impl: IGameListEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GameModeConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameListEntry2, BASE_OFFSET>(),
            LaunchableState: LaunchableState::<Impl, IMPL_OFFSET>,
            LauncherExecutable: LauncherExecutable::<Impl, IMPL_OFFSET>,
            LaunchParameters: LaunchParameters::<Impl, IMPL_OFFSET>,
            SetLauncherExecutableFileAsync: SetLauncherExecutableFileAsync::<Impl, IMPL_OFFSET>,
            SetLauncherExecutableFileWithParamsAsync: SetLauncherExecutableFileWithParamsAsync::<Impl, IMPL_OFFSET>,
            TitleId: TitleId::<Impl, IMPL_OFFSET>,
            SetTitleIdAsync: SetTitleIdAsync::<Impl, IMPL_OFFSET>,
            GameModeConfiguration: GameModeConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameListEntry2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGameListStatics_Impl: Sized {
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>;
    fn FindAllAsyncPackageFamilyName(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>;
    fn GameAdded(&mut self, handler: &::core::option::Option<GameListChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameAdded(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GameRemoved(&mut self, handler: &::core::option::Option<GameListRemovedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameRemoved(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GameUpdated(&mut self, handler: &::core::option::Option<GameListChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameUpdated(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameListStatics {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGameListStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameListStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameListStatics_Vtbl {
        unsafe extern "system" fn FindAllAsync<Impl: IGameListStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAllAsyncPackageFamilyName<Impl: IGameListStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncPackageFamilyName(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GameAdded<Impl: IGameListStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GameAdded(&*(&handler as *const <GameListChangedEventHandler as ::windows::core::Abi>::Abi as *const <GameListChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGameAdded<Impl: IGameListStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGameAdded(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GameRemoved<Impl: IGameListStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GameRemoved(&*(&handler as *const <GameListRemovedEventHandler as ::windows::core::Abi>::Abi as *const <GameListRemovedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGameRemoved<Impl: IGameListStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGameRemoved(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GameUpdated<Impl: IGameListStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GameUpdated(&*(&handler as *const <GameListChangedEventHandler as ::windows::core::Abi>::Abi as *const <GameListChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGameUpdated<Impl: IGameListStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGameUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameListStatics, BASE_OFFSET>(),
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FindAllAsyncPackageFamilyName: FindAllAsyncPackageFamilyName::<Impl, IMPL_OFFSET>,
            GameAdded: GameAdded::<Impl, IMPL_OFFSET>,
            RemoveGameAdded: RemoveGameAdded::<Impl, IMPL_OFFSET>,
            GameRemoved: GameRemoved::<Impl, IMPL_OFFSET>,
            RemoveGameRemoved: RemoveGameRemoved::<Impl, IMPL_OFFSET>,
            GameUpdated: GameUpdated::<Impl, IMPL_OFFSET>,
            RemoveGameUpdated: RemoveGameUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameListStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGameListStatics2_Impl: Sized {
    fn MergeEntriesAsync(&mut self, left: &::core::option::Option<GameListEntry>, right: &::core::option::Option<GameListEntry>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameListEntry>>;
    fn UnmergeEntryAsync(&mut self, mergedentry: &::core::option::Option<GameListEntry>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameListStatics2 {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGameListStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameListStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameListStatics2_Vtbl {
        unsafe extern "system" fn MergeEntriesAsync<Impl: IGameListStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: ::windows::core::RawPtr, right: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MergeEntriesAsync(&*(&left as *const <GameListEntry as ::windows::core::Abi>::Abi as *const <GameListEntry as ::windows::core::DefaultType>::DefaultType), &*(&right as *const <GameListEntry as ::windows::core::Abi>::Abi as *const <GameListEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmergeEntryAsync<Impl: IGameListStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mergedentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnmergeEntryAsync(&*(&mergedentry as *const <GameListEntry as ::windows::core::Abi>::Abi as *const <GameListEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameListStatics2, BASE_OFFSET>(),
            MergeEntriesAsync: MergeEntriesAsync::<Impl, IMPL_OFFSET>,
            UnmergeEntryAsync: UnmergeEntryAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameListStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGameModeConfiguration_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RelatedProcessNames(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PercentGpuTimeAllocatedToGame(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetPercentGpuTimeAllocatedToGame(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn PercentGpuMemoryAllocatedToGame(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetPercentGpuMemoryAllocatedToGame(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn PercentGpuMemoryAllocatedToSystemCompositor(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetPercentGpuMemoryAllocatedToSystemCompositor(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxCpuCount(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetMaxCpuCount(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn CpuExclusivityMaskLow(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetCpuExclusivityMaskLow(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn CpuExclusivityMaskHigh(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetCpuExclusivityMaskHigh(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn AffinitizeToExclusiveCpus(&mut self) -> ::windows::core::Result<bool>;
    fn SetAffinitizeToExclusiveCpus(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SaveAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameModeConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameModeConfiguration";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGameModeConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameModeConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameModeConfiguration_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn RelatedProcessNames<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelatedProcessNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentGpuTimeAllocatedToGame<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentGpuTimeAllocatedToGame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPercentGpuTimeAllocatedToGame<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPercentGpuTimeAllocatedToGame(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PercentGpuMemoryAllocatedToGame<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentGpuMemoryAllocatedToGame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPercentGpuMemoryAllocatedToGame<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPercentGpuMemoryAllocatedToGame(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PercentGpuMemoryAllocatedToSystemCompositor<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentGpuMemoryAllocatedToSystemCompositor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPercentGpuMemoryAllocatedToSystemCompositor<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPercentGpuMemoryAllocatedToSystemCompositor(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxCpuCount<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxCpuCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCpuCount<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxCpuCount(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CpuExclusivityMaskLow<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CpuExclusivityMaskLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCpuExclusivityMaskLow<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCpuExclusivityMaskLow(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CpuExclusivityMaskHigh<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CpuExclusivityMaskHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCpuExclusivityMaskHigh<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCpuExclusivityMaskHigh(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AffinitizeToExclusiveCpus<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AffinitizeToExclusiveCpus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAffinitizeToExclusiveCpus<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAffinitizeToExclusiveCpus(value).into()
        }
        unsafe extern "system" fn SaveAsync<Impl: IGameModeConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameModeConfiguration, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            RelatedProcessNames: RelatedProcessNames::<Impl, IMPL_OFFSET>,
            PercentGpuTimeAllocatedToGame: PercentGpuTimeAllocatedToGame::<Impl, IMPL_OFFSET>,
            SetPercentGpuTimeAllocatedToGame: SetPercentGpuTimeAllocatedToGame::<Impl, IMPL_OFFSET>,
            PercentGpuMemoryAllocatedToGame: PercentGpuMemoryAllocatedToGame::<Impl, IMPL_OFFSET>,
            SetPercentGpuMemoryAllocatedToGame: SetPercentGpuMemoryAllocatedToGame::<Impl, IMPL_OFFSET>,
            PercentGpuMemoryAllocatedToSystemCompositor: PercentGpuMemoryAllocatedToSystemCompositor::<Impl, IMPL_OFFSET>,
            SetPercentGpuMemoryAllocatedToSystemCompositor: SetPercentGpuMemoryAllocatedToSystemCompositor::<Impl, IMPL_OFFSET>,
            MaxCpuCount: MaxCpuCount::<Impl, IMPL_OFFSET>,
            SetMaxCpuCount: SetMaxCpuCount::<Impl, IMPL_OFFSET>,
            CpuExclusivityMaskLow: CpuExclusivityMaskLow::<Impl, IMPL_OFFSET>,
            SetCpuExclusivityMaskLow: SetCpuExclusivityMaskLow::<Impl, IMPL_OFFSET>,
            CpuExclusivityMaskHigh: CpuExclusivityMaskHigh::<Impl, IMPL_OFFSET>,
            SetCpuExclusivityMaskHigh: SetCpuExclusivityMaskHigh::<Impl, IMPL_OFFSET>,
            AffinitizeToExclusiveCpus: AffinitizeToExclusiveCpus::<Impl, IMPL_OFFSET>,
            SetAffinitizeToExclusiveCpus: SetAffinitizeToExclusiveCpus::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameModeConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGameModeUserConfiguration_Impl: Sized {
    fn GamingRelatedProcessNames(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SaveAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGameModeUserConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfiguration";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGameModeUserConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameModeUserConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameModeUserConfiguration_Vtbl {
        unsafe extern "system" fn GamingRelatedProcessNames<Impl: IGameModeUserConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GamingRelatedProcessNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IGameModeUserConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameModeUserConfiguration, BASE_OFFSET>(),
            GamingRelatedProcessNames: GamingRelatedProcessNames::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameModeUserConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameModeUserConfigurationStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<GameModeUserConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameModeUserConfigurationStatics {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfigurationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameModeUserConfigurationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameModeUserConfigurationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameModeUserConfigurationStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IGameModeUserConfigurationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameModeUserConfigurationStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameModeUserConfigurationStatics as ::windows::core::Interface>::IID
    }
}
