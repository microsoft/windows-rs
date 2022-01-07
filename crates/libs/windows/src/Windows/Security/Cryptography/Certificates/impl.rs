#[cfg(feature = "implement_exclusive")]
pub trait ICertificateImpl: Sized {
    fn BuildChainAsync(&self, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>;
    fn BuildChainWithParametersAsync(&self, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>, parameters: &::core::option::Option<ChainBuildingParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>;
    fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn GetHashValue(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn GetHashValueWithAlgorithm(&self, hashalgorithmname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn GetCertificateBlob(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Issuer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasPrivateKey(&self) -> ::windows::core::Result<bool>;
    fn IsStronglyProtected(&self) -> ::windows::core::Result<bool>;
    fn ValidFrom(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ValidTo(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificate {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificate";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateVtbl {
    pub const fn new<Impl: ICertificateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateVtbl {
        unsafe extern "system" fn BuildChainAsync<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BuildChainAsync(&*(&certificates as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildChainWithParametersAsync<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificates: ::windows::core::RawPtr, parameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BuildChainWithParametersAsync(&*(&certificates as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::DefaultType>::DefaultType), &*(&parameters as *const <ChainBuildingParameters as ::windows::core::Abi>::Abi as *const <ChainBuildingParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashValue<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHashValue() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashValueWithAlgorithm<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHashValueWithAlgorithm(&*(&hashalgorithmname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateBlob<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCertificateBlob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Issuer<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Issuer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasPrivateKey<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasPrivateKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStronglyProtected<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStronglyProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidFrom<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidTo<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidTo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnhancedKeyUsages<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnhancedKeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertificateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICertificate>,
            base.5,
            BuildChainAsync::<Impl, OFFSET>,
            BuildChainWithParametersAsync::<Impl, OFFSET>,
            SerialNumber::<Impl, OFFSET>,
            GetHashValue::<Impl, OFFSET>,
            GetHashValueWithAlgorithm::<Impl, OFFSET>,
            GetCertificateBlob::<Impl, OFFSET>,
            Subject::<Impl, OFFSET>,
            Issuer::<Impl, OFFSET>,
            HasPrivateKey::<Impl, OFFSET>,
            IsStronglyProtected::<Impl, OFFSET>,
            ValidFrom::<Impl, OFFSET>,
            ValidTo::<Impl, OFFSET>,
            EnhancedKeyUsages::<Impl, OFFSET>,
            SetFriendlyName::<Impl, OFFSET>,
            FriendlyName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificate2Impl: Sized {
    fn IsSecurityDeviceBound(&self) -> ::windows::core::Result<bool>;
    fn KeyUsages(&self) -> ::windows::core::Result<CertificateKeyUsages>;
    fn KeyAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignatureAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignatureHashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubjectAlternativeName(&self) -> ::windows::core::Result<SubjectAlternativeNameInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificate2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificate2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificate2Vtbl {
    pub const fn new<Impl: ICertificate2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificate2Vtbl {
        unsafe extern "system" fn IsSecurityDeviceBound<Impl: ICertificate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSecurityDeviceBound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUsages<Impl: ICertificate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyAlgorithmName<Impl: ICertificate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignatureAlgorithmName<Impl: ICertificate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignatureAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignatureHashAlgorithmName<Impl: ICertificate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignatureHashAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubjectAlternativeName<Impl: ICertificate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubjectAlternativeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificate2>, base.5, IsSecurityDeviceBound::<Impl, OFFSET>, KeyUsages::<Impl, OFFSET>, KeyAlgorithmName::<Impl, OFFSET>, SignatureAlgorithmName::<Impl, OFFSET>, SignatureHashAlgorithmName::<Impl, OFFSET>, SubjectAlternativeName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificate3Impl: Sized {
    fn IsPerUser(&self) -> ::windows::core::Result<bool>;
    fn StoreName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificate3 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificate3";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificate3Vtbl {
    pub const fn new<Impl: ICertificate3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificate3Vtbl {
        unsafe extern "system" fn IsPerUser<Impl: ICertificate3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPerUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreName<Impl: ICertificate3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStorageProviderName<Impl: ICertificate3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyStorageProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificate3>, base.5, IsPerUser::<Impl, OFFSET>, StoreName::<Impl, OFFSET>, KeyStorageProviderName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateChainImpl: Sized {
    fn Validate(&self) -> ::windows::core::Result<ChainValidationResult>;
    fn ValidateWithParameters(&self, parameter: &::core::option::Option<ChainValidationParameters>) -> ::windows::core::Result<ChainValidationResult>;
    fn GetCertificates(&self, includeroot: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateChain {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateChain";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateChainVtbl {
    pub const fn new<Impl: ICertificateChainImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateChainVtbl {
        unsafe extern "system" fn Validate<Impl: ICertificateChainImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ChainValidationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Validate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateWithParameters<Impl: ICertificateChainImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameter: ::windows::core::RawPtr, result__: *mut ChainValidationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidateWithParameters(&*(&parameter as *const <ChainValidationParameters as ::windows::core::Abi>::Abi as *const <ChainValidationParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificates<Impl: ICertificateChainImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, includeroot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCertificates(includeroot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateChain>, base.5, Validate::<Impl, OFFSET>, ValidateWithParameters::<Impl, OFFSET>, GetCertificates::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateEnrollmentManagerStaticsImpl: Sized {
    fn CreateRequestAsync(&self, request: &::core::option::Option<CertificateRequestProperties>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn InstallCertificateAsync(&self, certificate: &::windows::core::HSTRING, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportPfxDataAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateEnrollmentManagerStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateEnrollmentManagerStaticsVtbl {
    pub const fn new<Impl: ICertificateEnrollmentManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateEnrollmentManagerStaticsVtbl {
        unsafe extern "system" fn CreateRequestAsync<Impl: ICertificateEnrollmentManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRequestAsync(&*(&request as *const <CertificateRequestProperties as ::windows::core::Abi>::Abi as *const <CertificateRequestProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallCertificateAsync<Impl: ICertificateEnrollmentManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoption: InstallOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallCertificateAsync(&*(&certificate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), installoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportPfxDataAsync<Impl: ICertificateEnrollmentManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImportPfxDataAsync(
                &*(&pfxdata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                exportable,
                keyprotectionlevel,
                installoption,
                &*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateEnrollmentManagerStatics>, base.5, CreateRequestAsync::<Impl, OFFSET>, InstallCertificateAsync::<Impl, OFFSET>, ImportPfxDataAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateEnrollmentManagerStatics2Impl: Sized {
    fn UserCertificateEnrollmentManager(&self) -> ::windows::core::Result<UserCertificateEnrollmentManager>;
    fn ImportPfxDataToKspAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING, keystorageprovider: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateEnrollmentManagerStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateEnrollmentManagerStatics2Vtbl {
    pub const fn new<Impl: ICertificateEnrollmentManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateEnrollmentManagerStatics2Vtbl {
        unsafe extern "system" fn UserCertificateEnrollmentManager<Impl: ICertificateEnrollmentManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserCertificateEnrollmentManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportPfxDataToKspAsync<Impl: ICertificateEnrollmentManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keystorageprovider: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImportPfxDataToKspAsync(
                &*(&pfxdata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                exportable,
                keyprotectionlevel,
                installoption,
                &*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&keystorageprovider as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateEnrollmentManagerStatics2>, base.5, UserCertificateEnrollmentManager::<Impl, OFFSET>, ImportPfxDataToKspAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateEnrollmentManagerStatics3Impl: Sized {
    fn ImportPfxDataToKspWithParametersAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, pfximportparameters: &::core::option::Option<PfxImportParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateEnrollmentManagerStatics3 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateEnrollmentManagerStatics3Vtbl {
    pub const fn new<Impl: ICertificateEnrollmentManagerStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateEnrollmentManagerStatics3Vtbl {
        unsafe extern "system" fn ImportPfxDataToKspWithParametersAsync<Impl: ICertificateEnrollmentManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pfximportparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImportPfxDataToKspWithParametersAsync(
                &*(&pfxdata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&pfximportparameters as *const <PfxImportParameters as ::windows::core::Abi>::Abi as *const <PfxImportParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateEnrollmentManagerStatics3>, base.5, ImportPfxDataToKspWithParametersAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateExtensionImpl: Sized {
    fn ObjectId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetObjectId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCritical(&self) -> ::windows::core::Result<bool>;
    fn SetIsCritical(&self, value: bool) -> ::windows::core::Result<()>;
    fn EncodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetValue(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateExtension {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateExtension";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateExtensionVtbl {
    pub const fn new<Impl: ICertificateExtensionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateExtensionVtbl {
        unsafe extern "system" fn ObjectId<Impl: ICertificateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectId<Impl: ICertificateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetObjectId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCritical<Impl: ICertificateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCritical() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCritical<Impl: ICertificateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCritical(value).into()
        }
        unsafe extern "system" fn EncodeValue<Impl: ICertificateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EncodeValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: ICertificateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ICertificateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateExtension>, base.5, ObjectId::<Impl, OFFSET>, SetObjectId::<Impl, OFFSET>, IsCritical::<Impl, OFFSET>, SetIsCritical::<Impl, OFFSET>, EncodeValue::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateFactoryImpl: Sized {
    fn CreateCertificate(&self, certblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<Certificate>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateFactory {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateFactoryVtbl {
    pub const fn new<Impl: ICertificateFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateFactoryVtbl {
        unsafe extern "system" fn CreateCertificate<Impl: ICertificateFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCertificate(&*(&certblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateFactory>, base.5, CreateCertificate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateKeyUsagesImpl: Sized {
    fn EncipherOnly(&self) -> ::windows::core::Result<bool>;
    fn SetEncipherOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn CrlSign(&self) -> ::windows::core::Result<bool>;
    fn SetCrlSign(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyCertificateSign(&self) -> ::windows::core::Result<bool>;
    fn SetKeyCertificateSign(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyAgreement(&self) -> ::windows::core::Result<bool>;
    fn SetKeyAgreement(&self, value: bool) -> ::windows::core::Result<()>;
    fn DataEncipherment(&self) -> ::windows::core::Result<bool>;
    fn SetDataEncipherment(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyEncipherment(&self) -> ::windows::core::Result<bool>;
    fn SetKeyEncipherment(&self, value: bool) -> ::windows::core::Result<()>;
    fn NonRepudiation(&self) -> ::windows::core::Result<bool>;
    fn SetNonRepudiation(&self, value: bool) -> ::windows::core::Result<()>;
    fn DigitalSignature(&self) -> ::windows::core::Result<bool>;
    fn SetDigitalSignature(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateKeyUsages {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateKeyUsages";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateKeyUsagesVtbl {
    pub const fn new<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateKeyUsagesVtbl {
        unsafe extern "system" fn EncipherOnly<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncipherOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncipherOnly<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEncipherOnly(value).into()
        }
        unsafe extern "system" fn CrlSign<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CrlSign() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCrlSign<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCrlSign(value).into()
        }
        unsafe extern "system" fn KeyCertificateSign<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyCertificateSign() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyCertificateSign<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyCertificateSign(value).into()
        }
        unsafe extern "system" fn KeyAgreement<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyAgreement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyAgreement<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyAgreement(value).into()
        }
        unsafe extern "system" fn DataEncipherment<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataEncipherment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataEncipherment<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDataEncipherment(value).into()
        }
        unsafe extern "system" fn KeyEncipherment<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyEncipherment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyEncipherment<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyEncipherment(value).into()
        }
        unsafe extern "system" fn NonRepudiation<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NonRepudiation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonRepudiation<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNonRepudiation(value).into()
        }
        unsafe extern "system" fn DigitalSignature<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DigitalSignature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigitalSignature<Impl: ICertificateKeyUsagesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDigitalSignature(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICertificateKeyUsages>,
            base.5,
            EncipherOnly::<Impl, OFFSET>,
            SetEncipherOnly::<Impl, OFFSET>,
            CrlSign::<Impl, OFFSET>,
            SetCrlSign::<Impl, OFFSET>,
            KeyCertificateSign::<Impl, OFFSET>,
            SetKeyCertificateSign::<Impl, OFFSET>,
            KeyAgreement::<Impl, OFFSET>,
            SetKeyAgreement::<Impl, OFFSET>,
            DataEncipherment::<Impl, OFFSET>,
            SetDataEncipherment::<Impl, OFFSET>,
            KeyEncipherment::<Impl, OFFSET>,
            SetKeyEncipherment::<Impl, OFFSET>,
            NonRepudiation::<Impl, OFFSET>,
            SetNonRepudiation::<Impl, OFFSET>,
            DigitalSignature::<Impl, OFFSET>,
            SetDigitalSignature::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateQueryImpl: Sized {
    fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IssuerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIssuerName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbprint(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetThumbprint(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn HardwareOnly(&self) -> ::windows::core::Result<bool>;
    fn SetHardwareOnly(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateQuery {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateQuery";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateQueryVtbl {
    pub const fn new<Impl: ICertificateQueryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateQueryVtbl {
        unsafe extern "system" fn EnhancedKeyUsages<Impl: ICertificateQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnhancedKeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssuerName<Impl: ICertificateQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IssuerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIssuerName<Impl: ICertificateQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIssuerName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertificateQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: ICertificateQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbprint<Impl: ICertificateQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Thumbprint() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbprint<Impl: ICertificateQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetThumbprint(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn HardwareOnly<Impl: ICertificateQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HardwareOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHardwareOnly<Impl: ICertificateQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHardwareOnly(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateQuery>, base.5, EnhancedKeyUsages::<Impl, OFFSET>, IssuerName::<Impl, OFFSET>, SetIssuerName::<Impl, OFFSET>, FriendlyName::<Impl, OFFSET>, SetFriendlyName::<Impl, OFFSET>, Thumbprint::<Impl, OFFSET>, SetThumbprint::<Impl, OFFSET>, HardwareOnly::<Impl, OFFSET>, SetHardwareOnly::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateQuery2Impl: Sized {
    fn IncludeDuplicates(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeDuplicates(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeExpiredCertificates(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeExpiredCertificates(&self, value: bool) -> ::windows::core::Result<()>;
    fn StoreName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStoreName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateQuery2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateQuery2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateQuery2Vtbl {
    pub const fn new<Impl: ICertificateQuery2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateQuery2Vtbl {
        unsafe extern "system" fn IncludeDuplicates<Impl: ICertificateQuery2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IncludeDuplicates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeDuplicates<Impl: ICertificateQuery2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIncludeDuplicates(value).into()
        }
        unsafe extern "system" fn IncludeExpiredCertificates<Impl: ICertificateQuery2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IncludeExpiredCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeExpiredCertificates<Impl: ICertificateQuery2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIncludeExpiredCertificates(value).into()
        }
        unsafe extern "system" fn StoreName<Impl: ICertificateQuery2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoreName<Impl: ICertificateQuery2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStoreName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateQuery2>, base.5, IncludeDuplicates::<Impl, OFFSET>, SetIncludeDuplicates::<Impl, OFFSET>, IncludeExpiredCertificates::<Impl, OFFSET>, SetIncludeExpiredCertificates::<Impl, OFFSET>, StoreName::<Impl, OFFSET>, SetStoreName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestPropertiesImpl: Sized {
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn KeyAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyAlgorithmName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn KeySize(&self) -> ::windows::core::Result<u32>;
    fn SetKeySize(&self, value: u32) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHashAlgorithmName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Exportable(&self) -> ::windows::core::Result<ExportOption>;
    fn SetExportable(&self, value: ExportOption) -> ::windows::core::Result<()>;
    fn KeyUsages(&self) -> ::windows::core::Result<EnrollKeyUsages>;
    fn SetKeyUsages(&self, value: EnrollKeyUsages) -> ::windows::core::Result<()>;
    fn KeyProtectionLevel(&self) -> ::windows::core::Result<KeyProtectionLevel>;
    fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> ::windows::core::Result<()>;
    fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyStorageProviderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateRequestProperties {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateRequestProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateRequestPropertiesVtbl {
    pub const fn new<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateRequestPropertiesVtbl {
        unsafe extern "system" fn Subject<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSubject(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyAlgorithmName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyAlgorithmName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyAlgorithmName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeySize<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeySize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySize<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeySize(value).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HashAlgorithmName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HashAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithmName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithmName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Exportable<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ExportOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Exportable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExportable<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ExportOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExportable(value).into()
        }
        unsafe extern "system" fn KeyUsages<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut EnrollKeyUsages) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyUsages<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: EnrollKeyUsages) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyUsages(value).into()
        }
        unsafe extern "system" fn KeyProtectionLevel<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut KeyProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyProtectionLevel<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: KeyProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyProtectionLevel(value).into()
        }
        unsafe extern "system" fn KeyStorageProviderName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyStorageProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyStorageProviderName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyStorageProviderName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICertificateRequestProperties>,
            base.5,
            Subject::<Impl, OFFSET>,
            SetSubject::<Impl, OFFSET>,
            KeyAlgorithmName::<Impl, OFFSET>,
            SetKeyAlgorithmName::<Impl, OFFSET>,
            KeySize::<Impl, OFFSET>,
            SetKeySize::<Impl, OFFSET>,
            FriendlyName::<Impl, OFFSET>,
            SetFriendlyName::<Impl, OFFSET>,
            HashAlgorithmName::<Impl, OFFSET>,
            SetHashAlgorithmName::<Impl, OFFSET>,
            Exportable::<Impl, OFFSET>,
            SetExportable::<Impl, OFFSET>,
            KeyUsages::<Impl, OFFSET>,
            SetKeyUsages::<Impl, OFFSET>,
            KeyProtectionLevel::<Impl, OFFSET>,
            SetKeyProtectionLevel::<Impl, OFFSET>,
            KeyStorageProviderName::<Impl, OFFSET>,
            SetKeyStorageProviderName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestProperties2Impl: Sized {
    fn SmartcardReaderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSmartcardReaderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SigningCertificate(&self) -> ::windows::core::Result<Certificate>;
    fn SetSigningCertificate(&self, value: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
    fn AttestationCredentialCertificate(&self) -> ::windows::core::Result<Certificate>;
    fn SetAttestationCredentialCertificate(&self, value: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateRequestProperties2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateRequestProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateRequestProperties2Vtbl {
    pub const fn new<Impl: ICertificateRequestProperties2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateRequestProperties2Vtbl {
        unsafe extern "system" fn SmartcardReaderName<Impl: ICertificateRequestProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmartcardReaderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmartcardReaderName<Impl: ICertificateRequestProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSmartcardReaderName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SigningCertificate<Impl: ICertificateRequestProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SigningCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningCertificate<Impl: ICertificateRequestProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSigningCertificate(&*(&value as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AttestationCredentialCertificate<Impl: ICertificateRequestProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AttestationCredentialCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttestationCredentialCertificate<Impl: ICertificateRequestProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAttestationCredentialCertificate(&*(&value as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateRequestProperties2>, base.5, SmartcardReaderName::<Impl, OFFSET>, SetSmartcardReaderName::<Impl, OFFSET>, SigningCertificate::<Impl, OFFSET>, SetSigningCertificate::<Impl, OFFSET>, AttestationCredentialCertificate::<Impl, OFFSET>, SetAttestationCredentialCertificate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestProperties3Impl: Sized {
    fn CurveName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurveName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CurveParameters(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetCurveParameters(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContainerNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContainerName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UseExistingKey(&self) -> ::windows::core::Result<bool>;
    fn SetUseExistingKey(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateRequestProperties3 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateRequestProperties3";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateRequestProperties3Vtbl {
    pub const fn new<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateRequestProperties3Vtbl {
        unsafe extern "system" fn CurveName<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurveName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurveName<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCurveName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurveParameters<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurveParameters() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurveParameters<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCurveParameters(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn ContainerNamePrefix<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainerNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerNamePrefix<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContainerNamePrefix(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContainerName<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContainerName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UseExistingKey<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseExistingKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseExistingKey<Impl: ICertificateRequestProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUseExistingKey(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateRequestProperties3>, base.5, CurveName::<Impl, OFFSET>, SetCurveName::<Impl, OFFSET>, CurveParameters::<Impl, OFFSET>, SetCurveParameters::<Impl, OFFSET>, ContainerNamePrefix::<Impl, OFFSET>, SetContainerNamePrefix::<Impl, OFFSET>, ContainerName::<Impl, OFFSET>, SetContainerName::<Impl, OFFSET>, UseExistingKey::<Impl, OFFSET>, SetUseExistingKey::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestProperties4Impl: Sized {
    fn SuppressedDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SubjectAlternativeName(&self) -> ::windows::core::Result<SubjectAlternativeNameInfo>;
    fn Extensions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<CertificateExtension>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateRequestProperties4 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateRequestProperties4";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateRequestProperties4Vtbl {
    pub const fn new<Impl: ICertificateRequestProperties4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateRequestProperties4Vtbl {
        unsafe extern "system" fn SuppressedDefaults<Impl: ICertificateRequestProperties4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuppressedDefaults() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubjectAlternativeName<Impl: ICertificateRequestProperties4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubjectAlternativeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extensions<Impl: ICertificateRequestProperties4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateRequestProperties4>, base.5, SuppressedDefaults::<Impl, OFFSET>, SubjectAlternativeName::<Impl, OFFSET>, Extensions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStoreImpl: Sized {
    fn Add(&self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
    fn Delete(&self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateStore";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateStoreVtbl {
    pub const fn new<Impl: ICertificateStoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateStoreVtbl {
        unsafe extern "system" fn Add<Impl: ICertificateStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Add(&*(&certificate as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Delete<Impl: ICertificateStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Delete(&*(&certificate as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateStore>, base.5, Add::<Impl, OFFSET>, Delete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStore2Impl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateStore2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateStore2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateStore2Vtbl {
    pub const fn new<Impl: ICertificateStore2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateStore2Vtbl {
        unsafe extern "system" fn Name<Impl: ICertificateStore2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateStore2>, base.5, Name::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStoresStaticsImpl: Sized {
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>;
    fn FindAllWithQueryAsync(&self, query: &::core::option::Option<CertificateQuery>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>;
    fn TrustedRootCertificationAuthorities(&self) -> ::windows::core::Result<CertificateStore>;
    fn IntermediateCertificationAuthorities(&self) -> ::windows::core::Result<CertificateStore>;
    fn GetStoreByName(&self, storename: &::windows::core::HSTRING) -> ::windows::core::Result<CertificateStore>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateStoresStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateStoresStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateStoresStaticsVtbl {
    pub const fn new<Impl: ICertificateStoresStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateStoresStaticsVtbl {
        unsafe extern "system" fn FindAllAsync<Impl: ICertificateStoresStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithQueryAsync<Impl: ICertificateStoresStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllWithQueryAsync(&*(&query as *const <CertificateQuery as ::windows::core::Abi>::Abi as *const <CertificateQuery as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrustedRootCertificationAuthorities<Impl: ICertificateStoresStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrustedRootCertificationAuthorities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntermediateCertificationAuthorities<Impl: ICertificateStoresStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IntermediateCertificationAuthorities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreByName<Impl: ICertificateStoresStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStoreByName(&*(&storename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateStoresStatics>, base.5, FindAllAsync::<Impl, OFFSET>, FindAllWithQueryAsync::<Impl, OFFSET>, TrustedRootCertificationAuthorities::<Impl, OFFSET>, IntermediateCertificationAuthorities::<Impl, OFFSET>, GetStoreByName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStoresStatics2Impl: Sized {
    fn GetUserStoreByName(&self, storename: &::windows::core::HSTRING) -> ::windows::core::Result<UserCertificateStore>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateStoresStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateStoresStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateStoresStatics2Vtbl {
    pub const fn new<Impl: ICertificateStoresStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICertificateStoresStatics2Vtbl {
        unsafe extern "system" fn GetUserStoreByName<Impl: ICertificateStoresStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUserStoreByName(&*(&storename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICertificateStoresStatics2>, base.5, GetUserStoreByName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChainBuildingParametersImpl: Sized {
    fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ValidationTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetValidationTimestamp(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn RevocationCheckEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetRevocationCheckEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn NetworkRetrievalEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetNetworkRetrievalEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AuthorityInformationAccessEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAuthorityInformationAccessEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentTimeValidationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetCurrentTimeValidationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExclusiveTrustRoots(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<Certificate>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChainBuildingParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IChainBuildingParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IChainBuildingParametersVtbl {
    pub const fn new<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IChainBuildingParametersVtbl {
        unsafe extern "system" fn EnhancedKeyUsages<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnhancedKeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidationTimestamp<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValidationTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValidationTimestamp<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValidationTimestamp(&*(&value as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevocationCheckEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevocationCheckEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevocationCheckEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRevocationCheckEnabled(value).into()
        }
        unsafe extern "system" fn NetworkRetrievalEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetworkRetrievalEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkRetrievalEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNetworkRetrievalEnabled(value).into()
        }
        unsafe extern "system" fn AuthorityInformationAccessEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthorityInformationAccessEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorityInformationAccessEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAuthorityInformationAccessEnabled(value).into()
        }
        unsafe extern "system" fn CurrentTimeValidationEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentTimeValidationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentTimeValidationEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCurrentTimeValidationEnabled(value).into()
        }
        unsafe extern "system" fn ExclusiveTrustRoots<Impl: IChainBuildingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExclusiveTrustRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IChainBuildingParameters>,
            base.5,
            EnhancedKeyUsages::<Impl, OFFSET>,
            ValidationTimestamp::<Impl, OFFSET>,
            SetValidationTimestamp::<Impl, OFFSET>,
            RevocationCheckEnabled::<Impl, OFFSET>,
            SetRevocationCheckEnabled::<Impl, OFFSET>,
            NetworkRetrievalEnabled::<Impl, OFFSET>,
            SetNetworkRetrievalEnabled::<Impl, OFFSET>,
            AuthorityInformationAccessEnabled::<Impl, OFFSET>,
            SetAuthorityInformationAccessEnabled::<Impl, OFFSET>,
            CurrentTimeValidationEnabled::<Impl, OFFSET>,
            SetCurrentTimeValidationEnabled::<Impl, OFFSET>,
            ExclusiveTrustRoots::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChainValidationParametersImpl: Sized {
    fn CertificateChainPolicy(&self) -> ::windows::core::Result<CertificateChainPolicy>;
    fn SetCertificateChainPolicy(&self, value: CertificateChainPolicy) -> ::windows::core::Result<()>;
    fn ServerDnsName(&self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn SetServerDnsName(&self, value: &::core::option::Option<super::super::super::Networking::HostName>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChainValidationParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IChainValidationParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IChainValidationParametersVtbl {
    pub const fn new<Impl: IChainValidationParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IChainValidationParametersVtbl {
        unsafe extern "system" fn CertificateChainPolicy<Impl: IChainValidationParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CertificateChainPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CertificateChainPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateChainPolicy<Impl: IChainValidationParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CertificateChainPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCertificateChainPolicy(value).into()
        }
        unsafe extern "system" fn ServerDnsName<Impl: IChainValidationParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerDnsName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerDnsName<Impl: IChainValidationParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetServerDnsName(&*(&value as *const <super::super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IChainValidationParameters>, base.5, CertificateChainPolicy::<Impl, OFFSET>, SetCertificateChainPolicy::<Impl, OFFSET>, ServerDnsName::<Impl, OFFSET>, SetServerDnsName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsAttachedSignatureImpl: Sized {
    fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
    fn Content(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn Signers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>;
    fn VerifySignature(&self) -> ::windows::core::Result<SignatureValidationResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICmsAttachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsAttachedSignature";
}
#[cfg(feature = "implement_exclusive")]
impl ICmsAttachedSignatureVtbl {
    pub const fn new<Impl: ICmsAttachedSignatureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICmsAttachedSignatureVtbl {
        unsafe extern "system" fn Certificates<Impl: ICmsAttachedSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Certificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: ICmsAttachedSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signers<Impl: ICmsAttachedSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Signers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifySignature<Impl: ICmsAttachedSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SignatureValidationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerifySignature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICmsAttachedSignature>, base.5, Certificates::<Impl, OFFSET>, Content::<Impl, OFFSET>, Signers::<Impl, OFFSET>, VerifySignature::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsAttachedSignatureFactoryImpl: Sized {
    fn CreateCmsAttachedSignature(&self, inputblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CmsAttachedSignature>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICmsAttachedSignatureFactory {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICmsAttachedSignatureFactoryVtbl {
    pub const fn new<Impl: ICmsAttachedSignatureFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICmsAttachedSignatureFactoryVtbl {
        unsafe extern "system" fn CreateCmsAttachedSignature<Impl: ICmsAttachedSignatureFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCmsAttachedSignature(&*(&inputblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICmsAttachedSignatureFactory>, base.5, CreateCmsAttachedSignature::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsAttachedSignatureStaticsImpl: Sized {
    fn GenerateSignatureAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, signers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICmsAttachedSignatureStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICmsAttachedSignatureStaticsVtbl {
    pub const fn new<Impl: ICmsAttachedSignatureStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICmsAttachedSignatureStaticsVtbl {
        unsafe extern "system" fn GenerateSignatureAsync<Impl: ICmsAttachedSignatureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, signers: ::windows::core::RawPtr, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateSignatureAsync(
                &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&signers as *const <super::super::super::Foundation::Collections::IIterable<CmsSignerInfo> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CmsSignerInfo> as ::windows::core::DefaultType>::DefaultType),
                &*(&certificates as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICmsAttachedSignatureStatics>, base.5, GenerateSignatureAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsDetachedSignatureImpl: Sized {
    fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
    fn Signers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>;
    fn VerifySignatureAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SignatureValidationResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICmsDetachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsDetachedSignature";
}
#[cfg(feature = "implement_exclusive")]
impl ICmsDetachedSignatureVtbl {
    pub const fn new<Impl: ICmsDetachedSignatureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICmsDetachedSignatureVtbl {
        unsafe extern "system" fn Certificates<Impl: ICmsDetachedSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Certificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signers<Impl: ICmsDetachedSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Signers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifySignatureAsync<Impl: ICmsDetachedSignatureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerifySignatureAsync(&*(&data as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICmsDetachedSignature>, base.5, Certificates::<Impl, OFFSET>, Signers::<Impl, OFFSET>, VerifySignatureAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsDetachedSignatureFactoryImpl: Sized {
    fn CreateCmsDetachedSignature(&self, inputblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CmsDetachedSignature>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICmsDetachedSignatureFactory {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICmsDetachedSignatureFactoryVtbl {
    pub const fn new<Impl: ICmsDetachedSignatureFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICmsDetachedSignatureFactoryVtbl {
        unsafe extern "system" fn CreateCmsDetachedSignature<Impl: ICmsDetachedSignatureFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCmsDetachedSignature(&*(&inputblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICmsDetachedSignatureFactory>, base.5, CreateCmsDetachedSignature::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsDetachedSignatureStaticsImpl: Sized {
    fn GenerateSignatureAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, signers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICmsDetachedSignatureStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICmsDetachedSignatureStaticsVtbl {
    pub const fn new<Impl: ICmsDetachedSignatureStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICmsDetachedSignatureStaticsVtbl {
        unsafe extern "system" fn GenerateSignatureAsync<Impl: ICmsDetachedSignatureStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, signers: ::windows::core::RawPtr, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateSignatureAsync(
                &*(&data as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType),
                &*(&signers as *const <super::super::super::Foundation::Collections::IIterable<CmsSignerInfo> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<CmsSignerInfo> as ::windows::core::DefaultType>::DefaultType),
                &*(&certificates as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICmsDetachedSignatureStatics>, base.5, GenerateSignatureAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsSignerInfoImpl: Sized {
    fn Certificate(&self) -> ::windows::core::Result<Certificate>;
    fn SetCertificate(&self, value: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
    fn HashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHashAlgorithmName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TimestampInfo(&self) -> ::windows::core::Result<CmsTimestampInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICmsSignerInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsSignerInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ICmsSignerInfoVtbl {
    pub const fn new<Impl: ICmsSignerInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICmsSignerInfoVtbl {
        unsafe extern "system" fn Certificate<Impl: ICmsSignerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Certificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificate<Impl: ICmsSignerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCertificate(&*(&value as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HashAlgorithmName<Impl: ICmsSignerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HashAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithmName<Impl: ICmsSignerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithmName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TimestampInfo<Impl: ICmsSignerInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimestampInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICmsSignerInfo>, base.5, Certificate::<Impl, OFFSET>, SetCertificate::<Impl, OFFSET>, HashAlgorithmName::<Impl, OFFSET>, SetHashAlgorithmName::<Impl, OFFSET>, TimestampInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsTimestampInfoImpl: Sized {
    fn SigningCertificate(&self) -> ::windows::core::Result<Certificate>;
    fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICmsTimestampInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsTimestampInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ICmsTimestampInfoVtbl {
    pub const fn new<Impl: ICmsTimestampInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICmsTimestampInfoVtbl {
        unsafe extern "system" fn SigningCertificate<Impl: ICmsTimestampInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SigningCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Certificates<Impl: ICmsTimestampInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Certificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ICmsTimestampInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICmsTimestampInfo>, base.5, SigningCertificate::<Impl, OFFSET>, Certificates::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAlgorithmNamesStaticsImpl: Sized {
    fn Rsa(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Dsa(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh521(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdsa256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdsa384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdsa521(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyAlgorithmNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyAlgorithmNamesStaticsVtbl {
    pub const fn new<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyAlgorithmNamesStaticsVtbl {
        unsafe extern "system" fn Rsa<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rsa() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dsa<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dsa() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdh256<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ecdh256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdh384<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ecdh384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdh521<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ecdh521() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdsa256<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ecdsa256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdsa384<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ecdsa384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdsa521<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ecdsa521() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyAlgorithmNamesStatics>, base.5, Rsa::<Impl, OFFSET>, Dsa::<Impl, OFFSET>, Ecdh256::<Impl, OFFSET>, Ecdh384::<Impl, OFFSET>, Ecdh521::<Impl, OFFSET>, Ecdsa256::<Impl, OFFSET>, Ecdsa384::<Impl, OFFSET>, Ecdsa521::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAlgorithmNamesStatics2Impl: Sized {
    fn Ecdsa(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyAlgorithmNamesStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyAlgorithmNamesStatics2Vtbl {
    pub const fn new<Impl: IKeyAlgorithmNamesStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyAlgorithmNamesStatics2Vtbl {
        unsafe extern "system" fn Ecdsa<Impl: IKeyAlgorithmNamesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ecdsa() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdh<Impl: IKeyAlgorithmNamesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ecdh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyAlgorithmNamesStatics2>, base.5, Ecdsa::<Impl, OFFSET>, Ecdh::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAttestationHelperStaticsImpl: Sized {
    fn DecryptTpmAttestationCredentialAsync(&self, credential: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetTpmAttestationCredentialId(&self, credential: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyAttestationHelperStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyAttestationHelperStaticsVtbl {
    pub const fn new<Impl: IKeyAttestationHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyAttestationHelperStaticsVtbl {
        unsafe extern "system" fn DecryptTpmAttestationCredentialAsync<Impl: IKeyAttestationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecryptTpmAttestationCredentialAsync(&*(&credential as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTpmAttestationCredentialId<Impl: IKeyAttestationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTpmAttestationCredentialId(&*(&credential as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyAttestationHelperStatics>, base.5, DecryptTpmAttestationCredentialAsync::<Impl, OFFSET>, GetTpmAttestationCredentialId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAttestationHelperStatics2Impl: Sized {
    fn DecryptTpmAttestationCredentialWithContainerNameAsync(&self, credential: &::windows::core::HSTRING, containername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyAttestationHelperStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyAttestationHelperStatics2Vtbl {
    pub const fn new<Impl: IKeyAttestationHelperStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyAttestationHelperStatics2Vtbl {
        unsafe extern "system" fn DecryptTpmAttestationCredentialWithContainerNameAsync<Impl: IKeyAttestationHelperStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, containername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecryptTpmAttestationCredentialWithContainerNameAsync(&*(&credential as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&containername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyAttestationHelperStatics2>, base.5, DecryptTpmAttestationCredentialWithContainerNameAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyStorageProviderNamesStaticsImpl: Sized {
    fn SoftwareKeyStorageProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SmartcardKeyStorageProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PlatformKeyStorageProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyStorageProviderNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyStorageProviderNamesStaticsVtbl {
    pub const fn new<Impl: IKeyStorageProviderNamesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyStorageProviderNamesStaticsVtbl {
        unsafe extern "system" fn SoftwareKeyStorageProvider<Impl: IKeyStorageProviderNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoftwareKeyStorageProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmartcardKeyStorageProvider<Impl: IKeyStorageProviderNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmartcardKeyStorageProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlatformKeyStorageProvider<Impl: IKeyStorageProviderNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlatformKeyStorageProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyStorageProviderNamesStatics>, base.5, SoftwareKeyStorageProvider::<Impl, OFFSET>, SmartcardKeyStorageProvider::<Impl, OFFSET>, PlatformKeyStorageProvider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyStorageProviderNamesStatics2Impl: Sized {
    fn PassportKeyStorageProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyStorageProviderNamesStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyStorageProviderNamesStatics2Vtbl {
    pub const fn new<Impl: IKeyStorageProviderNamesStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyStorageProviderNamesStatics2Vtbl {
        unsafe extern "system" fn PassportKeyStorageProvider<Impl: IKeyStorageProviderNamesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PassportKeyStorageProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyStorageProviderNamesStatics2>, base.5, PassportKeyStorageProvider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPfxImportParametersImpl: Sized {
    fn Exportable(&self) -> ::windows::core::Result<ExportOption>;
    fn SetExportable(&self, value: ExportOption) -> ::windows::core::Result<()>;
    fn KeyProtectionLevel(&self) -> ::windows::core::Result<KeyProtectionLevel>;
    fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> ::windows::core::Result<()>;
    fn InstallOptions(&self) -> ::windows::core::Result<InstallOptions>;
    fn SetInstallOptions(&self, value: InstallOptions) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyStorageProviderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContainerNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ReaderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetReaderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPfxImportParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IPfxImportParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IPfxImportParametersVtbl {
    pub const fn new<Impl: IPfxImportParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPfxImportParametersVtbl {
        unsafe extern "system" fn Exportable<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ExportOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Exportable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExportable<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ExportOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExportable(value).into()
        }
        unsafe extern "system" fn KeyProtectionLevel<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut KeyProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyProtectionLevel<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: KeyProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyProtectionLevel(value).into()
        }
        unsafe extern "system" fn InstallOptions<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InstallOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInstallOptions<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InstallOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInstallOptions(value).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyStorageProviderName<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyStorageProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyStorageProviderName<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyStorageProviderName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContainerNamePrefix<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainerNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerNamePrefix<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContainerNamePrefix(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReaderName<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReaderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReaderName<Impl: IPfxImportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReaderName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPfxImportParameters>,
            base.5,
            Exportable::<Impl, OFFSET>,
            SetExportable::<Impl, OFFSET>,
            KeyProtectionLevel::<Impl, OFFSET>,
            SetKeyProtectionLevel::<Impl, OFFSET>,
            InstallOptions::<Impl, OFFSET>,
            SetInstallOptions::<Impl, OFFSET>,
            FriendlyName::<Impl, OFFSET>,
            SetFriendlyName::<Impl, OFFSET>,
            KeyStorageProviderName::<Impl, OFFSET>,
            SetKeyStorageProviderName::<Impl, OFFSET>,
            ContainerNamePrefix::<Impl, OFFSET>,
            SetContainerNamePrefix::<Impl, OFFSET>,
            ReaderName::<Impl, OFFSET>,
            SetReaderName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardCertificateStoreNamesStaticsImpl: Sized {
    fn Personal(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrustedRootCertificationAuthorities(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IntermediateCertificationAuthorities(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardCertificateStoreNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IStandardCertificateStoreNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardCertificateStoreNamesStaticsVtbl {
    pub const fn new<Impl: IStandardCertificateStoreNamesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStandardCertificateStoreNamesStaticsVtbl {
        unsafe extern "system" fn Personal<Impl: IStandardCertificateStoreNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Personal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrustedRootCertificationAuthorities<Impl: IStandardCertificateStoreNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrustedRootCertificationAuthorities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntermediateCertificationAuthorities<Impl: IStandardCertificateStoreNamesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IntermediateCertificationAuthorities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStandardCertificateStoreNamesStatics>, base.5, Personal::<Impl, OFFSET>, TrustedRootCertificationAuthorities::<Impl, OFFSET>, IntermediateCertificationAuthorities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISubjectAlternativeNameInfoImpl: Sized {
    fn EmailName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn IPAddress(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Url(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DnsName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DistinguishedName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn PrincipalName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISubjectAlternativeNameInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISubjectAlternativeNameInfoVtbl {
    pub const fn new<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISubjectAlternativeNameInfoVtbl {
        unsafe extern "system" fn EmailName<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EmailName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPAddress<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IPAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Url<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DnsName<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DnsName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistinguishedName<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DistinguishedName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrincipalName<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrincipalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISubjectAlternativeNameInfo>, base.5, EmailName::<Impl, OFFSET>, IPAddress::<Impl, OFFSET>, Url::<Impl, OFFSET>, DnsName::<Impl, OFFSET>, DistinguishedName::<Impl, OFFSET>, PrincipalName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISubjectAlternativeNameInfo2Impl: Sized {
    fn EmailNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IPAddresses(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Urls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DnsNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DistinguishedNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PrincipalNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Extension(&self) -> ::windows::core::Result<CertificateExtension>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISubjectAlternativeNameInfo2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl ISubjectAlternativeNameInfo2Vtbl {
    pub const fn new<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISubjectAlternativeNameInfo2Vtbl {
        unsafe extern "system" fn EmailNames<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EmailNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPAddresses<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IPAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Urls<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Urls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DnsNames<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DnsNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistinguishedNames<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DistinguishedNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrincipalNames<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrincipalNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISubjectAlternativeNameInfo2>, base.5, EmailNames::<Impl, OFFSET>, IPAddresses::<Impl, OFFSET>, Urls::<Impl, OFFSET>, DnsNames::<Impl, OFFSET>, DistinguishedNames::<Impl, OFFSET>, PrincipalNames::<Impl, OFFSET>, Extension::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserCertificateEnrollmentManagerImpl: Sized {
    fn CreateRequestAsync(&self, request: &::core::option::Option<CertificateRequestProperties>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn InstallCertificateAsync(&self, certificate: &::windows::core::HSTRING, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportPfxDataAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportPfxDataToKspAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING, keystorageprovider: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserCertificateEnrollmentManager {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager";
}
#[cfg(feature = "implement_exclusive")]
impl IUserCertificateEnrollmentManagerVtbl {
    pub const fn new<Impl: IUserCertificateEnrollmentManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserCertificateEnrollmentManagerVtbl {
        unsafe extern "system" fn CreateRequestAsync<Impl: IUserCertificateEnrollmentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRequestAsync(&*(&request as *const <CertificateRequestProperties as ::windows::core::Abi>::Abi as *const <CertificateRequestProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallCertificateAsync<Impl: IUserCertificateEnrollmentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoption: InstallOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallCertificateAsync(&*(&certificate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), installoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportPfxDataAsync<Impl: IUserCertificateEnrollmentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImportPfxDataAsync(
                &*(&pfxdata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                exportable,
                keyprotectionlevel,
                installoption,
                &*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportPfxDataToKspAsync<Impl: IUserCertificateEnrollmentManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keystorageprovider: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImportPfxDataToKspAsync(
                &*(&pfxdata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                exportable,
                keyprotectionlevel,
                installoption,
                &*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&keystorageprovider as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserCertificateEnrollmentManager>, base.5, CreateRequestAsync::<Impl, OFFSET>, InstallCertificateAsync::<Impl, OFFSET>, ImportPfxDataAsync::<Impl, OFFSET>, ImportPfxDataToKspAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserCertificateEnrollmentManager2Impl: Sized {
    fn ImportPfxDataToKspWithParametersAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, pfximportparameters: &::core::option::Option<PfxImportParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserCertificateEnrollmentManager2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IUserCertificateEnrollmentManager2Vtbl {
    pub const fn new<Impl: IUserCertificateEnrollmentManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserCertificateEnrollmentManager2Vtbl {
        unsafe extern "system" fn ImportPfxDataToKspWithParametersAsync<Impl: IUserCertificateEnrollmentManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pfximportparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImportPfxDataToKspWithParametersAsync(
                &*(&pfxdata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&pfximportparameters as *const <PfxImportParameters as ::windows::core::Abi>::Abi as *const <PfxImportParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserCertificateEnrollmentManager2>, base.5, ImportPfxDataToKspWithParametersAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserCertificateStoreImpl: Sized {
    fn RequestAddAsync(&self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsync(&self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserCertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IUserCertificateStore";
}
#[cfg(feature = "implement_exclusive")]
impl IUserCertificateStoreVtbl {
    pub const fn new<Impl: IUserCertificateStoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserCertificateStoreVtbl {
        unsafe extern "system" fn RequestAddAsync<Impl: IUserCertificateStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAddAsync(&*(&certificate as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDeleteAsync<Impl: IUserCertificateStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestDeleteAsync(&*(&certificate as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IUserCertificateStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserCertificateStore>, base.5, RequestAddAsync::<Impl, OFFSET>, RequestDeleteAsync::<Impl, OFFSET>, Name::<Impl, OFFSET>)
    }
}
