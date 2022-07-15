#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
pub struct AsymmetricAlgorithmNames;
impl AsymmetricAlgorithmNames {
    pub fn RsaPkcs1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaPkcs1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaOaepSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaOaepSha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaOaepSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaOaepSha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaOaepSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaOaepSha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaOaepSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaOaepSha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EcdsaP256Sha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EcdsaP256Sha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EcdsaP384Sha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EcdsaP384Sha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EcdsaP521Sha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EcdsaP521Sha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DsaSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DsaSha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DsaSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DsaSha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPkcs1Sha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaSignPkcs1Sha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPkcs1Sha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaSignPkcs1Sha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPkcs1Sha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaSignPkcs1Sha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPkcs1Sha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaSignPkcs1Sha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPssSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaSignPssSha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPssSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaSignPssSha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPssSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaSignPssSha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPssSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RsaSignPssSha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EcdsaSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EcdsaSha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EcdsaSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EcdsaSha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn EcdsaSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EcdsaSha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAsymmetricAlgorithmNamesStatics<R, F: FnOnce(&IAsymmetricAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AsymmetricAlgorithmNames, IAsymmetricAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAsymmetricAlgorithmNamesStatics2<R, F: FnOnce(&IAsymmetricAlgorithmNamesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AsymmetricAlgorithmNames, IAsymmetricAlgorithmNamesStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for AsymmetricAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames";
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct AsymmetricKeyAlgorithmProvider(::windows::core::IUnknown);
impl AsymmetricKeyAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CreateKeyPair(&self, keysize: u32) -> ::windows::core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateKeyPair)(::windows::core::Interface::as_raw(this), keysize, result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportDefaultPrivateKeyBlob<'a, P0, E0>(&self, keyblob: P0) -> ::windows::core::Result<CryptographicKey>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportDefaultPrivateKeyBlob)(::windows::core::Interface::as_raw(this), keyblob.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportKeyPairWithBlobType<'a, P0, E0>(&self, keyblob: P0, blobtype: CryptographicPrivateKeyBlobType) -> ::windows::core::Result<CryptographicKey>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportKeyPairWithBlobType)(::windows::core::Interface::as_raw(this), keyblob.try_into().map_err(|e| e.into())?.abi(), blobtype, result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportDefaultPublicKeyBlob<'a, P0, E0>(&self, keyblob: P0) -> ::windows::core::Result<CryptographicKey>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportDefaultPublicKeyBlob)(::windows::core::Interface::as_raw(this), keyblob.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportPublicKeyWithBlobType<'a, P0, E0>(&self, keyblob: P0, blobtype: CryptographicPublicKeyBlobType) -> ::windows::core::Result<CryptographicKey>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportPublicKeyWithBlobType)(::windows::core::Interface::as_raw(this), keyblob.try_into().map_err(|e| e.into())?.abi(), blobtype, result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn CreateKeyPairWithCurveName(&self, curvename: &::windows::core::HSTRING) -> ::windows::core::Result<CryptographicKey> {
        let this = &::windows::core::Interface::cast::<IAsymmetricKeyAlgorithmProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateKeyPairWithCurveName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(curvename), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn CreateKeyPairWithCurveParameters(&self, parameters: &[u8]) -> ::windows::core::Result<CryptographicKey> {
        let this = &::windows::core::Interface::cast::<IAsymmetricKeyAlgorithmProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateKeyPairWithCurveParameters)(::windows::core::Interface::as_raw(this), parameters.len() as u32, parameters.as_ptr(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn OpenAlgorithm(algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<AsymmetricKeyAlgorithmProvider> {
        Self::IAsymmetricKeyAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenAlgorithm)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(algorithm), result__.as_mut_ptr()).from_abi::<AsymmetricKeyAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAsymmetricKeyAlgorithmProviderStatics<R, F: FnOnce(&IAsymmetricKeyAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AsymmetricKeyAlgorithmProvider, IAsymmetricKeyAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AsymmetricKeyAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsymmetricKeyAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsymmetricKeyAlgorithmProvider {}
impl ::core::fmt::Debug for AsymmetricKeyAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsymmetricKeyAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AsymmetricKeyAlgorithmProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider;{e8d2ff37-6259-4e88-b7e0-94191fde699e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AsymmetricKeyAlgorithmProvider {
    type Vtable = IAsymmetricKeyAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = <IAsymmetricKeyAlgorithmProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AsymmetricKeyAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider";
}
impl ::core::convert::From<AsymmetricKeyAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: AsymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsymmetricKeyAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: &AsymmetricKeyAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AsymmetricKeyAlgorithmProvider> for &::windows::core::IUnknown {
    fn from(value: &AsymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AsymmetricKeyAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: AsymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsymmetricKeyAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: &AsymmetricKeyAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AsymmetricKeyAlgorithmProvider> for &::windows::core::IInspectable {
    fn from(value: &AsymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for AsymmetricKeyAlgorithmProvider {}
unsafe impl ::core::marker::Sync for AsymmetricKeyAlgorithmProvider {}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Capi1KdfTargetAlgorithm(pub i32);
impl Capi1KdfTargetAlgorithm {
    pub const NotAes: Self = Self(0i32);
    pub const Aes: Self = Self(1i32);
}
impl ::core::marker::Copy for Capi1KdfTargetAlgorithm {}
impl ::core::clone::Clone for Capi1KdfTargetAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Capi1KdfTargetAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Capi1KdfTargetAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for Capi1KdfTargetAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Capi1KdfTargetAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Capi1KdfTargetAlgorithm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.Capi1KdfTargetAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
pub struct CryptographicEngine;
impl CryptographicEngine {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Encrypt<'a, P0, P1, E1, P2, E2>(key: P0, data: P1, iv: P2) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Encrypt)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), iv.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Decrypt<'a, P0, P1, E1, P2, E2>(key: P0, data: P1, iv: P2) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Decrypt)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), iv.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncryptAndAuthenticate<'a, P0, P1, E1, P2, E2, P3, E3>(key: P0, data: P1, nonce: P2, authenticateddata: P3) -> ::windows::core::Result<EncryptedAndAuthenticatedData>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
        P3: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E3>,
        E3: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EncryptAndAuthenticate)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), nonce.try_into().map_err(|e| e.into())?.abi(), authenticateddata.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<EncryptedAndAuthenticatedData>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DecryptAndAuthenticate<'a, P0, P1, E1, P2, E2, P3, E3, P4, E4>(key: P0, data: P1, nonce: P2, authenticationtag: P3, authenticateddata: P4) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
        P3: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E3>,
        E3: ::std::convert::Into<::windows::core::Error>,
        P4: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E4>,
        E4: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DecryptAndAuthenticate)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), nonce.try_into().map_err(|e| e.into())?.abi(), authenticationtag.try_into().map_err(|e| e.into())?.abi(), authenticateddata.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Sign<'a, P0, P1, E1>(key: P0, data: P1) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sign)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn VerifySignature<'a, P0, P1, E1, P2, E2>(key: P0, data: P1, signature: P2) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VerifySignature)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), signature.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DeriveKeyMaterial<'a, P0, P1>(key: P0, parameters: P1, desiredkeysize: u32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, KeyDerivationParameters>>,
    {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeriveKeyMaterial)(::windows::core::Interface::as_raw(this), key.into().abi(), parameters.into().abi(), desiredkeysize, result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SignHashedData<'a, P0, P1, E1>(key: P0, data: P1) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SignHashedData)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn VerifySignatureWithHashInput<'a, P0, P1, E1, P2, E2>(key: P0, data: P1, signature: P2) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VerifySignatureWithHashInput)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), signature.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn DecryptAsync<'a, P0, P1, E1, P2, E2>(key: P0, data: P1, iv: P2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DecryptAsync)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), iv.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SignAsync<'a, P0, P1, E1>(key: P0, data: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SignAsync)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SignHashedDataAsync<'a, P0, P1, E1>(key: P0, data: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CryptographicKey>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SignHashedDataAsync)(::windows::core::Interface::as_raw(this), key.into().abi(), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICryptographicEngineStatics<R, F: FnOnce(&ICryptographicEngineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CryptographicEngine, ICryptographicEngineStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICryptographicEngineStatics2<R, F: FnOnce(&ICryptographicEngineStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CryptographicEngine, ICryptographicEngineStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CryptographicEngine {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.CryptographicEngine";
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct CryptographicHash(::windows::core::IUnknown);
impl CryptographicHash {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Append<'a, P0, E0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::windows::core::Interface::as_raw(this), data.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetValueAndReset(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetValueAndReset)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for CryptographicHash {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CryptographicHash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CryptographicHash {}
impl ::core::fmt::Debug for CryptographicHash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicHash").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CryptographicHash {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.CryptographicHash;{5904d1b6-ad31-4603-a3a4-b1bda98e2562})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CryptographicHash {
    type Vtable = IHashComputation_Vtbl;
    const IID: ::windows::core::GUID = <IHashComputation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CryptographicHash {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.CryptographicHash";
}
impl ::core::convert::From<CryptographicHash> for ::windows::core::IUnknown {
    fn from(value: CryptographicHash) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CryptographicHash> for ::windows::core::IUnknown {
    fn from(value: &CryptographicHash) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CryptographicHash> for &::windows::core::IUnknown {
    fn from(value: &CryptographicHash) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CryptographicHash> for ::windows::core::IInspectable {
    fn from(value: CryptographicHash) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CryptographicHash> for ::windows::core::IInspectable {
    fn from(value: &CryptographicHash) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CryptographicHash> for &::windows::core::IInspectable {
    fn from(value: &CryptographicHash) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CryptographicHash {}
unsafe impl ::core::marker::Sync for CryptographicHash {}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct CryptographicKey(::windows::core::IUnknown);
impl CryptographicKey {
    pub fn KeySize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeySize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportDefaultPrivateKeyBlobType(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExportDefaultPrivateKeyBlobType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportPrivateKeyWithBlobType(&self, blobtype: CryptographicPrivateKeyBlobType) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExportPrivateKeyWithBlobType)(::windows::core::Interface::as_raw(this), blobtype, result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportDefaultPublicKeyBlobType(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExportDefaultPublicKeyBlobType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportPublicKeyWithBlobType(&self, blobtype: CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExportPublicKeyWithBlobType)(::windows::core::Interface::as_raw(this), blobtype, result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for CryptographicKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CryptographicKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CryptographicKey {}
impl ::core::fmt::Debug for CryptographicKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicKey").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CryptographicKey {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.CryptographicKey;{ed2a3b70-8e7b-4009-8401-ffd1a62eeb27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CryptographicKey {
    type Vtable = ICryptographicKey_Vtbl;
    const IID: ::windows::core::GUID = <ICryptographicKey as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CryptographicKey {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.CryptographicKey";
}
impl ::core::convert::From<CryptographicKey> for ::windows::core::IUnknown {
    fn from(value: CryptographicKey) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CryptographicKey> for ::windows::core::IUnknown {
    fn from(value: &CryptographicKey) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CryptographicKey> for &::windows::core::IUnknown {
    fn from(value: &CryptographicKey) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CryptographicKey> for ::windows::core::IInspectable {
    fn from(value: CryptographicKey) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CryptographicKey> for ::windows::core::IInspectable {
    fn from(value: &CryptographicKey) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CryptographicKey> for &::windows::core::IInspectable {
    fn from(value: &CryptographicKey) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CryptographicKey {}
unsafe impl ::core::marker::Sync for CryptographicKey {}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CryptographicPadding(pub i32);
impl CryptographicPadding {
    pub const None: Self = Self(0i32);
    pub const RsaOaep: Self = Self(1i32);
    pub const RsaPkcs1V15: Self = Self(2i32);
    pub const RsaPss: Self = Self(3i32);
}
impl ::core::marker::Copy for CryptographicPadding {}
impl ::core::clone::Clone for CryptographicPadding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CryptographicPadding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CryptographicPadding {
    type Abi = Self;
}
impl ::core::fmt::Debug for CryptographicPadding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPadding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CryptographicPadding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.CryptographicPadding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CryptographicPrivateKeyBlobType(pub i32);
impl CryptographicPrivateKeyBlobType {
    pub const Pkcs8RawPrivateKeyInfo: Self = Self(0i32);
    pub const Pkcs1RsaPrivateKey: Self = Self(1i32);
    pub const BCryptPrivateKey: Self = Self(2i32);
    pub const Capi1PrivateKey: Self = Self(3i32);
    pub const BCryptEccFullPrivateKey: Self = Self(4i32);
}
impl ::core::marker::Copy for CryptographicPrivateKeyBlobType {}
impl ::core::clone::Clone for CryptographicPrivateKeyBlobType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CryptographicPrivateKeyBlobType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CryptographicPrivateKeyBlobType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CryptographicPrivateKeyBlobType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPrivateKeyBlobType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CryptographicPrivateKeyBlobType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.CryptographicPrivateKeyBlobType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CryptographicPublicKeyBlobType(pub i32);
impl CryptographicPublicKeyBlobType {
    pub const X509SubjectPublicKeyInfo: Self = Self(0i32);
    pub const Pkcs1RsaPublicKey: Self = Self(1i32);
    pub const BCryptPublicKey: Self = Self(2i32);
    pub const Capi1PublicKey: Self = Self(3i32);
    pub const BCryptEccFullPublicKey: Self = Self(4i32);
}
impl ::core::marker::Copy for CryptographicPublicKeyBlobType {}
impl ::core::clone::Clone for CryptographicPublicKeyBlobType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CryptographicPublicKeyBlobType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CryptographicPublicKeyBlobType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CryptographicPublicKeyBlobType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPublicKeyBlobType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CryptographicPublicKeyBlobType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.CryptographicPublicKeyBlobType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
pub struct EccCurveNames;
impl EccCurveNames {
    pub fn BrainpoolP160r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP160r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP160t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP160t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP192r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP192r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP192t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP192t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP224r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP224r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP224t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP224t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP256r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP256r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP256t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP256t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP320r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP320r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP320t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP320t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP384r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP384r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP384t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP384t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP512r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP512r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP512t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrainpoolP512t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Curve25519() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Curve25519)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ec192wapi() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ec192wapi)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NistP192() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NistP192)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NistP224() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NistP224)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NistP256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NistP256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NistP384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NistP384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NistP521() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NistP521)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NumsP256t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NumsP256t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NumsP384t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NumsP384t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NumsP512t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NumsP512t1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP160k1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP160k1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP160r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP160r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP160r2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP160r2)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP192k1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP192k1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP192r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP192r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP224k1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP224k1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP224r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP224r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP256k1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP256k1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP256r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP256r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP384r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP384r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SecP521r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SecP521r1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Wtls7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Wtls7)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Wtls9() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Wtls9)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Wtls12() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Wtls12)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn X962P192v1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).X962P192v1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn X962P192v2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).X962P192v2)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn X962P192v3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).X962P192v3)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn X962P239v1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).X962P239v1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn X962P239v2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).X962P239v2)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn X962P239v3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).X962P239v3)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn X962P256v1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).X962P256v1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllEccCurveNames() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AllEccCurveNames)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEccCurveNamesStatics<R, F: FnOnce(&IEccCurveNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<EccCurveNames, IEccCurveNamesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for EccCurveNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.EccCurveNames";
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct EncryptedAndAuthenticatedData(::windows::core::IUnknown);
impl EncryptedAndAuthenticatedData {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncryptedData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EncryptedData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AuthenticationTag(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationTag)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for EncryptedAndAuthenticatedData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EncryptedAndAuthenticatedData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EncryptedAndAuthenticatedData {}
impl ::core::fmt::Debug for EncryptedAndAuthenticatedData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EncryptedAndAuthenticatedData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EncryptedAndAuthenticatedData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData;{6fa42fe7-1ecb-4b00-bea5-60b83f862f17})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EncryptedAndAuthenticatedData {
    type Vtable = IEncryptedAndAuthenticatedData_Vtbl;
    const IID: ::windows::core::GUID = <IEncryptedAndAuthenticatedData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EncryptedAndAuthenticatedData {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData";
}
impl ::core::convert::From<EncryptedAndAuthenticatedData> for ::windows::core::IUnknown {
    fn from(value: EncryptedAndAuthenticatedData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EncryptedAndAuthenticatedData> for ::windows::core::IUnknown {
    fn from(value: &EncryptedAndAuthenticatedData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&EncryptedAndAuthenticatedData> for &::windows::core::IUnknown {
    fn from(value: &EncryptedAndAuthenticatedData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<EncryptedAndAuthenticatedData> for ::windows::core::IInspectable {
    fn from(value: EncryptedAndAuthenticatedData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EncryptedAndAuthenticatedData> for ::windows::core::IInspectable {
    fn from(value: &EncryptedAndAuthenticatedData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&EncryptedAndAuthenticatedData> for &::windows::core::IInspectable {
    fn from(value: &EncryptedAndAuthenticatedData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for EncryptedAndAuthenticatedData {}
unsafe impl ::core::marker::Sync for EncryptedAndAuthenticatedData {}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
pub struct HashAlgorithmNames;
impl HashAlgorithmNames {
    pub fn Md5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Md5)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHashAlgorithmNamesStatics<R, F: FnOnce(&IHashAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HashAlgorithmNames, IHashAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for HashAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.HashAlgorithmNames";
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct HashAlgorithmProvider(::windows::core::IUnknown);
impl HashAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn HashLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HashLength)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn HashData<'a, P0, E0>(&self, data: P0) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HashData)(::windows::core::Interface::as_raw(this), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn CreateHash(&self) -> ::windows::core::Result<CryptographicHash> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateHash)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CryptographicHash>(result__)
        }
    }
    pub fn OpenAlgorithm(algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<HashAlgorithmProvider> {
        Self::IHashAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenAlgorithm)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(algorithm), result__.as_mut_ptr()).from_abi::<HashAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHashAlgorithmProviderStatics<R, F: FnOnce(&IHashAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HashAlgorithmProvider, IHashAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HashAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HashAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HashAlgorithmProvider {}
impl ::core::fmt::Debug for HashAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HashAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HashAlgorithmProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.HashAlgorithmProvider;{be9b3080-b2c3-422b-bce1-ec90efb5d7b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HashAlgorithmProvider {
    type Vtable = IHashAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = <IHashAlgorithmProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HashAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.HashAlgorithmProvider";
}
impl ::core::convert::From<HashAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: HashAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HashAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: &HashAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&HashAlgorithmProvider> for &::windows::core::IUnknown {
    fn from(value: &HashAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<HashAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: HashAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HashAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: &HashAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&HashAlgorithmProvider> for &::windows::core::IInspectable {
    fn from(value: &HashAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for HashAlgorithmProvider {}
unsafe impl ::core::marker::Sync for HashAlgorithmProvider {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricAlgorithmNamesStatics {
    type Vtable = IAsymmetricAlgorithmNamesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaf6fce4_67c0_46aa_84f9_752e77449f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RsaPkcs1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaOaepSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaOaepSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaOaepSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaOaepSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EcdsaP256Sha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EcdsaP384Sha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EcdsaP521Sha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DsaSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DsaSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaSignPkcs1Sha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaSignPkcs1Sha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaSignPkcs1Sha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaSignPkcs1Sha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaSignPssSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaSignPssSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaSignPssSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RsaSignPssSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricAlgorithmNamesStatics2 {
    type Vtable = IAsymmetricAlgorithmNamesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf141c0d6_4bff_4f23_ba66_6045b137d5df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricAlgorithmNamesStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub EcdsaSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EcdsaSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EcdsaSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricKeyAlgorithmProvider {
    type Vtable = IAsymmetricKeyAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8d2ff37_6259_4e88_b7e0_94191fde699e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricKeyAlgorithmProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CreateKeyPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keysize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ImportDefaultPrivateKeyBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportDefaultPrivateKeyBlob: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportKeyPairWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: *mut ::core::ffi::c_void, blobtype: CryptographicPrivateKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportKeyPairWithBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportDefaultPublicKeyBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportDefaultPublicKeyBlob: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportPublicKeyWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: *mut ::core::ffi::c_void, blobtype: CryptographicPublicKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportPublicKeyWithBlobType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricKeyAlgorithmProvider2 {
    type Vtable = IAsymmetricKeyAlgorithmProvider2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e322a7e_7c4d_4997_ac4f_1b848b36306e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricKeyAlgorithmProvider2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateKeyPairWithCurveName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, curvename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateKeyPairWithCurveParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters_array_size: u32, parameters: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricKeyAlgorithmProviderStatics {
    type Vtable = IAsymmetricKeyAlgorithmProviderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x425bde18_a7f3_47a6_a8d2_c48d6033a65c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricKeyAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicEngineStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICryptographicEngineStatics {
    type Vtable = ICryptographicEngineStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fea0639_6ff7_4c85_a095_95eb31715eb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicEngineStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Encrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, iv: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Encrypt: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Decrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, iv: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Decrypt: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptAndAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, nonce: *mut ::core::ffi::c_void, authenticateddata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptAndAuthenticate: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DecryptAndAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, nonce: *mut ::core::ffi::c_void, authenticationtag: *mut ::core::ffi::c_void, authenticateddata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DecryptAndAuthenticate: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Sign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Sign: usize,
    #[cfg(feature = "Storage_Streams")]
    pub VerifySignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, signature: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VerifySignature: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DeriveKeyMaterial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, parameters: *mut ::core::ffi::c_void, desiredkeysize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DeriveKeyMaterial: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicEngineStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICryptographicEngineStatics2 {
    type Vtable = ICryptographicEngineStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x675948fe_df9f_4191_92c7_6ce6f58420e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicEngineStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SignHashedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SignHashedData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub VerifySignatureWithHashInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, signature: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VerifySignatureWithHashInput: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub DecryptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, iv: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    DecryptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SignAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SignAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SignHashedDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SignHashedDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicKey(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICryptographicKey {
    type Vtable = ICryptographicKey_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed2a3b70_8e7b_4009_8401_ffd1a62eeb27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicKey_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub KeySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ExportDefaultPrivateKeyBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportDefaultPrivateKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ExportPrivateKeyWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: CryptographicPrivateKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportPrivateKeyWithBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ExportDefaultPublicKeyBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportDefaultPublicKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ExportPublicKeyWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: CryptographicPublicKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportPublicKeyWithBlobType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEccCurveNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEccCurveNamesStatics {
    type Vtable = IEccCurveNamesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3ff930c_aeeb_409e_b7d4_9b95295aaecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEccCurveNamesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub BrainpoolP160r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP160t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP192r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP192t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP224r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP224t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP256r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP256t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP320r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP320t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP384r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP384t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP512r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BrainpoolP512t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Curve25519: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ec192wapi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NistP192: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NistP224: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NistP256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NistP384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NistP521: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NumsP256t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NumsP384t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NumsP512t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP160k1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP160r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP160r2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP192k1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP192r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP224k1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP224r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP256k1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP256r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP384r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecP521r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Wtls7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Wtls9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Wtls12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub X962P192v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub X962P192v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub X962P192v3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub X962P239v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub X962P239v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub X962P239v3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub X962P256v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllEccCurveNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllEccCurveNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEncryptedAndAuthenticatedData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEncryptedAndAuthenticatedData {
    type Vtable = IEncryptedAndAuthenticatedData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fa42fe7_1ecb_4b00_bea5_60b83f862f17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEncryptedAndAuthenticatedData_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptedData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AuthenticationTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AuthenticationTag: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHashAlgorithmNamesStatics {
    type Vtable = IHashAlgorithmNamesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b5e0516_de96_4f0a_8d57_dcc9dae36c76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Md5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHashAlgorithmProvider {
    type Vtable = IHashAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe9b3080_b2c3_422b_bce1_ec90efb5d7b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashAlgorithmProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HashLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub HashData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    HashData: usize,
    pub CreateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHashAlgorithmProviderStatics {
    type Vtable = IHashAlgorithmProviderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fac9741_5cc4_4336_ae38_6212b75a915a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashComputation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHashComputation {
    type Vtable = IHashComputation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5904d1b6_ad31_4603_a3a4_b1bda98e2562);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashComputation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Append: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetValueAndReset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetValueAndReset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationAlgorithmNamesStatics {
    type Vtable = IKeyDerivationAlgorithmNamesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b6e363e_94d2_4739_a57b_022e0c3a402a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Pbkdf2Md5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Pbkdf2Sha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Pbkdf2Sha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Pbkdf2Sha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Pbkdf2Sha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp800108CtrHmacMd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp800108CtrHmacSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp800108CtrHmacSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp800108CtrHmacSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp800108CtrHmacSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp80056aConcatMd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp80056aConcatSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp80056aConcatSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp80056aConcatSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Sp80056aConcatSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationAlgorithmNamesStatics2 {
    type Vtable = IKeyDerivationAlgorithmNamesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57953fab_6044_466f_97f4_337b7808384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmNamesStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CapiKdfMd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CapiKdfSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CapiKdfSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CapiKdfSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CapiKdfSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationAlgorithmProvider {
    type Vtable = IKeyDerivationAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1fba83b_4671_43b7_9158_763aaa98b6bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationAlgorithmProviderStatics {
    type Vtable = IKeyDerivationAlgorithmProviderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a22097a_0a1c_443b_9418_b9498aeb1603);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationParameters {
    type Vtable = IKeyDerivationParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bf05967_047b_4a8c_964a_469ffd5522e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParameters_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub KdfGenericBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    KdfGenericBinary: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetKdfGenericBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetKdfGenericBinary: usize,
    pub IterationCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParameters2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationParameters2 {
    type Vtable = IKeyDerivationParameters2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd4166d1_417e_4f4c_b666_c0d879f3f8e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParameters2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Capi1KdfTargetAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Capi1KdfTargetAlgorithm) -> ::windows::core::HRESULT,
    pub SetCapi1KdfTargetAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Capi1KdfTargetAlgorithm) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationParametersStatics {
    type Vtable = IKeyDerivationParametersStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea961fbe_f37f_4146_9dfe_a456f1735f4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParametersStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub BuildForPbkdf2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbkdf2salt: *mut ::core::ffi::c_void, iterationcount: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BuildForPbkdf2: usize,
    #[cfg(feature = "Storage_Streams")]
    pub BuildForSP800108: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BuildForSP800108: usize,
    #[cfg(feature = "Storage_Streams")]
    pub BuildForSP80056a: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithmid: *mut ::core::ffi::c_void, partyuinfo: *mut ::core::ffi::c_void, partyvinfo: *mut ::core::ffi::c_void, supppubinfo: *mut ::core::ffi::c_void, suppprivinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BuildForSP80056a: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationParametersStatics2 {
    type Vtable = IKeyDerivationParametersStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5783dd5_58e3_4efb_b283_a1653126e1be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParametersStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub BuildForCapi1Kdf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMacAlgorithmNamesStatics {
    type Vtable = IMacAlgorithmNamesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41412678_fb1e_43a4_895e_a9026e4390a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HmacMd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HmacSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HmacSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HmacSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HmacSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AesCmac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMacAlgorithmProvider {
    type Vtable = IMacAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a3fc5c3_1cbd_41ce_a092_aa0bc5d2d2f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MacLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmProvider2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMacAlgorithmProvider2 {
    type Vtable = IMacAlgorithmProvider2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6da32a15_d931_42ed_8e7e_c301caee119c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmProvider2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateHash: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMacAlgorithmProviderStatics {
    type Vtable = IMacAlgorithmProviderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9bdc147_cc77_4df0_9e4e_b921e080644c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPersistedKeyProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPersistedKeyProviderStatics {
    type Vtable = IPersistedKeyProviderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77274814_d9d4_4cf5_b668_e0457df30894);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistedKeyProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub OpenKeyPairFromCertificateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *mut ::core::ffi::c_void, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, padding: CryptographicPadding, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Certificates")))]
    OpenKeyPairFromCertificateAsync: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub OpenPublicKeyFromCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *mut ::core::ffi::c_void, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, padding: CryptographicPadding, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    OpenPublicKeyFromCertificate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISymmetricAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISymmetricAlgorithmNamesStatics {
    type Vtable = ISymmetricAlgorithmNamesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6870727b_c996_4eae_84d7_79b2aeb73b9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymmetricAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DesCbc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DesEcb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TripleDesCbc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TripleDesEcb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rc2Cbc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rc2Ecb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AesCbc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AesEcb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AesGcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AesCcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AesCbcPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AesEcbPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DesCbcPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DesEcbPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TripleDesCbcPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TripleDesEcbPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rc2CbcPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rc2EcbPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rc4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISymmetricKeyAlgorithmProvider {
    type Vtable = ISymmetricKeyAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d7e4a33_3bd0_4902_8ac8_470d50d21376);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymmetricKeyAlgorithmProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BlockLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateSymmetricKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISymmetricKeyAlgorithmProviderStatics {
    type Vtable = ISymmetricKeyAlgorithmProviderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d3b2326_1f37_491f_b60e_f5431b26b483);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymmetricKeyAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
pub struct KeyDerivationAlgorithmNames;
impl KeyDerivationAlgorithmNames {
    pub fn Pbkdf2Md5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Pbkdf2Md5)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Pbkdf2Sha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Pbkdf2Sha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Pbkdf2Sha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Pbkdf2Sha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Pbkdf2Sha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Pbkdf2Sha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Pbkdf2Sha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Pbkdf2Sha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacMd5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp800108CtrHmacMd5)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp800108CtrHmacSha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp800108CtrHmacSha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp800108CtrHmacSha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp800108CtrHmacSha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatMd5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp80056aConcatMd5)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp80056aConcatSha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp80056aConcatSha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp80056aConcatSha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sp80056aConcatSha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfMd5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CapiKdfMd5)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CapiKdfSha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CapiKdfSha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CapiKdfSha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CapiKdfSha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyDerivationAlgorithmNamesStatics<R, F: FnOnce(&IKeyDerivationAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyDerivationAlgorithmNames, IKeyDerivationAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKeyDerivationAlgorithmNamesStatics2<R, F: FnOnce(&IKeyDerivationAlgorithmNamesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyDerivationAlgorithmNames, IKeyDerivationAlgorithmNamesStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KeyDerivationAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames";
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct KeyDerivationAlgorithmProvider(::windows::core::IUnknown);
impl KeyDerivationAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateKey<'a, P0, E0>(&self, keymaterial: P0) -> ::windows::core::Result<CryptographicKey>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateKey)(::windows::core::Interface::as_raw(this), keymaterial.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn OpenAlgorithm(algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<KeyDerivationAlgorithmProvider> {
        Self::IKeyDerivationAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenAlgorithm)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(algorithm), result__.as_mut_ptr()).from_abi::<KeyDerivationAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyDerivationAlgorithmProviderStatics<R, F: FnOnce(&IKeyDerivationAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyDerivationAlgorithmProvider, IKeyDerivationAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for KeyDerivationAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyDerivationAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyDerivationAlgorithmProvider {}
impl ::core::fmt::Debug for KeyDerivationAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyDerivationAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyDerivationAlgorithmProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider;{e1fba83b-4671-43b7-9158-763aaa98b6bf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyDerivationAlgorithmProvider {
    type Vtable = IKeyDerivationAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = <IKeyDerivationAlgorithmProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyDerivationAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider";
}
impl ::core::convert::From<KeyDerivationAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: KeyDerivationAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyDerivationAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: &KeyDerivationAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyDerivationAlgorithmProvider> for &::windows::core::IUnknown {
    fn from(value: &KeyDerivationAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<KeyDerivationAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: KeyDerivationAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyDerivationAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: &KeyDerivationAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyDerivationAlgorithmProvider> for &::windows::core::IInspectable {
    fn from(value: &KeyDerivationAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for KeyDerivationAlgorithmProvider {}
unsafe impl ::core::marker::Sync for KeyDerivationAlgorithmProvider {}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct KeyDerivationParameters(::windows::core::IUnknown);
impl KeyDerivationParameters {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn KdfGenericBinary(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KdfGenericBinary)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetKdfGenericBinary<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKdfGenericBinary)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn IterationCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IterationCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Capi1KdfTargetAlgorithm(&self) -> ::windows::core::Result<Capi1KdfTargetAlgorithm> {
        let this = &::windows::core::Interface::cast::<IKeyDerivationParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Capi1KdfTargetAlgorithm)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Capi1KdfTargetAlgorithm>(result__)
        }
    }
    pub fn SetCapi1KdfTargetAlgorithm(&self, value: Capi1KdfTargetAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IKeyDerivationParameters2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCapi1KdfTargetAlgorithm)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn BuildForPbkdf2<'a, P0, E0>(pbkdf2salt: P0, iterationcount: u32) -> ::windows::core::Result<KeyDerivationParameters>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IKeyDerivationParametersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildForPbkdf2)(::windows::core::Interface::as_raw(this), pbkdf2salt.try_into().map_err(|e| e.into())?.abi(), iterationcount, result__.as_mut_ptr()).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn BuildForSP800108<'a, P0, E0, P1, E1>(label: P0, context: P1) -> ::windows::core::Result<KeyDerivationParameters>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IKeyDerivationParametersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildForSP800108)(::windows::core::Interface::as_raw(this), label.try_into().map_err(|e| e.into())?.abi(), context.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn BuildForSP80056a<'a, P0, E0, P1, E1, P2, E2, P3, E3, P4, E4>(algorithmid: P0, partyuinfo: P1, partyvinfo: P2, supppubinfo: P3, suppprivinfo: P4) -> ::windows::core::Result<KeyDerivationParameters>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
        P3: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E3>,
        E3: ::std::convert::Into<::windows::core::Error>,
        P4: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E4>,
        E4: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IKeyDerivationParametersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildForSP80056a)(::windows::core::Interface::as_raw(this), algorithmid.try_into().map_err(|e| e.into())?.abi(), partyuinfo.try_into().map_err(|e| e.into())?.abi(), partyvinfo.try_into().map_err(|e| e.into())?.abi(), supppubinfo.try_into().map_err(|e| e.into())?.abi(), suppprivinfo.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    pub fn BuildForCapi1Kdf(capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm) -> ::windows::core::Result<KeyDerivationParameters> {
        Self::IKeyDerivationParametersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildForCapi1Kdf)(::windows::core::Interface::as_raw(this), capi1kdftargetalgorithm, result__.as_mut_ptr()).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyDerivationParametersStatics<R, F: FnOnce(&IKeyDerivationParametersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyDerivationParameters, IKeyDerivationParametersStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKeyDerivationParametersStatics2<R, F: FnOnce(&IKeyDerivationParametersStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyDerivationParameters, IKeyDerivationParametersStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for KeyDerivationParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyDerivationParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyDerivationParameters {}
impl ::core::fmt::Debug for KeyDerivationParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyDerivationParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyDerivationParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.KeyDerivationParameters;{7bf05967-047b-4a8c-964a-469ffd5522e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyDerivationParameters {
    type Vtable = IKeyDerivationParameters_Vtbl;
    const IID: ::windows::core::GUID = <IKeyDerivationParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyDerivationParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.KeyDerivationParameters";
}
impl ::core::convert::From<KeyDerivationParameters> for ::windows::core::IUnknown {
    fn from(value: KeyDerivationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyDerivationParameters> for ::windows::core::IUnknown {
    fn from(value: &KeyDerivationParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyDerivationParameters> for &::windows::core::IUnknown {
    fn from(value: &KeyDerivationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<KeyDerivationParameters> for ::windows::core::IInspectable {
    fn from(value: KeyDerivationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyDerivationParameters> for ::windows::core::IInspectable {
    fn from(value: &KeyDerivationParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyDerivationParameters> for &::windows::core::IInspectable {
    fn from(value: &KeyDerivationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for KeyDerivationParameters {}
unsafe impl ::core::marker::Sync for KeyDerivationParameters {}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
pub struct MacAlgorithmNames;
impl MacAlgorithmNames {
    pub fn HmacMd5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HmacMd5)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HmacSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HmacSha1)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HmacSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HmacSha256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HmacSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HmacSha384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HmacSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HmacSha512)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AesCmac() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AesCmac)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMacAlgorithmNamesStatics<R, F: FnOnce(&IMacAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MacAlgorithmNames, IMacAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MacAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.MacAlgorithmNames";
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct MacAlgorithmProvider(::windows::core::IUnknown);
impl MacAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn MacLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MacLength)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateKey<'a, P0, E0>(&self, keymaterial: P0) -> ::windows::core::Result<CryptographicKey>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateKey)(::windows::core::Interface::as_raw(this), keymaterial.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateHash<'a, P0, E0>(&self, keymaterial: P0) -> ::windows::core::Result<CryptographicHash>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMacAlgorithmProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateHash)(::windows::core::Interface::as_raw(this), keymaterial.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<CryptographicHash>(result__)
        }
    }
    pub fn OpenAlgorithm(algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<MacAlgorithmProvider> {
        Self::IMacAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenAlgorithm)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(algorithm), result__.as_mut_ptr()).from_abi::<MacAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMacAlgorithmProviderStatics<R, F: FnOnce(&IMacAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MacAlgorithmProvider, IMacAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MacAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MacAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MacAlgorithmProvider {}
impl ::core::fmt::Debug for MacAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MacAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MacAlgorithmProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.MacAlgorithmProvider;{4a3fc5c3-1cbd-41ce-a092-aa0bc5d2d2f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MacAlgorithmProvider {
    type Vtable = IMacAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = <IMacAlgorithmProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MacAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.MacAlgorithmProvider";
}
impl ::core::convert::From<MacAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: MacAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MacAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: &MacAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MacAlgorithmProvider> for &::windows::core::IUnknown {
    fn from(value: &MacAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<MacAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: MacAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MacAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: &MacAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&MacAlgorithmProvider> for &::windows::core::IInspectable {
    fn from(value: &MacAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MacAlgorithmProvider {}
unsafe impl ::core::marker::Sync for MacAlgorithmProvider {}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
pub struct PersistedKeyProvider;
impl PersistedKeyProvider {
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub fn OpenKeyPairFromCertificateAsync<'a, P0>(certificate: P0, hashalgorithmname: &::windows::core::HSTRING, padding: CryptographicPadding) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CryptographicKey>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Certificates::Certificate>>,
    {
        Self::IPersistedKeyProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenKeyPairFromCertificateAsync)(::windows::core::Interface::as_raw(this), certificate.into().abi(), ::core::mem::transmute_copy(hashalgorithmname), padding, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<CryptographicKey>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn OpenPublicKeyFromCertificate<'a, P0>(certificate: P0, hashalgorithmname: &::windows::core::HSTRING, padding: CryptographicPadding) -> ::windows::core::Result<CryptographicKey>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Certificates::Certificate>>,
    {
        Self::IPersistedKeyProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenPublicKeyFromCertificate)(::windows::core::Interface::as_raw(this), certificate.into().abi(), ::core::mem::transmute_copy(hashalgorithmname), padding, result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPersistedKeyProviderStatics<R, F: FnOnce(&IPersistedKeyProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PersistedKeyProvider, IPersistedKeyProviderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PersistedKeyProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.PersistedKeyProvider";
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
pub struct SymmetricAlgorithmNames;
impl SymmetricAlgorithmNames {
    pub fn DesCbc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DesCbc)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DesEcb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DesEcb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TripleDesCbc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TripleDesCbc)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TripleDesEcb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TripleDesEcb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Rc2Cbc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Rc2Cbc)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Rc2Ecb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Rc2Ecb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AesCbc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AesCbc)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AesEcb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AesEcb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AesGcm() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AesGcm)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AesCcm() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AesCcm)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AesCbcPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AesCbcPkcs7)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AesEcbPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AesEcbPkcs7)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DesCbcPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DesCbcPkcs7)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DesEcbPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DesEcbPkcs7)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TripleDesCbcPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TripleDesCbcPkcs7)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TripleDesEcbPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TripleDesEcbPkcs7)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Rc2CbcPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Rc2CbcPkcs7)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Rc2EcbPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Rc2EcbPkcs7)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Rc4() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Rc4)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISymmetricAlgorithmNamesStatics<R, F: FnOnce(&ISymmetricAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SymmetricAlgorithmNames, ISymmetricAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SymmetricAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.SymmetricAlgorithmNames";
}
#[doc = "*Required features: `\"Security_Cryptography_Core\"`*"]
#[repr(transparent)]
pub struct SymmetricKeyAlgorithmProvider(::windows::core::IUnknown);
impl SymmetricKeyAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn BlockLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BlockLength)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateSymmetricKey<'a, P0, E0>(&self, keymaterial: P0) -> ::windows::core::Result<CryptographicKey>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateSymmetricKey)(::windows::core::Interface::as_raw(this), keymaterial.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn OpenAlgorithm(algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<SymmetricKeyAlgorithmProvider> {
        Self::ISymmetricKeyAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenAlgorithm)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(algorithm), result__.as_mut_ptr()).from_abi::<SymmetricKeyAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISymmetricKeyAlgorithmProviderStatics<R, F: FnOnce(&ISymmetricKeyAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SymmetricKeyAlgorithmProvider, ISymmetricKeyAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SymmetricKeyAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SymmetricKeyAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SymmetricKeyAlgorithmProvider {}
impl ::core::fmt::Debug for SymmetricKeyAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SymmetricKeyAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SymmetricKeyAlgorithmProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider;{3d7e4a33-3bd0-4902-8ac8-470d50d21376})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SymmetricKeyAlgorithmProvider {
    type Vtable = ISymmetricKeyAlgorithmProvider_Vtbl;
    const IID: ::windows::core::GUID = <ISymmetricKeyAlgorithmProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SymmetricKeyAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider";
}
impl ::core::convert::From<SymmetricKeyAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: SymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SymmetricKeyAlgorithmProvider> for ::windows::core::IUnknown {
    fn from(value: &SymmetricKeyAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SymmetricKeyAlgorithmProvider> for &::windows::core::IUnknown {
    fn from(value: &SymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SymmetricKeyAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: SymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SymmetricKeyAlgorithmProvider> for ::windows::core::IInspectable {
    fn from(value: &SymmetricKeyAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SymmetricKeyAlgorithmProvider> for &::windows::core::IInspectable {
    fn from(value: &SymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for SymmetricKeyAlgorithmProvider {}
unsafe impl ::core::marker::Sync for SymmetricKeyAlgorithmProvider {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
