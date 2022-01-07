#[cfg(feature = "implement_exclusive")]
pub trait ICustomDeviceImpl: Sized {
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn SendIOControlAsync(&self, iocontrolcode: &::core::option::Option<IIOControlCode>, inputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, outputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn TrySendIOControlAsync(&self, iocontrolcode: &::core::option::Option<IIOControlCode>, inputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, outputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomDevice {
    const NAME: &'static str = "Windows.Devices.Custom.ICustomDevice";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomDeviceVtbl {
    pub const fn new<Impl: ICustomDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomDeviceVtbl {
        unsafe extern "system" fn InputStream<Impl: ICustomDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputStream<Impl: ICustomDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendIOControlAsync<Impl: ICustomDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iocontrolcode: ::windows::core::RawPtr, inputbuffer: ::windows::core::RawPtr, outputbuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendIOControlAsync(
                &*(&iocontrolcode as *const <IIOControlCode as ::windows::core::Abi>::Abi as *const <IIOControlCode as ::windows::core::DefaultType>::DefaultType),
                &*(&inputbuffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&outputbuffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySendIOControlAsync<Impl: ICustomDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iocontrolcode: ::windows::core::RawPtr, inputbuffer: ::windows::core::RawPtr, outputbuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySendIOControlAsync(
                &*(&iocontrolcode as *const <IIOControlCode as ::windows::core::Abi>::Abi as *const <IIOControlCode as ::windows::core::DefaultType>::DefaultType),
                &*(&inputbuffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&outputbuffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomDevice>, base.5, InputStream::<Impl, OFFSET>, OutputStream::<Impl, OFFSET>, SendIOControlAsync::<Impl, OFFSET>, TrySendIOControlAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self, classguid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CustomDevice>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Custom.ICustomDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomDeviceStaticsVtbl {
    pub const fn new<Impl: ICustomDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ICustomDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classguid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(&*(&classguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ICustomDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), desiredaccess, sharingmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomDeviceStatics>, base.5, GetDeviceSelector::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>)
    }
}
pub trait IIOControlCodeImpl: Sized {
    fn AccessMode(&self) -> ::windows::core::Result<IOControlAccessMode>;
    fn BufferingMethod(&self) -> ::windows::core::Result<IOControlBufferingMethod>;
    fn Function(&self) -> ::windows::core::Result<u16>;
    fn DeviceType(&self) -> ::windows::core::Result<u16>;
    fn ControlCode(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IIOControlCode {
    const NAME: &'static str = "Windows.Devices.Custom.IIOControlCode";
}
impl IIOControlCodeVtbl {
    pub const fn new<Impl: IIOControlCodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIOControlCodeVtbl {
        unsafe extern "system" fn AccessMode<Impl: IIOControlCodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IOControlAccessMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferingMethod<Impl: IIOControlCodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IOControlBufferingMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BufferingMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Function<Impl: IIOControlCodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Function() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceType<Impl: IIOControlCodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlCode<Impl: IIOControlCodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIOControlCode>, base.5, AccessMode::<Impl, OFFSET>, BufferingMethod::<Impl, OFFSET>, Function::<Impl, OFFSET>, DeviceType::<Impl, OFFSET>, ControlCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIOControlCodeFactoryImpl: Sized {
    fn CreateIOControlCode(&self, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod) -> ::windows::core::Result<IOControlCode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIOControlCodeFactory {
    const NAME: &'static str = "Windows.Devices.Custom.IIOControlCodeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IIOControlCodeFactoryVtbl {
    pub const fn new<Impl: IIOControlCodeFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIOControlCodeFactoryVtbl {
        unsafe extern "system" fn CreateIOControlCode<Impl: IIOControlCodeFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateIOControlCode(devicetype, function, accessmode, bufferingmethod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIOControlCodeFactory>, base.5, CreateIOControlCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownDeviceTypesStaticsImpl: Sized {
    fn Unknown(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownDeviceTypesStatics {
    const NAME: &'static str = "Windows.Devices.Custom.IKnownDeviceTypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownDeviceTypesStaticsVtbl {
    pub const fn new<Impl: IKnownDeviceTypesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnownDeviceTypesStaticsVtbl {
        unsafe extern "system" fn Unknown<Impl: IKnownDeviceTypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unknown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKnownDeviceTypesStatics>, base.5, Unknown::<Impl, OFFSET>)
    }
}
