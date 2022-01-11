#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDualSimTileImpl: Sized {
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsPinnedToStart(&self) -> ::windows::core::Result<bool>;
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDualSimTile {
    const NAME: &'static str = "Windows.Phone.StartScreen.IDualSimTile";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDualSimTileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDualSimTileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDualSimTileVtbl {
        unsafe extern "system" fn SetDisplayName<Impl: IDualSimTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IDualSimTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPinnedToStart<Impl: IDualSimTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinnedToStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsync<Impl: IDualSimTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAsync<Impl: IDualSimTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteAsync<Impl: IDualSimTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDualSimTile, BASE_OFFSET>(),
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            IsPinnedToStart: IsPinnedToStart::<Impl, IMPL_OFFSET>,
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
            UpdateAsync: UpdateAsync::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDualSimTile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Notifications", feature = "implement_exclusive"))]
pub trait IDualSimTileStaticsImpl: Sized {
    fn GetTileForSim2(&self) -> ::windows::core::Result<DualSimTile>;
    fn UpdateDisplayNameForSim1Async(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn CreateTileUpdaterForSim1(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileUpdater>;
    fn CreateTileUpdaterForSim2(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileUpdater>;
    fn CreateBadgeUpdaterForSim1(&self) -> ::windows::core::Result<super::super::UI::Notifications::BadgeUpdater>;
    fn CreateBadgeUpdaterForSim2(&self) -> ::windows::core::Result<super::super::UI::Notifications::BadgeUpdater>;
    fn CreateToastNotifierForSim1(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotifier>;
    fn CreateToastNotifierForSim2(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotifier>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Notifications", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDualSimTileStatics {
    const NAME: &'static str = "Windows.Phone.StartScreen.IDualSimTileStatics";
}
#[cfg(all(feature = "Foundation", feature = "UI_Notifications", feature = "implement_exclusive"))]
impl IDualSimTileStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDualSimTileStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDualSimTileStaticsVtbl {
        unsafe extern "system" fn GetTileForSim2<Impl: IDualSimTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTileForSim2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateDisplayNameForSim1Async<Impl: IDualSimTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateDisplayNameForSim1Async(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForSim1<Impl: IDualSimTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForSim1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTileUpdaterForSim2<Impl: IDualSimTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTileUpdaterForSim2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForSim1<Impl: IDualSimTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForSim1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBadgeUpdaterForSim2<Impl: IDualSimTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBadgeUpdaterForSim2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateToastNotifierForSim1<Impl: IDualSimTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifierForSim1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateToastNotifierForSim2<Impl: IDualSimTileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifierForSim2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDualSimTileStatics, BASE_OFFSET>(),
            GetTileForSim2: GetTileForSim2::<Impl, IMPL_OFFSET>,
            UpdateDisplayNameForSim1Async: UpdateDisplayNameForSim1Async::<Impl, IMPL_OFFSET>,
            CreateTileUpdaterForSim1: CreateTileUpdaterForSim1::<Impl, IMPL_OFFSET>,
            CreateTileUpdaterForSim2: CreateTileUpdaterForSim2::<Impl, IMPL_OFFSET>,
            CreateBadgeUpdaterForSim1: CreateBadgeUpdaterForSim1::<Impl, IMPL_OFFSET>,
            CreateBadgeUpdaterForSim2: CreateBadgeUpdaterForSim2::<Impl, IMPL_OFFSET>,
            CreateToastNotifierForSim1: CreateToastNotifierForSim1::<Impl, IMPL_OFFSET>,
            CreateToastNotifierForSim2: CreateToastNotifierForSim2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDualSimTileStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Notifications")]
pub trait IToastNotificationManagerStatics3Impl: Sized {
    fn CreateToastNotifierForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotifier>;
}
#[cfg(feature = "UI_Notifications")]
impl ::windows::core::RuntimeName for IToastNotificationManagerStatics3 {
    const NAME: &'static str = "Windows.Phone.StartScreen.IToastNotificationManagerStatics3";
}
#[cfg(feature = "UI_Notifications")]
impl IToastNotificationManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationManagerStatics3Vtbl {
        unsafe extern "system" fn CreateToastNotifierForSecondaryTile<Impl: IToastNotificationManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateToastNotifierForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationManagerStatics3, BASE_OFFSET>(),
            CreateToastNotifierForSecondaryTile: CreateToastNotifierForSecondaryTile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationManagerStatics3 as ::windows::core::Interface>::IID
    }
}
