pub trait ICustomGameControllerFactory_Impl: Sized {
    fn CreateGameController(&mut self, provider: &::core::option::Option<IGameControllerProvider>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn OnGameControllerAdded(&mut self, value: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<()>;
    fn OnGameControllerRemoved(&mut self, value: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICustomGameControllerFactory {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.ICustomGameControllerFactory";
}
impl ICustomGameControllerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomGameControllerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomGameControllerFactory_Vtbl {
        unsafe extern "system" fn CreateGameController<Impl: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OnGameControllerAdded<Impl: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGameControllerAdded(&*(&value as *const <super::IGameController as ::windows::core::Abi>::Abi as *const <super::IGameController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnGameControllerRemoved<Impl: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGameControllerRemoved(&*(&value as *const <super::IGameController as ::windows::core::Abi>::Abi as *const <super::IGameController as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomGameControllerFactory, BASE_OFFSET>(),
            CreateGameController: CreateGameController::<Impl, IMPL_OFFSET>,
            OnGameControllerAdded: OnGameControllerAdded::<Impl, IMPL_OFFSET>,
            OnGameControllerRemoved: OnGameControllerRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomGameControllerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameControllerFactoryManagerStatics_Impl: Sized {
    fn RegisterCustomFactoryForGipInterface(&mut self, factory: &::core::option::Option<ICustomGameControllerFactory>, interfaceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterCustomFactoryForHardwareId(&mut self, factory: &::core::option::Option<ICustomGameControllerFactory>, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::Result<()>;
    fn RegisterCustomFactoryForXusbType(&mut self, factory: &::core::option::Option<ICustomGameControllerFactory>, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameControllerFactoryManagerStatics {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGameControllerFactoryManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerFactoryManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerFactoryManagerStatics_Vtbl {
        unsafe extern "system" fn RegisterCustomFactoryForGipInterface<Impl: IGameControllerFactoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, interfaceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCustomFactoryForGipInterface(&*(&factory as *const <ICustomGameControllerFactory as ::windows::core::Abi>::Abi as *const <ICustomGameControllerFactory as ::windows::core::DefaultType>::DefaultType), &*(&interfaceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegisterCustomFactoryForHardwareId<Impl: IGameControllerFactoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCustomFactoryForHardwareId(&*(&factory as *const <ICustomGameControllerFactory as ::windows::core::Abi>::Abi as *const <ICustomGameControllerFactory as ::windows::core::DefaultType>::DefaultType), hardwarevendorid, hardwareproductid).into()
        }
        unsafe extern "system" fn RegisterCustomFactoryForXusbType<Impl: IGameControllerFactoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCustomFactoryForXusbType(&*(&factory as *const <ICustomGameControllerFactory as ::windows::core::Abi>::Abi as *const <ICustomGameControllerFactory as ::windows::core::DefaultType>::DefaultType), xusbtype, xusbsubtype).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameControllerFactoryManagerStatics, BASE_OFFSET>(),
            RegisterCustomFactoryForGipInterface: RegisterCustomFactoryForGipInterface::<Impl, IMPL_OFFSET>,
            RegisterCustomFactoryForHardwareId: RegisterCustomFactoryForHardwareId::<Impl, IMPL_OFFSET>,
            RegisterCustomFactoryForXusbType: RegisterCustomFactoryForXusbType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerFactoryManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameControllerFactoryManagerStatics2_Impl: Sized + IGameControllerFactoryManagerStatics_Impl {
    fn TryGetFactoryControllerFromGameController(&mut self, factory: &::core::option::Option<ICustomGameControllerFactory>, gamecontroller: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<super::IGameController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGameControllerFactoryManagerStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGameControllerFactoryManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerFactoryManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerFactoryManagerStatics2_Vtbl {
        unsafe extern "system" fn TryGetFactoryControllerFromGameController<Impl: IGameControllerFactoryManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameControllerFactoryManagerStatics2, BASE_OFFSET>(),
            TryGetFactoryControllerFromGameController: TryGetFactoryControllerFromGameController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerFactoryManagerStatics2 as ::windows::core::Interface>::IID
    }
}
pub trait IGameControllerInputSink_Impl: Sized {
    fn OnInputResumed(&mut self, timestamp: u64) -> ::windows::core::Result<()>;
    fn OnInputSuspended(&mut self, timestamp: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerInputSink";
}
impl IGameControllerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerInputSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnInputResumed<Impl: IGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInputResumed(timestamp).into()
        }
        unsafe extern "system" fn OnInputSuspended<Impl: IGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInputSuspended(timestamp).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameControllerInputSink, BASE_OFFSET>(),
            OnInputResumed: OnInputResumed::<Impl, IMPL_OFFSET>,
            OnInputSuspended: OnInputSuspended::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
pub trait IGameControllerProvider_Impl: Sized {
    fn FirmwareVersionInfo(&mut self) -> ::windows::core::Result<GameControllerVersionInfo>;
    fn HardwareProductId(&mut self) -> ::windows::core::Result<u16>;
    fn HardwareVendorId(&mut self) -> ::windows::core::Result<u16>;
    fn HardwareVersionInfo(&mut self) -> ::windows::core::Result<GameControllerVersionInfo>;
    fn IsConnected(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerProvider";
}
impl IGameControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerProvider_Vtbl {
        unsafe extern "system" fn FirmwareVersionInfo<Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HardwareProductId<Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HardwareVendorId<Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HardwareVersionInfo<Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsConnected<Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameControllerProvider, BASE_OFFSET>(),
            FirmwareVersionInfo: FirmwareVersionInfo::<Impl, IMPL_OFFSET>,
            HardwareProductId: HardwareProductId::<Impl, IMPL_OFFSET>,
            HardwareVendorId: HardwareVendorId::<Impl, IMPL_OFFSET>,
            HardwareVersionInfo: HardwareVersionInfo::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGipFirmwareUpdateResult_Impl: Sized {
    fn ExtendedErrorCode(&mut self) -> ::windows::core::Result<u32>;
    fn FinalComponentId(&mut self) -> ::windows::core::Result<u32>;
    fn Status(&mut self) -> ::windows::core::Result<GipFirmwareUpdateStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGipFirmwareUpdateResult {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGipFirmwareUpdateResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGipFirmwareUpdateResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGipFirmwareUpdateResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGipFirmwareUpdateResult_Vtbl {
        unsafe extern "system" fn ExtendedErrorCode<Impl: IGipFirmwareUpdateResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FinalComponentId<Impl: IGipFirmwareUpdateResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Status<Impl: IGipFirmwareUpdateResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GipFirmwareUpdateStatus) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGipFirmwareUpdateResult, BASE_OFFSET>(),
            ExtendedErrorCode: ExtendedErrorCode::<Impl, IMPL_OFFSET>,
            FinalComponentId: FinalComponentId::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGipFirmwareUpdateResult as ::windows::core::Interface>::IID
    }
}
pub trait IGipGameControllerInputSink_Impl: Sized + IGameControllerInputSink_Impl {
    fn OnKeyReceived(&mut self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::Result<()>;
    fn OnMessageReceived(&mut self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGipGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGipGameControllerInputSink";
}
impl IGipGameControllerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGipGameControllerInputSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGipGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnKeyReceived<Impl: IGipGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKeyReceived(timestamp, keycode, ispressed).into()
        }
        unsafe extern "system" fn OnMessageReceived<Impl: IGipGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMessageReceived(timestamp, messageclass, messageid, sequenceid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&messagebuffer), messageBuffer_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGipGameControllerInputSink, BASE_OFFSET>(),
            OnKeyReceived: OnKeyReceived::<Impl, IMPL_OFFSET>,
            OnMessageReceived: OnMessageReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGipGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGipGameControllerProvider_Impl: Sized + IGameControllerProvider_Impl {
    fn SendMessage(&mut self, messageclass: GipMessageClass, messageid: u8, messagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SendReceiveMessage(&mut self, messageclass: GipMessageClass, messageid: u8, requestmessagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], responsemessagebuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UpdateFirmwareAsync(&mut self, firmwareimage: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGipGameControllerProvider";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGipGameControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGipGameControllerProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGipGameControllerProvider_Vtbl {
        unsafe extern "system" fn SendMessage<Impl: IGipGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessage(messageclass, messageid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&messagebuffer), messageBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn SendReceiveMessage<Impl: IGipGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, requestMessageBuffer_array_size: u32, requestmessagebuffer: *const u8, responseMessageBuffer_array_size: u32, responsemessagebuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendReceiveMessage(messageclass, messageid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&requestmessagebuffer), requestMessageBuffer_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&responsemessagebuffer), responseMessageBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn UpdateFirmwareAsync<Impl: IGipGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firmwareimage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGipGameControllerProvider, BASE_OFFSET>(),
            SendMessage: SendMessage::<Impl, IMPL_OFFSET>,
            SendReceiveMessage: SendReceiveMessage::<Impl, IMPL_OFFSET>,
            UpdateFirmwareAsync: UpdateFirmwareAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGipGameControllerProvider as ::windows::core::Interface>::IID
    }
}
pub trait IHidGameControllerInputSink_Impl: Sized + IGameControllerInputSink_Impl {
    fn OnInputReportReceived(&mut self, timestamp: u64, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHidGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IHidGameControllerInputSink";
}
impl IHidGameControllerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidGameControllerInputSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnInputReportReceived<Impl: IHidGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInputReportReceived(timestamp, reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidGameControllerInputSink, BASE_OFFSET>(),
            OnInputReportReceived: OnInputReportReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidGameControllerProvider_Impl: Sized + IGameControllerProvider_Impl {
    fn UsageId(&mut self) -> ::windows::core::Result<u16>;
    fn UsagePage(&mut self) -> ::windows::core::Result<u16>;
    fn GetFeatureReport(&mut self, reportid: u8, reportbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SendFeatureReport(&mut self, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SendOutputReport(&mut self, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHidGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IHidGameControllerProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IHidGameControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidGameControllerProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHidGameControllerProvider_Vtbl {
        unsafe extern "system" fn UsageId<Impl: IHidGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsagePage<Impl: IHidGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFeatureReport<Impl: IHidGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeatureReport(reportid, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn SendFeatureReport<Impl: IHidGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendFeatureReport(reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as _)).into()
        }
        unsafe extern "system" fn SendOutputReport<Impl: IHidGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOutputReport(reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidGameControllerProvider, BASE_OFFSET>(),
            UsageId: UsageId::<Impl, IMPL_OFFSET>,
            UsagePage: UsagePage::<Impl, IMPL_OFFSET>,
            GetFeatureReport: GetFeatureReport::<Impl, IMPL_OFFSET>,
            SendFeatureReport: SendFeatureReport::<Impl, IMPL_OFFSET>,
            SendOutputReport: SendOutputReport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidGameControllerProvider as ::windows::core::Interface>::IID
    }
}
pub trait IXusbGameControllerInputSink_Impl: Sized + IGameControllerInputSink_Impl {
    fn OnInputReceived(&mut self, timestamp: u64, reportid: u8, inputbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXusbGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IXusbGameControllerInputSink";
}
impl IXusbGameControllerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXusbGameControllerInputSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXusbGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnInputReceived<Impl: IXusbGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInputReceived(timestamp, reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&inputbuffer), inputBuffer_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXusbGameControllerInputSink, BASE_OFFSET>(),
            OnInputReceived: OnInputReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXusbGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXusbGameControllerProvider_Impl: Sized + IGameControllerProvider_Impl {
    fn SetVibration(&mut self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXusbGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IXusbGameControllerProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IXusbGameControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXusbGameControllerProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXusbGameControllerProvider_Vtbl {
        unsafe extern "system" fn SetVibration<Impl: IXusbGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVibration(lowfrequencymotorspeed, highfrequencymotorspeed).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXusbGameControllerProvider, BASE_OFFSET>(),
            SetVibration: SetVibration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXusbGameControllerProvider as ::windows::core::Interface>::IID
    }
}
