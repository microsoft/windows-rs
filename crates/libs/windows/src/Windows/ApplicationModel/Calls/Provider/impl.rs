#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOrigin_Impl: Sized {
    fn Category(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCategory(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CategoryDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCategoryDescription(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Location(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocation(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOrigin {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOrigin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallOrigin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallOrigin_Vtbl {
        unsafe extern "system" fn Category<Impl: IPhoneCallOrigin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCategory<Impl: IPhoneCallOrigin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCategory(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CategoryDescription<Impl: IPhoneCallOrigin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CategoryDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategoryDescription<Impl: IPhoneCallOrigin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCategoryDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Location<Impl: IPhoneCallOrigin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IPhoneCallOrigin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallOrigin, BASE_OFFSET>(),
            Category: Category::<Impl, IMPL_OFFSET>,
            SetCategory: SetCategory::<Impl, IMPL_OFFSET>,
            CategoryDescription: CategoryDescription::<Impl, IMPL_OFFSET>,
            SetCategoryDescription: SetCategoryDescription::<Impl, IMPL_OFFSET>,
            Location: Location::<Impl, IMPL_OFFSET>,
            SetLocation: SetLocation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallOrigin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOrigin2_Impl: Sized + IPhoneCallOrigin_Impl {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOrigin2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOrigin2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallOrigin2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallOrigin2_Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IPhoneCallOrigin2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IPhoneCallOrigin2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallOrigin2, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallOrigin2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait IPhoneCallOrigin3_Impl: Sized + IPhoneCallOrigin_Impl + IPhoneCallOrigin2_Impl {
    fn DisplayPicture(&mut self) -> ::windows::core::Result<super::super::super::Storage::StorageFile>;
    fn SetDisplayPicture(&mut self, value: &::core::option::Option<super::super::super::Storage::StorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallOrigin3 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin3";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl IPhoneCallOrigin3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallOrigin3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallOrigin3_Vtbl {
        unsafe extern "system" fn DisplayPicture<Impl: IPhoneCallOrigin3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayPicture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPicture<Impl: IPhoneCallOrigin3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayPicture(&*(&value as *const <super::super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallOrigin3, BASE_OFFSET>(),
            DisplayPicture: DisplayPicture::<Impl, IMPL_OFFSET>,
            SetDisplayPicture: SetDisplayPicture::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallOrigin3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginManagerStatics_Impl: Sized {
    fn IsCurrentAppActiveCallOriginApp(&mut self) -> ::windows::core::Result<bool>;
    fn ShowPhoneCallOriginSettingsUI(&mut self) -> ::windows::core::Result<()>;
    fn SetCallOrigin(&mut self, requestid: &::windows::core::GUID, callorigin: &::core::option::Option<PhoneCallOrigin>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOriginManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOriginManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallOriginManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallOriginManagerStatics_Vtbl {
        unsafe extern "system" fn IsCurrentAppActiveCallOriginApp<Impl: IPhoneCallOriginManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentAppActiveCallOriginApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPhoneCallOriginSettingsUI<Impl: IPhoneCallOriginManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowPhoneCallOriginSettingsUI().into()
        }
        unsafe extern "system" fn SetCallOrigin<Impl: IPhoneCallOriginManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, callorigin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallOrigin(&*(&requestid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&callorigin as *const <PhoneCallOrigin as ::windows::core::Abi>::Abi as *const <PhoneCallOrigin as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallOriginManagerStatics, BASE_OFFSET>(),
            IsCurrentAppActiveCallOriginApp: IsCurrentAppActiveCallOriginApp::<Impl, IMPL_OFFSET>,
            ShowPhoneCallOriginSettingsUI: ShowPhoneCallOriginSettingsUI::<Impl, IMPL_OFFSET>,
            SetCallOrigin: SetCallOrigin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallOriginManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPhoneCallOriginManagerStatics2_Impl: Sized + IPhoneCallOriginManagerStatics_Impl {
    fn RequestSetAsActiveCallOriginAppAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneCallOriginManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPhoneCallOriginManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallOriginManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallOriginManagerStatics2_Vtbl {
        unsafe extern "system" fn RequestSetAsActiveCallOriginAppAsync<Impl: IPhoneCallOriginManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSetAsActiveCallOriginAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallOriginManagerStatics2, BASE_OFFSET>(),
            RequestSetAsActiveCallOriginAppAsync: RequestSetAsActiveCallOriginAppAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallOriginManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginManagerStatics3_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOriginManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOriginManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneCallOriginManagerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneCallOriginManagerStatics3_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IPhoneCallOriginManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneCallOriginManagerStatics3, BASE_OFFSET>(),
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneCallOriginManagerStatics3 as ::windows::core::Interface>::IID
    }
}
