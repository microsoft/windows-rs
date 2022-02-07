pub trait ICustomGameControllerFactory_Impl: Sized {
    fn CreateGameController(&self, provider: &::core::option::Option<IGameControllerProvider>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn OnGameControllerAdded(&self, value: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<()>;
    fn OnGameControllerRemoved(&self, value: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICustomGameControllerFactory {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.ICustomGameControllerFactory";
}
impl ICustomGameControllerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomGameControllerFactory_Impl, const OFFSET: isize>() -> ICustomGameControllerFactory_Vtbl {
        unsafe extern "system" fn CreateGameController<Identity: ::windows::core::IUnknownImpl, Impl: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateGameController(::core::mem::transmute(&provider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnGameControllerAdded<Identity: ::windows::core::IUnknownImpl, Impl: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnGameControllerAdded(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn OnGameControllerRemoved<Identity: ::windows::core::IUnknownImpl, Impl: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnGameControllerRemoved(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomGameControllerFactory, OFFSET>(),
            CreateGameController: CreateGameController::<Identity, Impl, OFFSET>,
            OnGameControllerAdded: OnGameControllerAdded::<Identity, Impl, OFFSET>,
            OnGameControllerRemoved: OnGameControllerRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomGameControllerFactory as ::windows::core::Interface>::IID
    }
}
pub trait IGameControllerInputSink_Impl: Sized {
    fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()>;
    fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerInputSink";
}
impl IGameControllerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerInputSink_Impl, const OFFSET: isize>() -> IGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnInputResumed<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInputResumed(timestamp).into()
        }
        unsafe extern "system" fn OnInputSuspended<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInputSuspended(timestamp).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameControllerInputSink, OFFSET>(),
            OnInputResumed: OnInputResumed::<Identity, Impl, OFFSET>,
            OnInputSuspended: OnInputSuspended::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
pub trait IGameControllerProvider_Impl: Sized {
    fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo>;
    fn HardwareProductId(&self) -> ::windows::core::Result<u16>;
    fn HardwareVendorId(&self) -> ::windows::core::Result<u16>;
    fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo>;
    fn IsConnected(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerProvider";
}
impl IGameControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerProvider_Impl, const OFFSET: isize>() -> IGameControllerProvider_Vtbl {
        unsafe extern "system" fn FirmwareVersionInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirmwareVersionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareProductId<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HardwareProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareVendorId<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HardwareVendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareVersionInfo<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HardwareVersionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameControllerProvider, OFFSET>(),
            FirmwareVersionInfo: FirmwareVersionInfo::<Identity, Impl, OFFSET>,
            HardwareProductId: HardwareProductId::<Identity, Impl, OFFSET>,
            HardwareVendorId: HardwareVendorId::<Identity, Impl, OFFSET>,
            HardwareVersionInfo: HardwareVersionInfo::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerProvider as ::windows::core::Interface>::IID
    }
}
pub trait IGipGameControllerInputSink_Impl: Sized + IGameControllerInputSink_Impl {
    fn OnKeyReceived(&self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::Result<()>;
    fn OnMessageReceived(&self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[u8]) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGipGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGipGameControllerInputSink";
}
impl IGipGameControllerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGipGameControllerInputSink_Impl, const OFFSET: isize>() -> IGipGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnKeyReceived<Identity: ::windows::core::IUnknownImpl, Impl: IGipGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnKeyReceived(timestamp, keycode, ispressed).into()
        }
        unsafe extern "system" fn OnMessageReceived<Identity: ::windows::core::IUnknownImpl, Impl: IGipGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnMessageReceived(timestamp, messageclass, messageid, sequenceid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&messagebuffer), messageBuffer_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGipGameControllerInputSink, OFFSET>(),
            OnKeyReceived: OnKeyReceived::<Identity, Impl, OFFSET>,
            OnMessageReceived: OnMessageReceived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGipGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
pub trait IHidGameControllerInputSink_Impl: Sized + IGameControllerInputSink_Impl {
    fn OnInputReportReceived(&self, timestamp: u64, reportid: u8, reportbuffer: &[u8]) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHidGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IHidGameControllerInputSink";
}
impl IHidGameControllerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHidGameControllerInputSink_Impl, const OFFSET: isize>() -> IHidGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnInputReportReceived<Identity: ::windows::core::IUnknownImpl, Impl: IHidGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInputReportReceived(timestamp, reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHidGameControllerInputSink, OFFSET>(),
            OnInputReportReceived: OnInputReportReceived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHidGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
pub trait IXusbGameControllerInputSink_Impl: Sized + IGameControllerInputSink_Impl {
    fn OnInputReceived(&self, timestamp: u64, reportid: u8, inputbuffer: &[u8]) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXusbGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IXusbGameControllerInputSink";
}
impl IXusbGameControllerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXusbGameControllerInputSink_Impl, const OFFSET: isize>() -> IXusbGameControllerInputSink_Vtbl {
        unsafe extern "system" fn OnInputReceived<Identity: ::windows::core::IUnknownImpl, Impl: IXusbGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInputReceived(timestamp, reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&inputbuffer), inputBuffer_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXusbGameControllerInputSink, OFFSET>(),
            OnInputReceived: OnInputReceived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXusbGameControllerInputSink as ::windows::core::Interface>::IID
    }
}
