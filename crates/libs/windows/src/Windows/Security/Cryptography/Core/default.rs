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
impl ::core::default::Default for Capi1KdfTargetAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Capi1KdfTargetAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Capi1KdfTargetAlgorithm").field(&self.0).finish()
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
impl ::core::default::Default for CryptographicPadding {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CryptographicPadding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPadding").field(&self.0).finish()
    }
}
impl ::core::default::Default for CryptographicPrivateKeyBlobType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CryptographicPrivateKeyBlobType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPrivateKeyBlobType").field(&self.0).finish()
    }
}
impl ::core::default::Default for CryptographicPublicKeyBlobType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CryptographicPublicKeyBlobType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPublicKeyBlobType").field(&self.0).finish()
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
