#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Http_Headers\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http_Headers"))]
pub trait IHttpContent_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn Headers(&self) -> ::windows_core::Result<Headers::HttpContentHeaderCollection>;
    fn BufferAllAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn ReadAsBufferAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>;
    fn ReadAsInputStreamAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>;
    fn ReadAsStringAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>>;
    fn TryComputeLength(&self, length: &mut u64) -> ::windows_core::Result<bool>;
    fn WriteToStreamAsync(&self, outputstream: ::core::option::Option<&super::super::Storage::Streams::IOutputStream>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http_Headers"))]
impl ::windows_core::RuntimeName for IHttpContent {
    const NAME: &'static str = "Windows.Web.Http.IHttpContent";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http_Headers"))]
impl IHttpContent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: isize>() -> IHttpContent_Vtbl {
        unsafe extern "system" fn Headers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Headers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferAllAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BufferAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsBufferAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadAsBufferAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsInputStreamAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadAsInputStreamAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsStringAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadAsStringAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryComputeLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut u64, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryComputeLength(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToStreamAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteToStreamAsync(::windows_core::from_raw_borrowed(&outputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IHttpContent, OFFSET>(),
            Headers: Headers::<Identity, Impl, OFFSET>,
            BufferAllAsync: BufferAllAsync::<Identity, Impl, OFFSET>,
            ReadAsBufferAsync: ReadAsBufferAsync::<Identity, Impl, OFFSET>,
            ReadAsInputStreamAsync: ReadAsInputStreamAsync::<Identity, Impl, OFFSET>,
            ReadAsStringAsync: ReadAsStringAsync::<Identity, Impl, OFFSET>,
            TryComputeLength: TryComputeLength::<Identity, Impl, OFFSET>,
            WriteToStreamAsync: WriteToStreamAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IHttpContent as ::windows_core::ComInterface>::IID
    }
}
