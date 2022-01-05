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
pub trait IAsymmetricAlgorithmNamesStatics2Impl: Sized {
    fn EcdsaSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EcdsaSha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EcdsaSha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAsymmetricKeyAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateKeyPair(&self, keysize: u32) -> ::windows::core::Result<CryptographicKey>;
    fn ImportDefaultPrivateKeyBlob(&self, keyblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
    fn ImportKeyPairWithBlobType(&self, keyblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, blobtype: CryptographicPrivateKeyBlobType) -> ::windows::core::Result<CryptographicKey>;
    fn ImportDefaultPublicKeyBlob(&self, keyblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
    fn ImportPublicKeyWithBlobType(&self, keyblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, blobtype: CryptographicPublicKeyBlobType) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAsymmetricKeyAlgorithmProvider2Impl: Sized {
    fn CreateKeyPairWithCurveName(&self, curvename: &::windows::core::HSTRING) -> ::windows::core::Result<CryptographicKey>;
    fn CreateKeyPairWithCurveParameters(&self, parameters: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAsymmetricKeyAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<AsymmetricKeyAlgorithmProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICryptographicEngineStaticsImpl: Sized {
    fn Encrypt(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, iv: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Decrypt(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, iv: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn EncryptAndAuthenticate(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, nonce: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, authenticateddata: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<EncryptedAndAuthenticatedData>;
    fn DecryptAndAuthenticate(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, nonce: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, authenticationtag: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, authenticateddata: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Sign(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn VerifySignature(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, signature: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<bool>;
    fn DeriveKeyMaterial(&self, key: &::core::option::Option<CryptographicKey>, parameters: &::core::option::Option<KeyDerivationParameters>, desiredkeysize: u32) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICryptographicEngineStatics2Impl: Sized {
    fn SignHashedData(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn VerifySignatureWithHashInput(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, signature: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<bool>;
    fn DecryptAsync(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, iv: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
    fn SignAsync(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
    fn SignHashedDataAsync(&self, key: &::core::option::Option<CryptographicKey>, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICryptographicKeyImpl: Sized {
    fn KeySize(&self) -> ::windows::core::Result<u32>;
    fn ExportDefaultPrivateKeyBlobType(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn ExportPrivateKeyWithBlobType(&self, blobtype: CryptographicPrivateKeyBlobType) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn ExportDefaultPublicKeyBlobType(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn ExportPublicKeyWithBlobType(&self, blobtype: CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
pub trait IEncryptedAndAuthenticatedDataImpl: Sized {
    fn EncryptedData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn AuthenticationTag(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
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
pub trait IHashAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HashLength(&self) -> ::windows::core::Result<u32>;
    fn HashData(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn CreateHash(&self) -> ::windows::core::Result<CryptographicHash>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHashAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<HashAlgorithmProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHashComputationImpl: Sized {
    fn Append(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn GetValueAndReset(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
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
pub trait IKeyDerivationAlgorithmNamesStatics2Impl: Sized {
    fn CapiKdfMd5(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CapiKdfSha1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CapiKdfSha256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CapiKdfSha384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CapiKdfSha512(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateKey(&self, keymaterial: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<KeyDerivationAlgorithmProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationParametersImpl: Sized {
    fn KdfGenericBinary(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetKdfGenericBinary(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn IterationCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationParameters2Impl: Sized {
    fn Capi1KdfTargetAlgorithm(&self) -> ::windows::core::Result<Capi1KdfTargetAlgorithm>;
    fn SetCapi1KdfTargetAlgorithm(&self, value: Capi1KdfTargetAlgorithm) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationParametersStaticsImpl: Sized {
    fn BuildForPbkdf2(&self, pbkdf2salt: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, iterationcount: u32) -> ::windows::core::Result<KeyDerivationParameters>;
    fn BuildForSP800108(&self, label: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, context: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<KeyDerivationParameters>;
    fn BuildForSP80056a(&self, algorithmid: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, partyuinfo: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, partyvinfo: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, supppubinfo: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, suppprivinfo: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<KeyDerivationParameters>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyDerivationParametersStatics2Impl: Sized {
    fn BuildForCapi1Kdf(&self, capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm) -> ::windows::core::Result<KeyDerivationParameters>;
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
pub trait IMacAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MacLength(&self) -> ::windows::core::Result<u32>;
    fn CreateKey(&self, keymaterial: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMacAlgorithmProvider2Impl: Sized {
    fn CreateHash(&self, keymaterial: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicHash>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMacAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<MacAlgorithmProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPersistedKeyProviderStaticsImpl: Sized {
    fn OpenKeyPairFromCertificateAsync(&self, certificate: &::core::option::Option<super::Certificates::Certificate>, hashalgorithmname: &::windows::core::HSTRING, padding: CryptographicPadding) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CryptographicKey>>;
    fn OpenPublicKeyFromCertificate(&self, certificate: &::core::option::Option<super::Certificates::Certificate>, hashalgorithmname: &::windows::core::HSTRING, padding: CryptographicPadding) -> ::windows::core::Result<CryptographicKey>;
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
pub trait ISymmetricKeyAlgorithmProviderImpl: Sized {
    fn AlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BlockLength(&self) -> ::windows::core::Result<u32>;
    fn CreateSymmetricKey(&self, keymaterial: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CryptographicKey>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISymmetricKeyAlgorithmProviderStaticsImpl: Sized {
    fn OpenAlgorithm(&self, algorithm: &::windows::core::HSTRING) -> ::windows::core::Result<SymmetricKeyAlgorithmProvider>;
}
