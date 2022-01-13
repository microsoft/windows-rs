pub trait ICreateDeviceAccessAsyncImpl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Wait(&mut self, timeout: u32) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn GetResult(&mut self, riid: *const ::windows::core::GUID, deviceaccess: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ICreateDeviceAccessAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateDeviceAccessAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateDeviceAccessAsyncVtbl {
        unsafe extern "system" fn Cancel<Impl: ICreateDeviceAccessAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Wait<Impl: ICreateDeviceAccessAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Wait(::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn Close<Impl: ICreateDeviceAccessAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetResult<Impl: ICreateDeviceAccessAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, deviceaccess: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResult(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&deviceaccess)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Wait: Wait::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetResult: GetResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateDeviceAccessAsync as ::windows::core::Interface>::IID
    }
}
pub trait IDeviceIoControlImpl: Sized {
    fn DeviceIoControlSync(&mut self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn DeviceIoControlAsync(&mut self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: ::core::option::Option<IDeviceRequestCompletionCallback>, cancelcontext: *mut usize) -> ::windows::core::Result<()>;
    fn CancelOperation(&mut self, cancelcontext: usize) -> ::windows::core::Result<()>;
}
impl IDeviceIoControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceIoControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceIoControlVtbl {
        unsafe extern "system" fn DeviceIoControlSync<Impl: IDeviceIoControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceIoControlSync(::core::mem::transmute_copy(&iocontrolcode), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&inputbuffersize), ::core::mem::transmute_copy(&outputbuffer), ::core::mem::transmute_copy(&outputbuffersize), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn DeviceIoControlAsync<Impl: IDeviceIoControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: ::windows::core::RawPtr, cancelcontext: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceIoControlAsync(::core::mem::transmute_copy(&iocontrolcode), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&inputbuffersize), ::core::mem::transmute_copy(&outputbuffer), ::core::mem::transmute_copy(&outputbuffersize), ::core::mem::transmute(&requestcompletioncallback), ::core::mem::transmute_copy(&cancelcontext)).into()
        }
        unsafe extern "system" fn CancelOperation<Impl: IDeviceIoControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancelcontext: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelOperation(::core::mem::transmute_copy(&cancelcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DeviceIoControlSync: DeviceIoControlSync::<Impl, IMPL_OFFSET>,
            DeviceIoControlAsync: DeviceIoControlAsync::<Impl, IMPL_OFFSET>,
            CancelOperation: CancelOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceIoControl as ::windows::core::Interface>::IID
    }
}
pub trait IDeviceRequestCompletionCallbackImpl: Sized {
    fn Invoke(&mut self, requestresult: ::windows::core::HRESULT, bytesreturned: u32) -> ::windows::core::Result<()>;
}
impl IDeviceRequestCompletionCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceRequestCompletionCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceRequestCompletionCallbackVtbl {
        unsafe extern "system" fn Invoke<Impl: IDeviceRequestCompletionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestresult: ::windows::core::HRESULT, bytesreturned: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke(::core::mem::transmute_copy(&requestresult), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceRequestCompletionCallback as ::windows::core::Interface>::IID
    }
}
