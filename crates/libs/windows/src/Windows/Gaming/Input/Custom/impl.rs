pub trait ICustomGameControllerFactoryImpl: Sized {
    fn CreateGameController(&self, provider: &::core::option::Option<IGameControllerProvider>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn OnGameControllerAdded(&self, value: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<()>;
    fn OnGameControllerRemoved(&self, value: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICustomGameControllerFactory {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.ICustomGameControllerFactory";
}
impl ICustomGameControllerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomGameControllerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomGameControllerFactoryVtbl {
        unsafe extern "system" fn CreateGameController<Impl: ICustomGameControllerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGameController(&*(&provider as *const <IGameControllerProvider as ::windows::core::Abi>::Abi as *const <IGameControllerProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnGameControllerAdded<Impl: ICustomGameControllerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGameControllerAdded(&*(&value as *const <super::IGameController as ::windows::core::Abi>::Abi as *const <super::IGameController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnGameControllerRemoved<Impl: ICustomGameControllerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGameControllerRemoved(&*(&value as *const <super::IGameController as ::windows::core::Abi>::Abi as *const <super::IGameController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICustomGameControllerFactory>, ::windows::core::GetTrustLevel, CreateGameController::<Impl, IMPL_OFFSET>, OnGameControllerAdded::<Impl, IMPL_OFFSET>, OnGameControllerRemoved::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomGameControllerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameControllerFactoryManagerStaticsImpl: Sized {
    fn RegisterCustomFactoryForGipInterface(&self, factory: &::core::option::Option<ICustomGameControllerFactory>, interfaceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterCustomFactoryForHardwareId(&self, factory: &::core::option::Option<ICustomGameControllerFactory>, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::Result<()>;
    fn RegisterCustomFactoryForXusbType(&self, factory: &::core::option::Option<ICustomGameControllerFactory>, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameControllerFactoryManagerStatics {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameControllerFactoryManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerFactoryManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerFactoryManagerStaticsVtbl {
        unsafe extern "system" fn RegisterCustomFactoryForGipInterface<Impl: IGameControllerFactoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, interfaceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCustomFactoryForGipInterface(&*(&factory as *const <ICustomGameControllerFactory as ::windows::core::Abi>::Abi as *const <ICustomGameControllerFactory as ::windows::core::DefaultType>::DefaultType), &*(&interfaceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterCustomFactoryForHardwareId<Impl: IGameControllerFactoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCustomFactoryForHardwareId(&*(&factory as *const <ICustomGameControllerFactory as ::windows::core::Abi>::Abi as *const <ICustomGameControllerFactory as ::windows::core::DefaultType>::DefaultType), hardwarevendorid, hardwareproductid).into()
        }
        unsafe extern "system" fn RegisterCustomFactoryForXusbType<Impl: IGameControllerFactoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCustomFactoryForXusbType(&*(&factory as *const <ICustomGameControllerFactory as ::windows::core::Abi>::Abi as *const <ICustomGameControllerFactory as ::windows::core::DefaultType>::DefaultType), xusbtype, xusbsubtype).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameControllerFactoryManagerStatics>, ::windows::core::GetTrustLevel, RegisterCustomFactoryForGipInterface::<Impl, IMPL_OFFSET>, RegisterCustomFactoryForHardwareId::<Impl, IMPL_OFFSET>, RegisterCustomFactoryForXusbType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerFactoryManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameControllerFactoryManagerStatics2Impl: Sized + IGameControllerFactoryManagerStaticsImpl {
    fn TryGetFactoryControllerFromGameController(&self, factory: &::core::option::Option<ICustomGameControllerFactory>, gamecontroller: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<super::IGameController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameControllerFactoryManagerStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGameControllerFactoryManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerFactoryManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerFactoryManagerStatics2Vtbl {
        unsafe extern "system" fn TryGetFactoryControllerFromGameController<Impl: IGameControllerFactoryManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetFactoryControllerFromGameController(&*(&factory as *const <ICustomGameControllerFactory as ::windows::core::Abi>::Abi as *const <ICustomGameControllerFactory as ::windows::core::DefaultType>::DefaultType), &*(&gamecontroller as *const <super::IGameController as ::windows::core::Abi>::Abi as *const <super::IGameController as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameControllerFactoryManagerStatics2>, ::windows::core::GetTrustLevel, TryGetFactoryControllerFromGameController::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerFactoryManagerStatics2 as ::windows::core::Interface>::IID
    }
}
pub trait IGameControllerInputSinkImpl: Sized {
    fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()>;
    fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerInputSink";
}
impl IGameControllerInputSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerInputSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerInputSinkVtbl {
        unsafe extern "system" fn OnInputResumed<Impl: IGameControllerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInputResumed(timestamp).into()
        }
        unsafe extern "system" fn OnInputSuspended<Impl: IGameControllerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInputSuspended(timestamp).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameControllerInputSink>, ::windows::core::GetTrustLevel, OnInputResumed::<Impl, IMPL_OFFSET>, OnInputSuspended::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
pub trait IGameControllerProviderImpl: Sized {
    fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo>;
    fn HardwareProductId(&self) -> ::windows::core::Result<u16>;
    fn HardwareVendorId(&self) -> ::windows::core::Result<u16>;
    fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo>;
    fn IsConnected(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerProvider";
}
impl IGameControllerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerProviderVtbl {
        unsafe extern "system" fn FirmwareVersionInfo<Impl: IGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirmwareVersionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareProductId<Impl: IGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareVendorId<Impl: IGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareVendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareVersionInfo<Impl: IGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareVersionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: IGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGameControllerProvider>, ::windows::core::GetTrustLevel, FirmwareVersionInfo::<Impl, IMPL_OFFSET>, HardwareProductId::<Impl, IMPL_OFFSET>, HardwareVendorId::<Impl, IMPL_OFFSET>, HardwareVersionInfo::<Impl, IMPL_OFFSET>, IsConnected::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGipFirmwareUpdateResultImpl: Sized {
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<u32>;
    fn FinalComponentId(&self) -> ::windows::core::Result<u32>;
    fn Status(&self) -> ::windows::core::Result<GipFirmwareUpdateStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGipFirmwareUpdateResult {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGipFirmwareUpdateResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGipFirmwareUpdateResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGipFirmwareUpdateResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGipFirmwareUpdateResultVtbl {
        unsafe extern "system" fn ExtendedErrorCode<Impl: IGipFirmwareUpdateResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalComponentId<Impl: IGipFirmwareUpdateResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalComponentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IGipFirmwareUpdateResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GipFirmwareUpdateStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGipFirmwareUpdateResult>, ::windows::core::GetTrustLevel, ExtendedErrorCode::<Impl, IMPL_OFFSET>, FinalComponentId::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGipFirmwareUpdateResult as ::windows::core::Interface>::IID
    }
}
pub trait IGipGameControllerInputSinkImpl: Sized + IGameControllerInputSinkImpl {
    fn OnKeyReceived(&self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::Result<()>;
    fn OnMessageReceived(&self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGipGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGipGameControllerInputSink";
}
impl IGipGameControllerInputSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGipGameControllerInputSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGipGameControllerInputSinkVtbl {
        unsafe extern "system" fn OnKeyReceived<Impl: IGipGameControllerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKeyReceived(timestamp, keycode, ispressed).into()
        }
        unsafe extern "system" fn OnMessageReceived<Impl: IGipGameControllerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMessageReceived(timestamp, messageclass, messageid, sequenceid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&messagebuffer), messageBuffer_array_size as _)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGipGameControllerInputSink>, ::windows::core::GetTrustLevel, OnKeyReceived::<Impl, IMPL_OFFSET>, OnMessageReceived::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGipGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGipGameControllerProviderImpl: Sized + IGameControllerProviderImpl {
    fn SendMessage(&self, messageclass: GipMessageClass, messageid: u8, messagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SendReceiveMessage(&self, messageclass: GipMessageClass, messageid: u8, requestmessagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], responsemessagebuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UpdateFirmwareAsync(&self, firmwareimage: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGipGameControllerProvider";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGipGameControllerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGipGameControllerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGipGameControllerProviderVtbl {
        unsafe extern "system" fn SendMessage<Impl: IGipGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessage(messageclass, messageid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&messagebuffer), messageBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn SendReceiveMessage<Impl: IGipGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, requestMessageBuffer_array_size: u32, requestmessagebuffer: *const u8, responseMessageBuffer_array_size: u32, responsemessagebuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendReceiveMessage(messageclass, messageid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&requestmessagebuffer), requestMessageBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&responsemessagebuffer), responseMessageBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn UpdateFirmwareAsync<Impl: IGipGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firmwareimage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateFirmwareAsync(&*(&firmwareimage as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGipGameControllerProvider>, ::windows::core::GetTrustLevel, SendMessage::<Impl, IMPL_OFFSET>, SendReceiveMessage::<Impl, IMPL_OFFSET>, UpdateFirmwareAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGipGameControllerProvider as ::windows::core::Interface>::IID
    }
}
pub trait IHidGameControllerInputSinkImpl: Sized + IGameControllerInputSinkImpl {
    fn OnInputReportReceived(&self, timestamp: u64, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHidGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IHidGameControllerInputSink";
}
impl IHidGameControllerInputSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidGameControllerInputSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidGameControllerInputSinkVtbl {
        unsafe extern "system" fn OnInputReportReceived<Impl: IHidGameControllerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInputReportReceived(timestamp, reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as _)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHidGameControllerInputSink>, ::windows::core::GetTrustLevel, OnInputReportReceived::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidGameControllerProviderImpl: Sized + IGameControllerProviderImpl {
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn GetFeatureReport(&self, reportid: u8, reportbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SendFeatureReport(&self, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SendOutputReport(&self, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IHidGameControllerProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IHidGameControllerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidGameControllerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidGameControllerProviderVtbl {
        unsafe extern "system" fn UsageId<Impl: IHidGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsagePage<Impl: IHidGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsagePage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeatureReport<Impl: IHidGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureReport(reportid, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn SendFeatureReport<Impl: IHidGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendFeatureReport(reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn SendOutputReport<Impl: IHidGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOutputReport(reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as _)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHidGameControllerProvider>, ::windows::core::GetTrustLevel, UsageId::<Impl, IMPL_OFFSET>, UsagePage::<Impl, IMPL_OFFSET>, GetFeatureReport::<Impl, IMPL_OFFSET>, SendFeatureReport::<Impl, IMPL_OFFSET>, SendOutputReport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidGameControllerProvider as ::windows::core::Interface>::IID
    }
}
pub trait IXusbGameControllerInputSinkImpl: Sized + IGameControllerInputSinkImpl {
    fn OnInputReceived(&self, timestamp: u64, reportid: u8, inputbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXusbGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IXusbGameControllerInputSink";
}
impl IXusbGameControllerInputSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXusbGameControllerInputSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXusbGameControllerInputSinkVtbl {
        unsafe extern "system" fn OnInputReceived<Impl: IXusbGameControllerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInputReceived(timestamp, reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&inputbuffer), inputBuffer_array_size as _)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXusbGameControllerInputSink>, ::windows::core::GetTrustLevel, OnInputReceived::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXusbGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXusbGameControllerProviderImpl: Sized + IGameControllerProviderImpl {
    fn SetVibration(&self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXusbGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IXusbGameControllerProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IXusbGameControllerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXusbGameControllerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXusbGameControllerProviderVtbl {
        unsafe extern "system" fn SetVibration<Impl: IXusbGameControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVibration(lowfrequencymotorspeed, highfrequencymotorspeed).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXusbGameControllerProvider>, ::windows::core::GetTrustLevel, SetVibration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXusbGameControllerProvider as ::windows::core::Interface>::IID
    }
}
