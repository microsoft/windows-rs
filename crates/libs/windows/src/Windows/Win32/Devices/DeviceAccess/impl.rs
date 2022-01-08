pub trait ICreateDeviceAccessAsyncImpl: Sized {
    fn Cancel();
    fn Wait();
    fn Close();
    fn GetResult();
}
impl ::windows::core::RuntimeName for ICreateDeviceAccessAsync {
    const NAME: &'static str = "Windows.Win32.Devices.DeviceAccess.ICreateDeviceAccessAsync";
}
impl ICreateDeviceAccessAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateDeviceAccessAsyncImpl, const OFFSET: isize>() -> ICreateDeviceAccessAsyncVtbl {
        unsafe extern "system" fn Cancel<Impl: ICreateDeviceAccessAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wait<Impl: ICreateDeviceAccessAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wait(timeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ICreateDeviceAccessAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResult<Impl: ICreateDeviceAccessAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, deviceaccess: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResult(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&deviceaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICreateDeviceAccessAsync>, ::windows::core::GetTrustLevel, Cancel::<Impl, OFFSET>, Wait::<Impl, OFFSET>, Close::<Impl, OFFSET>, GetResult::<Impl, OFFSET>)
    }
}
pub trait IDeviceIoControlImpl: Sized {
    fn DeviceIoControlSync();
    fn DeviceIoControlAsync();
    fn CancelOperation();
}
impl ::windows::core::RuntimeName for IDeviceIoControl {
    const NAME: &'static str = "Windows.Win32.Devices.DeviceAccess.IDeviceIoControl";
}
impl IDeviceIoControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceIoControlImpl, const OFFSET: isize>() -> IDeviceIoControlVtbl {
        unsafe extern "system" fn DeviceIoControlSync<Impl: IDeviceIoControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceIoControlSync(iocontrolcode, inputbuffer, inputbuffersize, ::core::mem::transmute_copy(&outputbuffer), outputbuffersize, ::core::mem::transmute_copy(&bytesreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControlAsync<Impl: IDeviceIoControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: ::windows::core::RawPtr, cancelcontext: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceIoControlAsync(iocontrolcode, inputbuffer, inputbuffersize, ::core::mem::transmute_copy(&outputbuffer), outputbuffersize, &*(&requestcompletioncallback as *const <IDeviceRequestCompletionCallback as ::windows::core::Abi>::Abi as *const <IDeviceRequestCompletionCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cancelcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelOperation<Impl: IDeviceIoControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancelcontext: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelOperation(cancelcontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceIoControl>, ::windows::core::GetTrustLevel, DeviceIoControlSync::<Impl, OFFSET>, DeviceIoControlAsync::<Impl, OFFSET>, CancelOperation::<Impl, OFFSET>)
    }
}
pub trait IDeviceRequestCompletionCallbackImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for IDeviceRequestCompletionCallback {
    const NAME: &'static str = "Windows.Win32.Devices.DeviceAccess.IDeviceRequestCompletionCallback";
}
impl IDeviceRequestCompletionCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceRequestCompletionCallbackImpl, const OFFSET: isize>() -> IDeviceRequestCompletionCallbackVtbl {
        unsafe extern "system" fn Invoke<Impl: IDeviceRequestCompletionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestresult: ::windows::core::HRESULT, bytesreturned: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke(requestresult, bytesreturned) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceRequestCompletionCallback>, ::windows::core::GetTrustLevel, Invoke::<Impl, OFFSET>)
    }
}
