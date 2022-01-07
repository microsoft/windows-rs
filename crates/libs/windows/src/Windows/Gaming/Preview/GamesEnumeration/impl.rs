pub trait IGameListEntryImpl: Sized {
    fn DisplayInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppDisplayInfo>;
    fn LaunchAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn Category(&self) -> ::windows::core::Result<GameListCategory>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SetCategoryAsync(&self, value: GameListCategory) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
impl ::windows::core::RuntimeName for IGameListEntry {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListEntry";
}
impl IGameListEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameListEntryImpl, const OFFSET: isize>() -> IGameListEntryVtbl {
        unsafe extern "system" fn DisplayInfo<Impl: IGameListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchAsync<Impl: IGameListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Category<Impl: IGameListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameListCategory) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IGameListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCategoryAsync<Impl: IGameListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GameListCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameListEntry>, ::windows::core::GetTrustLevel, DisplayInfo::<Impl, OFFSET>, LaunchAsync::<Impl, OFFSET>, Category::<Impl, OFFSET>, Properties::<Impl, OFFSET>, SetCategoryAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameListEntry2Impl: Sized + IGameListEntryImpl {
    fn LaunchableState(&self) -> ::windows::core::Result<GameListEntryLaunchableState>;
    fn LauncherExecutable(&self) -> ::windows::core::Result<super::super::super::Storage::IStorageFile>;
    fn LaunchParameters(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLauncherExecutableFileAsync(&self, executablefile: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn SetLauncherExecutableFileWithParamsAsync(&self, executablefile: &::core::option::Option<super::super::super::Storage::IStorageFile>, launchparams: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitleIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn GameModeConfiguration(&self) -> ::windows::core::Result<GameModeConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameListEntry2 {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListEntry2";
}
#[cfg(feature = "implement_exclusive")]
impl IGameListEntry2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameListEntry2Impl, const OFFSET: isize>() -> IGameListEntry2Vtbl {
        unsafe extern "system" fn LaunchableState<Impl: IGameListEntry2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameListEntryLaunchableState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LauncherExecutable<Impl: IGameListEntry2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchParameters<Impl: IGameListEntry2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLauncherExecutableFileAsync<Impl: IGameListEntry2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLauncherExecutableFileWithParamsAsync<Impl: IGameListEntry2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablefile: ::windows::core::RawPtr, launchparams: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TitleId<Impl: IGameListEntry2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitleIdAsync<Impl: IGameListEntry2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GameModeConfiguration<Impl: IGameListEntry2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGameListEntry2>,
            ::windows::core::GetTrustLevel,
            LaunchableState::<Impl, OFFSET>,
            LauncherExecutable::<Impl, OFFSET>,
            LaunchParameters::<Impl, OFFSET>,
            SetLauncherExecutableFileAsync::<Impl, OFFSET>,
            SetLauncherExecutableFileWithParamsAsync::<Impl, OFFSET>,
            TitleId::<Impl, OFFSET>,
            SetTitleIdAsync::<Impl, OFFSET>,
            GameModeConfiguration::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameListStaticsImpl: Sized {
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>;
    fn FindAllAsyncPackageFamilyName(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>;
    fn GameAdded(&self, handler: &::core::option::Option<GameListChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameAdded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GameRemoved(&self, handler: &::core::option::Option<GameListRemovedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameRemoved(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GameUpdated(&self, handler: &::core::option::Option<GameListChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameListStatics {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameListStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameListStaticsImpl, const OFFSET: isize>() -> IGameListStaticsVtbl {
        unsafe extern "system" fn FindAllAsync<Impl: IGameListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAllAsyncPackageFamilyName<Impl: IGameListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GameAdded<Impl: IGameListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveGameAdded<Impl: IGameListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGameAdded(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GameRemoved<Impl: IGameListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveGameRemoved<Impl: IGameListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGameRemoved(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GameUpdated<Impl: IGameListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveGameUpdated<Impl: IGameListStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGameUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGameListStatics>,
            ::windows::core::GetTrustLevel,
            FindAllAsync::<Impl, OFFSET>,
            FindAllAsyncPackageFamilyName::<Impl, OFFSET>,
            GameAdded::<Impl, OFFSET>,
            RemoveGameAdded::<Impl, OFFSET>,
            GameRemoved::<Impl, OFFSET>,
            RemoveGameRemoved::<Impl, OFFSET>,
            GameUpdated::<Impl, OFFSET>,
            RemoveGameUpdated::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameListStatics2Impl: Sized {
    fn MergeEntriesAsync(&self, left: &::core::option::Option<GameListEntry>, right: &::core::option::Option<GameListEntry>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameListEntry>>;
    fn UnmergeEntryAsync(&self, mergedentry: &::core::option::Option<GameListEntry>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameListStatics2 {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGameListStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameListStatics2Impl, const OFFSET: isize>() -> IGameListStatics2Vtbl {
        unsafe extern "system" fn MergeEntriesAsync<Impl: IGameListStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: ::windows::core::RawPtr, right: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnmergeEntryAsync<Impl: IGameListStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mergedentry: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameListStatics2>, ::windows::core::GetTrustLevel, MergeEntriesAsync::<Impl, OFFSET>, UnmergeEntryAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameModeConfigurationImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RelatedProcessNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PercentGpuTimeAllocatedToGame(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetPercentGpuTimeAllocatedToGame(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn PercentGpuMemoryAllocatedToGame(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetPercentGpuMemoryAllocatedToGame(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn PercentGpuMemoryAllocatedToSystemCompositor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetPercentGpuMemoryAllocatedToSystemCompositor(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn MaxCpuCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetMaxCpuCount(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn CpuExclusivityMaskLow(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetCpuExclusivityMaskLow(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn CpuExclusivityMaskHigh(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn SetCpuExclusivityMaskHigh(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn AffinitizeToExclusiveCpus(&self) -> ::windows::core::Result<bool>;
    fn SetAffinitizeToExclusiveCpus(&self, value: bool) -> ::windows::core::Result<()>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameModeConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameModeConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IGameModeConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameModeConfigurationImpl, const OFFSET: isize>() -> IGameModeConfigurationVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn RelatedProcessNames<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PercentGpuTimeAllocatedToGame<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPercentGpuTimeAllocatedToGame<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPercentGpuTimeAllocatedToGame(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PercentGpuMemoryAllocatedToGame<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPercentGpuMemoryAllocatedToGame<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPercentGpuMemoryAllocatedToGame(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PercentGpuMemoryAllocatedToSystemCompositor<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPercentGpuMemoryAllocatedToSystemCompositor<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPercentGpuMemoryAllocatedToSystemCompositor(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxCpuCount<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxCpuCount<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxCpuCount(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CpuExclusivityMaskLow<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCpuExclusivityMaskLow<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCpuExclusivityMaskLow(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CpuExclusivityMaskHigh<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCpuExclusivityMaskHigh<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCpuExclusivityMaskHigh(&*(&value as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AffinitizeToExclusiveCpus<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAffinitizeToExclusiveCpus<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAffinitizeToExclusiveCpus(value).into()
        }
        unsafe extern "system" fn SaveAsync<Impl: IGameModeConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGameModeConfiguration>,
            ::windows::core::GetTrustLevel,
            IsEnabled::<Impl, OFFSET>,
            SetIsEnabled::<Impl, OFFSET>,
            RelatedProcessNames::<Impl, OFFSET>,
            PercentGpuTimeAllocatedToGame::<Impl, OFFSET>,
            SetPercentGpuTimeAllocatedToGame::<Impl, OFFSET>,
            PercentGpuMemoryAllocatedToGame::<Impl, OFFSET>,
            SetPercentGpuMemoryAllocatedToGame::<Impl, OFFSET>,
            PercentGpuMemoryAllocatedToSystemCompositor::<Impl, OFFSET>,
            SetPercentGpuMemoryAllocatedToSystemCompositor::<Impl, OFFSET>,
            MaxCpuCount::<Impl, OFFSET>,
            SetMaxCpuCount::<Impl, OFFSET>,
            CpuExclusivityMaskLow::<Impl, OFFSET>,
            SetCpuExclusivityMaskLow::<Impl, OFFSET>,
            CpuExclusivityMaskHigh::<Impl, OFFSET>,
            SetCpuExclusivityMaskHigh::<Impl, OFFSET>,
            AffinitizeToExclusiveCpus::<Impl, OFFSET>,
            SetAffinitizeToExclusiveCpus::<Impl, OFFSET>,
            SaveAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameModeUserConfigurationImpl: Sized {
    fn GamingRelatedProcessNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameModeUserConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IGameModeUserConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameModeUserConfigurationImpl, const OFFSET: isize>() -> IGameModeUserConfigurationVtbl {
        unsafe extern "system" fn GamingRelatedProcessNames<Impl: IGameModeUserConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SaveAsync<Impl: IGameModeUserConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameModeUserConfiguration>, ::windows::core::GetTrustLevel, GamingRelatedProcessNames::<Impl, OFFSET>, SaveAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameModeUserConfigurationStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<GameModeUserConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameModeUserConfigurationStatics {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameModeUserConfigurationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameModeUserConfigurationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameModeUserConfigurationStaticsImpl, const OFFSET: isize>() -> IGameModeUserConfigurationStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IGameModeUserConfigurationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameModeUserConfigurationStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>)
    }
}
