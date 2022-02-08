#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http_Headers"))]
pub trait IHttpContent_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn Headers(&self) -> ::windows::core::Result<Headers::HttpContentHeaderCollection>;
    fn BufferAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn ReadAsBufferAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>;
    fn ReadAsInputStreamAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>;
    fn ReadAsStringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows::core::HSTRING, u64>>;
    fn TryComputeLength(&self, length: &mut u64) -> ::windows::core::Result<bool>;
    fn WriteToStreamAsync(&self, outputstream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http_Headers"))]
impl ::windows::core::RuntimeName for IHttpContent {
    const NAME: &'static str = "Windows.Web.Http.IHttpContent";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http_Headers"))]
impl IHttpContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContent_Impl, const OFFSET: isize>() -> IHttpContent_Vtbl {
        unsafe extern "system" fn Headers<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Headers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferAllAsync<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsBufferAsync<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadAsBufferAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsInputStreamAsync<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadAsInputStreamAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAsStringAsync<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadAsStringAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryComputeLength<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut u64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TryComputeLength(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToStreamAsync<Identity: ::windows::core::IUnknownImpl, Impl: IHttpContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WriteToStreamAsync(::core::mem::transmute(&outputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHttpContent, OFFSET>(),
            Headers: Headers::<Identity, Impl, OFFSET>,
            BufferAllAsync: BufferAllAsync::<Identity, Impl, OFFSET>,
            ReadAsBufferAsync: ReadAsBufferAsync::<Identity, Impl, OFFSET>,
            ReadAsInputStreamAsync: ReadAsInputStreamAsync::<Identity, Impl, OFFSET>,
            ReadAsStringAsync: ReadAsStringAsync::<Identity, Impl, OFFSET>,
            TryComputeLength: TryComputeLength::<Identity, Impl, OFFSET>,
            WriteToStreamAsync: WriteToStreamAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpContent as ::windows::core::Interface>::IID
    }
}
