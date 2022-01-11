#[cfg(feature = "implement_exclusive")]
pub trait IAsymmetricAlgorithmNamesStaticsImpl: Sized {
    fn RsaPkcs1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaOaepSha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaOaepSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaOaepSha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaOaepSha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EcdsaP256Sha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EcdsaP384Sha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EcdsaP521Sha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DsaSha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DsaSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaSignPkcs1Sha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaSignPkcs1Sha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaSignPkcs1Sha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaSignPkcs1Sha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaSignPssSha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaSignPssSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaSignPssSha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RsaSignPssSha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAsymmetricAlgorithmNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAsymmetricAlgorithmNamesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsymmetricAlgorithmNamesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsymmetricAlgorithmNamesStaticsVtbl {
        unsafe extern "system" fn RsaPkcs1<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaPkcs1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaOaepSha1<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaOaepSha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaOaepSha256<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaOaepSha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaOaepSha384<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaOaepSha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaOaepSha512<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaOaepSha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EcdsaP256Sha256<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EcdsaP256Sha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EcdsaP384Sha384<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EcdsaP384Sha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EcdsaP521Sha512<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EcdsaP521Sha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DsaSha1<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DsaSha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DsaSha256<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DsaSha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaSignPkcs1Sha1<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaSignPkcs1Sha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaSignPkcs1Sha256<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaSignPkcs1Sha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaSignPkcs1Sha384<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaSignPkcs1Sha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaSignPkcs1Sha512<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaSignPkcs1Sha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaSignPssSha1<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaSignPssSha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaSignPssSha256<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaSignPssSha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaSignPssSha384<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaSignPssSha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsaSignPssSha512<Impl: IAsymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RsaSignPssSha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsymmetricAlgorithmNamesStatics, BASE_OFFSET>(),
            RsaPkcs1: RsaPkcs1::<Impl, IMPL_OFFSET>,
            RsaOaepSha1: RsaOaepSha1::<Impl, IMPL_OFFSET>,
            RsaOaepSha256: RsaOaepSha256::<Impl, IMPL_OFFSET>,
            RsaOaepSha384: RsaOaepSha384::<Impl, IMPL_OFFSET>,
            RsaOaepSha512: RsaOaepSha512::<Impl, IMPL_OFFSET>,
            EcdsaP256Sha256: EcdsaP256Sha256::<Impl, IMPL_OFFSET>,
            EcdsaP384Sha384: EcdsaP384Sha384::<Impl, IMPL_OFFSET>,
            EcdsaP521Sha512: EcdsaP521Sha512::<Impl, IMPL_OFFSET>,
            DsaSha1: DsaSha1::<Impl, IMPL_OFFSET>,
            DsaSha256: DsaSha256::<Impl, IMPL_OFFSET>,
            RsaSignPkcs1Sha1: RsaSignPkcs1Sha1::<Impl, IMPL_OFFSET>,
            RsaSignPkcs1Sha256: RsaSignPkcs1Sha256::<Impl, IMPL_OFFSET>,
            RsaSignPkcs1Sha384: RsaSignPkcs1Sha384::<Impl, IMPL_OFFSET>,
            RsaSignPkcs1Sha512: RsaSignPkcs1Sha512::<Impl, IMPL_OFFSET>,
            RsaSignPssSha1: RsaSignPssSha1::<Impl, IMPL_OFFSET>,
            RsaSignPssSha256: RsaSignPssSha256::<Impl, IMPL_OFFSET>,
            RsaSignPssSha384: RsaSignPssSha384::<Impl, IMPL_OFFSET>,
            RsaSignPssSha512: RsaSignPssSha512::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsymmetricAlgorithmNamesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAsymmetricAlgorithmNamesStatics2Impl: Sized {
    fn EcdsaSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EcdsaSha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EcdsaSha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAsymmetricAlgorithmNamesStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IAsymmetricAlgorithmNamesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAsymmetricAlgorithmNamesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsymmetricAlgorithmNamesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsymmetricAlgorithmNamesStatics2Vtbl {
        unsafe extern "system" fn EcdsaSha256<Impl: IAsymmetricAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EcdsaSha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EcdsaSha384<Impl: IAsymmetricAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EcdsaSha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EcdsaSha512<Impl: IAsymmetricAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EcdsaSha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsymmetricAlgorithmNamesStatics2, BASE_OFFSET>(),
            EcdsaSha256: EcdsaSha256::<Impl, IMPL_OFFSET>,
            EcdsaSha384: EcdsaSha384::<Impl, IMPL_OFFSET>,
            EcdsaSha512: EcdsaSha512::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsymmetricAlgorithmNamesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAsymmetricKeyAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateKeyPair(&self, keysize: u32) -> ::windows::core::Result<CryptographicKey>;
    fn ImportDefaultPrivateKeyBlob(&self, keyblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
    fn ImportKeyPairWithBlobType(&self, keyblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, blobtype: CryptographicPrivateKeyBlobType) -> ::windows::core::Result<CryptographicKey>;
    fn ImportDefaultPublicKeyBlob(&self, keyblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
    fn ImportPublicKeyWithBlobType(&self, keyblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, blobtype: CryptographicPublicKeyBlobType) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAsymmetricKeyAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAsymmetricKeyAlgorithmProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsymmetricKeyAlgorithmProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsymmetricKeyAlgorithmProviderVtbl {
        unsafe extern "system" fn AlgorithmName<Impl: IAsymmetricKeyAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKeyPair<Impl: IAsymmetricKeyAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keysize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKeyPair(keysize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportDefaultPrivateKeyBlob<Impl: IAsymmetricKeyAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportDefaultPrivateKeyBlob(&*(&keyblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportKeyPairWithBlobType<Impl: IAsymmetricKeyAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyblob: ::windows::core::RawPtr, blobtype: CryptographicPrivateKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportKeyPairWithBlobType(&*(&keyblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), blobtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportDefaultPublicKeyBlob<Impl: IAsymmetricKeyAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportDefaultPublicKeyBlob(&*(&keyblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportPublicKeyWithBlobType<Impl: IAsymmetricKeyAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keyblob: ::windows::core::RawPtr, blobtype: CryptographicPublicKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportPublicKeyWithBlobType(&*(&keyblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), blobtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsymmetricKeyAlgorithmProvider, BASE_OFFSET>(),
            AlgorithmName: AlgorithmName::<Impl, IMPL_OFFSET>,
            CreateKeyPair: CreateKeyPair::<Impl, IMPL_OFFSET>,
            ImportDefaultPrivateKeyBlob: ImportDefaultPrivateKeyBlob::<Impl, IMPL_OFFSET>,
            ImportKeyPairWithBlobType: ImportKeyPairWithBlobType::<Impl, IMPL_OFFSET>,
            ImportDefaultPublicKeyBlob: ImportDefaultPublicKeyBlob::<Impl, IMPL_OFFSET>,
            ImportPublicKeyWithBlobType: ImportPublicKeyWithBlobType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsymmetricKeyAlgorithmProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAsymmetricKeyAlgorithmProvider2Impl: Sized {
    fn CreateKeyPairWithCurveName(&self, curvename: &::windows::core::HSTRING) -> ::windows::core::Result<CryptographicKey>;
    fn CreateKeyPairWithCurveParameters(&self, parameters: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAsymmetricKeyAlgorithmProvider2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProvider2";
}
#[cfg(feature = "implement_exclusive")]
impl IAsymmetricKeyAlgorithmProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsymmetricKeyAlgorithmProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsymmetricKeyAlgorithmProvider2Vtbl {
        unsafe extern "system" fn CreateKeyPairWithCurveName<Impl: IAsymmetricKeyAlgorithmProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, curvename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKeyPairWithCurveName(&*(&curvename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKeyPairWithCurveParameters<Impl: IAsymmetricKeyAlgorithmProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters_array_size: u32, parameters: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKeyPairWithCurveParameters(::core::slice::from_raw_parts(::core::mem::transmute_copy(&parameters), parameters_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsymmetricKeyAlgorithmProvider2, BASE_OFFSET>(),
            CreateKeyPairWithCurveName: CreateKeyPairWithCurveName::<Impl, IMPL_OFFSET>,
            CreateKeyPairWithCurveParameters: CreateKeyPairWithCurveParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsymmetricKeyAlgorithmProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAsymmetricKeyAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<AsymmetricKeyAlgorithmProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAsymmetricKeyAlgorithmProviderStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IAsymmetricKeyAlgorithmProviderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAsymmetricKeyAlgorithmProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsymmetricKeyAlgorithmProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsymmetricKeyAlgorithmProviderStaticsVtbl {
        unsafe extern "system" fn OpenAlgorithm<Impl: IAsymmetricKeyAlgorithmProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAlgorithm(&*(&algorithm as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsymmetricKeyAlgorithmProviderStatics, BASE_OFFSET>(),
            OpenAlgorithm: OpenAlgorithm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsymmetricKeyAlgorithmProviderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICryptographicEngineStaticsImpl: Sized {
    fn Encrypt(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, iv: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Decrypt(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, iv: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn EncryptAndAuthenticate(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, nonce: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, authenticateddata: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<EncryptedAndAuthenticatedData>;
    fn DecryptAndAuthenticate(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, nonce: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, authenticationtag: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, authenticateddata: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Sign(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn VerifySignature(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, signature: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<bool>;
    fn DeriveKeyMaterial(&self, key: &::core::option::Option<CryptographicKey>, parameters: &::core::option::Option<KeyDerivationParameters>, desiredkeysize: u32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICryptographicEngineStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.ICryptographicEngineStatics";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICryptographicEngineStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICryptographicEngineStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICryptographicEngineStaticsVtbl {
        unsafe extern "system" fn Encrypt<Impl: ICryptographicEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, iv: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Encrypt(
                &*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&iv as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Decrypt<Impl: ICryptographicEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, iv: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Decrypt(
                &*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&iv as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptAndAuthenticate<Impl: ICryptographicEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, nonce: ::windows::core::RawPtr, authenticateddata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAndAuthenticate(
                &*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&nonce as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&authenticateddata as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecryptAndAuthenticate<Impl: ICryptographicEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, nonce: ::windows::core::RawPtr, authenticationtag: ::windows::core::RawPtr, authenticateddata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecryptAndAuthenticate(
                &*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&nonce as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&authenticationtag as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&authenticateddata as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sign<Impl: ICryptographicEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sign(&*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifySignature<Impl: ICryptographicEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, signature: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifySignature(
                &*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&signature as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeriveKeyMaterial<Impl: ICryptographicEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, parameters: ::windows::core::RawPtr, desiredkeysize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeriveKeyMaterial(&*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType), &*(&parameters as *const <KeyDerivationParameters as ::windows::core::Abi>::Abi as *const <KeyDerivationParameters as ::windows::core::DefaultType>::DefaultType), desiredkeysize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICryptographicEngineStatics, BASE_OFFSET>(),
            Encrypt: Encrypt::<Impl, IMPL_OFFSET>,
            Decrypt: Decrypt::<Impl, IMPL_OFFSET>,
            EncryptAndAuthenticate: EncryptAndAuthenticate::<Impl, IMPL_OFFSET>,
            DecryptAndAuthenticate: DecryptAndAuthenticate::<Impl, IMPL_OFFSET>,
            Sign: Sign::<Impl, IMPL_OFFSET>,
            VerifySignature: VerifySignature::<Impl, IMPL_OFFSET>,
            DeriveKeyMaterial: DeriveKeyMaterial::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICryptographicEngineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICryptographicEngineStatics2Impl: Sized {
    fn SignHashedData(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn VerifySignatureWithHashInput(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, signature: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<bool>;
    fn DecryptAsync(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, iv: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
    fn SignAsync(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
    fn SignHashedDataAsync(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICryptographicEngineStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.ICryptographicEngineStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICryptographicEngineStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICryptographicEngineStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICryptographicEngineStatics2Vtbl {
        unsafe extern "system" fn SignHashedData<Impl: ICryptographicEngineStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignHashedData(&*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifySignatureWithHashInput<Impl: ICryptographicEngineStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, signature: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifySignatureWithHashInput(
                &*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&signature as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecryptAsync<Impl: ICryptographicEngineStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, iv: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecryptAsync(
                &*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&iv as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignAsync<Impl: ICryptographicEngineStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignAsync(&*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignHashedDataAsync<Impl: ICryptographicEngineStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignHashedDataAsync(&*(&key as *const <CryptographicKey as ::windows::core::Abi>::Abi as *const <CryptographicKey as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICryptographicEngineStatics2, BASE_OFFSET>(),
            SignHashedData: SignHashedData::<Impl, IMPL_OFFSET>,
            VerifySignatureWithHashInput: VerifySignatureWithHashInput::<Impl, IMPL_OFFSET>,
            DecryptAsync: DecryptAsync::<Impl, IMPL_OFFSET>,
            SignAsync: SignAsync::<Impl, IMPL_OFFSET>,
            SignHashedDataAsync: SignHashedDataAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICryptographicEngineStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICryptographicKeyImpl: Sized {
    fn KeySize(&self) -> ::windows::core::Result<u32>;
    fn ExportDefaultPrivateKeyBlobType(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn ExportPrivateKeyWithBlobType(&self, blobtype: CryptographicPrivateKeyBlobType) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn ExportDefaultPublicKeyBlobType(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn ExportPublicKeyWithBlobType(&self, blobtype: CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICryptographicKey {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.ICryptographicKey";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICryptographicKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICryptographicKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICryptographicKeyVtbl {
        unsafe extern "system" fn KeySize<Impl: ICryptographicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeySize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportDefaultPrivateKeyBlobType<Impl: ICryptographicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportDefaultPrivateKeyBlobType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPrivateKeyWithBlobType<Impl: ICryptographicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blobtype: CryptographicPrivateKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportPrivateKeyWithBlobType(blobtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportDefaultPublicKeyBlobType<Impl: ICryptographicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportDefaultPublicKeyBlobType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPublicKeyWithBlobType<Impl: ICryptographicKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blobtype: CryptographicPublicKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportPublicKeyWithBlobType(blobtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICryptographicKey, BASE_OFFSET>(),
            KeySize: KeySize::<Impl, IMPL_OFFSET>,
            ExportDefaultPrivateKeyBlobType: ExportDefaultPrivateKeyBlobType::<Impl, IMPL_OFFSET>,
            ExportPrivateKeyWithBlobType: ExportPrivateKeyWithBlobType::<Impl, IMPL_OFFSET>,
            ExportDefaultPublicKeyBlobType: ExportDefaultPublicKeyBlobType::<Impl, IMPL_OFFSET>,
            ExportPublicKeyWithBlobType: ExportPublicKeyWithBlobType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICryptographicKey as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEccCurveNamesStaticsImpl: Sized {
    fn BrainpoolP160r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP160t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP192r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP192t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP224r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP224t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP256r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP256t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP320r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP320t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP384r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP384t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP512r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BrainpoolP512t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Curve25519(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ec192wapi(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NistP192(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NistP224(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NistP256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NistP384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NistP521(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumsP256t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumsP384t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumsP512t1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP160k1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP160r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP160r2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP192k1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP192r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP224k1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP224r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP256k1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP256r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP384r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SecP521r1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wtls7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wtls9(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wtls12(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn X962P192v1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn X962P192v2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn X962P192v3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn X962P239v1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn X962P239v2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn X962P239v3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn X962P256v1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllEccCurveNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEccCurveNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IEccCurveNamesStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEccCurveNamesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEccCurveNamesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEccCurveNamesStaticsVtbl {
        unsafe extern "system" fn BrainpoolP160r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP160r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP160t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP160t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP192r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP192r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP192t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP192t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP224r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP224r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP224t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP224t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP256r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP256r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP256t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP256t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP320r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP320r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP320t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP320t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP384r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP384r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP384t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP384t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP512r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP512r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrainpoolP512t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrainpoolP512t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Curve25519<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Curve25519() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ec192wapi<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ec192wapi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NistP192<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NistP192() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NistP224<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NistP224() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NistP256<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NistP256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NistP384<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NistP384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NistP521<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NistP521() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumsP256t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumsP256t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumsP384t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumsP384t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumsP512t1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumsP512t1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP160k1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP160k1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP160r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP160r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP160r2<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP160r2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP192k1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP192k1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP192r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP192r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP224k1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP224k1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP224r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP224r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP256k1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP256k1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP256r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP256r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP384r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP384r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecP521r1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecP521r1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wtls7<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wtls7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wtls9<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wtls9() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wtls12<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wtls12() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X962P192v1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X962P192v1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X962P192v2<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X962P192v2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X962P192v3<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X962P192v3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X962P239v1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X962P239v1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X962P239v2<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X962P239v2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X962P239v3<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X962P239v3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X962P256v1<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).X962P256v1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllEccCurveNames<Impl: IEccCurveNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllEccCurveNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEccCurveNamesStatics, BASE_OFFSET>(),
            BrainpoolP160r1: BrainpoolP160r1::<Impl, IMPL_OFFSET>,
            BrainpoolP160t1: BrainpoolP160t1::<Impl, IMPL_OFFSET>,
            BrainpoolP192r1: BrainpoolP192r1::<Impl, IMPL_OFFSET>,
            BrainpoolP192t1: BrainpoolP192t1::<Impl, IMPL_OFFSET>,
            BrainpoolP224r1: BrainpoolP224r1::<Impl, IMPL_OFFSET>,
            BrainpoolP224t1: BrainpoolP224t1::<Impl, IMPL_OFFSET>,
            BrainpoolP256r1: BrainpoolP256r1::<Impl, IMPL_OFFSET>,
            BrainpoolP256t1: BrainpoolP256t1::<Impl, IMPL_OFFSET>,
            BrainpoolP320r1: BrainpoolP320r1::<Impl, IMPL_OFFSET>,
            BrainpoolP320t1: BrainpoolP320t1::<Impl, IMPL_OFFSET>,
            BrainpoolP384r1: BrainpoolP384r1::<Impl, IMPL_OFFSET>,
            BrainpoolP384t1: BrainpoolP384t1::<Impl, IMPL_OFFSET>,
            BrainpoolP512r1: BrainpoolP512r1::<Impl, IMPL_OFFSET>,
            BrainpoolP512t1: BrainpoolP512t1::<Impl, IMPL_OFFSET>,
            Curve25519: Curve25519::<Impl, IMPL_OFFSET>,
            Ec192wapi: Ec192wapi::<Impl, IMPL_OFFSET>,
            NistP192: NistP192::<Impl, IMPL_OFFSET>,
            NistP224: NistP224::<Impl, IMPL_OFFSET>,
            NistP256: NistP256::<Impl, IMPL_OFFSET>,
            NistP384: NistP384::<Impl, IMPL_OFFSET>,
            NistP521: NistP521::<Impl, IMPL_OFFSET>,
            NumsP256t1: NumsP256t1::<Impl, IMPL_OFFSET>,
            NumsP384t1: NumsP384t1::<Impl, IMPL_OFFSET>,
            NumsP512t1: NumsP512t1::<Impl, IMPL_OFFSET>,
            SecP160k1: SecP160k1::<Impl, IMPL_OFFSET>,
            SecP160r1: SecP160r1::<Impl, IMPL_OFFSET>,
            SecP160r2: SecP160r2::<Impl, IMPL_OFFSET>,
            SecP192k1: SecP192k1::<Impl, IMPL_OFFSET>,
            SecP192r1: SecP192r1::<Impl, IMPL_OFFSET>,
            SecP224k1: SecP224k1::<Impl, IMPL_OFFSET>,
            SecP224r1: SecP224r1::<Impl, IMPL_OFFSET>,
            SecP256k1: SecP256k1::<Impl, IMPL_OFFSET>,
            SecP256r1: SecP256r1::<Impl, IMPL_OFFSET>,
            SecP384r1: SecP384r1::<Impl, IMPL_OFFSET>,
            SecP521r1: SecP521r1::<Impl, IMPL_OFFSET>,
            Wtls7: Wtls7::<Impl, IMPL_OFFSET>,
            Wtls9: Wtls9::<Impl, IMPL_OFFSET>,
            Wtls12: Wtls12::<Impl, IMPL_OFFSET>,
            X962P192v1: X962P192v1::<Impl, IMPL_OFFSET>,
            X962P192v2: X962P192v2::<Impl, IMPL_OFFSET>,
            X962P192v3: X962P192v3::<Impl, IMPL_OFFSET>,
            X962P239v1: X962P239v1::<Impl, IMPL_OFFSET>,
            X962P239v2: X962P239v2::<Impl, IMPL_OFFSET>,
            X962P239v3: X962P239v3::<Impl, IMPL_OFFSET>,
            X962P256v1: X962P256v1::<Impl, IMPL_OFFSET>,
            AllEccCurveNames: AllEccCurveNames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEccCurveNamesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IEncryptedAndAuthenticatedDataImpl: Sized {
    fn EncryptedData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn AuthenticationTag(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEncryptedAndAuthenticatedData {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IEncryptedAndAuthenticatedData";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IEncryptedAndAuthenticatedDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEncryptedAndAuthenticatedDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEncryptedAndAuthenticatedDataVtbl {
        unsafe extern "system" fn EncryptedData<Impl: IEncryptedAndAuthenticatedDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptedData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationTag<Impl: IEncryptedAndAuthenticatedDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationTag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEncryptedAndAuthenticatedData, BASE_OFFSET>(),
            EncryptedData: EncryptedData::<Impl, IMPL_OFFSET>,
            AuthenticationTag: AuthenticationTag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEncryptedAndAuthenticatedData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHashAlgorithmNamesStaticsImpl: Sized {
    fn Md5(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHashAlgorithmNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IHashAlgorithmNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHashAlgorithmNamesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHashAlgorithmNamesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHashAlgorithmNamesStaticsVtbl {
        unsafe extern "system" fn Md5<Impl: IHashAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Md5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sha1<Impl: IHashAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sha256<Impl: IHashAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sha384<Impl: IHashAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sha512<Impl: IHashAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHashAlgorithmNamesStatics, BASE_OFFSET>(),
            Md5: Md5::<Impl, IMPL_OFFSET>,
            Sha1: Sha1::<Impl, IMPL_OFFSET>,
            Sha256: Sha256::<Impl, IMPL_OFFSET>,
            Sha384: Sha384::<Impl, IMPL_OFFSET>,
            Sha512: Sha512::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHashAlgorithmNamesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IHashAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HashLength(&self) -> ::windows::core::Result<u32>;
    fn HashData(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn CreateHash(&self) -> ::windows::core::Result<CryptographicHash>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHashAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IHashAlgorithmProvider";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IHashAlgorithmProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHashAlgorithmProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHashAlgorithmProviderVtbl {
        unsafe extern "system" fn AlgorithmName<Impl: IHashAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashLength<Impl: IHashAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashData<Impl: IHashAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashData(&*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHash<Impl: IHashAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHash() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHashAlgorithmProvider, BASE_OFFSET>(),
            AlgorithmName: AlgorithmName::<Impl, IMPL_OFFSET>,
            HashLength: HashLength::<Impl, IMPL_OFFSET>,
            HashData: HashData::<Impl, IMPL_OFFSET>,
            CreateHash: CreateHash::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHashAlgorithmProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHashAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<HashAlgorithmProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHashAlgorithmProviderStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IHashAlgorithmProviderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHashAlgorithmProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHashAlgorithmProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHashAlgorithmProviderStaticsVtbl {
        unsafe extern "system" fn OpenAlgorithm<Impl: IHashAlgorithmProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAlgorithm(&*(&algorithm as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHashAlgorithmProviderStatics, BASE_OFFSET>(),
            OpenAlgorithm: OpenAlgorithm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHashAlgorithmProviderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IHashComputationImpl: Sized {
    fn Append(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetValueAndReset(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHashComputation {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IHashComputation";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IHashComputationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHashComputationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHashComputationVtbl {
        unsafe extern "system" fn Append<Impl: IHashComputationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetValueAndReset<Impl: IHashComputationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueAndReset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHashComputation, BASE_OFFSET>(),
            Append: Append::<Impl, IMPL_OFFSET>,
            GetValueAndReset: GetValueAndReset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHashComputation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationAlgorithmNamesStaticsImpl: Sized {
    fn Pbkdf2Md5(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pbkdf2Sha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pbkdf2Sha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pbkdf2Sha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pbkdf2Sha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp800108CtrHmacMd5(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp800108CtrHmacSha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp800108CtrHmacSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp800108CtrHmacSha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp800108CtrHmacSha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp80056aConcatMd5(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp80056aConcatSha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp80056aConcatSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp80056aConcatSha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sp80056aConcatSha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyDerivationAlgorithmNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyDerivationAlgorithmNamesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyDerivationAlgorithmNamesStaticsVtbl {
        unsafe extern "system" fn Pbkdf2Md5<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pbkdf2Md5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pbkdf2Sha1<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pbkdf2Sha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pbkdf2Sha256<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pbkdf2Sha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pbkdf2Sha384<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pbkdf2Sha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pbkdf2Sha512<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pbkdf2Sha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp800108CtrHmacMd5<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp800108CtrHmacMd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp800108CtrHmacSha1<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp800108CtrHmacSha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp800108CtrHmacSha256<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp800108CtrHmacSha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp800108CtrHmacSha384<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp800108CtrHmacSha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp800108CtrHmacSha512<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp800108CtrHmacSha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp80056aConcatMd5<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp80056aConcatMd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp80056aConcatSha1<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp80056aConcatSha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp80056aConcatSha256<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp80056aConcatSha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp80056aConcatSha384<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp80056aConcatSha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sp80056aConcatSha512<Impl: IKeyDerivationAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sp80056aConcatSha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyDerivationAlgorithmNamesStatics, BASE_OFFSET>(),
            Pbkdf2Md5: Pbkdf2Md5::<Impl, IMPL_OFFSET>,
            Pbkdf2Sha1: Pbkdf2Sha1::<Impl, IMPL_OFFSET>,
            Pbkdf2Sha256: Pbkdf2Sha256::<Impl, IMPL_OFFSET>,
            Pbkdf2Sha384: Pbkdf2Sha384::<Impl, IMPL_OFFSET>,
            Pbkdf2Sha512: Pbkdf2Sha512::<Impl, IMPL_OFFSET>,
            Sp800108CtrHmacMd5: Sp800108CtrHmacMd5::<Impl, IMPL_OFFSET>,
            Sp800108CtrHmacSha1: Sp800108CtrHmacSha1::<Impl, IMPL_OFFSET>,
            Sp800108CtrHmacSha256: Sp800108CtrHmacSha256::<Impl, IMPL_OFFSET>,
            Sp800108CtrHmacSha384: Sp800108CtrHmacSha384::<Impl, IMPL_OFFSET>,
            Sp800108CtrHmacSha512: Sp800108CtrHmacSha512::<Impl, IMPL_OFFSET>,
            Sp80056aConcatMd5: Sp80056aConcatMd5::<Impl, IMPL_OFFSET>,
            Sp80056aConcatSha1: Sp80056aConcatSha1::<Impl, IMPL_OFFSET>,
            Sp80056aConcatSha256: Sp80056aConcatSha256::<Impl, IMPL_OFFSET>,
            Sp80056aConcatSha384: Sp80056aConcatSha384::<Impl, IMPL_OFFSET>,
            Sp80056aConcatSha512: Sp80056aConcatSha512::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyDerivationAlgorithmNamesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationAlgorithmNamesStatics2Impl: Sized {
    fn CapiKdfMd5(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CapiKdfSha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CapiKdfSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CapiKdfSha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CapiKdfSha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyDerivationAlgorithmNamesStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmNamesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyDerivationAlgorithmNamesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyDerivationAlgorithmNamesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyDerivationAlgorithmNamesStatics2Vtbl {
        unsafe extern "system" fn CapiKdfMd5<Impl: IKeyDerivationAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapiKdfMd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapiKdfSha1<Impl: IKeyDerivationAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapiKdfSha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapiKdfSha256<Impl: IKeyDerivationAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapiKdfSha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapiKdfSha384<Impl: IKeyDerivationAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapiKdfSha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapiKdfSha512<Impl: IKeyDerivationAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapiKdfSha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyDerivationAlgorithmNamesStatics2, BASE_OFFSET>(),
            CapiKdfMd5: CapiKdfMd5::<Impl, IMPL_OFFSET>,
            CapiKdfSha1: CapiKdfSha1::<Impl, IMPL_OFFSET>,
            CapiKdfSha256: CapiKdfSha256::<Impl, IMPL_OFFSET>,
            CapiKdfSha384: CapiKdfSha384::<Impl, IMPL_OFFSET>,
            CapiKdfSha512: CapiKdfSha512::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyDerivationAlgorithmNamesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IKeyDerivationAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateKey(&self, keymaterial: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyDerivationAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProvider";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IKeyDerivationAlgorithmProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyDerivationAlgorithmProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyDerivationAlgorithmProviderVtbl {
        unsafe extern "system" fn AlgorithmName<Impl: IKeyDerivationAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Impl: IKeyDerivationAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keymaterial: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKey(&*(&keymaterial as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyDerivationAlgorithmProvider, BASE_OFFSET>(),
            AlgorithmName: AlgorithmName::<Impl, IMPL_OFFSET>,
            CreateKey: CreateKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyDerivationAlgorithmProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<KeyDerivationAlgorithmProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyDerivationAlgorithmProviderStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IKeyDerivationAlgorithmProviderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyDerivationAlgorithmProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyDerivationAlgorithmProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyDerivationAlgorithmProviderStaticsVtbl {
        unsafe extern "system" fn OpenAlgorithm<Impl: IKeyDerivationAlgorithmProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAlgorithm(&*(&algorithm as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyDerivationAlgorithmProviderStatics, BASE_OFFSET>(),
            OpenAlgorithm: OpenAlgorithm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyDerivationAlgorithmProviderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IKeyDerivationParametersImpl: Sized {
    fn KdfGenericBinary(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetKdfGenericBinary(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn IterationCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyDerivationParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IKeyDerivationParameters";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IKeyDerivationParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyDerivationParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyDerivationParametersVtbl {
        unsafe extern "system" fn KdfGenericBinary<Impl: IKeyDerivationParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KdfGenericBinary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKdfGenericBinary<Impl: IKeyDerivationParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKdfGenericBinary(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IterationCount<Impl: IKeyDerivationParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IterationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyDerivationParameters, BASE_OFFSET>(),
            KdfGenericBinary: KdfGenericBinary::<Impl, IMPL_OFFSET>,
            SetKdfGenericBinary: SetKdfGenericBinary::<Impl, IMPL_OFFSET>,
            IterationCount: IterationCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyDerivationParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationParameters2Impl: Sized {
    fn Capi1KdfTargetAlgorithm(&self) -> ::windows::core::Result<Capi1KdfTargetAlgorithm>;
    fn SetCapi1KdfTargetAlgorithm(&self, value: Capi1KdfTargetAlgorithm) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyDerivationParameters2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IKeyDerivationParameters2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyDerivationParameters2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyDerivationParameters2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyDerivationParameters2Vtbl {
        unsafe extern "system" fn Capi1KdfTargetAlgorithm<Impl: IKeyDerivationParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Capi1KdfTargetAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capi1KdfTargetAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCapi1KdfTargetAlgorithm<Impl: IKeyDerivationParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Capi1KdfTargetAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCapi1KdfTargetAlgorithm(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyDerivationParameters2, BASE_OFFSET>(),
            Capi1KdfTargetAlgorithm: Capi1KdfTargetAlgorithm::<Impl, IMPL_OFFSET>,
            SetCapi1KdfTargetAlgorithm: SetCapi1KdfTargetAlgorithm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyDerivationParameters2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IKeyDerivationParametersStaticsImpl: Sized {
    fn BuildForPbkdf2(&self, pbkdf2salt: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, iterationcount: u32) -> ::windows::core::Result<KeyDerivationParameters>;
    fn BuildForSP800108(&self, label: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, context: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<KeyDerivationParameters>;
    fn BuildForSP80056a(&self, algorithmid: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, partyuinfo: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, partyvinfo: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, supppubinfo: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, suppprivinfo: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<KeyDerivationParameters>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyDerivationParametersStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IKeyDerivationParametersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyDerivationParametersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyDerivationParametersStaticsVtbl {
        unsafe extern "system" fn BuildForPbkdf2<Impl: IKeyDerivationParametersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbkdf2salt: ::windows::core::RawPtr, iterationcount: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildForPbkdf2(&*(&pbkdf2salt as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), iterationcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildForSP800108<Impl: IKeyDerivationParametersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, label: ::windows::core::RawPtr, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildForSP800108(&*(&label as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildForSP80056a<Impl: IKeyDerivationParametersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algorithmid: ::windows::core::RawPtr, partyuinfo: ::windows::core::RawPtr, partyvinfo: ::windows::core::RawPtr, supppubinfo: ::windows::core::RawPtr, suppprivinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildForSP80056a(
                &*(&algorithmid as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&partyuinfo as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&partyvinfo as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&supppubinfo as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&suppprivinfo as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyDerivationParametersStatics, BASE_OFFSET>(),
            BuildForPbkdf2: BuildForPbkdf2::<Impl, IMPL_OFFSET>,
            BuildForSP800108: BuildForSP800108::<Impl, IMPL_OFFSET>,
            BuildForSP80056a: BuildForSP80056a::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyDerivationParametersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationParametersStatics2Impl: Sized {
    fn BuildForCapi1Kdf(&self, capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm) -> ::windows::core::Result<KeyDerivationParameters>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyDerivationParametersStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IKeyDerivationParametersStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyDerivationParametersStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyDerivationParametersStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyDerivationParametersStatics2Vtbl {
        unsafe extern "system" fn BuildForCapi1Kdf<Impl: IKeyDerivationParametersStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildForCapi1Kdf(capi1kdftargetalgorithm) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyDerivationParametersStatics2, BASE_OFFSET>(),
            BuildForCapi1Kdf: BuildForCapi1Kdf::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyDerivationParametersStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMacAlgorithmNamesStaticsImpl: Sized {
    fn HmacMd5(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HmacSha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HmacSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HmacSha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HmacSha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AesCmac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMacAlgorithmNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IMacAlgorithmNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMacAlgorithmNamesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMacAlgorithmNamesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMacAlgorithmNamesStaticsVtbl {
        unsafe extern "system" fn HmacMd5<Impl: IMacAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HmacMd5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HmacSha1<Impl: IMacAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HmacSha1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HmacSha256<Impl: IMacAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HmacSha256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HmacSha384<Impl: IMacAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HmacSha384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HmacSha512<Impl: IMacAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HmacSha512() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AesCmac<Impl: IMacAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AesCmac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMacAlgorithmNamesStatics, BASE_OFFSET>(),
            HmacMd5: HmacMd5::<Impl, IMPL_OFFSET>,
            HmacSha1: HmacSha1::<Impl, IMPL_OFFSET>,
            HmacSha256: HmacSha256::<Impl, IMPL_OFFSET>,
            HmacSha384: HmacSha384::<Impl, IMPL_OFFSET>,
            HmacSha512: HmacSha512::<Impl, IMPL_OFFSET>,
            AesCmac: AesCmac::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMacAlgorithmNamesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMacAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MacLength(&self) -> ::windows::core::Result<u32>;
    fn CreateKey(&self, keymaterial: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMacAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IMacAlgorithmProvider";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMacAlgorithmProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMacAlgorithmProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMacAlgorithmProviderVtbl {
        unsafe extern "system" fn AlgorithmName<Impl: IMacAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MacLength<Impl: IMacAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MacLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Impl: IMacAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keymaterial: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateKey(&*(&keymaterial as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMacAlgorithmProvider, BASE_OFFSET>(),
            AlgorithmName: AlgorithmName::<Impl, IMPL_OFFSET>,
            MacLength: MacLength::<Impl, IMPL_OFFSET>,
            CreateKey: CreateKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMacAlgorithmProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMacAlgorithmProvider2Impl: Sized {
    fn CreateHash(&self, keymaterial: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicHash>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMacAlgorithmProvider2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IMacAlgorithmProvider2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMacAlgorithmProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMacAlgorithmProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMacAlgorithmProvider2Vtbl {
        unsafe extern "system" fn CreateHash<Impl: IMacAlgorithmProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keymaterial: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHash(&*(&keymaterial as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMacAlgorithmProvider2, BASE_OFFSET>(), CreateHash: CreateHash::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMacAlgorithmProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMacAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<MacAlgorithmProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMacAlgorithmProviderStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IMacAlgorithmProviderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMacAlgorithmProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMacAlgorithmProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMacAlgorithmProviderStaticsVtbl {
        unsafe extern "system" fn OpenAlgorithm<Impl: IMacAlgorithmProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAlgorithm(&*(&algorithm as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMacAlgorithmProviderStatics, BASE_OFFSET>(),
            OpenAlgorithm: OpenAlgorithm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMacAlgorithmProviderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IPersistedKeyProviderStaticsImpl: Sized {
    fn OpenKeyPairFromCertificateAsync(&self, certificate: &::core::option::Option<super::Certificates::Certificate>, hashalgorithmname: &::windows::core::HSTRING, padding: CryptographicPadding) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CryptographicKey>>;
    fn OpenPublicKeyFromCertificate(&self, certificate: &::core::option::Option<super::Certificates::Certificate>, hashalgorithmname: &::windows::core::HSTRING, padding: CryptographicPadding) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPersistedKeyProviderStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.IPersistedKeyProviderStatics";
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IPersistedKeyProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistedKeyProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistedKeyProviderStaticsVtbl {
        unsafe extern "system" fn OpenKeyPairFromCertificateAsync<Impl: IPersistedKeyProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, padding: CryptographicPadding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenKeyPairFromCertificateAsync(&*(&certificate as *const <super::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType), &*(&hashalgorithmname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), padding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPublicKeyFromCertificate<Impl: IPersistedKeyProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, padding: CryptographicPadding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPublicKeyFromCertificate(&*(&certificate as *const <super::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType), &*(&hashalgorithmname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), padding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPersistedKeyProviderStatics, BASE_OFFSET>(),
            OpenKeyPairFromCertificateAsync: OpenKeyPairFromCertificateAsync::<Impl, IMPL_OFFSET>,
            OpenPublicKeyFromCertificate: OpenPublicKeyFromCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistedKeyProviderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISymmetricAlgorithmNamesStaticsImpl: Sized {
    fn DesCbc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DesEcb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TripleDesCbc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TripleDesEcb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rc2Cbc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rc2Ecb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AesCbc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AesEcb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AesGcm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AesCcm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AesCbcPkcs7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AesEcbPkcs7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DesCbcPkcs7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DesEcbPkcs7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TripleDesCbcPkcs7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TripleDesEcbPkcs7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rc2CbcPkcs7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rc2EcbPkcs7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rc4(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISymmetricAlgorithmNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.ISymmetricAlgorithmNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISymmetricAlgorithmNamesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISymmetricAlgorithmNamesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISymmetricAlgorithmNamesStaticsVtbl {
        unsafe extern "system" fn DesCbc<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesCbc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesEcb<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesEcb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TripleDesCbc<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TripleDesCbc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TripleDesEcb<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TripleDesEcb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rc2Cbc<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rc2Cbc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rc2Ecb<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rc2Ecb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AesCbc<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AesCbc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AesEcb<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AesEcb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AesGcm<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AesGcm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AesCcm<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AesCcm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AesCbcPkcs7<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AesCbcPkcs7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AesEcbPkcs7<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AesEcbPkcs7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesCbcPkcs7<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesCbcPkcs7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesEcbPkcs7<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesEcbPkcs7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TripleDesCbcPkcs7<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TripleDesCbcPkcs7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TripleDesEcbPkcs7<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TripleDesEcbPkcs7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rc2CbcPkcs7<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rc2CbcPkcs7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rc2EcbPkcs7<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rc2EcbPkcs7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rc4<Impl: ISymmetricAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rc4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISymmetricAlgorithmNamesStatics, BASE_OFFSET>(),
            DesCbc: DesCbc::<Impl, IMPL_OFFSET>,
            DesEcb: DesEcb::<Impl, IMPL_OFFSET>,
            TripleDesCbc: TripleDesCbc::<Impl, IMPL_OFFSET>,
            TripleDesEcb: TripleDesEcb::<Impl, IMPL_OFFSET>,
            Rc2Cbc: Rc2Cbc::<Impl, IMPL_OFFSET>,
            Rc2Ecb: Rc2Ecb::<Impl, IMPL_OFFSET>,
            AesCbc: AesCbc::<Impl, IMPL_OFFSET>,
            AesEcb: AesEcb::<Impl, IMPL_OFFSET>,
            AesGcm: AesGcm::<Impl, IMPL_OFFSET>,
            AesCcm: AesCcm::<Impl, IMPL_OFFSET>,
            AesCbcPkcs7: AesCbcPkcs7::<Impl, IMPL_OFFSET>,
            AesEcbPkcs7: AesEcbPkcs7::<Impl, IMPL_OFFSET>,
            DesCbcPkcs7: DesCbcPkcs7::<Impl, IMPL_OFFSET>,
            DesEcbPkcs7: DesEcbPkcs7::<Impl, IMPL_OFFSET>,
            TripleDesCbcPkcs7: TripleDesCbcPkcs7::<Impl, IMPL_OFFSET>,
            TripleDesEcbPkcs7: TripleDesEcbPkcs7::<Impl, IMPL_OFFSET>,
            Rc2CbcPkcs7: Rc2CbcPkcs7::<Impl, IMPL_OFFSET>,
            Rc2EcbPkcs7: Rc2EcbPkcs7::<Impl, IMPL_OFFSET>,
            Rc4: Rc4::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISymmetricAlgorithmNamesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISymmetricKeyAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BlockLength(&self) -> ::windows::core::Result<u32>;
    fn CreateSymmetricKey(&self, keymaterial: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISymmetricKeyAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProvider";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISymmetricKeyAlgorithmProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISymmetricKeyAlgorithmProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISymmetricKeyAlgorithmProviderVtbl {
        unsafe extern "system" fn AlgorithmName<Impl: ISymmetricKeyAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockLength<Impl: ISymmetricKeyAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSymmetricKey<Impl: ISymmetricKeyAlgorithmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keymaterial: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSymmetricKey(&*(&keymaterial as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISymmetricKeyAlgorithmProvider, BASE_OFFSET>(),
            AlgorithmName: AlgorithmName::<Impl, IMPL_OFFSET>,
            BlockLength: BlockLength::<Impl, IMPL_OFFSET>,
            CreateSymmetricKey: CreateSymmetricKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISymmetricKeyAlgorithmProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISymmetricKeyAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<SymmetricKeyAlgorithmProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISymmetricKeyAlgorithmProviderStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.ISymmetricKeyAlgorithmProviderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISymmetricKeyAlgorithmProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISymmetricKeyAlgorithmProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISymmetricKeyAlgorithmProviderStaticsVtbl {
        unsafe extern "system" fn OpenAlgorithm<Impl: ISymmetricKeyAlgorithmProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAlgorithm(&*(&algorithm as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISymmetricKeyAlgorithmProviderStatics, BASE_OFFSET>(),
            OpenAlgorithm: OpenAlgorithm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISymmetricKeyAlgorithmProviderStatics as ::windows::core::Interface>::IID
    }
}
