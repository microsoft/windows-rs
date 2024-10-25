#[cfg(all(feature = "Storage_Streams", feature = "Web_Http_Headers"))]
pub trait IHttpContent_Impl: Sized + windows_core::IUnknownImpl + super::super::Foundation::IClosable_Impl {
    fn Headers(&self) -> windows_core::Result<Headers::HttpContentHeaderCollection>;
    fn BufferAllAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn ReadAsBufferAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>;
    fn ReadAsInputStreamAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>;
    fn ReadAsStringAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<windows_core::HSTRING, u64>>;
    fn TryComputeLength(&self, length: &mut u64) -> windows_core::Result<bool>;
    fn WriteToStreamAsync(&self, outputstream: Option<&super::super::Storage::Streams::IOutputStream>) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
}
#[cfg(all(feature = "Storage_Streams", feature = "Web_Http_Headers"))]
impl windows_core::RuntimeName for IHttpContent {
    const NAME: &'static str = "Windows.Web.Http.IHttpContent";
}
#[cfg(all(feature = "Storage_Streams", feature = "Web_Http_Headers"))]
impl IHttpContent_Vtbl {
    pub const fn new<Identity: IHttpContent_Impl, const OFFSET: isize>() -> IHttpContent_Vtbl {
        unsafe extern "system" fn Headers<Identity: IHttpContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpContent_Impl::Headers(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferAllAsync<Identity: IHttpContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpContent_Impl::BufferAllAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsBufferAsync<Identity: IHttpContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpContent_Impl::ReadAsBufferAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsInputStreamAsync<Identity: IHttpContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpContent_Impl::ReadAsInputStreamAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsStringAsync<Identity: IHttpContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpContent_Impl::ReadAsStringAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryComputeLength<Identity: IHttpContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut u64, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpContent_Impl::TryComputeLength(this, core::mem::transmute_copy(&length)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToStreamAsync<Identity: IHttpContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpContent_Impl::WriteToStreamAsync(this, windows_core::from_raw_borrowed(&outputstream)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHttpContent, OFFSET>(),
            Headers: Headers::<Identity, OFFSET>,
            BufferAllAsync: BufferAllAsync::<Identity, OFFSET>,
            ReadAsBufferAsync: ReadAsBufferAsync::<Identity, OFFSET>,
            ReadAsInputStreamAsync: ReadAsInputStreamAsync::<Identity, OFFSET>,
            ReadAsStringAsync: ReadAsStringAsync::<Identity, OFFSET>,
            TryComputeLength: TryComputeLength::<Identity, OFFSET>,
            WriteToStreamAsync: WriteToStreamAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpContent as windows_core::Interface>::IID
    }
}
