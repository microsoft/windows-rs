#[cfg(feature = "implement_exclusive")]
pub trait ICredentialFactoryImpl: Sized {
    fn CreatePasswordCredential(&self, resource: &::windows::core::HSTRING, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING) -> ::windows::core::Result<PasswordCredential>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICredentialFactory {
    const NAME: &'static str = "Windows.Security.Credentials.ICredentialFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICredentialFactoryVtbl {
    pub const fn new<Impl: ICredentialFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICredentialFactoryVtbl {
        unsafe extern "system" fn CreatePasswordCredential<Impl: ICredentialFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePasswordCredential(
                &*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICredentialFactory>, base.5, CreatePasswordCredential::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RetrievePublicKeyWithDefaultBlobType(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn RetrievePublicKeyWithBlobType(&self, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn RequestSignAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>>;
    fn GetAttestationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyCredential {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredential";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyCredentialVtbl {
    pub const fn new<Impl: IKeyCredentialImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyCredentialVtbl {
        unsafe extern "system" fn Name<Impl: IKeyCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrievePublicKeyWithDefaultBlobType<Impl: IKeyCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetrievePublicKeyWithDefaultBlobType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrievePublicKeyWithBlobType<Impl: IKeyCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetrievePublicKeyWithBlobType(blobtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSignAsync<Impl: IKeyCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestSignAsync(&*(&data as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttestationAsync<Impl: IKeyCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttestationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyCredential>, base.5, Name::<Impl, OFFSET>, RetrievePublicKeyWithDefaultBlobType::<Impl, OFFSET>, RetrievePublicKeyWithBlobType::<Impl, OFFSET>, RequestSignAsync::<Impl, OFFSET>, GetAttestationAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialAttestationResultImpl: Sized {
    fn CertificateChainBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AttestationBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Status(&self) -> ::windows::core::Result<KeyCredentialAttestationStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyCredentialAttestationResult {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredentialAttestationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyCredentialAttestationResultVtbl {
    pub const fn new<Impl: IKeyCredentialAttestationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyCredentialAttestationResultVtbl {
        unsafe extern "system" fn CertificateChainBuffer<Impl: IKeyCredentialAttestationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CertificateChainBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttestationBuffer<Impl: IKeyCredentialAttestationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AttestationBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IKeyCredentialAttestationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialAttestationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyCredentialAttestationResult>, base.5, CertificateChainBuffer::<Impl, OFFSET>, AttestationBuffer::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialManagerStaticsImpl: Sized {
    fn IsSupportedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RenewAttestationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestCreateAsync(&self, name: &::windows::core::HSTRING, option: KeyCredentialCreationOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>;
    fn OpenAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>;
    fn DeleteAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyCredentialManagerStatics {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredentialManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyCredentialManagerStaticsVtbl {
    pub const fn new<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyCredentialManagerStaticsVtbl {
        unsafe extern "system" fn IsSupportedAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupportedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewAttestationAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenewAttestationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCreateAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: KeyCredentialCreationOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCreateAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyCredentialManagerStatics>, base.5, IsSupportedAsync::<Impl, OFFSET>, RenewAttestationAsync::<Impl, OFFSET>, RequestCreateAsync::<Impl, OFFSET>, OpenAsync::<Impl, OFFSET>, DeleteAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialOperationResultImpl: Sized {
    fn Result(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Status(&self) -> ::windows::core::Result<KeyCredentialStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyCredentialOperationResult {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredentialOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyCredentialOperationResultVtbl {
    pub const fn new<Impl: IKeyCredentialOperationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyCredentialOperationResultVtbl {
        unsafe extern "system" fn Result<Impl: IKeyCredentialOperationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IKeyCredentialOperationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyCredentialOperationResult>, base.5, Result::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialRetrievalResultImpl: Sized {
    fn Credential(&self) -> ::windows::core::Result<KeyCredential>;
    fn Status(&self) -> ::windows::core::Result<KeyCredentialStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyCredentialRetrievalResult {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredentialRetrievalResult";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyCredentialRetrievalResultVtbl {
    pub const fn new<Impl: IKeyCredentialRetrievalResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyCredentialRetrievalResultVtbl {
        unsafe extern "system" fn Credential<Impl: IKeyCredentialRetrievalResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Credential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IKeyCredentialRetrievalResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyCredentialRetrievalResult>, base.5, Credential::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordCredentialImpl: Sized {
    fn Resource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetResource(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserName(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassword(&self, password: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RetrievePassword(&self) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPasswordCredential {
    const NAME: &'static str = "Windows.Security.Credentials.IPasswordCredential";
}
#[cfg(feature = "implement_exclusive")]
impl IPasswordCredentialVtbl {
    pub const fn new<Impl: IPasswordCredentialImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPasswordCredentialVtbl {
        unsafe extern "system" fn Resource<Impl: IPasswordCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResource<Impl: IPasswordCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetResource(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserName<Impl: IPasswordCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: IPasswordCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUserName(&*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Password<Impl: IPasswordCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Impl: IPasswordCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPassword(&*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RetrievePassword<Impl: IPasswordCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RetrievePassword().into()
        }
        unsafe extern "system" fn Properties<Impl: IPasswordCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPasswordCredential>, base.5, Resource::<Impl, OFFSET>, SetResource::<Impl, OFFSET>, UserName::<Impl, OFFSET>, SetUserName::<Impl, OFFSET>, Password::<Impl, OFFSET>, SetPassword::<Impl, OFFSET>, RetrievePassword::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordVaultImpl: Sized {
    fn Add(&self, credential: &::core::option::Option<PasswordCredential>) -> ::windows::core::Result<()>;
    fn Remove(&self, credential: &::core::option::Option<PasswordCredential>) -> ::windows::core::Result<()>;
    fn Retrieve(&self, resource: &::windows::core::HSTRING, username: &::windows::core::HSTRING) -> ::windows::core::Result<PasswordCredential>;
    fn FindAllByResource(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>>;
    fn FindAllByUserName(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>>;
    fn RetrieveAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPasswordVault {
    const NAME: &'static str = "Windows.Security.Credentials.IPasswordVault";
}
#[cfg(feature = "implement_exclusive")]
impl IPasswordVaultVtbl {
    pub const fn new<Impl: IPasswordVaultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPasswordVaultVtbl {
        unsafe extern "system" fn Add<Impl: IPasswordVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Add(&*(&credential as *const <PasswordCredential as ::windows::core::Abi>::Abi as *const <PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: IPasswordVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Remove(&*(&credential as *const <PasswordCredential as ::windows::core::Abi>::Abi as *const <PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Retrieve<Impl: IPasswordVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Retrieve(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllByResource<Impl: IPasswordVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllByResource(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllByUserName<Impl: IPasswordVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllByUserName(&*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveAll<Impl: IPasswordVaultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetrieveAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPasswordVault>, base.5, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Retrieve::<Impl, OFFSET>, FindAllByResource::<Impl, OFFSET>, FindAllByUserName::<Impl, OFFSET>, RetrieveAll::<Impl, OFFSET>)
    }
}
pub trait IWebAccountImpl: Sized {
    fn WebAccountProvider(&self) -> ::windows::core::Result<WebAccountProvider>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<WebAccountState>;
}
impl ::windows::core::RuntimeName for IWebAccount {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccount";
}
impl IWebAccountVtbl {
    pub const fn new<Impl: IWebAccountImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountVtbl {
        unsafe extern "system" fn WebAccountProvider<Impl: IWebAccountImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccountProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Impl: IWebAccountImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IWebAccountImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccount>, base.5, WebAccountProvider::<Impl, OFFSET>, UserName::<Impl, OFFSET>, State::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccount2Impl: Sized + IWebAccountImpl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn GetPictureAsync(&self, desizedsize: WebAccountPictureSize) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn SignOutAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SignOutWithClientIdAsync(&self, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccount2 {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccount2";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccount2Vtbl {
    pub const fn new<Impl: IWebAccount2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccount2Vtbl {
        unsafe extern "system" fn Id<Impl: IWebAccount2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IWebAccount2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPictureAsync<Impl: IWebAccount2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desizedsize: WebAccountPictureSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPictureAsync(desizedsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignOutAsync<Impl: IWebAccount2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignOutAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignOutWithClientIdAsync<Impl: IWebAccount2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignOutWithClientIdAsync(&*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccount2>, base.5, Id::<Impl, OFFSET>, Properties::<Impl, OFFSET>, GetPictureAsync::<Impl, OFFSET>, SignOutAsync::<Impl, OFFSET>, SignOutWithClientIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountFactoryImpl: Sized {
    fn CreateWebAccount(&self, webaccountprovider: &::core::option::Option<WebAccountProvider>, username: &::windows::core::HSTRING, state: WebAccountState) -> ::windows::core::Result<WebAccount>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountFactory {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountFactoryVtbl {
    pub const fn new<Impl: IWebAccountFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountFactoryVtbl {
        unsafe extern "system" fn CreateWebAccount<Impl: IWebAccountFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccountprovider: ::windows::core::RawPtr, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: WebAccountState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWebAccount(&*(&webaccountprovider as *const <WebAccountProvider as ::windows::core::Abi>::Abi as *const <WebAccountProvider as ::windows::core::DefaultType>::DefaultType), &*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountFactory>, base.5, CreateWebAccount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProvider {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderVtbl {
    pub const fn new<Impl: IWebAccountProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderVtbl {
        unsafe extern "system" fn Id<Impl: IWebAccountProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IWebAccountProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconUri<Impl: IWebAccountProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IconUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProvider>, base.5, Id::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, IconUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProvider2Impl: Sized + IWebAccountProviderImpl {
    fn DisplayPurpose(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Authority(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProvider2 {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProvider2";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProvider2Vtbl {
    pub const fn new<Impl: IWebAccountProvider2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProvider2Vtbl {
        unsafe extern "system" fn DisplayPurpose<Impl: IWebAccountProvider2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayPurpose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authority<Impl: IWebAccountProvider2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Authority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProvider2>, base.5, DisplayPurpose::<Impl, OFFSET>, Authority::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProvider3Impl: Sized + IWebAccountProviderImpl + IWebAccountProvider2Impl {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProvider3 {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProvider3";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProvider3Vtbl {
    pub const fn new<Impl: IWebAccountProvider3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProvider3Vtbl {
        unsafe extern "system" fn User<Impl: IWebAccountProvider3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProvider3>, base.5, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProvider4Impl: Sized {
    fn IsSystemProvider(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProvider4 {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProvider4";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProvider4Vtbl {
    pub const fn new<Impl: IWebAccountProvider4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProvider4Vtbl {
        unsafe extern "system" fn IsSystemProvider<Impl: IWebAccountProvider4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSystemProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProvider4>, base.5, IsSystemProvider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderFactoryImpl: Sized {
    fn CreateWebAccountProvider(&self, id: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, iconuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<WebAccountProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderFactory {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProviderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderFactoryVtbl {
    pub const fn new<Impl: IWebAccountProviderFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderFactoryVtbl {
        unsafe extern "system" fn CreateWebAccountProvider<Impl: IWebAccountProviderFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWebAccountProvider(
                &*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&iconuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderFactory>, base.5, CreateWebAccountProvider::<Impl, OFFSET>)
    }
}
