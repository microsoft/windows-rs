#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICertificateImpl: Sized {
    fn BuildChainAsync(&mut self, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>;
    fn BuildChainWithParametersAsync(&mut self, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>, parameters: &::core::option::Option<ChainBuildingParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>;
    fn SerialNumber(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn GetHashValue(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn GetHashValueWithAlgorithm(&mut self, hashalgorithmname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn GetCertificateBlob(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Subject(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Issuer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasPrivateKey(&mut self) -> ::windows::core::Result<bool>;
    fn IsStronglyProtected(&mut self) -> ::windows::core::Result<bool>;
    fn ValidFrom(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ValidTo(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn EnhancedKeyUsages(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SetFriendlyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICertificate {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificate";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICertificateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateVtbl {
        unsafe extern "system" fn BuildChainAsync<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildChainAsync(&*(&certificates as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildChainWithParametersAsync<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificates: ::windows::core::RawPtr, parameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildChainWithParametersAsync(&*(&certificates as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<Certificate> as ::windows::core::DefaultType>::DefaultType), &*(&parameters as *const <ChainBuildingParameters as ::windows::core::Abi>::Abi as *const <ChainBuildingParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn GetHashValue<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn GetHashValueWithAlgorithm<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn GetCertificateBlob<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificateBlob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Issuer<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Issuer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasPrivateKey<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasPrivateKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStronglyProtected<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStronglyProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidFrom<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidTo<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidTo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnhancedKeyUsages<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnhancedKeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificate, BASE_OFFSET>(),
            BuildChainAsync: BuildChainAsync::<Impl, IMPL_OFFSET>,
            BuildChainWithParametersAsync: BuildChainWithParametersAsync::<Impl, IMPL_OFFSET>,
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
            GetHashValue: GetHashValue::<Impl, IMPL_OFFSET>,
            GetHashValueWithAlgorithm: GetHashValueWithAlgorithm::<Impl, IMPL_OFFSET>,
            GetCertificateBlob: GetCertificateBlob::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            Issuer: Issuer::<Impl, IMPL_OFFSET>,
            HasPrivateKey: HasPrivateKey::<Impl, IMPL_OFFSET>,
            IsStronglyProtected: IsStronglyProtected::<Impl, IMPL_OFFSET>,
            ValidFrom: ValidFrom::<Impl, IMPL_OFFSET>,
            ValidTo: ValidTo::<Impl, IMPL_OFFSET>,
            EnhancedKeyUsages: EnhancedKeyUsages::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificate2Impl: Sized {
    fn IsSecurityDeviceBound(&mut self) -> ::windows::core::Result<bool>;
    fn KeyUsages(&mut self) -> ::windows::core::Result<CertificateKeyUsages>;
    fn KeyAlgorithmName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignatureAlgorithmName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignatureHashAlgorithmName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubjectAlternativeName(&mut self) -> ::windows::core::Result<SubjectAlternativeNameInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificate2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificate2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificate2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificate2Vtbl {
        unsafe extern "system" fn IsSecurityDeviceBound<Impl: ICertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecurityDeviceBound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUsages<Impl: ICertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyAlgorithmName<Impl: ICertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignatureAlgorithmName<Impl: ICertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignatureAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignatureHashAlgorithmName<Impl: ICertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignatureHashAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubjectAlternativeName<Impl: ICertificate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubjectAlternativeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificate2, BASE_OFFSET>(),
            IsSecurityDeviceBound: IsSecurityDeviceBound::<Impl, IMPL_OFFSET>,
            KeyUsages: KeyUsages::<Impl, IMPL_OFFSET>,
            KeyAlgorithmName: KeyAlgorithmName::<Impl, IMPL_OFFSET>,
            SignatureAlgorithmName: SignatureAlgorithmName::<Impl, IMPL_OFFSET>,
            SignatureHashAlgorithmName: SignatureHashAlgorithmName::<Impl, IMPL_OFFSET>,
            SubjectAlternativeName: SubjectAlternativeName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificate3Impl: Sized {
    fn IsPerUser(&mut self) -> ::windows::core::Result<bool>;
    fn StoreName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyStorageProviderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificate3 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificate3";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificate3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificate3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificate3Vtbl {
        unsafe extern "system" fn IsPerUser<Impl: ICertificate3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPerUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreName<Impl: ICertificate3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStorageProviderName<Impl: ICertificate3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyStorageProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificate3, BASE_OFFSET>(),
            IsPerUser: IsPerUser::<Impl, IMPL_OFFSET>,
            StoreName: StoreName::<Impl, IMPL_OFFSET>,
            KeyStorageProviderName: KeyStorageProviderName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificate3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICertificateChainImpl: Sized {
    fn Validate(&mut self) -> ::windows::core::Result<ChainValidationResult>;
    fn ValidateWithParameters(&mut self, parameter: &::core::option::Option<ChainValidationParameters>) -> ::windows::core::Result<ChainValidationResult>;
    fn GetCertificates(&mut self, includeroot: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICertificateChain {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateChain";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICertificateChainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateChainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateChainVtbl {
        unsafe extern "system" fn Validate<Impl: ICertificateChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ChainValidationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Validate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateWithParameters<Impl: ICertificateChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: ::windows::core::RawPtr, result__: *mut ChainValidationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateWithParameters(&*(&parameter as *const <ChainValidationParameters as ::windows::core::Abi>::Abi as *const <ChainValidationParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificates<Impl: ICertificateChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeroot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertificates(includeroot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateChain, BASE_OFFSET>(),
            Validate: Validate::<Impl, IMPL_OFFSET>,
            ValidateWithParameters: ValidateWithParameters::<Impl, IMPL_OFFSET>,
            GetCertificates: GetCertificates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateChain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICertificateEnrollmentManagerStaticsImpl: Sized {
    fn CreateRequestAsync(&mut self, request: &::core::option::Option<CertificateRequestProperties>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn InstallCertificateAsync(&mut self, certificate: &::windows::core::HSTRING, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportPfxDataAsync(&mut self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICertificateEnrollmentManagerStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICertificateEnrollmentManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateEnrollmentManagerStaticsVtbl {
        unsafe extern "system" fn CreateRequestAsync<Impl: ICertificateEnrollmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRequestAsync(&*(&request as *const <CertificateRequestProperties as ::windows::core::Abi>::Abi as *const <CertificateRequestProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallCertificateAsync<Impl: ICertificateEnrollmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoption: InstallOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallCertificateAsync(&*(&certificate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), installoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportPfxDataAsync<Impl: ICertificateEnrollmentManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateEnrollmentManagerStatics, BASE_OFFSET>(),
            CreateRequestAsync: CreateRequestAsync::<Impl, IMPL_OFFSET>,
            InstallCertificateAsync: InstallCertificateAsync::<Impl, IMPL_OFFSET>,
            ImportPfxDataAsync: ImportPfxDataAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateEnrollmentManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICertificateEnrollmentManagerStatics2Impl: Sized {
    fn UserCertificateEnrollmentManager(&mut self) -> ::windows::core::Result<UserCertificateEnrollmentManager>;
    fn ImportPfxDataToKspAsync(&mut self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING, keystorageprovider: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICertificateEnrollmentManagerStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICertificateEnrollmentManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateEnrollmentManagerStatics2Vtbl {
        unsafe extern "system" fn UserCertificateEnrollmentManager<Impl: ICertificateEnrollmentManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserCertificateEnrollmentManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportPfxDataToKspAsync<Impl: ICertificateEnrollmentManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keystorageprovider: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateEnrollmentManagerStatics2, BASE_OFFSET>(),
            UserCertificateEnrollmentManager: UserCertificateEnrollmentManager::<Impl, IMPL_OFFSET>,
            ImportPfxDataToKspAsync: ImportPfxDataToKspAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateEnrollmentManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICertificateEnrollmentManagerStatics3Impl: Sized {
    fn ImportPfxDataToKspWithParametersAsync(&mut self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, pfximportparameters: &::core::option::Option<PfxImportParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICertificateEnrollmentManagerStatics3 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateEnrollmentManagerStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICertificateEnrollmentManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateEnrollmentManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateEnrollmentManagerStatics3Vtbl {
        unsafe extern "system" fn ImportPfxDataToKspWithParametersAsync<Impl: ICertificateEnrollmentManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pfximportparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateEnrollmentManagerStatics3, BASE_OFFSET>(),
            ImportPfxDataToKspWithParametersAsync: ImportPfxDataToKspWithParametersAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateEnrollmentManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateExtensionImpl: Sized {
    fn ObjectId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetObjectId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCritical(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCritical(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn EncodeValue(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetValue(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateExtension {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateExtension";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateExtensionVtbl {
        unsafe extern "system" fn ObjectId<Impl: ICertificateExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectId<Impl: ICertificateExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCritical<Impl: ICertificateExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCritical() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCritical<Impl: ICertificateExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCritical(value).into()
        }
        unsafe extern "system" fn EncodeValue<Impl: ICertificateExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncodeValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: ICertificateExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn SetValue<Impl: ICertificateExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateExtension, BASE_OFFSET>(),
            ObjectId: ObjectId::<Impl, IMPL_OFFSET>,
            SetObjectId: SetObjectId::<Impl, IMPL_OFFSET>,
            IsCritical: IsCritical::<Impl, IMPL_OFFSET>,
            SetIsCritical: SetIsCritical::<Impl, IMPL_OFFSET>,
            EncodeValue: EncodeValue::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICertificateFactoryImpl: Sized {
    fn CreateCertificate(&mut self, certblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<Certificate>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICertificateFactory {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICertificateFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateFactoryVtbl {
        unsafe extern "system" fn CreateCertificate<Impl: ICertificateFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCertificate(&*(&certblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateFactory, BASE_OFFSET>(),
            CreateCertificate: CreateCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateKeyUsagesImpl: Sized {
    fn EncipherOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetEncipherOnly(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CrlSign(&mut self) -> ::windows::core::Result<bool>;
    fn SetCrlSign(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn KeyCertificateSign(&mut self) -> ::windows::core::Result<bool>;
    fn SetKeyCertificateSign(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn KeyAgreement(&mut self) -> ::windows::core::Result<bool>;
    fn SetKeyAgreement(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DataEncipherment(&mut self) -> ::windows::core::Result<bool>;
    fn SetDataEncipherment(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn KeyEncipherment(&mut self) -> ::windows::core::Result<bool>;
    fn SetKeyEncipherment(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn NonRepudiation(&mut self) -> ::windows::core::Result<bool>;
    fn SetNonRepudiation(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DigitalSignature(&mut self) -> ::windows::core::Result<bool>;
    fn SetDigitalSignature(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateKeyUsages {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateKeyUsages";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateKeyUsagesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateKeyUsagesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateKeyUsagesVtbl {
        unsafe extern "system" fn EncipherOnly<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncipherOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncipherOnly<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncipherOnly(value).into()
        }
        unsafe extern "system" fn CrlSign<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CrlSign() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCrlSign<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCrlSign(value).into()
        }
        unsafe extern "system" fn KeyCertificateSign<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyCertificateSign() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyCertificateSign<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyCertificateSign(value).into()
        }
        unsafe extern "system" fn KeyAgreement<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyAgreement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyAgreement<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyAgreement(value).into()
        }
        unsafe extern "system" fn DataEncipherment<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataEncipherment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataEncipherment<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataEncipherment(value).into()
        }
        unsafe extern "system" fn KeyEncipherment<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyEncipherment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyEncipherment<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyEncipherment(value).into()
        }
        unsafe extern "system" fn NonRepudiation<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonRepudiation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonRepudiation<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNonRepudiation(value).into()
        }
        unsafe extern "system" fn DigitalSignature<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DigitalSignature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigitalSignature<Impl: ICertificateKeyUsagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDigitalSignature(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateKeyUsages, BASE_OFFSET>(),
            EncipherOnly: EncipherOnly::<Impl, IMPL_OFFSET>,
            SetEncipherOnly: SetEncipherOnly::<Impl, IMPL_OFFSET>,
            CrlSign: CrlSign::<Impl, IMPL_OFFSET>,
            SetCrlSign: SetCrlSign::<Impl, IMPL_OFFSET>,
            KeyCertificateSign: KeyCertificateSign::<Impl, IMPL_OFFSET>,
            SetKeyCertificateSign: SetKeyCertificateSign::<Impl, IMPL_OFFSET>,
            KeyAgreement: KeyAgreement::<Impl, IMPL_OFFSET>,
            SetKeyAgreement: SetKeyAgreement::<Impl, IMPL_OFFSET>,
            DataEncipherment: DataEncipherment::<Impl, IMPL_OFFSET>,
            SetDataEncipherment: SetDataEncipherment::<Impl, IMPL_OFFSET>,
            KeyEncipherment: KeyEncipherment::<Impl, IMPL_OFFSET>,
            SetKeyEncipherment: SetKeyEncipherment::<Impl, IMPL_OFFSET>,
            NonRepudiation: NonRepudiation::<Impl, IMPL_OFFSET>,
            SetNonRepudiation: SetNonRepudiation::<Impl, IMPL_OFFSET>,
            DigitalSignature: DigitalSignature::<Impl, IMPL_OFFSET>,
            SetDigitalSignature: SetDigitalSignature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateKeyUsages as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICertificateQueryImpl: Sized {
    fn EnhancedKeyUsages(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IssuerName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIssuerName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbprint(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetThumbprint(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn HardwareOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetHardwareOnly(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICertificateQuery {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateQuery";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICertificateQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateQueryVtbl {
        unsafe extern "system" fn EnhancedKeyUsages<Impl: ICertificateQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnhancedKeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssuerName<Impl: ICertificateQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IssuerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIssuerName<Impl: ICertificateQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIssuerName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertificateQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: ICertificateQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbprint<Impl: ICertificateQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn SetThumbprint<Impl: ICertificateQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbprint(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn HardwareOnly<Impl: ICertificateQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHardwareOnly<Impl: ICertificateQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHardwareOnly(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateQuery, BASE_OFFSET>(),
            EnhancedKeyUsages: EnhancedKeyUsages::<Impl, IMPL_OFFSET>,
            IssuerName: IssuerName::<Impl, IMPL_OFFSET>,
            SetIssuerName: SetIssuerName::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            Thumbprint: Thumbprint::<Impl, IMPL_OFFSET>,
            SetThumbprint: SetThumbprint::<Impl, IMPL_OFFSET>,
            HardwareOnly: HardwareOnly::<Impl, IMPL_OFFSET>,
            SetHardwareOnly: SetHardwareOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateQuery2Impl: Sized {
    fn IncludeDuplicates(&mut self) -> ::windows::core::Result<bool>;
    fn SetIncludeDuplicates(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeExpiredCertificates(&mut self) -> ::windows::core::Result<bool>;
    fn SetIncludeExpiredCertificates(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StoreName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStoreName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateQuery2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateQuery2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateQuery2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateQuery2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateQuery2Vtbl {
        unsafe extern "system" fn IncludeDuplicates<Impl: ICertificateQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeDuplicates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeDuplicates<Impl: ICertificateQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeDuplicates(value).into()
        }
        unsafe extern "system" fn IncludeExpiredCertificates<Impl: ICertificateQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeExpiredCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeExpiredCertificates<Impl: ICertificateQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeExpiredCertificates(value).into()
        }
        unsafe extern "system" fn StoreName<Impl: ICertificateQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoreName<Impl: ICertificateQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoreName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateQuery2, BASE_OFFSET>(),
            IncludeDuplicates: IncludeDuplicates::<Impl, IMPL_OFFSET>,
            SetIncludeDuplicates: SetIncludeDuplicates::<Impl, IMPL_OFFSET>,
            IncludeExpiredCertificates: IncludeExpiredCertificates::<Impl, IMPL_OFFSET>,
            SetIncludeExpiredCertificates: SetIncludeExpiredCertificates::<Impl, IMPL_OFFSET>,
            StoreName: StoreName::<Impl, IMPL_OFFSET>,
            SetStoreName: SetStoreName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateQuery2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestPropertiesImpl: Sized {
    fn Subject(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn KeyAlgorithmName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyAlgorithmName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn KeySize(&mut self) -> ::windows::core::Result<u32>;
    fn SetKeySize(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HashAlgorithmName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHashAlgorithmName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Exportable(&mut self) -> ::windows::core::Result<ExportOption>;
    fn SetExportable(&mut self, value: ExportOption) -> ::windows::core::Result<()>;
    fn KeyUsages(&mut self) -> ::windows::core::Result<EnrollKeyUsages>;
    fn SetKeyUsages(&mut self, value: EnrollKeyUsages) -> ::windows::core::Result<()>;
    fn KeyProtectionLevel(&mut self) -> ::windows::core::Result<KeyProtectionLevel>;
    fn SetKeyProtectionLevel(&mut self, value: KeyProtectionLevel) -> ::windows::core::Result<()>;
    fn KeyStorageProviderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyStorageProviderName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateRequestProperties {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateRequestProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateRequestPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateRequestPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateRequestPropertiesVtbl {
        unsafe extern "system" fn Subject<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyAlgorithmName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyAlgorithmName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyAlgorithmName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeySize<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeySize<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeySize(value).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HashAlgorithmName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithmName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithmName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Exportable<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ExportOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exportable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExportable<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ExportOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExportable(value).into()
        }
        unsafe extern "system" fn KeyUsages<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut EnrollKeyUsages) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyUsages<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: EnrollKeyUsages) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyUsages(value).into()
        }
        unsafe extern "system" fn KeyProtectionLevel<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyProtectionLevel<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyProtectionLevel(value).into()
        }
        unsafe extern "system" fn KeyStorageProviderName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyStorageProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyStorageProviderName<Impl: ICertificateRequestPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyStorageProviderName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateRequestProperties, BASE_OFFSET>(),
            Subject: Subject::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            KeyAlgorithmName: KeyAlgorithmName::<Impl, IMPL_OFFSET>,
            SetKeyAlgorithmName: SetKeyAlgorithmName::<Impl, IMPL_OFFSET>,
            KeySize: KeySize::<Impl, IMPL_OFFSET>,
            SetKeySize: SetKeySize::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            HashAlgorithmName: HashAlgorithmName::<Impl, IMPL_OFFSET>,
            SetHashAlgorithmName: SetHashAlgorithmName::<Impl, IMPL_OFFSET>,
            Exportable: Exportable::<Impl, IMPL_OFFSET>,
            SetExportable: SetExportable::<Impl, IMPL_OFFSET>,
            KeyUsages: KeyUsages::<Impl, IMPL_OFFSET>,
            SetKeyUsages: SetKeyUsages::<Impl, IMPL_OFFSET>,
            KeyProtectionLevel: KeyProtectionLevel::<Impl, IMPL_OFFSET>,
            SetKeyProtectionLevel: SetKeyProtectionLevel::<Impl, IMPL_OFFSET>,
            KeyStorageProviderName: KeyStorageProviderName::<Impl, IMPL_OFFSET>,
            SetKeyStorageProviderName: SetKeyStorageProviderName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateRequestProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestProperties2Impl: Sized {
    fn SmartcardReaderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSmartcardReaderName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SigningCertificate(&mut self) -> ::windows::core::Result<Certificate>;
    fn SetSigningCertificate(&mut self, value: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
    fn AttestationCredentialCertificate(&mut self) -> ::windows::core::Result<Certificate>;
    fn SetAttestationCredentialCertificate(&mut self, value: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateRequestProperties2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateRequestProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateRequestProperties2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateRequestProperties2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateRequestProperties2Vtbl {
        unsafe extern "system" fn SmartcardReaderName<Impl: ICertificateRequestProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartcardReaderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmartcardReaderName<Impl: ICertificateRequestProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmartcardReaderName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SigningCertificate<Impl: ICertificateRequestProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SigningCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningCertificate<Impl: ICertificateRequestProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSigningCertificate(&*(&value as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AttestationCredentialCertificate<Impl: ICertificateRequestProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttestationCredentialCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttestationCredentialCertificate<Impl: ICertificateRequestProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttestationCredentialCertificate(&*(&value as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateRequestProperties2, BASE_OFFSET>(),
            SmartcardReaderName: SmartcardReaderName::<Impl, IMPL_OFFSET>,
            SetSmartcardReaderName: SetSmartcardReaderName::<Impl, IMPL_OFFSET>,
            SigningCertificate: SigningCertificate::<Impl, IMPL_OFFSET>,
            SetSigningCertificate: SetSigningCertificate::<Impl, IMPL_OFFSET>,
            AttestationCredentialCertificate: AttestationCredentialCertificate::<Impl, IMPL_OFFSET>,
            SetAttestationCredentialCertificate: SetAttestationCredentialCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateRequestProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestProperties3Impl: Sized {
    fn CurveName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurveName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CurveParameters(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetCurveParameters(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ContainerNamePrefix(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContainerNamePrefix(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainerName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContainerName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UseExistingKey(&mut self) -> ::windows::core::Result<bool>;
    fn SetUseExistingKey(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateRequestProperties3 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateRequestProperties3";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateRequestProperties3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateRequestProperties3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateRequestProperties3Vtbl {
        unsafe extern "system" fn CurveName<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurveName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurveName<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurveName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurveParameters<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn SetCurveParameters<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurveParameters(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn ContainerNamePrefix<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerNamePrefix<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainerNamePrefix(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContainerName<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainerName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UseExistingKey<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseExistingKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseExistingKey<Impl: ICertificateRequestProperties3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseExistingKey(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateRequestProperties3, BASE_OFFSET>(),
            CurveName: CurveName::<Impl, IMPL_OFFSET>,
            SetCurveName: SetCurveName::<Impl, IMPL_OFFSET>,
            CurveParameters: CurveParameters::<Impl, IMPL_OFFSET>,
            SetCurveParameters: SetCurveParameters::<Impl, IMPL_OFFSET>,
            ContainerNamePrefix: ContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetContainerNamePrefix: SetContainerNamePrefix::<Impl, IMPL_OFFSET>,
            ContainerName: ContainerName::<Impl, IMPL_OFFSET>,
            SetContainerName: SetContainerName::<Impl, IMPL_OFFSET>,
            UseExistingKey: UseExistingKey::<Impl, IMPL_OFFSET>,
            SetUseExistingKey: SetUseExistingKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateRequestProperties3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICertificateRequestProperties4Impl: Sized {
    fn SuppressedDefaults(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SubjectAlternativeName(&mut self) -> ::windows::core::Result<SubjectAlternativeNameInfo>;
    fn Extensions(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<CertificateExtension>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICertificateRequestProperties4 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateRequestProperties4";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICertificateRequestProperties4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateRequestProperties4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateRequestProperties4Vtbl {
        unsafe extern "system" fn SuppressedDefaults<Impl: ICertificateRequestProperties4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressedDefaults() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubjectAlternativeName<Impl: ICertificateRequestProperties4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubjectAlternativeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extensions<Impl: ICertificateRequestProperties4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateRequestProperties4, BASE_OFFSET>(),
            SuppressedDefaults: SuppressedDefaults::<Impl, IMPL_OFFSET>,
            SubjectAlternativeName: SubjectAlternativeName::<Impl, IMPL_OFFSET>,
            Extensions: Extensions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateRequestProperties4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStoreImpl: Sized {
    fn Add(&mut self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
    fn Delete(&mut self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateStore";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateStoreVtbl {
        unsafe extern "system" fn Add<Impl: ICertificateStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(&*(&certificate as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Delete<Impl: ICertificateStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(&*(&certificate as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateStore, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStore2Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateStore2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateStore2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateStore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateStore2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateStore2Vtbl {
        unsafe extern "system" fn Name<Impl: ICertificateStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateStore2, BASE_OFFSET>(), Name: Name::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateStore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICertificateStoresStaticsImpl: Sized {
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>;
    fn FindAllWithQueryAsync(&mut self, query: &::core::option::Option<CertificateQuery>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>;
    fn TrustedRootCertificationAuthorities(&mut self) -> ::windows::core::Result<CertificateStore>;
    fn IntermediateCertificationAuthorities(&mut self) -> ::windows::core::Result<CertificateStore>;
    fn GetStoreByName(&mut self, storename: &::windows::core::HSTRING) -> ::windows::core::Result<CertificateStore>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICertificateStoresStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateStoresStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICertificateStoresStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateStoresStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateStoresStaticsVtbl {
        unsafe extern "system" fn FindAllAsync<Impl: ICertificateStoresStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithQueryAsync<Impl: ICertificateStoresStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllWithQueryAsync(&*(&query as *const <CertificateQuery as ::windows::core::Abi>::Abi as *const <CertificateQuery as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrustedRootCertificationAuthorities<Impl: ICertificateStoresStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrustedRootCertificationAuthorities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntermediateCertificationAuthorities<Impl: ICertificateStoresStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntermediateCertificationAuthorities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreByName<Impl: ICertificateStoresStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoreByName(&*(&storename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateStoresStatics, BASE_OFFSET>(),
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FindAllWithQueryAsync: FindAllWithQueryAsync::<Impl, IMPL_OFFSET>,
            TrustedRootCertificationAuthorities: TrustedRootCertificationAuthorities::<Impl, IMPL_OFFSET>,
            IntermediateCertificationAuthorities: IntermediateCertificationAuthorities::<Impl, IMPL_OFFSET>,
            GetStoreByName: GetStoreByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateStoresStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStoresStatics2Impl: Sized {
    fn GetUserStoreByName(&mut self, storename: &::windows::core::HSTRING) -> ::windows::core::Result<UserCertificateStore>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICertificateStoresStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateStoresStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ICertificateStoresStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICertificateStoresStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICertificateStoresStatics2Vtbl {
        unsafe extern "system" fn GetUserStoreByName<Impl: ICertificateStoresStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserStoreByName(&*(&storename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICertificateStoresStatics2, BASE_OFFSET>(),
            GetUserStoreByName: GetUserStoreByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICertificateStoresStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IChainBuildingParametersImpl: Sized {
    fn EnhancedKeyUsages(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ValidationTimestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetValidationTimestamp(&mut self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn RevocationCheckEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetRevocationCheckEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn NetworkRetrievalEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetNetworkRetrievalEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AuthorityInformationAccessEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAuthorityInformationAccessEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentTimeValidationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetCurrentTimeValidationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ExclusiveTrustRoots(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<Certificate>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChainBuildingParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IChainBuildingParameters";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IChainBuildingParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChainBuildingParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChainBuildingParametersVtbl {
        unsafe extern "system" fn EnhancedKeyUsages<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnhancedKeyUsages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidationTimestamp<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidationTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValidationTimestamp<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValidationTimestamp(&*(&value as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevocationCheckEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevocationCheckEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevocationCheckEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevocationCheckEnabled(value).into()
        }
        unsafe extern "system" fn NetworkRetrievalEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkRetrievalEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkRetrievalEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkRetrievalEnabled(value).into()
        }
        unsafe extern "system" fn AuthorityInformationAccessEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthorityInformationAccessEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorityInformationAccessEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthorityInformationAccessEnabled(value).into()
        }
        unsafe extern "system" fn CurrentTimeValidationEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentTimeValidationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentTimeValidationEnabled<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentTimeValidationEnabled(value).into()
        }
        unsafe extern "system" fn ExclusiveTrustRoots<Impl: IChainBuildingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExclusiveTrustRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChainBuildingParameters, BASE_OFFSET>(),
            EnhancedKeyUsages: EnhancedKeyUsages::<Impl, IMPL_OFFSET>,
            ValidationTimestamp: ValidationTimestamp::<Impl, IMPL_OFFSET>,
            SetValidationTimestamp: SetValidationTimestamp::<Impl, IMPL_OFFSET>,
            RevocationCheckEnabled: RevocationCheckEnabled::<Impl, IMPL_OFFSET>,
            SetRevocationCheckEnabled: SetRevocationCheckEnabled::<Impl, IMPL_OFFSET>,
            NetworkRetrievalEnabled: NetworkRetrievalEnabled::<Impl, IMPL_OFFSET>,
            SetNetworkRetrievalEnabled: SetNetworkRetrievalEnabled::<Impl, IMPL_OFFSET>,
            AuthorityInformationAccessEnabled: AuthorityInformationAccessEnabled::<Impl, IMPL_OFFSET>,
            SetAuthorityInformationAccessEnabled: SetAuthorityInformationAccessEnabled::<Impl, IMPL_OFFSET>,
            CurrentTimeValidationEnabled: CurrentTimeValidationEnabled::<Impl, IMPL_OFFSET>,
            SetCurrentTimeValidationEnabled: SetCurrentTimeValidationEnabled::<Impl, IMPL_OFFSET>,
            ExclusiveTrustRoots: ExclusiveTrustRoots::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChainBuildingParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking", feature = "implement_exclusive"))]
pub trait IChainValidationParametersImpl: Sized {
    fn CertificateChainPolicy(&mut self) -> ::windows::core::Result<CertificateChainPolicy>;
    fn SetCertificateChainPolicy(&mut self, value: CertificateChainPolicy) -> ::windows::core::Result<()>;
    fn ServerDnsName(&mut self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn SetServerDnsName(&mut self, value: &::core::option::Option<super::super::super::Networking::HostName>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Networking", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChainValidationParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IChainValidationParameters";
}
#[cfg(all(feature = "Networking", feature = "implement_exclusive"))]
impl IChainValidationParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChainValidationParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChainValidationParametersVtbl {
        unsafe extern "system" fn CertificateChainPolicy<Impl: IChainValidationParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CertificateChainPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CertificateChainPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateChainPolicy<Impl: IChainValidationParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CertificateChainPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificateChainPolicy(value).into()
        }
        unsafe extern "system" fn ServerDnsName<Impl: IChainValidationParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerDnsName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerDnsName<Impl: IChainValidationParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerDnsName(&*(&value as *const <super::super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChainValidationParameters, BASE_OFFSET>(),
            CertificateChainPolicy: CertificateChainPolicy::<Impl, IMPL_OFFSET>,
            SetCertificateChainPolicy: SetCertificateChainPolicy::<Impl, IMPL_OFFSET>,
            ServerDnsName: ServerDnsName::<Impl, IMPL_OFFSET>,
            SetServerDnsName: SetServerDnsName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChainValidationParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICmsAttachedSignatureImpl: Sized {
    fn Certificates(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
    fn Content(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn Signers(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>;
    fn VerifySignature(&mut self) -> ::windows::core::Result<SignatureValidationResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICmsAttachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsAttachedSignature";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICmsAttachedSignatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICmsAttachedSignatureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICmsAttachedSignatureVtbl {
        unsafe extern "system" fn Certificates<Impl: ICmsAttachedSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: ICmsAttachedSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn Signers<Impl: ICmsAttachedSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifySignature<Impl: ICmsAttachedSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SignatureValidationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifySignature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICmsAttachedSignature, BASE_OFFSET>(),
            Certificates: Certificates::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
            Signers: Signers::<Impl, IMPL_OFFSET>,
            VerifySignature: VerifySignature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICmsAttachedSignature as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICmsAttachedSignatureFactoryImpl: Sized {
    fn CreateCmsAttachedSignature(&mut self, inputblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CmsAttachedSignature>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICmsAttachedSignatureFactory {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICmsAttachedSignatureFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICmsAttachedSignatureFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICmsAttachedSignatureFactoryVtbl {
        unsafe extern "system" fn CreateCmsAttachedSignature<Impl: ICmsAttachedSignatureFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCmsAttachedSignature(&*(&inputblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICmsAttachedSignatureFactory, BASE_OFFSET>(),
            CreateCmsAttachedSignature: CreateCmsAttachedSignature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICmsAttachedSignatureFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICmsAttachedSignatureStaticsImpl: Sized {
    fn GenerateSignatureAsync(&mut self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, signers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICmsAttachedSignatureStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsAttachedSignatureStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICmsAttachedSignatureStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICmsAttachedSignatureStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICmsAttachedSignatureStaticsVtbl {
        unsafe extern "system" fn GenerateSignatureAsync<Impl: ICmsAttachedSignatureStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, signers: ::windows::core::RawPtr, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICmsAttachedSignatureStatics, BASE_OFFSET>(),
            GenerateSignatureAsync: GenerateSignatureAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICmsAttachedSignatureStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICmsDetachedSignatureImpl: Sized {
    fn Certificates(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
    fn Signers(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>;
    fn VerifySignatureAsync(&mut self, data: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SignatureValidationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICmsDetachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsDetachedSignature";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICmsDetachedSignatureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICmsDetachedSignatureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICmsDetachedSignatureVtbl {
        unsafe extern "system" fn Certificates<Impl: ICmsDetachedSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signers<Impl: ICmsDetachedSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifySignatureAsync<Impl: ICmsDetachedSignatureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifySignatureAsync(&*(&data as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICmsDetachedSignature, BASE_OFFSET>(),
            Certificates: Certificates::<Impl, IMPL_OFFSET>,
            Signers: Signers::<Impl, IMPL_OFFSET>,
            VerifySignatureAsync: VerifySignatureAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICmsDetachedSignature as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICmsDetachedSignatureFactoryImpl: Sized {
    fn CreateCmsDetachedSignature(&mut self, inputblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CmsDetachedSignature>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICmsDetachedSignatureFactory {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICmsDetachedSignatureFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICmsDetachedSignatureFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICmsDetachedSignatureFactoryVtbl {
        unsafe extern "system" fn CreateCmsDetachedSignature<Impl: ICmsDetachedSignatureFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCmsDetachedSignature(&*(&inputblob as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICmsDetachedSignatureFactory, BASE_OFFSET>(),
            CreateCmsDetachedSignature: CreateCmsDetachedSignature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICmsDetachedSignatureFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICmsDetachedSignatureStaticsImpl: Sized {
    fn GenerateSignatureAsync(&mut self, data: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, signers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICmsDetachedSignatureStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsDetachedSignatureStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICmsDetachedSignatureStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICmsDetachedSignatureStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICmsDetachedSignatureStaticsVtbl {
        unsafe extern "system" fn GenerateSignatureAsync<Impl: ICmsDetachedSignatureStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, signers: ::windows::core::RawPtr, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICmsDetachedSignatureStatics, BASE_OFFSET>(),
            GenerateSignatureAsync: GenerateSignatureAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICmsDetachedSignatureStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsSignerInfoImpl: Sized {
    fn Certificate(&mut self) -> ::windows::core::Result<Certificate>;
    fn SetCertificate(&mut self, value: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
    fn HashAlgorithmName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHashAlgorithmName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TimestampInfo(&mut self) -> ::windows::core::Result<CmsTimestampInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICmsSignerInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsSignerInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ICmsSignerInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICmsSignerInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICmsSignerInfoVtbl {
        unsafe extern "system" fn Certificate<Impl: ICmsSignerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificate<Impl: ICmsSignerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificate(&*(&value as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HashAlgorithmName<Impl: ICmsSignerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithmName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithmName<Impl: ICmsSignerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithmName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TimestampInfo<Impl: ICmsSignerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimestampInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICmsSignerInfo, BASE_OFFSET>(),
            Certificate: Certificate::<Impl, IMPL_OFFSET>,
            SetCertificate: SetCertificate::<Impl, IMPL_OFFSET>,
            HashAlgorithmName: HashAlgorithmName::<Impl, IMPL_OFFSET>,
            SetHashAlgorithmName: SetHashAlgorithmName::<Impl, IMPL_OFFSET>,
            TimestampInfo: TimestampInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICmsSignerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICmsTimestampInfoImpl: Sized {
    fn SigningCertificate(&mut self) -> ::windows::core::Result<Certificate>;
    fn Certificates(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICmsTimestampInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICmsTimestampInfo";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICmsTimestampInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICmsTimestampInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICmsTimestampInfoVtbl {
        unsafe extern "system" fn SigningCertificate<Impl: ICmsTimestampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SigningCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Certificates<Impl: ICmsTimestampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ICmsTimestampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICmsTimestampInfo, BASE_OFFSET>(),
            SigningCertificate: SigningCertificate::<Impl, IMPL_OFFSET>,
            Certificates: Certificates::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICmsTimestampInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAlgorithmNamesStaticsImpl: Sized {
    fn Rsa(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Dsa(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh256(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh384(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh521(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdsa256(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdsa384(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdsa521(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyAlgorithmNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyAlgorithmNamesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyAlgorithmNamesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyAlgorithmNamesStaticsVtbl {
        unsafe extern "system" fn Rsa<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rsa() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dsa<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dsa() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdh256<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ecdh256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdh384<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ecdh384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdh521<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ecdh521() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdsa256<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ecdsa256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdsa384<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ecdsa384() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdsa521<Impl: IKeyAlgorithmNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ecdsa521() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyAlgorithmNamesStatics, BASE_OFFSET>(),
            Rsa: Rsa::<Impl, IMPL_OFFSET>,
            Dsa: Dsa::<Impl, IMPL_OFFSET>,
            Ecdh256: Ecdh256::<Impl, IMPL_OFFSET>,
            Ecdh384: Ecdh384::<Impl, IMPL_OFFSET>,
            Ecdh521: Ecdh521::<Impl, IMPL_OFFSET>,
            Ecdsa256: Ecdsa256::<Impl, IMPL_OFFSET>,
            Ecdsa384: Ecdsa384::<Impl, IMPL_OFFSET>,
            Ecdsa521: Ecdsa521::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyAlgorithmNamesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAlgorithmNamesStatics2Impl: Sized {
    fn Ecdsa(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyAlgorithmNamesStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyAlgorithmNamesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyAlgorithmNamesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyAlgorithmNamesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyAlgorithmNamesStatics2Vtbl {
        unsafe extern "system" fn Ecdsa<Impl: IKeyAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ecdsa() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ecdh<Impl: IKeyAlgorithmNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ecdh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyAlgorithmNamesStatics2, BASE_OFFSET>(),
            Ecdsa: Ecdsa::<Impl, IMPL_OFFSET>,
            Ecdh: Ecdh::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyAlgorithmNamesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IKeyAttestationHelperStaticsImpl: Sized {
    fn DecryptTpmAttestationCredentialAsync(&mut self, credential: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetTpmAttestationCredentialId(&mut self, credential: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyAttestationHelperStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IKeyAttestationHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyAttestationHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyAttestationHelperStaticsVtbl {
        unsafe extern "system" fn DecryptTpmAttestationCredentialAsync<Impl: IKeyAttestationHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecryptTpmAttestationCredentialAsync(&*(&credential as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTpmAttestationCredentialId<Impl: IKeyAttestationHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTpmAttestationCredentialId(&*(&credential as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyAttestationHelperStatics, BASE_OFFSET>(),
            DecryptTpmAttestationCredentialAsync: DecryptTpmAttestationCredentialAsync::<Impl, IMPL_OFFSET>,
            GetTpmAttestationCredentialId: GetTpmAttestationCredentialId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyAttestationHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IKeyAttestationHelperStatics2Impl: Sized {
    fn DecryptTpmAttestationCredentialWithContainerNameAsync(&mut self, credential: &::windows::core::HSTRING, containername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyAttestationHelperStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyAttestationHelperStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IKeyAttestationHelperStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyAttestationHelperStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyAttestationHelperStatics2Vtbl {
        unsafe extern "system" fn DecryptTpmAttestationCredentialWithContainerNameAsync<Impl: IKeyAttestationHelperStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, containername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecryptTpmAttestationCredentialWithContainerNameAsync(&*(&credential as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&containername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyAttestationHelperStatics2, BASE_OFFSET>(),
            DecryptTpmAttestationCredentialWithContainerNameAsync: DecryptTpmAttestationCredentialWithContainerNameAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyAttestationHelperStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyStorageProviderNamesStaticsImpl: Sized {
    fn SoftwareKeyStorageProvider(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SmartcardKeyStorageProvider(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PlatformKeyStorageProvider(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyStorageProviderNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyStorageProviderNamesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyStorageProviderNamesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyStorageProviderNamesStaticsVtbl {
        unsafe extern "system" fn SoftwareKeyStorageProvider<Impl: IKeyStorageProviderNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoftwareKeyStorageProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmartcardKeyStorageProvider<Impl: IKeyStorageProviderNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartcardKeyStorageProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlatformKeyStorageProvider<Impl: IKeyStorageProviderNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlatformKeyStorageProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyStorageProviderNamesStatics, BASE_OFFSET>(),
            SoftwareKeyStorageProvider: SoftwareKeyStorageProvider::<Impl, IMPL_OFFSET>,
            SmartcardKeyStorageProvider: SmartcardKeyStorageProvider::<Impl, IMPL_OFFSET>,
            PlatformKeyStorageProvider: PlatformKeyStorageProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyStorageProviderNamesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyStorageProviderNamesStatics2Impl: Sized {
    fn PassportKeyStorageProvider(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyStorageProviderNamesStatics2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IKeyStorageProviderNamesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyStorageProviderNamesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyStorageProviderNamesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyStorageProviderNamesStatics2Vtbl {
        unsafe extern "system" fn PassportKeyStorageProvider<Impl: IKeyStorageProviderNamesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PassportKeyStorageProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyStorageProviderNamesStatics2, BASE_OFFSET>(),
            PassportKeyStorageProvider: PassportKeyStorageProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyStorageProviderNamesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPfxImportParametersImpl: Sized {
    fn Exportable(&mut self) -> ::windows::core::Result<ExportOption>;
    fn SetExportable(&mut self, value: ExportOption) -> ::windows::core::Result<()>;
    fn KeyProtectionLevel(&mut self) -> ::windows::core::Result<KeyProtectionLevel>;
    fn SetKeyProtectionLevel(&mut self, value: KeyProtectionLevel) -> ::windows::core::Result<()>;
    fn InstallOptions(&mut self) -> ::windows::core::Result<InstallOptions>;
    fn SetInstallOptions(&mut self, value: InstallOptions) -> ::windows::core::Result<()>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn KeyStorageProviderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyStorageProviderName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainerNamePrefix(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContainerNamePrefix(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ReaderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetReaderName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPfxImportParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IPfxImportParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IPfxImportParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPfxImportParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPfxImportParametersVtbl {
        unsafe extern "system" fn Exportable<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ExportOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exportable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExportable<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ExportOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExportable(value).into()
        }
        unsafe extern "system" fn KeyProtectionLevel<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut KeyProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyProtectionLevel<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: KeyProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyProtectionLevel(value).into()
        }
        unsafe extern "system" fn InstallOptions<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InstallOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInstallOptions<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InstallOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallOptions(value).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyStorageProviderName<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyStorageProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyStorageProviderName<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyStorageProviderName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContainerNamePrefix<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerNamePrefix<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainerNamePrefix(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReaderName<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReaderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReaderName<Impl: IPfxImportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReaderName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPfxImportParameters, BASE_OFFSET>(),
            Exportable: Exportable::<Impl, IMPL_OFFSET>,
            SetExportable: SetExportable::<Impl, IMPL_OFFSET>,
            KeyProtectionLevel: KeyProtectionLevel::<Impl, IMPL_OFFSET>,
            SetKeyProtectionLevel: SetKeyProtectionLevel::<Impl, IMPL_OFFSET>,
            InstallOptions: InstallOptions::<Impl, IMPL_OFFSET>,
            SetInstallOptions: SetInstallOptions::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            KeyStorageProviderName: KeyStorageProviderName::<Impl, IMPL_OFFSET>,
            SetKeyStorageProviderName: SetKeyStorageProviderName::<Impl, IMPL_OFFSET>,
            ContainerNamePrefix: ContainerNamePrefix::<Impl, IMPL_OFFSET>,
            SetContainerNamePrefix: SetContainerNamePrefix::<Impl, IMPL_OFFSET>,
            ReaderName: ReaderName::<Impl, IMPL_OFFSET>,
            SetReaderName: SetReaderName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPfxImportParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardCertificateStoreNamesStaticsImpl: Sized {
    fn Personal(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrustedRootCertificationAuthorities(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IntermediateCertificationAuthorities(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardCertificateStoreNamesStatics {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IStandardCertificateStoreNamesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardCertificateStoreNamesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardCertificateStoreNamesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardCertificateStoreNamesStaticsVtbl {
        unsafe extern "system" fn Personal<Impl: IStandardCertificateStoreNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Personal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrustedRootCertificationAuthorities<Impl: IStandardCertificateStoreNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrustedRootCertificationAuthorities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntermediateCertificationAuthorities<Impl: IStandardCertificateStoreNamesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntermediateCertificationAuthorities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardCertificateStoreNamesStatics, BASE_OFFSET>(),
            Personal: Personal::<Impl, IMPL_OFFSET>,
            TrustedRootCertificationAuthorities: TrustedRootCertificationAuthorities::<Impl, IMPL_OFFSET>,
            IntermediateCertificationAuthorities: IntermediateCertificationAuthorities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardCertificateStoreNamesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISubjectAlternativeNameInfoImpl: Sized {
    fn EmailName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn IPAddress(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Url(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DnsName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DistinguishedName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn PrincipalName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISubjectAlternativeNameInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISubjectAlternativeNameInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISubjectAlternativeNameInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISubjectAlternativeNameInfoVtbl {
        unsafe extern "system" fn EmailName<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPAddress<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IPAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Url<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DnsName<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DnsName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistinguishedName<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DistinguishedName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrincipalName<Impl: ISubjectAlternativeNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrincipalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISubjectAlternativeNameInfo, BASE_OFFSET>(),
            EmailName: EmailName::<Impl, IMPL_OFFSET>,
            IPAddress: IPAddress::<Impl, IMPL_OFFSET>,
            Url: Url::<Impl, IMPL_OFFSET>,
            DnsName: DnsName::<Impl, IMPL_OFFSET>,
            DistinguishedName: DistinguishedName::<Impl, IMPL_OFFSET>,
            PrincipalName: PrincipalName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISubjectAlternativeNameInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISubjectAlternativeNameInfo2Impl: Sized {
    fn EmailNames(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IPAddresses(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Urls(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DnsNames(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DistinguishedNames(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PrincipalNames(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Extension(&mut self) -> ::windows::core::Result<CertificateExtension>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISubjectAlternativeNameInfo2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ISubjectAlternativeNameInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISubjectAlternativeNameInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISubjectAlternativeNameInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISubjectAlternativeNameInfo2Vtbl {
        unsafe extern "system" fn EmailNames<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPAddresses<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IPAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Urls<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Urls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DnsNames<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DnsNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistinguishedNames<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DistinguishedNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrincipalNames<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrincipalNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: ISubjectAlternativeNameInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISubjectAlternativeNameInfo2, BASE_OFFSET>(),
            EmailNames: EmailNames::<Impl, IMPL_OFFSET>,
            IPAddresses: IPAddresses::<Impl, IMPL_OFFSET>,
            Urls: Urls::<Impl, IMPL_OFFSET>,
            DnsNames: DnsNames::<Impl, IMPL_OFFSET>,
            DistinguishedNames: DistinguishedNames::<Impl, IMPL_OFFSET>,
            PrincipalNames: PrincipalNames::<Impl, IMPL_OFFSET>,
            Extension: Extension::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISubjectAlternativeNameInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserCertificateEnrollmentManagerImpl: Sized {
    fn CreateRequestAsync(&mut self, request: &::core::option::Option<CertificateRequestProperties>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn InstallCertificateAsync(&mut self, certificate: &::windows::core::HSTRING, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportPfxDataAsync(&mut self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportPfxDataToKspAsync(&mut self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING, keystorageprovider: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserCertificateEnrollmentManager {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserCertificateEnrollmentManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserCertificateEnrollmentManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserCertificateEnrollmentManagerVtbl {
        unsafe extern "system" fn CreateRequestAsync<Impl: IUserCertificateEnrollmentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRequestAsync(&*(&request as *const <CertificateRequestProperties as ::windows::core::Abi>::Abi as *const <CertificateRequestProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallCertificateAsync<Impl: IUserCertificateEnrollmentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoption: InstallOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallCertificateAsync(&*(&certificate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), installoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportPfxDataAsync<Impl: IUserCertificateEnrollmentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn ImportPfxDataToKspAsync<Impl: IUserCertificateEnrollmentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keystorageprovider: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserCertificateEnrollmentManager, BASE_OFFSET>(),
            CreateRequestAsync: CreateRequestAsync::<Impl, IMPL_OFFSET>,
            InstallCertificateAsync: InstallCertificateAsync::<Impl, IMPL_OFFSET>,
            ImportPfxDataAsync: ImportPfxDataAsync::<Impl, IMPL_OFFSET>,
            ImportPfxDataToKspAsync: ImportPfxDataToKspAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserCertificateEnrollmentManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserCertificateEnrollmentManager2Impl: Sized {
    fn ImportPfxDataToKspWithParametersAsync(&mut self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, pfximportparameters: &::core::option::Option<PfxImportParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserCertificateEnrollmentManager2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IUserCertificateEnrollmentManager2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserCertificateEnrollmentManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserCertificateEnrollmentManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserCertificateEnrollmentManager2Vtbl {
        unsafe extern "system" fn ImportPfxDataToKspWithParametersAsync<Impl: IUserCertificateEnrollmentManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pfximportparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserCertificateEnrollmentManager2, BASE_OFFSET>(),
            ImportPfxDataToKspWithParametersAsync: ImportPfxDataToKspWithParametersAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserCertificateEnrollmentManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserCertificateStoreImpl: Sized {
    fn RequestAddAsync(&mut self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsync(&mut self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserCertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.IUserCertificateStore";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserCertificateStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserCertificateStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserCertificateStoreVtbl {
        unsafe extern "system" fn RequestAddAsync<Impl: IUserCertificateStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAddAsync(&*(&certificate as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDeleteAsync<Impl: IUserCertificateStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDeleteAsync(&*(&certificate as *const <Certificate as ::windows::core::Abi>::Abi as *const <Certificate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IUserCertificateStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserCertificateStore, BASE_OFFSET>(),
            RequestAddAsync: RequestAddAsync::<Impl, IMPL_OFFSET>,
            RequestDeleteAsync: RequestDeleteAsync::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserCertificateStore as ::windows::core::Interface>::IID
    }
}
