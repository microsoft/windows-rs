#[cfg(feature = "implement_exclusive")]
pub trait IDataProtectionProviderImpl: Sized {
    fn ProtectAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
    fn UnprotectAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
    fn ProtectStreamAsync(&self, src: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, dest: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn UnprotectStreamAsync(&self, src: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, dest: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataProtectionProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.DataProtection.IDataProtectionProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IDataProtectionProviderVtbl {
    pub const fn new<Impl: IDataProtectionProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataProtectionProviderVtbl {
        unsafe extern "system" fn ProtectAsync<Impl: IDataProtectionProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProtectAsync(&*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprotectAsync<Impl: IDataProtectionProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnprotectAsync(&*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectStreamAsync<Impl: IDataProtectionProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, src: ::windows::core::RawPtr, dest: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProtectStreamAsync(&*(&src as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType), &*(&dest as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprotectStreamAsync<Impl: IDataProtectionProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, src: ::windows::core::RawPtr, dest: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnprotectStreamAsync(&*(&src as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType), &*(&dest as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataProtectionProvider>, base.5, ProtectAsync::<Impl, OFFSET>, UnprotectAsync::<Impl, OFFSET>, ProtectStreamAsync::<Impl, OFFSET>, UnprotectStreamAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProtectionProviderFactoryImpl: Sized {
    fn CreateOverloadExplicit(&self, protectiondescriptor: &::windows::core::HSTRING) -> ::windows::core::Result<DataProtectionProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataProtectionProviderFactory {
    const NAME: &'static str = "Windows.Security.Cryptography.DataProtection.IDataProtectionProviderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDataProtectionProviderFactoryVtbl {
    pub const fn new<Impl: IDataProtectionProviderFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataProtectionProviderFactoryVtbl {
        unsafe extern "system" fn CreateOverloadExplicit<Impl: IDataProtectionProviderFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, protectiondescriptor: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateOverloadExplicit(&*(&protectiondescriptor as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataProtectionProviderFactory>, base.5, CreateOverloadExplicit::<Impl, OFFSET>)
    }
}
