#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICustomDeviceImpl: Sized {
    fn InputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn SendIOControlAsync(&mut self, iocontrolcode: &::core::option::Option<IIOControlCode>, inputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, outputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn TrySendIOControlAsync(&mut self, iocontrolcode: &::core::option::Option<IIOControlCode>, inputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, outputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICustomDevice {
    const NAME: &'static str = "Windows.Devices.Custom.ICustomDevice";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICustomDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomDeviceVtbl {
        unsafe extern "system" fn InputStream<Impl: ICustomDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputStream<Impl: ICustomDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendIOControlAsync<Impl: ICustomDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iocontrolcode: ::windows::core::RawPtr, inputbuffer: ::windows::core::RawPtr, outputbuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn TrySendIOControlAsync<Impl: ICustomDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iocontrolcode: ::windows::core::RawPtr, inputbuffer: ::windows::core::RawPtr, outputbuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomDevice, BASE_OFFSET>(),
            InputStream: InputStream::<Impl, IMPL_OFFSET>,
            OutputStream: OutputStream::<Impl, IMPL_OFFSET>,
            SendIOControlAsync: SendIOControlAsync::<Impl, IMPL_OFFSET>,
            TrySendIOControlAsync: TrySendIOControlAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICustomDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&mut self, classguid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CustomDevice>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICustomDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Custom.ICustomDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICustomDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ICustomDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classguid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(&*(&classguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ICustomDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), desiredaccess, sharingmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomDeviceStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomDeviceStatics as ::windows::core::Interface>::IID
    }
}
pub trait IIOControlCodeImpl: Sized {
    fn AccessMode(&mut self) -> ::windows::core::Result<IOControlAccessMode>;
    fn BufferingMethod(&mut self) -> ::windows::core::Result<IOControlBufferingMethod>;
    fn Function(&mut self) -> ::windows::core::Result<u16>;
    fn DeviceType(&mut self) -> ::windows::core::Result<u16>;
    fn ControlCode(&mut self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IIOControlCode {
    const NAME: &'static str = "Windows.Devices.Custom.IIOControlCode";
}
impl IIOControlCodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIOControlCodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIOControlCodeVtbl {
        unsafe extern "system" fn AccessMode<Impl: IIOControlCodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IOControlAccessMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferingMethod<Impl: IIOControlCodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IOControlBufferingMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferingMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Function<Impl: IIOControlCodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Function() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceType<Impl: IIOControlCodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlCode<Impl: IIOControlCodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIOControlCode, BASE_OFFSET>(),
            AccessMode: AccessMode::<Impl, IMPL_OFFSET>,
            BufferingMethod: BufferingMethod::<Impl, IMPL_OFFSET>,
            Function: Function::<Impl, IMPL_OFFSET>,
            DeviceType: DeviceType::<Impl, IMPL_OFFSET>,
            ControlCode: ControlCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIOControlCode as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIOControlCodeFactoryImpl: Sized {
    fn CreateIOControlCode(&mut self, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod) -> ::windows::core::Result<IOControlCode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIOControlCodeFactory {
    const NAME: &'static str = "Windows.Devices.Custom.IIOControlCodeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IIOControlCodeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIOControlCodeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIOControlCodeFactoryVtbl {
        unsafe extern "system" fn CreateIOControlCode<Impl: IIOControlCodeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateIOControlCode(devicetype, function, accessmode, bufferingmethod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIOControlCodeFactory, BASE_OFFSET>(),
            CreateIOControlCode: CreateIOControlCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIOControlCodeFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownDeviceTypesStaticsImpl: Sized {
    fn Unknown(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownDeviceTypesStatics {
    const NAME: &'static str = "Windows.Devices.Custom.IKnownDeviceTypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownDeviceTypesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownDeviceTypesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownDeviceTypesStaticsVtbl {
        unsafe extern "system" fn Unknown<Impl: IKnownDeviceTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unknown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownDeviceTypesStatics, BASE_OFFSET>(), Unknown: Unknown::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownDeviceTypesStatics as ::windows::core::Interface>::IID
    }
}
