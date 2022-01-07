#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAboutDataImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn DefaultAppName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultAppName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn DateOfManufacture(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetDateOfManufacture(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn DefaultDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Descriptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn DefaultManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultManufacturer(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Manufacturers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetModelNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SoftwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSoftwareVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SupportUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSupportUrl(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetAppId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynAboutData {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynAboutData";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynAboutDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynAboutDataImpl, const OFFSET: isize>() -> IAllJoynAboutDataVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn DefaultAppName<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultAppName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultAppName<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultAppName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppNames<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateOfManufacture<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateOfManufacture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDateOfManufacture<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDateOfManufacture(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultDescription<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDescription<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Descriptions<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultManufacturer<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultManufacturer<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultManufacturer(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Manufacturers<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Manufacturers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModelNumber<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModelNumber(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SoftwareVersion<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoftwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoftwareVersion<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoftwareVersion(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportUrl<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportUrl<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportUrl(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppId<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppId<Impl: IAllJoynAboutDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAllJoynAboutData>,
            ::windows::core::GetTrustLevel,
            IsEnabled::<Impl, OFFSET>,
            SetIsEnabled::<Impl, OFFSET>,
            DefaultAppName::<Impl, OFFSET>,
            SetDefaultAppName::<Impl, OFFSET>,
            AppNames::<Impl, OFFSET>,
            DateOfManufacture::<Impl, OFFSET>,
            SetDateOfManufacture::<Impl, OFFSET>,
            DefaultDescription::<Impl, OFFSET>,
            SetDefaultDescription::<Impl, OFFSET>,
            Descriptions::<Impl, OFFSET>,
            DefaultManufacturer::<Impl, OFFSET>,
            SetDefaultManufacturer::<Impl, OFFSET>,
            Manufacturers::<Impl, OFFSET>,
            ModelNumber::<Impl, OFFSET>,
            SetModelNumber::<Impl, OFFSET>,
            SoftwareVersion::<Impl, OFFSET>,
            SetSoftwareVersion::<Impl, OFFSET>,
            SupportUrl::<Impl, OFFSET>,
            SetSupportUrl::<Impl, OFFSET>,
            AppId::<Impl, OFFSET>,
            SetAppId::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAboutDataViewImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<i32>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn AJSoftwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DateOfManufacture(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn DefaultLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HardwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SoftwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedLanguages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>;
    fn SupportUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Manufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynAboutDataView {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynAboutDataView";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynAboutDataViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>() -> IAllJoynAboutDataViewVtbl {
        unsafe extern "system" fn Status<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AJSoftwareVersion<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AJSoftwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DateOfManufacture<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateOfManufacture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultLanguage<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HardwareVersion<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoftwareVersion<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoftwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedLanguages<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppName<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceName<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Manufacturer<Impl: IAllJoynAboutDataViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Manufacturer() {
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
            ::windows::core::GetRuntimeClassName::<IAllJoynAboutDataView>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            AJSoftwareVersion::<Impl, OFFSET>,
            AppId::<Impl, OFFSET>,
            DateOfManufacture::<Impl, OFFSET>,
            DefaultLanguage::<Impl, OFFSET>,
            DeviceId::<Impl, OFFSET>,
            HardwareVersion::<Impl, OFFSET>,
            ModelNumber::<Impl, OFFSET>,
            SoftwareVersion::<Impl, OFFSET>,
            SupportedLanguages::<Impl, OFFSET>,
            SupportUrl::<Impl, OFFSET>,
            AppName::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            DeviceName::<Impl, OFFSET>,
            Manufacturer::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAboutDataViewStaticsImpl: Sized {
    fn GetDataBySessionPortAsync(&self, uniquename: &::windows::core::HSTRING, busattachment: &::core::option::Option<AllJoynBusAttachment>, sessionport: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>;
    fn GetDataBySessionPortWithLanguageAsync(&self, uniquename: &::windows::core::HSTRING, busattachment: &::core::option::Option<AllJoynBusAttachment>, sessionport: u16, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynAboutDataViewStatics {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynAboutDataViewStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynAboutDataViewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynAboutDataViewStaticsImpl, const OFFSET: isize>() -> IAllJoynAboutDataViewStaticsVtbl {
        unsafe extern "system" fn GetDataBySessionPortAsync<Impl: IAllJoynAboutDataViewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, busattachment: ::windows::core::RawPtr, sessionport: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataBySessionPortAsync(&*(&uniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&busattachment as *const <AllJoynBusAttachment as ::windows::core::Abi>::Abi as *const <AllJoynBusAttachment as ::windows::core::DefaultType>::DefaultType), sessionport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataBySessionPortWithLanguageAsync<Impl: IAllJoynAboutDataViewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, busattachment: ::windows::core::RawPtr, sessionport: u16, language: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataBySessionPortWithLanguageAsync(
                &*(&uniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&busattachment as *const <AllJoynBusAttachment as ::windows::core::Abi>::Abi as *const <AllJoynBusAttachment as ::windows::core::DefaultType>::DefaultType),
                sessionport,
                &*(&language as *const <super::super::Globalization::Language as ::windows::core::Abi>::Abi as *const <super::super::Globalization::Language as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynAboutDataViewStatics>, ::windows::core::GetTrustLevel, GetDataBySessionPortAsync::<Impl, OFFSET>, GetDataBySessionPortWithLanguageAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait IAllJoynAcceptSessionJoinerImpl: Sized {
    fn Accept(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IAllJoynAcceptSessionJoiner {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynAcceptSessionJoiner";
}
#[cfg(feature = "deprecated")]
impl IAllJoynAcceptSessionJoinerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynAcceptSessionJoinerImpl, const OFFSET: isize>() -> IAllJoynAcceptSessionJoinerVtbl {
        unsafe extern "system" fn Accept<Impl: IAllJoynAcceptSessionJoinerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynAcceptSessionJoiner>, ::windows::core::GetTrustLevel, Accept::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAcceptSessionJoinerEventArgsImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionPort(&self) -> ::windows::core::Result<u16>;
    fn TrafficType(&self) -> ::windows::core::Result<AllJoynTrafficType>;
    fn SamePhysicalNode(&self) -> ::windows::core::Result<bool>;
    fn SameNetwork(&self) -> ::windows::core::Result<bool>;
    fn Accept(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynAcceptSessionJoinerEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynAcceptSessionJoinerEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynAcceptSessionJoinerEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynAcceptSessionJoinerEventArgsImpl, const OFFSET: isize>() -> IAllJoynAcceptSessionJoinerEventArgsVtbl {
        unsafe extern "system" fn UniqueName<Impl: IAllJoynAcceptSessionJoinerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionPort<Impl: IAllJoynAcceptSessionJoinerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionPort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficType<Impl: IAllJoynAcceptSessionJoinerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AllJoynTrafficType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrafficType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SamePhysicalNode<Impl: IAllJoynAcceptSessionJoinerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SamePhysicalNode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SameNetwork<Impl: IAllJoynAcceptSessionJoinerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SameNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IAllJoynAcceptSessionJoinerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynAcceptSessionJoinerEventArgs>, ::windows::core::GetTrustLevel, UniqueName::<Impl, OFFSET>, SessionPort::<Impl, OFFSET>, TrafficType::<Impl, OFFSET>, SamePhysicalNode::<Impl, OFFSET>, SameNetwork::<Impl, OFFSET>, Accept::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAcceptSessionJoinerEventArgsFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING, sessionport: u16, traffictype: AllJoynTrafficType, proximity: u8, acceptsessionjoiner: &::core::option::Option<IAllJoynAcceptSessionJoiner>) -> ::windows::core::Result<AllJoynAcceptSessionJoinerEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynAcceptSessionJoinerEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynAcceptSessionJoinerEventArgsFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynAcceptSessionJoinerEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynAcceptSessionJoinerEventArgsFactoryImpl, const OFFSET: isize>() -> IAllJoynAcceptSessionJoinerEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynAcceptSessionJoinerEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionport: u16, traffictype: AllJoynTrafficType, proximity: u8, acceptsessionjoiner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&uniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), sessionport, traffictype, proximity, &*(&acceptsessionjoiner as *const <IAllJoynAcceptSessionJoiner as ::windows::core::Abi>::Abi as *const <IAllJoynAcceptSessionJoiner as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynAcceptSessionJoinerEventArgsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAuthenticationCompleteEventArgsImpl: Sized {
    fn AuthenticationMechanism(&self) -> ::windows::core::Result<AllJoynAuthenticationMechanism>;
    fn PeerUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynAuthenticationCompleteEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynAuthenticationCompleteEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynAuthenticationCompleteEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynAuthenticationCompleteEventArgsImpl, const OFFSET: isize>() -> IAllJoynAuthenticationCompleteEventArgsVtbl {
        unsafe extern "system" fn AuthenticationMechanism<Impl: IAllJoynAuthenticationCompleteEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AllJoynAuthenticationMechanism) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationMechanism() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerUniqueName<Impl: IAllJoynAuthenticationCompleteEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerUniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Succeeded<Impl: IAllJoynAuthenticationCompleteEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynAuthenticationCompleteEventArgs>, ::windows::core::GetTrustLevel, AuthenticationMechanism::<Impl, OFFSET>, PeerUniqueName::<Impl, OFFSET>, Succeeded::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachmentImpl: Sized {
    fn AboutData(&self) -> ::windows::core::Result<AllJoynAboutData>;
    fn ConnectionSpecification(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<AllJoynBusAttachmentState>;
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PingAsync(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>>;
    fn Connect(&self) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynBusAttachmentStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationMechanisms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<AllJoynAuthenticationMechanism>>;
    fn CredentialsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCredentialsRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CredentialsVerificationRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsVerificationRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCredentialsVerificationRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationComplete(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAuthenticationCompleteEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationComplete(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynBusAttachment {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynBusAttachment";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynBusAttachmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>() -> IAllJoynBusAttachmentVtbl {
        unsafe extern "system" fn AboutData<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AboutData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionSpecification<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionSpecification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AllJoynBusAttachmentState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UniqueName<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PingAsync<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PingAsync(&*(&uniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect().into()
        }
        unsafe extern "system" fn Disconnect<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn StateChanged<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynBusAttachmentStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynBusAttachmentStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticationMechanisms<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationMechanisms() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialsRequested<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCredentialsRequested<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCredentialsRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CredentialsVerificationRequested<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialsVerificationRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsVerificationRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsVerificationRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCredentialsVerificationRequested<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCredentialsVerificationRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticationComplete<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationComplete(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAuthenticationCompleteEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAuthenticationCompleteEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAuthenticationComplete<Impl: IAllJoynBusAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAuthenticationComplete(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAllJoynBusAttachment>,
            ::windows::core::GetTrustLevel,
            AboutData::<Impl, OFFSET>,
            ConnectionSpecification::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            UniqueName::<Impl, OFFSET>,
            PingAsync::<Impl, OFFSET>,
            Connect::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
            StateChanged::<Impl, OFFSET>,
            RemoveStateChanged::<Impl, OFFSET>,
            AuthenticationMechanisms::<Impl, OFFSET>,
            CredentialsRequested::<Impl, OFFSET>,
            RemoveCredentialsRequested::<Impl, OFFSET>,
            CredentialsVerificationRequested::<Impl, OFFSET>,
            RemoveCredentialsVerificationRequested::<Impl, OFFSET>,
            AuthenticationComplete::<Impl, OFFSET>,
            RemoveAuthenticationComplete::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachment2Impl: Sized {
    fn GetAboutDataAsync(&self, serviceinfo: &::core::option::Option<AllJoynServiceInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>;
    fn GetAboutDataWithLanguageAsync(&self, serviceinfo: &::core::option::Option<AllJoynServiceInfo>, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>;
    fn AcceptSessionJoinerRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAcceptSessionJoinerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceptSessionJoinerRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SessionJoined(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynSessionJoinedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionJoined(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynBusAttachment2 {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynBusAttachment2";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynBusAttachment2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynBusAttachment2Impl, const OFFSET: isize>() -> IAllJoynBusAttachment2Vtbl {
        unsafe extern "system" fn GetAboutDataAsync<Impl: IAllJoynBusAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAboutDataAsync(&*(&serviceinfo as *const <AllJoynServiceInfo as ::windows::core::Abi>::Abi as *const <AllJoynServiceInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAboutDataWithLanguageAsync<Impl: IAllJoynBusAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceinfo: ::windows::core::RawPtr, language: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAboutDataWithLanguageAsync(&*(&serviceinfo as *const <AllJoynServiceInfo as ::windows::core::Abi>::Abi as *const <AllJoynServiceInfo as ::windows::core::DefaultType>::DefaultType), &*(&language as *const <super::super::Globalization::Language as ::windows::core::Abi>::Abi as *const <super::super::Globalization::Language as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptSessionJoinerRequested<Impl: IAllJoynBusAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptSessionJoinerRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAcceptSessionJoinerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAcceptSessionJoinerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAcceptSessionJoinerRequested<Impl: IAllJoynBusAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAcceptSessionJoinerRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SessionJoined<Impl: IAllJoynBusAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionJoined(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynSessionJoinedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynSessionJoinedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSessionJoined<Impl: IAllJoynBusAttachment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSessionJoined(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAllJoynBusAttachment2>,
            ::windows::core::GetTrustLevel,
            GetAboutDataAsync::<Impl, OFFSET>,
            GetAboutDataWithLanguageAsync::<Impl, OFFSET>,
            AcceptSessionJoinerRequested::<Impl, OFFSET>,
            RemoveAcceptSessionJoinerRequested::<Impl, OFFSET>,
            SessionJoined::<Impl, OFFSET>,
            RemoveSessionJoined::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachmentFactoryImpl: Sized {
    fn Create(&self, connectionspecification: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynBusAttachment>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynBusAttachmentFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynBusAttachmentFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynBusAttachmentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynBusAttachmentFactoryImpl, const OFFSET: isize>() -> IAllJoynBusAttachmentFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynBusAttachmentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionspecification: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&connectionspecification as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynBusAttachmentFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachmentStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AllJoynBusAttachmentState>;
    fn Status(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynBusAttachmentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynBusAttachmentStateChangedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynBusAttachmentStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynBusAttachmentStateChangedEventArgsImpl, const OFFSET: isize>() -> IAllJoynBusAttachmentStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IAllJoynBusAttachmentStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AllJoynBusAttachmentState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Status<Impl: IAllJoynBusAttachmentStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynBusAttachmentStateChangedEventArgs>, ::windows::core::GetTrustLevel, State::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachmentStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AllJoynBusAttachment>;
    fn GetWatcher(&self, requiredinterfaces: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Enumeration::DeviceWatcher>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynBusAttachmentStatics {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynBusAttachmentStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynBusAttachmentStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynBusAttachmentStaticsImpl, const OFFSET: isize>() -> IAllJoynBusAttachmentStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAllJoynBusAttachmentStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetWatcher<Impl: IAllJoynBusAttachmentStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredinterfaces: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatcher(&*(&requiredinterfaces as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynBusAttachmentStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>, GetWatcher::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusObjectImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn AddProducer(&self, producer: &::core::option::Option<IAllJoynProducer>) -> ::windows::core::Result<()>;
    fn BusAttachment(&self) -> ::windows::core::Result<AllJoynBusAttachment>;
    fn Session(&self) -> ::windows::core::Result<AllJoynSession>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusObject, AllJoynBusObjectStoppedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynBusObject {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynBusObject";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynBusObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynBusObjectImpl, const OFFSET: isize>() -> IAllJoynBusObjectVtbl {
        unsafe extern "system" fn Start<Impl: IAllJoynBusObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAllJoynBusObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn AddProducer<Impl: IAllJoynBusObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddProducer(&*(&producer as *const <IAllJoynProducer as ::windows::core::Abi>::Abi as *const <IAllJoynProducer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BusAttachment<Impl: IAllJoynBusObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusAttachment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Impl: IAllJoynBusObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stopped<Impl: IAllJoynBusObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynBusObject, AllJoynBusObjectStoppedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynBusObject, AllJoynBusObjectStoppedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IAllJoynBusObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynBusObject>, ::windows::core::GetTrustLevel, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, AddProducer::<Impl, OFFSET>, BusAttachment::<Impl, OFFSET>, Session::<Impl, OFFSET>, Stopped::<Impl, OFFSET>, RemoveStopped::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusObjectFactoryImpl: Sized {
    fn Create(&self, objectpath: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynBusObject>;
    fn CreateWithBusAttachment(&self, objectpath: &::windows::core::HSTRING, busattachment: &::core::option::Option<AllJoynBusAttachment>) -> ::windows::core::Result<AllJoynBusObject>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynBusObjectFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynBusObjectFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynBusObjectFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynBusObjectFactoryImpl, const OFFSET: isize>() -> IAllJoynBusObjectFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynBusObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&objectpath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithBusAttachment<Impl: IAllJoynBusObjectFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, busattachment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithBusAttachment(&*(&objectpath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&busattachment as *const <AllJoynBusAttachment as ::windows::core::Abi>::Abi as *const <AllJoynBusAttachment as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynBusObjectFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>, CreateWithBusAttachment::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusObjectStoppedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynBusObjectStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynBusObjectStoppedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynBusObjectStoppedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynBusObjectStoppedEventArgsImpl, const OFFSET: isize>() -> IAllJoynBusObjectStoppedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IAllJoynBusObjectStoppedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynBusObjectStoppedEventArgs>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusObjectStoppedEventArgsFactoryImpl: Sized {
    fn Create(&self, status: i32) -> ::windows::core::Result<AllJoynBusObjectStoppedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynBusObjectStoppedEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynBusObjectStoppedEventArgsFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynBusObjectStoppedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynBusObjectStoppedEventArgsFactoryImpl, const OFFSET: isize>() -> IAllJoynBusObjectStoppedEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynBusObjectStoppedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynBusObjectStoppedEventArgsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynCredentialsImpl: Sized {
    fn AuthenticationMechanism(&self) -> ::windows::core::Result<AllJoynAuthenticationMechanism>;
    fn Certificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetCertificate(&self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
    fn PasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetPasswordCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn Timeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynCredentials {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynCredentials";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynCredentialsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynCredentialsImpl, const OFFSET: isize>() -> IAllJoynCredentialsVtbl {
        unsafe extern "system" fn AuthenticationMechanism<Impl: IAllJoynCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AllJoynAuthenticationMechanism) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationMechanism() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Certificate<Impl: IAllJoynCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificate<Impl: IAllJoynCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificate(&*(&value as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PasswordCredential<Impl: IAllJoynCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordCredential<Impl: IAllJoynCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPasswordCredential(&*(&value as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Timeout<Impl: IAllJoynCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeout<Impl: IAllJoynCredentialsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAllJoynCredentials>,
            ::windows::core::GetTrustLevel,
            AuthenticationMechanism::<Impl, OFFSET>,
            Certificate::<Impl, OFFSET>,
            SetCertificate::<Impl, OFFSET>,
            PasswordCredential::<Impl, OFFSET>,
            SetPasswordCredential::<Impl, OFFSET>,
            Timeout::<Impl, OFFSET>,
            SetTimeout::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynCredentialsRequestedEventArgsImpl: Sized {
    fn AttemptCount(&self) -> ::windows::core::Result<u16>;
    fn Credentials(&self) -> ::windows::core::Result<AllJoynCredentials>;
    fn PeerUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RequestedUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynCredentialsRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynCredentialsRequestedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynCredentialsRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynCredentialsRequestedEventArgsImpl, const OFFSET: isize>() -> IAllJoynCredentialsRequestedEventArgsVtbl {
        unsafe extern "system" fn AttemptCount<Impl: IAllJoynCredentialsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttemptCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Credentials<Impl: IAllJoynCredentialsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Credentials() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerUniqueName<Impl: IAllJoynCredentialsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerUniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUserName<Impl: IAllJoynCredentialsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedUserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IAllJoynCredentialsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynCredentialsRequestedEventArgs>, ::windows::core::GetTrustLevel, AttemptCount::<Impl, OFFSET>, Credentials::<Impl, OFFSET>, PeerUniqueName::<Impl, OFFSET>, RequestedUserName::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynCredentialsVerificationRequestedEventArgsImpl: Sized {
    fn AuthenticationMechanism(&self) -> ::windows::core::Result<AllJoynAuthenticationMechanism>;
    fn PeerUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PeerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn PeerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketSslErrorSeverity>;
    fn PeerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn PeerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
    fn Accept(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynCredentialsVerificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynCredentialsVerificationRequestedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynCredentialsVerificationRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynCredentialsVerificationRequestedEventArgsImpl, const OFFSET: isize>() -> IAllJoynCredentialsVerificationRequestedEventArgsVtbl {
        unsafe extern "system" fn AuthenticationMechanism<Impl: IAllJoynCredentialsVerificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AllJoynAuthenticationMechanism) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationMechanism() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerUniqueName<Impl: IAllJoynCredentialsVerificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerUniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerCertificate<Impl: IAllJoynCredentialsVerificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerCertificateErrorSeverity<Impl: IAllJoynCredentialsVerificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerCertificateErrorSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerCertificateErrors<Impl: IAllJoynCredentialsVerificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerIntermediateCertificates<Impl: IAllJoynCredentialsVerificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerIntermediateCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IAllJoynCredentialsVerificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept().into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IAllJoynCredentialsVerificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAllJoynCredentialsVerificationRequestedEventArgs>,
            ::windows::core::GetTrustLevel,
            AuthenticationMechanism::<Impl, OFFSET>,
            PeerUniqueName::<Impl, OFFSET>,
            PeerCertificate::<Impl, OFFSET>,
            PeerCertificateErrorSeverity::<Impl, OFFSET>,
            PeerCertificateErrors::<Impl, OFFSET>,
            PeerIntermediateCertificates::<Impl, OFFSET>,
            Accept::<Impl, OFFSET>,
            GetDeferral::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynMessageInfoImpl: Sized {
    fn SenderUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynMessageInfo {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynMessageInfo";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynMessageInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynMessageInfoImpl, const OFFSET: isize>() -> IAllJoynMessageInfoVtbl {
        unsafe extern "system" fn SenderUniqueName<Impl: IAllJoynMessageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderUniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynMessageInfo>, ::windows::core::GetTrustLevel, SenderUniqueName::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynMessageInfoFactoryImpl: Sized {
    fn Create(&self, senderuniquename: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynMessageInfo>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynMessageInfoFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynMessageInfoFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynMessageInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynMessageInfoFactoryImpl, const OFFSET: isize>() -> IAllJoynMessageInfoFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynMessageInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, senderuniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&senderuniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynMessageInfoFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait IAllJoynProducerImpl: Sized {
    fn SetBusObject(&self, busobject: &::core::option::Option<AllJoynBusObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for IAllJoynProducer {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynProducer";
}
#[cfg(feature = "deprecated")]
impl IAllJoynProducerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynProducerImpl, const OFFSET: isize>() -> IAllJoynProducerVtbl {
        unsafe extern "system" fn SetBusObject<Impl: IAllJoynProducerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusObject(&*(&busobject as *const <AllJoynBusObject as ::windows::core::Abi>::Abi as *const <AllJoynBusObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynProducer>, ::windows::core::GetTrustLevel, SetBusObject::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynProducerStoppedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynProducerStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynProducerStoppedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynProducerStoppedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynProducerStoppedEventArgsImpl, const OFFSET: isize>() -> IAllJoynProducerStoppedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IAllJoynProducerStoppedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynProducerStoppedEventArgs>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynProducerStoppedEventArgsFactoryImpl: Sized {
    fn Create(&self, status: i32) -> ::windows::core::Result<AllJoynProducerStoppedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynProducerStoppedEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynProducerStoppedEventArgsFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynProducerStoppedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynProducerStoppedEventArgsFactoryImpl, const OFFSET: isize>() -> IAllJoynProducerStoppedEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynProducerStoppedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynProducerStoppedEventArgsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ObjectPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionPort(&self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynServiceInfo {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynServiceInfo";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynServiceInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynServiceInfoImpl, const OFFSET: isize>() -> IAllJoynServiceInfoVtbl {
        unsafe extern "system" fn UniqueName<Impl: IAllJoynServiceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectPath<Impl: IAllJoynServiceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionPort<Impl: IAllJoynServiceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionPort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynServiceInfo>, ::windows::core::GetTrustLevel, UniqueName::<Impl, OFFSET>, ObjectPath::<Impl, OFFSET>, SessionPort::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING, objectpath: &::windows::core::HSTRING, sessionport: u16) -> ::windows::core::Result<AllJoynServiceInfo>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynServiceInfoFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynServiceInfoFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynServiceInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynServiceInfoFactoryImpl, const OFFSET: isize>() -> IAllJoynServiceInfoFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynServiceInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, objectpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionport: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&uniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&objectpath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), sessionport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynServiceInfoFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoRemovedEventArgsImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynServiceInfoRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynServiceInfoRemovedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynServiceInfoRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynServiceInfoRemovedEventArgsImpl, const OFFSET: isize>() -> IAllJoynServiceInfoRemovedEventArgsVtbl {
        unsafe extern "system" fn UniqueName<Impl: IAllJoynServiceInfoRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynServiceInfoRemovedEventArgs>, ::windows::core::GetTrustLevel, UniqueName::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoRemovedEventArgsFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynServiceInfoRemovedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynServiceInfoRemovedEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynServiceInfoRemovedEventArgsFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynServiceInfoRemovedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynServiceInfoRemovedEventArgsFactoryImpl, const OFFSET: isize>() -> IAllJoynServiceInfoRemovedEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynServiceInfoRemovedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&uniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynServiceInfoRemovedEventArgsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynServiceInfo>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynServiceInfoStatics {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynServiceInfoStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynServiceInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynServiceInfoStaticsImpl, const OFFSET: isize>() -> IAllJoynServiceInfoStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IAllJoynServiceInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynServiceInfoStatics>, ::windows::core::GetTrustLevel, FromIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn Status(&self) -> ::windows::core::Result<i32>;
    fn RemoveMemberAsync(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>>;
    fn MemberAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMemberAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MemberRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMemberRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Lost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionLostEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLost(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSession {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSession";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionImpl, const OFFSET: isize>() -> IAllJoynSessionVtbl {
        unsafe extern "system" fn Id<Impl: IAllJoynSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Status<Impl: IAllJoynSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMemberAsync<Impl: IAllJoynSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveMemberAsync(&*(&uniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MemberAdded<Impl: IAllJoynSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MemberAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMemberAdded<Impl: IAllJoynSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMemberAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MemberRemoved<Impl: IAllJoynSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MemberRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMemberRemoved<Impl: IAllJoynSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMemberRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Lost<Impl: IAllJoynSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lost(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionLostEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionLostEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLost<Impl: IAllJoynSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLost(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAllJoynSession>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            RemoveMemberAsync::<Impl, OFFSET>,
            MemberAdded::<Impl, OFFSET>,
            RemoveMemberAdded::<Impl, OFFSET>,
            MemberRemoved::<Impl, OFFSET>,
            RemoveMemberRemoved::<Impl, OFFSET>,
            Lost::<Impl, OFFSET>,
            RemoveLost::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionJoinedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<AllJoynSession>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSessionJoinedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSessionJoinedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionJoinedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionJoinedEventArgsImpl, const OFFSET: isize>() -> IAllJoynSessionJoinedEventArgsVtbl {
        unsafe extern "system" fn Session<Impl: IAllJoynSessionJoinedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynSessionJoinedEventArgs>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionJoinedEventArgsFactoryImpl: Sized {
    fn Create(&self, session: &::core::option::Option<AllJoynSession>) -> ::windows::core::Result<AllJoynSessionJoinedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSessionJoinedEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSessionJoinedEventArgsFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionJoinedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionJoinedEventArgsFactoryImpl, const OFFSET: isize>() -> IAllJoynSessionJoinedEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynSessionJoinedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&session as *const <AllJoynSession as ::windows::core::Abi>::Abi as *const <AllJoynSession as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynSessionJoinedEventArgsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionLostEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<AllJoynSessionLostReason>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSessionLostEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSessionLostEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionLostEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionLostEventArgsImpl, const OFFSET: isize>() -> IAllJoynSessionLostEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IAllJoynSessionLostEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AllJoynSessionLostReason) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynSessionLostEventArgs>, ::windows::core::GetTrustLevel, Reason::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionLostEventArgsFactoryImpl: Sized {
    fn Create(&self, reason: AllJoynSessionLostReason) -> ::windows::core::Result<AllJoynSessionLostEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSessionLostEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSessionLostEventArgsFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionLostEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionLostEventArgsFactoryImpl, const OFFSET: isize>() -> IAllJoynSessionLostEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynSessionLostEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: AllJoynSessionLostReason, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynSessionLostEventArgsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionMemberAddedEventArgsImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSessionMemberAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSessionMemberAddedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionMemberAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionMemberAddedEventArgsImpl, const OFFSET: isize>() -> IAllJoynSessionMemberAddedEventArgsVtbl {
        unsafe extern "system" fn UniqueName<Impl: IAllJoynSessionMemberAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynSessionMemberAddedEventArgs>, ::windows::core::GetTrustLevel, UniqueName::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionMemberAddedEventArgsFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynSessionMemberAddedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSessionMemberAddedEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSessionMemberAddedEventArgsFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionMemberAddedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionMemberAddedEventArgsFactoryImpl, const OFFSET: isize>() -> IAllJoynSessionMemberAddedEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynSessionMemberAddedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&uniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynSessionMemberAddedEventArgsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionMemberRemovedEventArgsImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSessionMemberRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSessionMemberRemovedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionMemberRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionMemberRemovedEventArgsImpl, const OFFSET: isize>() -> IAllJoynSessionMemberRemovedEventArgsVtbl {
        unsafe extern "system" fn UniqueName<Impl: IAllJoynSessionMemberRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynSessionMemberRemovedEventArgs>, ::windows::core::GetTrustLevel, UniqueName::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionMemberRemovedEventArgsFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynSessionMemberRemovedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSessionMemberRemovedEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSessionMemberRemovedEventArgsFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionMemberRemovedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionMemberRemovedEventArgsFactoryImpl, const OFFSET: isize>() -> IAllJoynSessionMemberRemovedEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynSessionMemberRemovedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniquename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&uniquename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynSessionMemberRemovedEventArgsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionStaticsImpl: Sized {
    fn GetFromServiceInfoAsync(&self, serviceinfo: &::core::option::Option<AllJoynServiceInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynSession>>;
    fn GetFromServiceInfoAndBusAttachmentAsync(&self, serviceinfo: &::core::option::Option<AllJoynServiceInfo>, busattachment: &::core::option::Option<AllJoynBusAttachment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynSession>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynSessionStatics {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynSessionStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynSessionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynSessionStaticsImpl, const OFFSET: isize>() -> IAllJoynSessionStaticsVtbl {
        unsafe extern "system" fn GetFromServiceInfoAsync<Impl: IAllJoynSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFromServiceInfoAsync(&*(&serviceinfo as *const <AllJoynServiceInfo as ::windows::core::Abi>::Abi as *const <AllJoynServiceInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFromServiceInfoAndBusAttachmentAsync<Impl: IAllJoynSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceinfo: ::windows::core::RawPtr, busattachment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFromServiceInfoAndBusAttachmentAsync(&*(&serviceinfo as *const <AllJoynServiceInfo as ::windows::core::Abi>::Abi as *const <AllJoynServiceInfo as ::windows::core::DefaultType>::DefaultType), &*(&busattachment as *const <AllJoynBusAttachment as ::windows::core::Abi>::Abi as *const <AllJoynBusAttachment as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynSessionStatics>, ::windows::core::GetTrustLevel, GetFromServiceInfoAsync::<Impl, OFFSET>, GetFromServiceInfoAndBusAttachmentAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynStatusStaticsImpl: Sized {
    fn Ok(&self) -> ::windows::core::Result<i32>;
    fn Fail(&self) -> ::windows::core::Result<i32>;
    fn OperationTimedOut(&self) -> ::windows::core::Result<i32>;
    fn OtherEndClosed(&self) -> ::windows::core::Result<i32>;
    fn ConnectionRefused(&self) -> ::windows::core::Result<i32>;
    fn AuthenticationFailed(&self) -> ::windows::core::Result<i32>;
    fn AuthenticationRejectedByUser(&self) -> ::windows::core::Result<i32>;
    fn SslConnectFailed(&self) -> ::windows::core::Result<i32>;
    fn SslIdentityVerificationFailed(&self) -> ::windows::core::Result<i32>;
    fn InsufficientSecurity(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument1(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument2(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument3(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument4(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument5(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument6(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument7(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument8(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynStatusStatics {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynStatusStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynStatusStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>() -> IAllJoynStatusStaticsVtbl {
        unsafe extern "system" fn Ok<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ok() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Fail<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Fail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OperationTimedOut<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationTimedOut() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OtherEndClosed<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherEndClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionRefused<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionRefused() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationFailed<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationFailed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationRejectedByUser<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationRejectedByUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SslConnectFailed<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SslConnectFailed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SslIdentityVerificationFailed<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SslIdentityVerificationFailed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsufficientSecurity<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsufficientSecurity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidArgument1<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidArgument1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidArgument2<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidArgument2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidArgument3<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidArgument3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidArgument4<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidArgument4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidArgument5<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidArgument5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidArgument6<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidArgument6() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidArgument7<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidArgument7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidArgument8<Impl: IAllJoynStatusStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidArgument8() {
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
            ::windows::core::GetRuntimeClassName::<IAllJoynStatusStatics>,
            ::windows::core::GetTrustLevel,
            Ok::<Impl, OFFSET>,
            Fail::<Impl, OFFSET>,
            OperationTimedOut::<Impl, OFFSET>,
            OtherEndClosed::<Impl, OFFSET>,
            ConnectionRefused::<Impl, OFFSET>,
            AuthenticationFailed::<Impl, OFFSET>,
            AuthenticationRejectedByUser::<Impl, OFFSET>,
            SslConnectFailed::<Impl, OFFSET>,
            SslIdentityVerificationFailed::<Impl, OFFSET>,
            InsufficientSecurity::<Impl, OFFSET>,
            InvalidArgument1::<Impl, OFFSET>,
            InvalidArgument2::<Impl, OFFSET>,
            InvalidArgument3::<Impl, OFFSET>,
            InvalidArgument4::<Impl, OFFSET>,
            InvalidArgument5::<Impl, OFFSET>,
            InvalidArgument6::<Impl, OFFSET>,
            InvalidArgument7::<Impl, OFFSET>,
            InvalidArgument8::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynWatcherStoppedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynWatcherStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynWatcherStoppedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynWatcherStoppedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynWatcherStoppedEventArgsImpl, const OFFSET: isize>() -> IAllJoynWatcherStoppedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IAllJoynWatcherStoppedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynWatcherStoppedEventArgs>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynWatcherStoppedEventArgsFactoryImpl: Sized {
    fn Create(&self, status: i32) -> ::windows::core::Result<AllJoynWatcherStoppedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAllJoynWatcherStoppedEventArgsFactory {
    const NAME: &'static str = "Windows.Devices.AllJoyn.IAllJoynWatcherStoppedEventArgsFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAllJoynWatcherStoppedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAllJoynWatcherStoppedEventArgsFactoryImpl, const OFFSET: isize>() -> IAllJoynWatcherStoppedEventArgsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAllJoynWatcherStoppedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAllJoynWatcherStoppedEventArgsFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
