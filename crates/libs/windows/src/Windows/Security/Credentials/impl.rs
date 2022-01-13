#[cfg(feature = "implement_exclusive")]
pub trait ICredentialFactoryImpl: Sized {
    fn CreatePasswordCredential(&mut self, resource: &::windows::core::HSTRING, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING) -> ::windows::core::Result<PasswordCredential>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICredentialFactory {
    const NAME: &'static str = "Windows.Security.Credentials.ICredentialFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICredentialFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICredentialFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICredentialFactoryVtbl {
        unsafe extern "system" fn CreatePasswordCredential<Impl: ICredentialFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICredentialFactory, BASE_OFFSET>(),
            CreatePasswordCredential: CreatePasswordCredential::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICredentialFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IKeyCredentialImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RetrievePublicKeyWithDefaultBlobType(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn RetrievePublicKeyWithBlobType(&mut self, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn RequestSignAsync(&mut self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>>;
    fn GetAttestationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyCredential {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredential";
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IKeyCredentialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyCredentialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyCredentialVtbl {
        unsafe extern "system" fn Name<Impl: IKeyCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrievePublicKeyWithDefaultBlobType<Impl: IKeyCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrievePublicKeyWithDefaultBlobType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrievePublicKeyWithBlobType<Impl: IKeyCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrievePublicKeyWithBlobType(blobtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestSignAsync<Impl: IKeyCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestSignAsync(&*(&data as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttestationAsync<Impl: IKeyCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttestationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyCredential, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            RetrievePublicKeyWithDefaultBlobType: RetrievePublicKeyWithDefaultBlobType::<Impl, IMPL_OFFSET>,
            RetrievePublicKeyWithBlobType: RetrievePublicKeyWithBlobType::<Impl, IMPL_OFFSET>,
            RequestSignAsync: RequestSignAsync::<Impl, IMPL_OFFSET>,
            GetAttestationAsync: GetAttestationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyCredential as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IKeyCredentialAttestationResultImpl: Sized {
    fn CertificateChainBuffer(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AttestationBuffer(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Status(&mut self) -> ::windows::core::Result<KeyCredentialAttestationStatus>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyCredentialAttestationResult {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredentialAttestationResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IKeyCredentialAttestationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyCredentialAttestationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyCredentialAttestationResultVtbl {
        unsafe extern "system" fn CertificateChainBuffer<Impl: IKeyCredentialAttestationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CertificateChainBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttestationBuffer<Impl: IKeyCredentialAttestationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttestationBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IKeyCredentialAttestationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialAttestationStatus) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyCredentialAttestationResult, BASE_OFFSET>(),
            CertificateChainBuffer: CertificateChainBuffer::<Impl, IMPL_OFFSET>,
            AttestationBuffer: AttestationBuffer::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyCredentialAttestationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IKeyCredentialManagerStaticsImpl: Sized {
    fn IsSupportedAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RenewAttestationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestCreateAsync(&mut self, name: &::windows::core::HSTRING, option: KeyCredentialCreationOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>;
    fn OpenAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>;
    fn DeleteAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyCredentialManagerStatics {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredentialManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IKeyCredentialManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyCredentialManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyCredentialManagerStaticsVtbl {
        unsafe extern "system" fn IsSupportedAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewAttestationAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewAttestationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCreateAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: KeyCredentialCreationOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCreateAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IKeyCredentialManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyCredentialManagerStatics, BASE_OFFSET>(),
            IsSupportedAsync: IsSupportedAsync::<Impl, IMPL_OFFSET>,
            RenewAttestationAsync: RenewAttestationAsync::<Impl, IMPL_OFFSET>,
            RequestCreateAsync: RequestCreateAsync::<Impl, IMPL_OFFSET>,
            OpenAsync: OpenAsync::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyCredentialManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IKeyCredentialOperationResultImpl: Sized {
    fn Result(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Status(&mut self) -> ::windows::core::Result<KeyCredentialStatus>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyCredentialOperationResult {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredentialOperationResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IKeyCredentialOperationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyCredentialOperationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyCredentialOperationResultVtbl {
        unsafe extern "system" fn Result<Impl: IKeyCredentialOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IKeyCredentialOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyCredentialOperationResult, BASE_OFFSET>(),
            Result: Result::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyCredentialOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialRetrievalResultImpl: Sized {
    fn Credential(&mut self) -> ::windows::core::Result<KeyCredential>;
    fn Status(&mut self) -> ::windows::core::Result<KeyCredentialStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyCredentialRetrievalResult {
    const NAME: &'static str = "Windows.Security.Credentials.IKeyCredentialRetrievalResult";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyCredentialRetrievalResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyCredentialRetrievalResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyCredentialRetrievalResultVtbl {
        unsafe extern "system" fn Credential<Impl: IKeyCredentialRetrievalResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Credential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IKeyCredentialRetrievalResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyCredentialRetrievalResult, BASE_OFFSET>(),
            Credential: Credential::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyCredentialRetrievalResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPasswordCredentialImpl: Sized {
    fn Resource(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetResource(&mut self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserName(&mut self, username: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Password(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassword(&mut self, password: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RetrievePassword(&mut self) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPasswordCredential {
    const NAME: &'static str = "Windows.Security.Credentials.IPasswordCredential";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPasswordCredentialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPasswordCredentialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPasswordCredentialVtbl {
        unsafe extern "system" fn Resource<Impl: IPasswordCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResource<Impl: IPasswordCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResource(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserName<Impl: IPasswordCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: IPasswordCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserName(&*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Password<Impl: IPasswordCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Impl: IPasswordCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassword(&*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RetrievePassword<Impl: IPasswordCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetrievePassword().into()
        }
        unsafe extern "system" fn Properties<Impl: IPasswordCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPasswordCredential, BASE_OFFSET>(),
            Resource: Resource::<Impl, IMPL_OFFSET>,
            SetResource: SetResource::<Impl, IMPL_OFFSET>,
            UserName: UserName::<Impl, IMPL_OFFSET>,
            SetUserName: SetUserName::<Impl, IMPL_OFFSET>,
            Password: Password::<Impl, IMPL_OFFSET>,
            SetPassword: SetPassword::<Impl, IMPL_OFFSET>,
            RetrievePassword: RetrievePassword::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPasswordCredential as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPasswordVaultImpl: Sized {
    fn Add(&mut self, credential: &::core::option::Option<PasswordCredential>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, credential: &::core::option::Option<PasswordCredential>) -> ::windows::core::Result<()>;
    fn Retrieve(&mut self, resource: &::windows::core::HSTRING, username: &::windows::core::HSTRING) -> ::windows::core::Result<PasswordCredential>;
    fn FindAllByResource(&mut self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>>;
    fn FindAllByUserName(&mut self, username: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>>;
    fn RetrieveAll(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPasswordVault {
    const NAME: &'static str = "Windows.Security.Credentials.IPasswordVault";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPasswordVaultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPasswordVaultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPasswordVaultVtbl {
        unsafe extern "system" fn Add<Impl: IPasswordVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&credential as *const <PasswordCredential as ::windows::core::Abi>::Abi as *const <PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Remove<Impl: IPasswordVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&credential as *const <PasswordCredential as ::windows::core::Abi>::Abi as *const <PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Retrieve<Impl: IPasswordVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retrieve(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllByResource<Impl: IPasswordVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllByResource(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllByUserName<Impl: IPasswordVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllByUserName(&*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveAll<Impl: IPasswordVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPasswordVault, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Retrieve: Retrieve::<Impl, IMPL_OFFSET>,
            FindAllByResource: FindAllByResource::<Impl, IMPL_OFFSET>,
            FindAllByUserName: FindAllByUserName::<Impl, IMPL_OFFSET>,
            RetrieveAll: RetrieveAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPasswordVault as ::windows::core::Interface>::IID
    }
}
pub trait IWebAccountImpl: Sized {
    fn WebAccountProvider(&mut self) -> ::windows::core::Result<WebAccountProvider>;
    fn UserName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&mut self) -> ::windows::core::Result<WebAccountState>;
}
impl ::windows::core::RuntimeName for IWebAccount {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccount";
}
impl IWebAccountVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountVtbl {
        unsafe extern "system" fn WebAccountProvider<Impl: IWebAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccountProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Impl: IWebAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IWebAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccount, BASE_OFFSET>(),
            WebAccountProvider: WebAccountProvider::<Impl, IMPL_OFFSET>,
            UserName: UserName::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccount as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWebAccount2Impl: Sized + IWebAccountImpl {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn GetPictureAsync(&mut self, desizedsize: WebAccountPictureSize) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn SignOutAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SignOutWithClientIdAsync(&mut self, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccount2 {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccount2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWebAccount2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccount2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccount2Vtbl {
        unsafe extern "system" fn Id<Impl: IWebAccount2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IWebAccount2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPictureAsync<Impl: IWebAccount2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desizedsize: WebAccountPictureSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPictureAsync(desizedsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignOutAsync<Impl: IWebAccount2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignOutAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignOutWithClientIdAsync<Impl: IWebAccount2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignOutWithClientIdAsync(&*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccount2, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            GetPictureAsync: GetPictureAsync::<Impl, IMPL_OFFSET>,
            SignOutAsync: SignOutAsync::<Impl, IMPL_OFFSET>,
            SignOutWithClientIdAsync: SignOutWithClientIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccount2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountFactoryImpl: Sized {
    fn CreateWebAccount(&mut self, webaccountprovider: &::core::option::Option<WebAccountProvider>, username: &::windows::core::HSTRING, state: WebAccountState) -> ::windows::core::Result<WebAccount>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountFactory {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountFactoryVtbl {
        unsafe extern "system" fn CreateWebAccount<Impl: IWebAccountFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountprovider: ::windows::core::RawPtr, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: WebAccountState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWebAccount(&*(&webaccountprovider as *const <WebAccountProvider as ::windows::core::Abi>::Abi as *const <WebAccountProvider as ::windows::core::DefaultType>::DefaultType), &*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountFactory, BASE_OFFSET>(),
            CreateWebAccount: CreateWebAccount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAccountProviderImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IconUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProvider {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProvider";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderVtbl {
        unsafe extern "system" fn Id<Impl: IWebAccountProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IWebAccountProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconUri<Impl: IWebAccountProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProvider, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            IconUri: IconUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAccountProvider2Impl: Sized + IWebAccountProviderImpl {
    fn DisplayPurpose(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Authority(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProvider2 {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProvider2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProvider2Vtbl {
        unsafe extern "system" fn DisplayPurpose<Impl: IWebAccountProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayPurpose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authority<Impl: IWebAccountProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProvider2, BASE_OFFSET>(),
            DisplayPurpose: DisplayPurpose::<Impl, IMPL_OFFSET>,
            Authority: Authority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IWebAccountProvider3Impl: Sized + IWebAccountProviderImpl + IWebAccountProvider2Impl {
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProvider3 {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProvider3";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IWebAccountProvider3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProvider3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProvider3Vtbl {
        unsafe extern "system" fn User<Impl: IWebAccountProvider3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProvider3, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProvider3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProvider4Impl: Sized {
    fn IsSystemProvider(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProvider4 {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProvider4";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProvider4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProvider4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProvider4Vtbl {
        unsafe extern "system" fn IsSystemProvider<Impl: IWebAccountProvider4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSystemProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProvider4, BASE_OFFSET>(),
            IsSystemProvider: IsSystemProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProvider4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebAccountProviderFactoryImpl: Sized {
    fn CreateWebAccountProvider(&mut self, id: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, iconuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<WebAccountProvider>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderFactory {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccountProviderFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebAccountProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderFactoryVtbl {
        unsafe extern "system" fn CreateWebAccountProvider<Impl: IWebAccountProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderFactory, BASE_OFFSET>(),
            CreateWebAccountProvider: CreateWebAccountProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderFactory as ::windows::core::Interface>::IID
    }
}
