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
