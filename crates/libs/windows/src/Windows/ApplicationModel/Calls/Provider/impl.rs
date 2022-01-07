#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginImpl: Sized {
    fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCategory(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CategoryDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCategoryDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOrigin {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOriginVtbl {
    pub const fn new<Impl: IPhoneCallOriginImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhoneCallOriginVtbl {
        unsafe extern "system" fn Category<Impl: IPhoneCallOriginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Impl: IPhoneCallOriginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCategory(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CategoryDescription<Impl: IPhoneCallOriginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CategoryDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategoryDescription<Impl: IPhoneCallOriginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCategoryDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Location<Impl: IPhoneCallOriginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Impl: IPhoneCallOriginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocation(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhoneCallOrigin>, base.5, Category::<Impl, OFFSET>, SetCategory::<Impl, OFFSET>, CategoryDescription::<Impl, OFFSET>, SetCategoryDescription::<Impl, OFFSET>, Location::<Impl, OFFSET>, SetLocation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOrigin2Impl: Sized + IPhoneCallOriginImpl {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOrigin2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOrigin2Vtbl {
    pub const fn new<Impl: IPhoneCallOrigin2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhoneCallOrigin2Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IPhoneCallOrigin2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IPhoneCallOrigin2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhoneCallOrigin2>, base.5, DisplayName::<Impl, OFFSET>, SetDisplayName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOrigin3Impl: Sized + IPhoneCallOriginImpl + IPhoneCallOrigin2Impl {
    fn DisplayPicture(&self) -> ::windows::core::Result<super::super::super::Storage::StorageFile>;
    fn SetDisplayPicture(&self, value: &::core::option::Option<super::super::super::Storage::StorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOrigin3 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin3";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOrigin3Vtbl {
    pub const fn new<Impl: IPhoneCallOrigin3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhoneCallOrigin3Vtbl {
        unsafe extern "system" fn DisplayPicture<Impl: IPhoneCallOrigin3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayPicture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPicture<Impl: IPhoneCallOrigin3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayPicture(&*(&value as *const <super::super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhoneCallOrigin3>, base.5, DisplayPicture::<Impl, OFFSET>, SetDisplayPicture::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginManagerStaticsImpl: Sized {
    fn IsCurrentAppActiveCallOriginApp(&self) -> ::windows::core::Result<bool>;
    fn ShowPhoneCallOriginSettingsUI(&self) -> ::windows::core::Result<()>;
    fn SetCallOrigin(&self, requestid: &::windows::core::GUID, callorigin: &::core::option::Option<PhoneCallOrigin>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOriginManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOriginManagerStaticsVtbl {
    pub const fn new<Impl: IPhoneCallOriginManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhoneCallOriginManagerStaticsVtbl {
        unsafe extern "system" fn IsCurrentAppActiveCallOriginApp<Impl: IPhoneCallOriginManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCurrentAppActiveCallOriginApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPhoneCallOriginSettingsUI<Impl: IPhoneCallOriginManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowPhoneCallOriginSettingsUI().into()
        }
        unsafe extern "system" fn SetCallOrigin<Impl: IPhoneCallOriginManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, callorigin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCallOrigin(&*(&requestid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&callorigin as *const <PhoneCallOrigin as ::windows::core::Abi>::Abi as *const <PhoneCallOrigin as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhoneCallOriginManagerStatics>, base.5, IsCurrentAppActiveCallOriginApp::<Impl, OFFSET>, ShowPhoneCallOriginSettingsUI::<Impl, OFFSET>, SetCallOrigin::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginManagerStatics2Impl: Sized + IPhoneCallOriginManagerStaticsImpl {
    fn RequestSetAsActiveCallOriginAppAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOriginManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOriginManagerStatics2Vtbl {
    pub const fn new<Impl: IPhoneCallOriginManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhoneCallOriginManagerStatics2Vtbl {
        unsafe extern "system" fn RequestSetAsActiveCallOriginAppAsync<Impl: IPhoneCallOriginManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestSetAsActiveCallOriginAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhoneCallOriginManagerStatics2>, base.5, RequestSetAsActiveCallOriginAppAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginManagerStatics3Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneCallOriginManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneCallOriginManagerStatics3Vtbl {
    pub const fn new<Impl: IPhoneCallOriginManagerStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhoneCallOriginManagerStatics3Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IPhoneCallOriginManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhoneCallOriginManagerStatics3>, base.5, IsSupported::<Impl, OFFSET>)
    }
}
