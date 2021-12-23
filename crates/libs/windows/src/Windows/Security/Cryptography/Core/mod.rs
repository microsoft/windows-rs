#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
pub struct AsymmetricAlgorithmNames {}
impl AsymmetricAlgorithmNames {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaPkcs1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaOaepSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaOaepSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaOaepSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaOaepSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn EcdsaP256Sha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn EcdsaP384Sha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn EcdsaP521Sha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn DsaSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn DsaSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaSignPkcs1Sha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaSignPkcs1Sha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaSignPkcs1Sha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaSignPkcs1Sha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaSignPssSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaSignPssSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaSignPssSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn RsaSignPssSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn EcdsaSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn EcdsaSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn EcdsaSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAsymmetricAlgorithmNamesStatics<R, F: FnOnce(&IAsymmetricAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AsymmetricAlgorithmNames, IAsymmetricAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAsymmetricAlgorithmNamesStatics2<R, F: FnOnce(&IAsymmetricAlgorithmNamesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AsymmetricAlgorithmNames, IAsymmetricAlgorithmNamesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AsymmetricAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames";
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
pub struct AsymmetricKeyAlgorithmProvider(::windows::core::IUnknown);
impl AsymmetricKeyAlgorithmProvider {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn CreateKeyPair(&self, keysize: u32) -> ::windows::core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), keysize, &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportDefaultPrivateKeyBlob<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keyblob: Param0) -> ::windows::core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), keyblob.into_param().abi(), &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportKeyPairWithBlobType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keyblob: Param0, blobtype: CryptographicPrivateKeyBlobType) -> ::windows::core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), keyblob.into_param().abi(), blobtype, &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportDefaultPublicKeyBlob<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keyblob: Param0) -> ::windows::core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), keyblob.into_param().abi(), &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportPublicKeyWithBlobType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keyblob: Param0, blobtype: CryptographicPublicKeyBlobType) -> ::windows::core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), keyblob.into_param().abi(), blobtype, &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn CreateKeyPairWithCurveName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, curvename: Param0) -> ::windows::core::Result<CryptographicKey> {
        let this = &::windows::core::Interface::cast::<IAsymmetricKeyAlgorithmProvider2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), curvename.into_param().abi(), &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn CreateKeyPairWithCurveParameters(&self, parameters: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<CryptographicKey> {
        let this = &::windows::core::Interface::cast::<IAsymmetricKeyAlgorithmProvider2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), parameters.len() as u32, ::core::mem::transmute(parameters.as_ptr()), &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn OpenAlgorithm<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(algorithm: Param0) -> ::windows::core::Result<AsymmetricKeyAlgorithmProvider> {
        Self::IAsymmetricKeyAlgorithmProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), algorithm.into_param().abi(), &mut result__).from_abi::<AsymmetricKeyAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAsymmetricKeyAlgorithmProviderStatics<R, F: FnOnce(&IAsymmetricKeyAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AsymmetricKeyAlgorithmProvider, IAsymmetricKeyAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
}
unsafe impl ::windows::core::Interface for AsymmetricKeyAlgorithmProvider {
    type Vtable = IAsymmetricKeyAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8d2ff37_6259_4e88_b7e0_94191fde699e);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AsymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AsymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AsymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AsymmetricKeyAlgorithmProvider {}
unsafe impl ::core::marker::Sync for AsymmetricKeyAlgorithmProvider {}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for Capi1KdfTargetAlgorithm {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Capi1KdfTargetAlgorithm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Capi1KdfTargetAlgorithm {}
impl ::core::fmt::Debug for Capi1KdfTargetAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Capi1KdfTargetAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Capi1KdfTargetAlgorithm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.Capi1KdfTargetAlgorithm;i4)");
}
impl ::windows::core::DefaultType for Capi1KdfTargetAlgorithm {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
pub struct CryptographicEngine {}
impl CryptographicEngine {
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Encrypt<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, iv: Param2) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), iv.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Decrypt<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, iv: Param2) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), iv.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncryptAndAuthenticate<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param3: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, nonce: Param2, authenticateddata: Param3) -> ::windows::core::Result<EncryptedAndAuthenticatedData> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), nonce.into_param().abi(), authenticateddata.into_param().abi(), &mut result__).from_abi::<EncryptedAndAuthenticatedData>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DecryptAndAuthenticate<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param3: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param4: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, nonce: Param2, authenticationtag: Param3, authenticateddata: Param4) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), nonce.into_param().abi(), authenticationtag.into_param().abi(), authenticateddata.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Sign<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn VerifySignature<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, signature: Param2) -> ::windows::core::Result<bool> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), signature.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DeriveKeyMaterial<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, KeyDerivationParameters>>(key: Param0, parameters: Param1, desiredkeysize: u32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), key.into_param().abi(), parameters.into_param().abi(), desiredkeysize, &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SignHashedData<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn VerifySignatureWithHashInput<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, signature: Param2) -> ::windows::core::Result<bool> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), signature.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn DecryptAsync<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, iv: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), iv.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SignAsync<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SignHashedDataAsync<'a, Param0: ::windows::core::IntoParam<'a, CryptographicKey>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICryptographicEngineStatics<R, F: FnOnce(&ICryptographicEngineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CryptographicEngine, ICryptographicEngineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICryptographicEngineStatics2<R, F: FnOnce(&ICryptographicEngineStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CryptographicEngine, ICryptographicEngineStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CryptographicEngine {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.CryptographicEngine";
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
pub struct CryptographicHash(::windows::core::IUnknown);
impl CryptographicHash {
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetValueAndReset(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
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
}
unsafe impl ::windows::core::Interface for CryptographicHash {
    type Vtable = IHashComputationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5904d1b6_ad31_4603_a3a4_b1bda98e2562);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CryptographicHash {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CryptographicHash {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CryptographicHash {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CryptographicHash {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CryptographicHash {}
unsafe impl ::core::marker::Sync for CryptographicHash {}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
pub struct CryptographicKey(::windows::core::IUnknown);
impl CryptographicKey {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn KeySize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportDefaultPrivateKeyBlobType(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportPrivateKeyWithBlobType(&self, blobtype: CryptographicPrivateKeyBlobType) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), blobtype, &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportDefaultPublicKeyBlobType(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportPublicKeyWithBlobType(&self, blobtype: CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), blobtype, &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
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
}
unsafe impl ::windows::core::Interface for CryptographicKey {
    type Vtable = ICryptographicKeyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed2a3b70_8e7b_4009_8401_ffd1a62eeb27);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CryptographicKey {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CryptographicKey {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CryptographicKey {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CryptographicKey {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CryptographicKey {}
unsafe impl ::core::marker::Sync for CryptographicKey {}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for CryptographicPadding {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CryptographicPadding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CryptographicPadding {}
impl ::core::fmt::Debug for CryptographicPadding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPadding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CryptographicPadding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.CryptographicPadding;i4)");
}
impl ::windows::core::DefaultType for CryptographicPadding {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for CryptographicPrivateKeyBlobType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CryptographicPrivateKeyBlobType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CryptographicPrivateKeyBlobType {}
impl ::core::fmt::Debug for CryptographicPrivateKeyBlobType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPrivateKeyBlobType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CryptographicPrivateKeyBlobType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.CryptographicPrivateKeyBlobType;i4)");
}
impl ::windows::core::DefaultType for CryptographicPrivateKeyBlobType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for CryptographicPublicKeyBlobType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CryptographicPublicKeyBlobType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CryptographicPublicKeyBlobType {}
impl ::core::fmt::Debug for CryptographicPublicKeyBlobType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPublicKeyBlobType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CryptographicPublicKeyBlobType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.CryptographicPublicKeyBlobType;i4)");
}
impl ::windows::core::DefaultType for CryptographicPublicKeyBlobType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
pub struct EccCurveNames {}
impl EccCurveNames {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP160r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP160t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP192r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP192t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP224r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP224t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP256r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP256t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP320r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP320t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP384r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP384t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP512r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BrainpoolP512t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Curve25519() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Ec192wapi() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn NistP192() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn NistP224() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn NistP256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn NistP384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn NistP521() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn NumsP256t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn NumsP384t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn NumsP512t1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP160k1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP160r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP160r2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP192k1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP192r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP224k1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP224r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP256k1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP256r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP384r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SecP521r1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Wtls7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Wtls9() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Wtls12() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn X962P192v1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn X962P192v2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn X962P192v3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn X962P239v1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn X962P239v2() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn X962P239v3() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn X962P256v1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllEccCurveNames() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEccCurveNamesStatics<R, F: FnOnce(&IEccCurveNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EccCurveNames, IEccCurveNamesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for EccCurveNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.EccCurveNames";
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
pub struct EncryptedAndAuthenticatedData(::windows::core::IUnknown);
impl EncryptedAndAuthenticatedData {
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncryptedData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AuthenticationTag(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
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
}
unsafe impl ::windows::core::Interface for EncryptedAndAuthenticatedData {
    type Vtable = IEncryptedAndAuthenticatedDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fa42fe7_1ecb_4b00_bea5_60b83f862f17);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EncryptedAndAuthenticatedData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &EncryptedAndAuthenticatedData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EncryptedAndAuthenticatedData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &EncryptedAndAuthenticatedData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EncryptedAndAuthenticatedData {}
unsafe impl ::core::marker::Sync for EncryptedAndAuthenticatedData {}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
pub struct HashAlgorithmNames {}
impl HashAlgorithmNames {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Md5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHashAlgorithmNamesStatics<R, F: FnOnce(&IHashAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HashAlgorithmNames, IHashAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for HashAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.HashAlgorithmNames";
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
pub struct HashAlgorithmProvider(::windows::core::IUnknown);
impl HashAlgorithmProvider {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn HashLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn HashData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn CreateHash(&self) -> ::windows::core::Result<CryptographicHash> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CryptographicHash>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn OpenAlgorithm<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(algorithm: Param0) -> ::windows::core::Result<HashAlgorithmProvider> {
        Self::IHashAlgorithmProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), algorithm.into_param().abi(), &mut result__).from_abi::<HashAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHashAlgorithmProviderStatics<R, F: FnOnce(&IHashAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HashAlgorithmProvider, IHashAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
}
unsafe impl ::windows::core::Interface for HashAlgorithmProvider {
    type Vtable = IHashAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe9b3080_b2c3_422b_bce1_ec90efb5d7b5);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HashAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &HashAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HashAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &HashAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HashAlgorithmProvider {}
unsafe impl ::core::marker::Sync for HashAlgorithmProvider {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricAlgorithmNamesStatics {
    type Vtable = IAsymmetricAlgorithmNamesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaf6fce4_67c0_46aa_84f9_752e77449f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricAlgorithmNamesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricAlgorithmNamesStatics2 {
    type Vtable = IAsymmetricAlgorithmNamesStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf141c0d6_4bff_4f23_ba66_6045b137d5df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricAlgorithmNamesStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricKeyAlgorithmProvider {
    type Vtable = IAsymmetricKeyAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8d2ff37_6259_4e88_b7e0_94191fde699e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricKeyAlgorithmProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keysize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: ::windows::core::RawPtr, blobtype: CryptographicPrivateKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: ::windows::core::RawPtr, blobtype: CryptographicPublicKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricKeyAlgorithmProvider2 {
    type Vtable = IAsymmetricKeyAlgorithmProvider2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e322a7e_7c4d_4997_ac4f_1b848b36306e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricKeyAlgorithmProvider2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, curvename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters_array_size: u32, parameters: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsymmetricKeyAlgorithmProviderStatics {
    type Vtable = IAsymmetricKeyAlgorithmProviderStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x425bde18_a7f3_47a6_a8d2_c48d6033a65c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricKeyAlgorithmProviderStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicEngineStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICryptographicEngineStatics {
    type Vtable = ICryptographicEngineStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fea0639_6ff7_4c85_a095_95eb31715eb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicEngineStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, iv: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, iv: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, nonce: ::windows::core::RawPtr, authenticateddata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, nonce: ::windows::core::RawPtr, authenticationtag: ::windows::core::RawPtr, authenticateddata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, signature: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, parameters: ::windows::core::RawPtr, desiredkeysize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicEngineStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICryptographicEngineStatics2 {
    type Vtable = ICryptographicEngineStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x675948fe_df9f_4191_92c7_6ce6f58420e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicEngineStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, signature: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, iv: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicKey(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICryptographicKey {
    type Vtable = ICryptographicKeyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed2a3b70_8e7b_4009_8401_ffd1a62eeb27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicKeyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: CryptographicPrivateKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: CryptographicPublicKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEccCurveNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEccCurveNamesStatics {
    type Vtable = IEccCurveNamesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3ff930c_aeeb_409e_b7d4_9b95295aaecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEccCurveNamesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IEncryptedAndAuthenticatedData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEncryptedAndAuthenticatedData {
    type Vtable = IEncryptedAndAuthenticatedDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fa42fe7_1ecb_4b00_bea5_60b83f862f17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEncryptedAndAuthenticatedDataVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHashAlgorithmNamesStatics {
    type Vtable = IHashAlgorithmNamesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b5e0516_de96_4f0a_8d57_dcc9dae36c76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashAlgorithmNamesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHashAlgorithmProvider {
    type Vtable = IHashAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe9b3080_b2c3_422b_bce1_ec90efb5d7b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashAlgorithmProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHashAlgorithmProviderStatics {
    type Vtable = IHashAlgorithmProviderStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fac9741_5cc4_4336_ae38_6212b75a915a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashAlgorithmProviderStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashComputation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHashComputation {
    type Vtable = IHashComputationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5904d1b6_ad31_4603_a3a4_b1bda98e2562);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashComputationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationAlgorithmNamesStatics {
    type Vtable = IKeyDerivationAlgorithmNamesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b6e363e_94d2_4739_a57b_022e0c3a402a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmNamesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationAlgorithmNamesStatics2 {
    type Vtable = IKeyDerivationAlgorithmNamesStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57953fab_6044_466f_97f4_337b7808384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmNamesStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationAlgorithmProvider {
    type Vtable = IKeyDerivationAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1fba83b_4671_43b7_9158_763aaa98b6bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationAlgorithmProviderStatics {
    type Vtable = IKeyDerivationAlgorithmProviderStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a22097a_0a1c_443b_9418_b9498aeb1603);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmProviderStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationParameters {
    type Vtable = IKeyDerivationParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bf05967_047b_4a8c_964a_469ffd5522e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParametersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParameters2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationParameters2 {
    type Vtable = IKeyDerivationParameters2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd4166d1_417e_4f4c_b666_c0d879f3f8e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParameters2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Capi1KdfTargetAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Capi1KdfTargetAlgorithm) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationParametersStatics {
    type Vtable = IKeyDerivationParametersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea961fbe_f37f_4146_9dfe_a456f1735f4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParametersStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbkdf2salt: ::windows::core::RawPtr, iterationcount: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::windows::core::RawPtr, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithmid: ::windows::core::RawPtr, partyuinfo: ::windows::core::RawPtr, partyvinfo: ::windows::core::RawPtr, supppubinfo: ::windows::core::RawPtr, suppprivinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyDerivationParametersStatics2 {
    type Vtable = IKeyDerivationParametersStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5783dd5_58e3_4efb_b283_a1653126e1be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParametersStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMacAlgorithmNamesStatics {
    type Vtable = IMacAlgorithmNamesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41412678_fb1e_43a4_895e_a9026e4390a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmNamesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMacAlgorithmProvider {
    type Vtable = IMacAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a3fc5c3_1cbd_41ce_a092_aa0bc5d2d2f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmProvider2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMacAlgorithmProvider2 {
    type Vtable = IMacAlgorithmProvider2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6da32a15_d931_42ed_8e7e_c301caee119c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmProvider2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMacAlgorithmProviderStatics {
    type Vtable = IMacAlgorithmProviderStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9bdc147_cc77_4df0_9e4e_b921e080644c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmProviderStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPersistedKeyProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPersistedKeyProviderStatics {
    type Vtable = IPersistedKeyProviderStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77274814_d9d4_4cf5_b668_e0457df30894);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistedKeyProviderStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, padding: CryptographicPadding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(feature = "Security_Cryptography_Certificates")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: ::windows::core::RawPtr, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, padding: CryptographicPadding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISymmetricAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISymmetricAlgorithmNamesStatics {
    type Vtable = ISymmetricAlgorithmNamesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6870727b_c996_4eae_84d7_79b2aeb73b9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymmetricAlgorithmNamesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISymmetricKeyAlgorithmProvider {
    type Vtable = ISymmetricKeyAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d7e4a33_3bd0_4902_8ac8_470d50d21376);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymmetricKeyAlgorithmProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISymmetricKeyAlgorithmProviderStatics {
    type Vtable = ISymmetricKeyAlgorithmProviderStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d3b2326_1f37_491f_b60e_f5431b26b483);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymmetricKeyAlgorithmProviderStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
pub struct KeyDerivationAlgorithmNames {}
impl KeyDerivationAlgorithmNames {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Pbkdf2Md5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Pbkdf2Sha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Pbkdf2Sha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Pbkdf2Sha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Pbkdf2Sha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp800108CtrHmacMd5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp800108CtrHmacSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp800108CtrHmacSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp800108CtrHmacSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp800108CtrHmacSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp80056aConcatMd5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp80056aConcatSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp80056aConcatSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp80056aConcatSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Sp80056aConcatSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn CapiKdfMd5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn CapiKdfSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn CapiKdfSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn CapiKdfSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn CapiKdfSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyDerivationAlgorithmNamesStatics<R, F: FnOnce(&IKeyDerivationAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyDerivationAlgorithmNames, IKeyDerivationAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKeyDerivationAlgorithmNamesStatics2<R, F: FnOnce(&IKeyDerivationAlgorithmNamesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyDerivationAlgorithmNames, IKeyDerivationAlgorithmNamesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KeyDerivationAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames";
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
pub struct KeyDerivationAlgorithmProvider(::windows::core::IUnknown);
impl KeyDerivationAlgorithmProvider {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keymaterial: Param0) -> ::windows::core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), keymaterial.into_param().abi(), &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn OpenAlgorithm<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(algorithm: Param0) -> ::windows::core::Result<KeyDerivationAlgorithmProvider> {
        Self::IKeyDerivationAlgorithmProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), algorithm.into_param().abi(), &mut result__).from_abi::<KeyDerivationAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyDerivationAlgorithmProviderStatics<R, F: FnOnce(&IKeyDerivationAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyDerivationAlgorithmProvider, IKeyDerivationAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
}
unsafe impl ::windows::core::Interface for KeyDerivationAlgorithmProvider {
    type Vtable = IKeyDerivationAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1fba83b_4671_43b7_9158_763aaa98b6bf);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyDerivationAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &KeyDerivationAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyDerivationAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &KeyDerivationAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyDerivationAlgorithmProvider {}
unsafe impl ::core::marker::Sync for KeyDerivationAlgorithmProvider {}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
pub struct KeyDerivationParameters(::windows::core::IUnknown);
impl KeyDerivationParameters {
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn KdfGenericBinary(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetKdfGenericBinary<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn IterationCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Capi1KdfTargetAlgorithm(&self) -> ::windows::core::Result<Capi1KdfTargetAlgorithm> {
        let this = &::windows::core::Interface::cast::<IKeyDerivationParameters2>(self)?;
        unsafe {
            let mut result__: Capi1KdfTargetAlgorithm = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Capi1KdfTargetAlgorithm>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn SetCapi1KdfTargetAlgorithm(&self, value: Capi1KdfTargetAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IKeyDerivationParameters2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn BuildForPbkdf2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(pbkdf2salt: Param0, iterationcount: u32) -> ::windows::core::Result<KeyDerivationParameters> {
        Self::IKeyDerivationParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pbkdf2salt.into_param().abi(), iterationcount, &mut result__).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn BuildForSP800108<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(label: Param0, context: Param1) -> ::windows::core::Result<KeyDerivationParameters> {
        Self::IKeyDerivationParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), label.into_param().abi(), context.into_param().abi(), &mut result__).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn BuildForSP80056a<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param3: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param4: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(algorithmid: Param0, partyuinfo: Param1, partyvinfo: Param2, supppubinfo: Param3, suppprivinfo: Param4) -> ::windows::core::Result<KeyDerivationParameters> {
        Self::IKeyDerivationParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), algorithmid.into_param().abi(), partyuinfo.into_param().abi(), partyvinfo.into_param().abi(), supppubinfo.into_param().abi(), suppprivinfo.into_param().abi(), &mut result__).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BuildForCapi1Kdf(capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm) -> ::windows::core::Result<KeyDerivationParameters> {
        Self::IKeyDerivationParametersStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), capi1kdftargetalgorithm, &mut result__).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyDerivationParametersStatics<R, F: FnOnce(&IKeyDerivationParametersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyDerivationParameters, IKeyDerivationParametersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKeyDerivationParametersStatics2<R, F: FnOnce(&IKeyDerivationParametersStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyDerivationParameters, IKeyDerivationParametersStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
}
unsafe impl ::windows::core::Interface for KeyDerivationParameters {
    type Vtable = IKeyDerivationParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bf05967_047b_4a8c_964a_469ffd5522e2);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyDerivationParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &KeyDerivationParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyDerivationParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &KeyDerivationParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyDerivationParameters {}
unsafe impl ::core::marker::Sync for KeyDerivationParameters {}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
pub struct MacAlgorithmNames {}
impl MacAlgorithmNames {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn HmacMd5() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn HmacSha1() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn HmacSha256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn HmacSha384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn HmacSha512() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AesCmac() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMacAlgorithmNamesStatics<R, F: FnOnce(&IMacAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MacAlgorithmNames, IMacAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MacAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.MacAlgorithmNames";
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
pub struct MacAlgorithmProvider(::windows::core::IUnknown);
impl MacAlgorithmProvider {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn MacLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keymaterial: Param0) -> ::windows::core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), keymaterial.into_param().abi(), &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateHash<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keymaterial: Param0) -> ::windows::core::Result<CryptographicHash> {
        let this = &::windows::core::Interface::cast::<IMacAlgorithmProvider2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), keymaterial.into_param().abi(), &mut result__).from_abi::<CryptographicHash>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn OpenAlgorithm<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(algorithm: Param0) -> ::windows::core::Result<MacAlgorithmProvider> {
        Self::IMacAlgorithmProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), algorithm.into_param().abi(), &mut result__).from_abi::<MacAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMacAlgorithmProviderStatics<R, F: FnOnce(&IMacAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MacAlgorithmProvider, IMacAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
}
unsafe impl ::windows::core::Interface for MacAlgorithmProvider {
    type Vtable = IMacAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a3fc5c3_1cbd_41ce_a092_aa0bc5d2d2f5);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MacAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MacAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MacAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MacAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MacAlgorithmProvider {}
unsafe impl ::core::marker::Sync for MacAlgorithmProvider {}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
pub struct PersistedKeyProvider {}
impl PersistedKeyProvider {
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Foundation', 'Security_Cryptography_Certificates'*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub fn OpenKeyPairFromCertificateAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Certificates::Certificate>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(certificate: Param0, hashalgorithmname: Param1, padding: CryptographicPadding) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CryptographicKey>> {
        Self::IPersistedKeyProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), certificate.into_param().abi(), hashalgorithmname.into_param().abi(), padding, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<CryptographicKey>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Security_Cryptography_Certificates'*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn OpenPublicKeyFromCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::Certificates::Certificate>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(certificate: Param0, hashalgorithmname: Param1, padding: CryptographicPadding) -> ::windows::core::Result<CryptographicKey> {
        Self::IPersistedKeyProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), certificate.into_param().abi(), hashalgorithmname.into_param().abi(), padding, &mut result__).from_abi::<CryptographicKey>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPersistedKeyProviderStatics<R, F: FnOnce(&IPersistedKeyProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PersistedKeyProvider, IPersistedKeyProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PersistedKeyProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.PersistedKeyProvider";
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
pub struct SymmetricAlgorithmNames {}
impl SymmetricAlgorithmNames {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn DesCbc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn DesEcb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn TripleDesCbc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn TripleDesEcb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Rc2Cbc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Rc2Ecb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AesCbc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AesEcb() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AesGcm() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AesCcm() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AesCbcPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AesEcbPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn DesCbcPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn DesEcbPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn TripleDesCbcPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn TripleDesEcbPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Rc2CbcPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Rc2EcbPkcs7() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn Rc4() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISymmetricAlgorithmNamesStatics<R, F: FnOnce(&ISymmetricAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SymmetricAlgorithmNames, ISymmetricAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SymmetricAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.SymmetricAlgorithmNames";
}
#[doc = "*Required features: 'Security_Cryptography_Core'*"]
#[repr(transparent)]
pub struct SymmetricKeyAlgorithmProvider(::windows::core::IUnknown);
impl SymmetricKeyAlgorithmProvider {
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn BlockLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateSymmetricKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keymaterial: Param0) -> ::windows::core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), keymaterial.into_param().abi(), &mut result__).from_abi::<CryptographicKey>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Cryptography_Core'*"]
    pub fn OpenAlgorithm<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(algorithm: Param0) -> ::windows::core::Result<SymmetricKeyAlgorithmProvider> {
        Self::ISymmetricKeyAlgorithmProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), algorithm.into_param().abi(), &mut result__).from_abi::<SymmetricKeyAlgorithmProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISymmetricKeyAlgorithmProviderStatics<R, F: FnOnce(&ISymmetricKeyAlgorithmProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SymmetricKeyAlgorithmProvider, ISymmetricKeyAlgorithmProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
}
unsafe impl ::windows::core::Interface for SymmetricKeyAlgorithmProvider {
    type Vtable = ISymmetricKeyAlgorithmProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d7e4a33_3bd0_4902_8ac8_470d50d21376);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SymmetricKeyAlgorithmProvider {}
unsafe impl ::core::marker::Sync for SymmetricKeyAlgorithmProvider {}
