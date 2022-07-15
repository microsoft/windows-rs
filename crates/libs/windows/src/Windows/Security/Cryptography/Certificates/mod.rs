#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct Certificate(::windows::core::IUnknown);
impl Certificate {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BuildChainAsync<'a, P0, E0>(&self, certificates: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<Certificate>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildChainAsync)(::windows::core::Interface::as_raw(this), certificates.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<CertificateChain>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BuildChainWithParametersAsync<'a, P0, E0, P1>(&self, certificates: P0, parameters: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<Certificate>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ChainBuildingParameters>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildChainWithParametersAsync)(::windows::core::Interface::as_raw(this), certificates.try_into().map_err(|e| e.into())?.abi(), parameters.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<CertificateChain>>(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SerialNumber)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn GetHashValue(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetHashValue)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn GetHashValueWithAlgorithm(&self, hashalgorithmname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetHashValueWithAlgorithm)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(hashalgorithmname), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetCertificateBlob(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCertificateBlob)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Issuer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Issuer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn HasPrivateKey(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasPrivateKey)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsStronglyProtected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsStronglyProtected)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValidFrom(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ValidFrom)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValidTo(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ValidTo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnhancedKeyUsages)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFriendlyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FriendlyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsSecurityDeviceBound(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSecurityDeviceBound)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn KeyUsages(&self) -> ::windows::core::Result<CertificateKeyUsages> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyUsages)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CertificateKeyUsages>(result__)
        }
    }
    pub fn KeyAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyAlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SignatureAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SignatureAlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SignatureHashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SignatureHashAlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SubjectAlternativeName(&self) -> ::windows::core::Result<SubjectAlternativeNameInfo> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubjectAlternativeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SubjectAlternativeNameInfo>(result__)
        }
    }
    pub fn IsPerUser(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsPerUser)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn StoreName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StoreName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyStorageProviderName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCertificate<'a, P0, E0>(certblob: P0) -> ::windows::core::Result<Certificate>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICertificateFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateCertificate)(::windows::core::Interface::as_raw(this), certblob.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<Certificate>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICertificateFactory<R, F: FnOnce(&ICertificateFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Certificate, ICertificateFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Certificate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Certificate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Certificate {}
impl ::core::fmt::Debug for Certificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Certificate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Certificate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.Certificate;{333f740c-04d8-43b3-b278-8c5fcc9be5a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Certificate {
    type Vtable = ICertificate_Vtbl;
    const IID: ::windows::core::GUID = <ICertificate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Certificate {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.Certificate";
}
impl ::core::convert::From<Certificate> for ::windows::core::IUnknown {
    fn from(value: Certificate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Certificate> for ::windows::core::IUnknown {
    fn from(value: &Certificate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Certificate> for &::windows::core::IUnknown {
    fn from(value: &Certificate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<Certificate> for ::windows::core::IInspectable {
    fn from(value: Certificate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Certificate> for ::windows::core::IInspectable {
    fn from(value: &Certificate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Certificate> for &::windows::core::IInspectable {
    fn from(value: &Certificate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for Certificate {}
unsafe impl ::core::marker::Sync for Certificate {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CertificateChain(::windows::core::IUnknown);
impl CertificateChain {
    pub fn Validate(&self) -> ::windows::core::Result<ChainValidationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Validate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ChainValidationResult>(result__)
        }
    }
    pub fn ValidateWithParameters<'a, P0>(&self, parameter: P0) -> ::windows::core::Result<ChainValidationResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ChainValidationParameters>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ValidateWithParameters)(::windows::core::Interface::as_raw(this), parameter.into().abi(), result__.as_mut_ptr()).from_abi::<ChainValidationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCertificates(&self, includeroot: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCertificates)(::windows::core::Interface::as_raw(this), includeroot, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<Certificate>>(result__)
        }
    }
}
impl ::core::clone::Clone for CertificateChain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CertificateChain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateChain {}
impl ::core::fmt::Debug for CertificateChain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateChain").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateChain {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateChain;{20bf5385-3691-4501-a62c-fd97278b31ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CertificateChain {
    type Vtable = ICertificateChain_Vtbl;
    const IID: ::windows::core::GUID = <ICertificateChain as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CertificateChain {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateChain";
}
impl ::core::convert::From<CertificateChain> for ::windows::core::IUnknown {
    fn from(value: CertificateChain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateChain> for ::windows::core::IUnknown {
    fn from(value: &CertificateChain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateChain> for &::windows::core::IUnknown {
    fn from(value: &CertificateChain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CertificateChain> for ::windows::core::IInspectable {
    fn from(value: CertificateChain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateChain> for ::windows::core::IInspectable {
    fn from(value: &CertificateChain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateChain> for &::windows::core::IInspectable {
    fn from(value: &CertificateChain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CertificateChain {}
unsafe impl ::core::marker::Sync for CertificateChain {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CertificateChainPolicy(pub i32);
impl CertificateChainPolicy {
    pub const Base: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const NTAuthentication: Self = Self(2i32);
    pub const MicrosoftRoot: Self = Self(3i32);
}
impl ::core::marker::Copy for CertificateChainPolicy {}
impl ::core::clone::Clone for CertificateChainPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CertificateChainPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CertificateChainPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for CertificateChainPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateChainPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateChainPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.CertificateChainPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
pub struct CertificateEnrollmentManager;
impl CertificateEnrollmentManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateRequestAsync<'a, P0>(request: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CertificateRequestProperties>>,
    {
        Self::ICertificateEnrollmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateRequestAsync)(::windows::core::Interface::as_raw(this), request.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstallCertificateAsync(certificate: &::windows::core::HSTRING, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InstallCertificateAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(certificate), installoption, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataAsync(pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportPfxDataAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(pfxdata), ::core::mem::transmute_copy(password), exportable, keyprotectionlevel, installoption, ::core::mem::transmute_copy(friendlyname), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn UserCertificateEnrollmentManager() -> ::windows::core::Result<UserCertificateEnrollmentManager> {
        Self::ICertificateEnrollmentManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserCertificateEnrollmentManager)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserCertificateEnrollmentManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataToKspAsync(pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING, keystorageprovider: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportPfxDataToKspAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(pfxdata), ::core::mem::transmute_copy(password), exportable, keyprotectionlevel, installoption, ::core::mem::transmute_copy(friendlyname), ::core::mem::transmute_copy(keystorageprovider), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataToKspWithParametersAsync<'a, P0>(pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, pfximportparameters: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PfxImportParameters>>,
    {
        Self::ICertificateEnrollmentManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportPfxDataToKspWithParametersAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(pfxdata), ::core::mem::transmute_copy(password), pfximportparameters.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICertificateEnrollmentManagerStatics<R, F: FnOnce(&ICertificateEnrollmentManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CertificateEnrollmentManager, ICertificateEnrollmentManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICertificateEnrollmentManagerStatics2<R, F: FnOnce(&ICertificateEnrollmentManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CertificateEnrollmentManager, ICertificateEnrollmentManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICertificateEnrollmentManagerStatics3<R, F: FnOnce(&ICertificateEnrollmentManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CertificateEnrollmentManager, ICertificateEnrollmentManagerStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CertificateEnrollmentManager {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager";
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CertificateExtension(::windows::core::IUnknown);
impl CertificateExtension {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CertificateExtension, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ObjectId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ObjectId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetObjectId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetObjectId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsCritical(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsCritical)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCritical(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCritical)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn EncodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EncodeValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn SetValue(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
}
impl ::core::clone::Clone for CertificateExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CertificateExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateExtension {}
impl ::core::fmt::Debug for CertificateExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateExtension {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateExtension;{84cf0656-a9e6-454d-8e45-2ea7c4bcd53b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CertificateExtension {
    type Vtable = ICertificateExtension_Vtbl;
    const IID: ::windows::core::GUID = <ICertificateExtension as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CertificateExtension {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateExtension";
}
impl ::core::convert::From<CertificateExtension> for ::windows::core::IUnknown {
    fn from(value: CertificateExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateExtension> for ::windows::core::IUnknown {
    fn from(value: &CertificateExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateExtension> for &::windows::core::IUnknown {
    fn from(value: &CertificateExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CertificateExtension> for ::windows::core::IInspectable {
    fn from(value: CertificateExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateExtension> for ::windows::core::IInspectable {
    fn from(value: &CertificateExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateExtension> for &::windows::core::IInspectable {
    fn from(value: &CertificateExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CertificateExtension {}
unsafe impl ::core::marker::Sync for CertificateExtension {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CertificateKeyUsages(::windows::core::IUnknown);
impl CertificateKeyUsages {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CertificateKeyUsages, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn EncipherOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EncipherOnly)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEncipherOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncipherOnly)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CrlSign(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CrlSign)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCrlSign(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCrlSign)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyCertificateSign(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyCertificateSign)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetKeyCertificateSign(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyCertificateSign)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyAgreement(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyAgreement)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetKeyAgreement(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyAgreement)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DataEncipherment(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DataEncipherment)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDataEncipherment(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDataEncipherment)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyEncipherment(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyEncipherment)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetKeyEncipherment(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyEncipherment)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn NonRepudiation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NonRepudiation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetNonRepudiation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNonRepudiation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DigitalSignature(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DigitalSignature)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDigitalSignature(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDigitalSignature)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CertificateKeyUsages {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CertificateKeyUsages {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateKeyUsages {}
impl ::core::fmt::Debug for CertificateKeyUsages {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateKeyUsages").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateKeyUsages {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateKeyUsages;{6ac6206f-e1cf-486a-b485-a69c83e46fd1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CertificateKeyUsages {
    type Vtable = ICertificateKeyUsages_Vtbl;
    const IID: ::windows::core::GUID = <ICertificateKeyUsages as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CertificateKeyUsages {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateKeyUsages";
}
impl ::core::convert::From<CertificateKeyUsages> for ::windows::core::IUnknown {
    fn from(value: CertificateKeyUsages) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateKeyUsages> for ::windows::core::IUnknown {
    fn from(value: &CertificateKeyUsages) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateKeyUsages> for &::windows::core::IUnknown {
    fn from(value: &CertificateKeyUsages) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CertificateKeyUsages> for ::windows::core::IInspectable {
    fn from(value: CertificateKeyUsages) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateKeyUsages> for ::windows::core::IInspectable {
    fn from(value: &CertificateKeyUsages) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateKeyUsages> for &::windows::core::IInspectable {
    fn from(value: &CertificateKeyUsages) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CertificateKeyUsages {}
unsafe impl ::core::marker::Sync for CertificateKeyUsages {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CertificateQuery(::windows::core::IUnknown);
impl CertificateQuery {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CertificateQuery, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnhancedKeyUsages)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn IssuerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IssuerName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetIssuerName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIssuerName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FriendlyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFriendlyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Thumbprint(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Thumbprint)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn SetThumbprint(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThumbprint)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn HardwareOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareOnly)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHardwareOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHardwareOnly)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeDuplicates(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IncludeDuplicates)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeDuplicates(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIncludeDuplicates)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeExpiredCertificates(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IncludeExpiredCertificates)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeExpiredCertificates(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIncludeExpiredCertificates)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn StoreName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StoreName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetStoreName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStoreName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for CertificateQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CertificateQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateQuery {}
impl ::core::fmt::Debug for CertificateQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateQuery;{5b082a31-a728-4916-b5ee-ffcb8acf2417})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CertificateQuery {
    type Vtable = ICertificateQuery_Vtbl;
    const IID: ::windows::core::GUID = <ICertificateQuery as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CertificateQuery {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateQuery";
}
impl ::core::convert::From<CertificateQuery> for ::windows::core::IUnknown {
    fn from(value: CertificateQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateQuery> for ::windows::core::IUnknown {
    fn from(value: &CertificateQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateQuery> for &::windows::core::IUnknown {
    fn from(value: &CertificateQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CertificateQuery> for ::windows::core::IInspectable {
    fn from(value: CertificateQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateQuery> for ::windows::core::IInspectable {
    fn from(value: &CertificateQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateQuery> for &::windows::core::IInspectable {
    fn from(value: &CertificateQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CertificateQuery {}
unsafe impl ::core::marker::Sync for CertificateQuery {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CertificateRequestProperties(::windows::core::IUnknown);
impl CertificateRequestProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CertificateRequestProperties, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubject)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn KeyAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyAlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetKeyAlgorithmName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyAlgorithmName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn KeySize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeySize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetKeySize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeySize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FriendlyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFriendlyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HashAlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetHashAlgorithmName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHashAlgorithmName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Exportable(&self) -> ::windows::core::Result<ExportOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Exportable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ExportOption>(result__)
        }
    }
    pub fn SetExportable(&self, value: ExportOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExportable)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyUsages(&self) -> ::windows::core::Result<EnrollKeyUsages> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyUsages)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EnrollKeyUsages>(result__)
        }
    }
    pub fn SetKeyUsages(&self, value: EnrollKeyUsages) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyUsages)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyProtectionLevel(&self) -> ::windows::core::Result<KeyProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyProtectionLevel)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyProtectionLevel>(result__)
        }
    }
    pub fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyStorageProviderName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetKeyStorageProviderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyStorageProviderName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SmartcardReaderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SmartcardReaderName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSmartcardReaderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSmartcardReaderName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SigningCertificate(&self) -> ::windows::core::Result<Certificate> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SigningCertificate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Certificate>(result__)
        }
    }
    pub fn SetSigningCertificate<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Certificate>>,
    {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSigningCertificate)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn AttestationCredentialCertificate(&self) -> ::windows::core::Result<Certificate> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AttestationCredentialCertificate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Certificate>(result__)
        }
    }
    pub fn SetAttestationCredentialCertificate<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Certificate>>,
    {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAttestationCredentialCertificate)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn CurveName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurveName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCurveName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCurveName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CurveParameters(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurveParameters)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn SetCurveParameters(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCurveParameters)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn ContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContainerNamePrefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContainerNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetContainerNamePrefix)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContainerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContainerName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContainerName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetContainerName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn UseExistingKey(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UseExistingKey)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetUseExistingKey(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetUseExistingKey)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SuppressedDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SuppressedDefaults)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn SubjectAlternativeName(&self) -> ::windows::core::Result<SubjectAlternativeNameInfo> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubjectAlternativeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SubjectAlternativeNameInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<CertificateExtension>> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Extensions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<CertificateExtension>>(result__)
        }
    }
}
impl ::core::clone::Clone for CertificateRequestProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CertificateRequestProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateRequestProperties {}
impl ::core::fmt::Debug for CertificateRequestProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateRequestProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateRequestProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateRequestProperties;{487e84f6-94e2-4dce-8833-1a700a37a29a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CertificateRequestProperties {
    type Vtable = ICertificateRequestProperties_Vtbl;
    const IID: ::windows::core::GUID = <ICertificateRequestProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CertificateRequestProperties {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateRequestProperties";
}
impl ::core::convert::From<CertificateRequestProperties> for ::windows::core::IUnknown {
    fn from(value: CertificateRequestProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateRequestProperties> for ::windows::core::IUnknown {
    fn from(value: &CertificateRequestProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateRequestProperties> for &::windows::core::IUnknown {
    fn from(value: &CertificateRequestProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CertificateRequestProperties> for ::windows::core::IInspectable {
    fn from(value: CertificateRequestProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateRequestProperties> for ::windows::core::IInspectable {
    fn from(value: &CertificateRequestProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateRequestProperties> for &::windows::core::IInspectable {
    fn from(value: &CertificateRequestProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CertificateRequestProperties {}
unsafe impl ::core::marker::Sync for CertificateRequestProperties {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CertificateStore(::windows::core::IUnknown);
impl CertificateStore {
    pub fn Add<'a, P0>(&self, certificate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Certificate>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Add)(::windows::core::Interface::as_raw(this), certificate.into().abi()).ok() }
    }
    pub fn Delete<'a, P0>(&self, certificate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Certificate>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Delete)(::windows::core::Interface::as_raw(this), certificate.into().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CertificateStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CertificateStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CertificateStore {}
impl ::core::fmt::Debug for CertificateStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CertificateStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateStore;{b0bff720-344e-4331-af14-a7f7a7ebc93a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CertificateStore {
    type Vtable = ICertificateStore_Vtbl;
    const IID: ::windows::core::GUID = <ICertificateStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateStore";
}
impl ::core::convert::From<CertificateStore> for ::windows::core::IUnknown {
    fn from(value: CertificateStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateStore> for ::windows::core::IUnknown {
    fn from(value: &CertificateStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateStore> for &::windows::core::IUnknown {
    fn from(value: &CertificateStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CertificateStore> for ::windows::core::IInspectable {
    fn from(value: CertificateStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CertificateStore> for ::windows::core::IInspectable {
    fn from(value: &CertificateStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CertificateStore> for &::windows::core::IInspectable {
    fn from(value: &CertificateStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CertificateStore {}
unsafe impl ::core::marker::Sync for CertificateStore {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
pub struct CertificateStores;
impl CertificateStores {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindAllAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllWithQueryAsync<'a, P0>(query: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CertificateQuery>>,
    {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindAllWithQueryAsync)(::windows::core::Interface::as_raw(this), query.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>(result__)
        })
    }
    pub fn TrustedRootCertificationAuthorities() -> ::windows::core::Result<CertificateStore> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrustedRootCertificationAuthorities)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CertificateStore>(result__)
        })
    }
    pub fn IntermediateCertificationAuthorities() -> ::windows::core::Result<CertificateStore> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IntermediateCertificationAuthorities)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CertificateStore>(result__)
        })
    }
    pub fn GetStoreByName(storename: &::windows::core::HSTRING) -> ::windows::core::Result<CertificateStore> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetStoreByName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(storename), result__.as_mut_ptr()).from_abi::<CertificateStore>(result__)
        })
    }
    pub fn GetUserStoreByName(storename: &::windows::core::HSTRING) -> ::windows::core::Result<UserCertificateStore> {
        Self::ICertificateStoresStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetUserStoreByName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(storename), result__.as_mut_ptr()).from_abi::<UserCertificateStore>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICertificateStoresStatics<R, F: FnOnce(&ICertificateStoresStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CertificateStores, ICertificateStoresStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICertificateStoresStatics2<R, F: FnOnce(&ICertificateStoresStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CertificateStores, ICertificateStoresStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CertificateStores {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateStores";
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct ChainBuildingParameters(::windows::core::IUnknown);
impl ChainBuildingParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChainBuildingParameters, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnhancedKeyUsages)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValidationTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ValidationTimestamp)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValidationTimestamp(&self, value: super::super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValidationTimestamp)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RevocationCheckEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RevocationCheckEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRevocationCheckEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRevocationCheckEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn NetworkRetrievalEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NetworkRetrievalEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetNetworkRetrievalEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNetworkRetrievalEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AuthorityInformationAccessEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AuthorityInformationAccessEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuthorityInformationAccessEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAuthorityInformationAccessEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentTimeValidationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentTimeValidationEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCurrentTimeValidationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCurrentTimeValidationEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExclusiveTrustRoots(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExclusiveTrustRoots)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<Certificate>>(result__)
        }
    }
}
impl ::core::clone::Clone for ChainBuildingParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChainBuildingParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChainBuildingParameters {}
impl ::core::fmt::Debug for ChainBuildingParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChainBuildingParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChainBuildingParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.ChainBuildingParameters;{422ba922-7c8d-47b7-b59b-b12703733ac3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ChainBuildingParameters {
    type Vtable = IChainBuildingParameters_Vtbl;
    const IID: ::windows::core::GUID = <IChainBuildingParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChainBuildingParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ChainBuildingParameters";
}
impl ::core::convert::From<ChainBuildingParameters> for ::windows::core::IUnknown {
    fn from(value: ChainBuildingParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChainBuildingParameters> for ::windows::core::IUnknown {
    fn from(value: &ChainBuildingParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ChainBuildingParameters> for &::windows::core::IUnknown {
    fn from(value: &ChainBuildingParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ChainBuildingParameters> for ::windows::core::IInspectable {
    fn from(value: ChainBuildingParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChainBuildingParameters> for ::windows::core::IInspectable {
    fn from(value: &ChainBuildingParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ChainBuildingParameters> for &::windows::core::IInspectable {
    fn from(value: &ChainBuildingParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ChainBuildingParameters {}
unsafe impl ::core::marker::Sync for ChainBuildingParameters {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct ChainValidationParameters(::windows::core::IUnknown);
impl ChainValidationParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChainValidationParameters, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CertificateChainPolicy(&self) -> ::windows::core::Result<CertificateChainPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CertificateChainPolicy)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CertificateChainPolicy>(result__)
        }
    }
    pub fn SetCertificateChainPolicy(&self, value: CertificateChainPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCertificateChainPolicy)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    #[cfg(feature = "Networking")]
    pub fn ServerDnsName(&self) -> ::windows::core::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServerDnsName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Networking::HostName>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    #[cfg(feature = "Networking")]
    pub fn SetServerDnsName<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Networking::HostName>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetServerDnsName)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
}
impl ::core::clone::Clone for ChainValidationParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChainValidationParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChainValidationParameters {}
impl ::core::fmt::Debug for ChainValidationParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChainValidationParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChainValidationParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.ChainValidationParameters;{c4743b4a-7eb0-4b56-a040-b9c8e655ddf3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ChainValidationParameters {
    type Vtable = IChainValidationParameters_Vtbl;
    const IID: ::windows::core::GUID = <IChainValidationParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChainValidationParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ChainValidationParameters";
}
impl ::core::convert::From<ChainValidationParameters> for ::windows::core::IUnknown {
    fn from(value: ChainValidationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChainValidationParameters> for ::windows::core::IUnknown {
    fn from(value: &ChainValidationParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ChainValidationParameters> for &::windows::core::IUnknown {
    fn from(value: &ChainValidationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ChainValidationParameters> for ::windows::core::IInspectable {
    fn from(value: ChainValidationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChainValidationParameters> for ::windows::core::IInspectable {
    fn from(value: &ChainValidationParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ChainValidationParameters> for &::windows::core::IInspectable {
    fn from(value: &ChainValidationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ChainValidationParameters {}
unsafe impl ::core::marker::Sync for ChainValidationParameters {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChainValidationResult(pub i32);
impl ChainValidationResult {
    pub const Success: Self = Self(0i32);
    pub const Untrusted: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Expired: Self = Self(3i32);
    pub const IncompleteChain: Self = Self(4i32);
    pub const InvalidSignature: Self = Self(5i32);
    pub const WrongUsage: Self = Self(6i32);
    pub const InvalidName: Self = Self(7i32);
    pub const InvalidCertificateAuthorityPolicy: Self = Self(8i32);
    pub const BasicConstraintsError: Self = Self(9i32);
    pub const UnknownCriticalExtension: Self = Self(10i32);
    pub const RevocationInformationMissing: Self = Self(11i32);
    pub const RevocationFailure: Self = Self(12i32);
    pub const OtherErrors: Self = Self(13i32);
}
impl ::core::marker::Copy for ChainValidationResult {}
impl ::core::clone::Clone for ChainValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChainValidationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChainValidationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChainValidationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChainValidationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChainValidationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.ChainValidationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CmsAttachedSignature(::windows::core::IUnknown);
impl CmsAttachedSignature {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Certificates)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<Certificate>>(result__)
        }
    }
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Signers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Signers)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>(result__)
        }
    }
    pub fn VerifySignature(&self) -> ::windows::core::Result<SignatureValidationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VerifySignature)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SignatureValidationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCmsAttachedSignature<'a, P0, E0>(inputblob: P0) -> ::windows::core::Result<CmsAttachedSignature>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICmsAttachedSignatureFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateCmsAttachedSignature)(::windows::core::Interface::as_raw(this), inputblob.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<CmsAttachedSignature>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GenerateSignatureAsync<'a, P0, E0, P1, E1, P2, E2>(data: P0, signers: P1, certificates: P2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<Certificate>>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICmsAttachedSignatureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GenerateSignatureAsync)(::windows::core::Interface::as_raw(this), data.try_into().map_err(|e| e.into())?.abi(), signers.try_into().map_err(|e| e.into())?.abi(), certificates.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICmsAttachedSignatureFactory<R, F: FnOnce(&ICmsAttachedSignatureFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CmsAttachedSignature, ICmsAttachedSignatureFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICmsAttachedSignatureStatics<R, F: FnOnce(&ICmsAttachedSignatureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CmsAttachedSignature, ICmsAttachedSignatureStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CmsAttachedSignature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CmsAttachedSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CmsAttachedSignature {}
impl ::core::fmt::Debug for CmsAttachedSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CmsAttachedSignature").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CmsAttachedSignature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CmsAttachedSignature;{61899d9d-3757-4ecb-bddc-0ca357d7a936})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CmsAttachedSignature {
    type Vtable = ICmsAttachedSignature_Vtbl;
    const IID: ::windows::core::GUID = <ICmsAttachedSignature as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CmsAttachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsAttachedSignature";
}
impl ::core::convert::From<CmsAttachedSignature> for ::windows::core::IUnknown {
    fn from(value: CmsAttachedSignature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CmsAttachedSignature> for ::windows::core::IUnknown {
    fn from(value: &CmsAttachedSignature) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CmsAttachedSignature> for &::windows::core::IUnknown {
    fn from(value: &CmsAttachedSignature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CmsAttachedSignature> for ::windows::core::IInspectable {
    fn from(value: CmsAttachedSignature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CmsAttachedSignature> for ::windows::core::IInspectable {
    fn from(value: &CmsAttachedSignature) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CmsAttachedSignature> for &::windows::core::IInspectable {
    fn from(value: &CmsAttachedSignature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CmsAttachedSignature {}
unsafe impl ::core::marker::Sync for CmsAttachedSignature {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CmsDetachedSignature(::windows::core::IUnknown);
impl CmsDetachedSignature {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Certificates)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<Certificate>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Signers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Signers)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn VerifySignatureAsync<'a, P0, E0>(&self, data: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SignatureValidationResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VerifySignatureAsync)(::windows::core::Interface::as_raw(this), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<SignatureValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCmsDetachedSignature<'a, P0, E0>(inputblob: P0) -> ::windows::core::Result<CmsDetachedSignature>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICmsDetachedSignatureFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateCmsDetachedSignature)(::windows::core::Interface::as_raw(this), inputblob.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<CmsDetachedSignature>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GenerateSignatureAsync<'a, P0, E0, P1, E1, P2, E2>(data: P0, signers: P1, certificates: P2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<Certificate>>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICmsDetachedSignatureStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GenerateSignatureAsync)(::windows::core::Interface::as_raw(this), data.try_into().map_err(|e| e.into())?.abi(), signers.try_into().map_err(|e| e.into())?.abi(), certificates.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICmsDetachedSignatureFactory<R, F: FnOnce(&ICmsDetachedSignatureFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CmsDetachedSignature, ICmsDetachedSignatureFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICmsDetachedSignatureStatics<R, F: FnOnce(&ICmsDetachedSignatureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CmsDetachedSignature, ICmsDetachedSignatureStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CmsDetachedSignature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CmsDetachedSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CmsDetachedSignature {}
impl ::core::fmt::Debug for CmsDetachedSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CmsDetachedSignature").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CmsDetachedSignature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CmsDetachedSignature;{0f1ef154-f65e-4536-8339-5944081db2ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CmsDetachedSignature {
    type Vtable = ICmsDetachedSignature_Vtbl;
    const IID: ::windows::core::GUID = <ICmsDetachedSignature as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CmsDetachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsDetachedSignature";
}
impl ::core::convert::From<CmsDetachedSignature> for ::windows::core::IUnknown {
    fn from(value: CmsDetachedSignature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CmsDetachedSignature> for ::windows::core::IUnknown {
    fn from(value: &CmsDetachedSignature) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CmsDetachedSignature> for &::windows::core::IUnknown {
    fn from(value: &CmsDetachedSignature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CmsDetachedSignature> for ::windows::core::IInspectable {
    fn from(value: CmsDetachedSignature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CmsDetachedSignature> for ::windows::core::IInspectable {
    fn from(value: &CmsDetachedSignature) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CmsDetachedSignature> for &::windows::core::IInspectable {
    fn from(value: &CmsDetachedSignature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CmsDetachedSignature {}
unsafe impl ::core::marker::Sync for CmsDetachedSignature {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CmsSignerInfo(::windows::core::IUnknown);
impl CmsSignerInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CmsSignerInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Certificate(&self) -> ::windows::core::Result<Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Certificate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Certificate>(result__)
        }
    }
    pub fn SetCertificate<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Certificate>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCertificate)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn HashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HashAlgorithmName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetHashAlgorithmName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHashAlgorithmName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TimestampInfo(&self) -> ::windows::core::Result<CmsTimestampInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TimestampInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CmsTimestampInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for CmsSignerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CmsSignerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CmsSignerInfo {}
impl ::core::fmt::Debug for CmsSignerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CmsSignerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CmsSignerInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CmsSignerInfo;{50d020db-1d2f-4c1a-b5c5-d0188ff91f47})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CmsSignerInfo {
    type Vtable = ICmsSignerInfo_Vtbl;
    const IID: ::windows::core::GUID = <ICmsSignerInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CmsSignerInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsSignerInfo";
}
impl ::core::convert::From<CmsSignerInfo> for ::windows::core::IUnknown {
    fn from(value: CmsSignerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CmsSignerInfo> for ::windows::core::IUnknown {
    fn from(value: &CmsSignerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CmsSignerInfo> for &::windows::core::IUnknown {
    fn from(value: &CmsSignerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CmsSignerInfo> for ::windows::core::IInspectable {
    fn from(value: CmsSignerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CmsSignerInfo> for ::windows::core::IInspectable {
    fn from(value: &CmsSignerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CmsSignerInfo> for &::windows::core::IInspectable {
    fn from(value: &CmsSignerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CmsSignerInfo {}
unsafe impl ::core::marker::Sync for CmsSignerInfo {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct CmsTimestampInfo(::windows::core::IUnknown);
impl CmsTimestampInfo {
    pub fn SigningCertificate(&self) -> ::windows::core::Result<Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SigningCertificate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Certificate>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Certificates)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<Certificate>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for CmsTimestampInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CmsTimestampInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CmsTimestampInfo {}
impl ::core::fmt::Debug for CmsTimestampInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CmsTimestampInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CmsTimestampInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CmsTimestampInfo;{2f5f00f2-2c18-4f88-8435-c534086076f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CmsTimestampInfo {
    type Vtable = ICmsTimestampInfo_Vtbl;
    const IID: ::windows::core::GUID = <ICmsTimestampInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CmsTimestampInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsTimestampInfo";
}
impl ::core::convert::From<CmsTimestampInfo> for ::windows::core::IUnknown {
    fn from(value: CmsTimestampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CmsTimestampInfo> for ::windows::core::IUnknown {
    fn from(value: &CmsTimestampInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CmsTimestampInfo> for &::windows::core::IUnknown {
    fn from(value: &CmsTimestampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CmsTimestampInfo> for ::windows::core::IInspectable {
    fn from(value: CmsTimestampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CmsTimestampInfo> for ::windows::core::IInspectable {
    fn from(value: &CmsTimestampInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CmsTimestampInfo> for &::windows::core::IInspectable {
    fn from(value: &CmsTimestampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CmsTimestampInfo {}
unsafe impl ::core::marker::Sync for CmsTimestampInfo {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EnrollKeyUsages(pub u32);
impl EnrollKeyUsages {
    pub const None: Self = Self(0u32);
    pub const Decryption: Self = Self(1u32);
    pub const Signing: Self = Self(2u32);
    pub const KeyAgreement: Self = Self(4u32);
    pub const All: Self = Self(16777215u32);
}
impl ::core::marker::Copy for EnrollKeyUsages {}
impl ::core::clone::Clone for EnrollKeyUsages {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnrollKeyUsages {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EnrollKeyUsages {
    type Abi = Self;
}
impl ::core::fmt::Debug for EnrollKeyUsages {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnrollKeyUsages").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EnrollKeyUsages {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EnrollKeyUsages {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EnrollKeyUsages {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EnrollKeyUsages {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EnrollKeyUsages {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for EnrollKeyUsages {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.EnrollKeyUsages;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ExportOption(pub i32);
impl ExportOption {
    pub const NotExportable: Self = Self(0i32);
    pub const Exportable: Self = Self(1i32);
}
impl ::core::marker::Copy for ExportOption {}
impl ::core::clone::Clone for ExportOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExportOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ExportOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExportOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExportOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExportOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.ExportOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificate(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificate {
    type Vtable = ICertificate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x333f740c_04d8_43b3_b278_8c5fcc9be5a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificate_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub BuildChainAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BuildChainAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BuildChainWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificates: *mut ::core::ffi::c_void, parameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BuildChainWithParametersAsync: usize,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetHashValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetHashValueWithAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetCertificateBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetCertificateBlob: usize,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Issuer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HasPrivateKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStronglyProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ValidFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidFrom: usize,
    #[cfg(feature = "Foundation")]
    pub ValidTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidTo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EnhancedKeyUsages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnhancedKeyUsages: usize,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificate2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificate2 {
    type Vtable = ICertificate2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17b8374c_8a25_4d96_a492_8fc29ac4fda6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificate2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsSecurityDeviceBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub KeyUsages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub KeyAlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SignatureAlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SignatureHashAlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SubjectAlternativeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificate3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificate3 {
    type Vtable = ICertificate3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe51a966_ae5f_4652_ace7_c6d7e7724cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificate3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsPerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub StoreName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateChain(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateChain {
    type Vtable = ICertificateChain_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20bf5385_3691_4501_a62c_fd97278b31ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateChain_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChainValidationResult) -> ::windows::core::HRESULT,
    pub ValidateWithParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void, result__: *mut ChainValidationResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includeroot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCertificates: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateEnrollmentManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateEnrollmentManagerStatics {
    type Vtable = ICertificateEnrollmentManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8846ef3f_a986_48fb_9fd7_9aec06935bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub InstallCertificateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoption: InstallOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstallCertificateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateEnrollmentManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateEnrollmentManagerStatics2 {
    type Vtable = ICertificateEnrollmentManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc5b1c33_6429_4014_999c_5d9735802d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub UserCertificateEnrollmentManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataToKspAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keystorageprovider: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataToKspAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateEnrollmentManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateEnrollmentManagerStatics3 {
    type Vtable = ICertificateEnrollmentManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdec82be_617c_425a_b72d_398b26ac7264);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentManagerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataToKspWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pfximportparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataToKspWithParametersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateExtension(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateExtension {
    type Vtable = ICertificateExtension_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84cf0656_a9e6_454d_8e45_2ea7c4bcd53b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateExtension_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetObjectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsCritical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCritical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub EncodeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateFactory {
    type Vtable = ICertificateFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17b4221c_4baf_44a2_9608_04fb62b16942);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCertificate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateKeyUsages(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateKeyUsages {
    type Vtable = ICertificateKeyUsages_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac6206f_e1cf_486a_b485_a69c83e46fd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateKeyUsages_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub EncipherOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEncipherOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CrlSign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCrlSign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub KeyCertificateSign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetKeyCertificateSign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub KeyAgreement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetKeyAgreement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DataEncipherment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDataEncipherment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub KeyEncipherment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetKeyEncipherment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub NonRepudiation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetNonRepudiation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DigitalSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDigitalSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateQuery(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateQuery {
    type Vtable = ICertificateQuery_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b082a31_a728_4916_b5ee_ffcb8acf2417);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateQuery_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EnhancedKeyUsages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnhancedKeyUsages: usize,
    pub IssuerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIssuerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Thumbprint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetThumbprint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub HardwareOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHardwareOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateQuery2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateQuery2 {
    type Vtable = ICertificateQuery2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x935a0af7_0bd9_4f75_b8c2_e27a7f74eecd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateQuery2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IncludeDuplicates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeDuplicates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncludeExpiredCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeExpiredCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub StoreName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetStoreName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateRequestProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateRequestProperties {
    type Vtable = ICertificateRequestProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x487e84f6_94e2_4dce_8833_1a700a37a29a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateRequestProperties_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KeyAlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetKeyAlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KeySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetKeySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HashAlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetHashAlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Exportable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ExportOption) -> ::windows::core::HRESULT,
    pub SetExportable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ExportOption) -> ::windows::core::HRESULT,
    pub KeyUsages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EnrollKeyUsages) -> ::windows::core::HRESULT,
    pub SetKeyUsages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EnrollKeyUsages) -> ::windows::core::HRESULT,
    pub KeyProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyProtectionLevel) -> ::windows::core::HRESULT,
    pub SetKeyProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyProtectionLevel) -> ::windows::core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetKeyStorageProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateRequestProperties2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateRequestProperties2 {
    type Vtable = ICertificateRequestProperties2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da0c954_d73f_4ff3_a0a6_0677c0ada05b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateRequestProperties2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SmartcardReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSmartcardReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SigningCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSigningCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AttestationCredentialCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAttestationCredentialCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateRequestProperties3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateRequestProperties3 {
    type Vtable = ICertificateRequestProperties3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe687f616_734d_46b1_9d4c_6edfdbfc845b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateRequestProperties3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CurveName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCurveName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CurveParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetCurveParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub ContainerNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContainerNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContainerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContainerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UseExistingKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetUseExistingKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateRequestProperties4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateRequestProperties4 {
    type Vtable = ICertificateRequestProperties4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e429ad2_1c61_4fea_b8fe_135fb19cdce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateRequestProperties4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SuppressedDefaults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SuppressedDefaults: usize,
    pub SubjectAlternativeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateStore {
    type Vtable = ICertificateStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0bff720_344e_4331_af14_a7f7a7ebc93a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateStore_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateStore2 {
    type Vtable = ICertificateStore2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7e68e4a_417d_4d1a_babd_15687e549974);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateStore2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateStoresStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateStoresStatics {
    type Vtable = ICertificateStoresStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbecc739_c6fe_4de7_99cf_74c3e596e032);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateStoresStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllWithQueryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllWithQueryAsync: usize,
    pub TrustedRootCertificationAuthorities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IntermediateCertificationAuthorities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICertificateStoresStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICertificateStoresStatics2 {
    type Vtable = ICertificateStoresStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa900b79_a0d4_4b8c_bc55_c0a37eb141ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateStoresStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetUserStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChainBuildingParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChainBuildingParameters {
    type Vtable = IChainBuildingParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x422ba922_7c8d_47b7_b59b_b12703733ac3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChainBuildingParameters_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EnhancedKeyUsages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnhancedKeyUsages: usize,
    #[cfg(feature = "Foundation")]
    pub ValidationTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidationTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub SetValidationTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValidationTimestamp: usize,
    pub RevocationCheckEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRevocationCheckEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub NetworkRetrievalEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetNetworkRetrievalEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AuthorityInformationAccessEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAuthorityInformationAccessEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CurrentTimeValidationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCurrentTimeValidationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExclusiveTrustRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExclusiveTrustRoots: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChainValidationParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChainValidationParameters {
    type Vtable = IChainValidationParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4743b4a_7eb0_4b56_a040_b9c8e655ddf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChainValidationParameters_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CertificateChainPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CertificateChainPolicy) -> ::windows::core::HRESULT,
    pub SetCertificateChainPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CertificateChainPolicy) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking")]
    pub ServerDnsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    ServerDnsName: usize,
    #[cfg(feature = "Networking")]
    pub SetServerDnsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetServerDnsName: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICmsAttachedSignature(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICmsAttachedSignature {
    type Vtable = ICmsAttachedSignature_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61899d9d_3757_4ecb_bddc_0ca357d7a936);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsAttachedSignature_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Certificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Certificates: usize,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Signers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Signers: usize,
    pub VerifySignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SignatureValidationResult) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICmsAttachedSignatureFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICmsAttachedSignatureFactory {
    type Vtable = ICmsAttachedSignatureFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0c8fc15_f757_4c64_a362_52cc1c77cffb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsAttachedSignatureFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCmsAttachedSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCmsAttachedSignature: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICmsAttachedSignatureStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICmsAttachedSignatureStatics {
    type Vtable = ICmsAttachedSignatureStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87989c8e_b0ad_498d_a7f5_78b59bce4b36);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsAttachedSignatureStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GenerateSignatureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, signers: *mut ::core::ffi::c_void, certificates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GenerateSignatureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICmsDetachedSignature(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICmsDetachedSignature {
    type Vtable = ICmsDetachedSignature_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f1ef154_f65e_4536_8339_5944081db2ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsDetachedSignature_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Certificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Certificates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Signers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Signers: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub VerifySignatureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    VerifySignatureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICmsDetachedSignatureFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICmsDetachedSignatureFactory {
    type Vtable = ICmsDetachedSignatureFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4ab3503_ae7f_4387_ad19_00f150e48ebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsDetachedSignatureFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCmsDetachedSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputblob: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCmsDetachedSignature: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICmsDetachedSignatureStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICmsDetachedSignatureStatics {
    type Vtable = ICmsDetachedSignatureStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d114cfd_bf9b_4682_9be6_91f57c053808);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsDetachedSignatureStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GenerateSignatureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, signers: *mut ::core::ffi::c_void, certificates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GenerateSignatureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICmsSignerInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICmsSignerInfo {
    type Vtable = ICmsSignerInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d020db_1d2f_4c1a_b5c5_d0188ff91f47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsSignerInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Certificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HashAlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetHashAlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TimestampInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICmsTimestampInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICmsTimestampInfo {
    type Vtable = ICmsTimestampInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f5f00f2_2c18_4f88_8435_c534086076f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsTimestampInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SigningCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Certificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Certificates: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyAlgorithmNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyAlgorithmNamesStatics {
    type Vtable = IKeyAlgorithmNamesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x479065d7_7ac7_4581_8c3b_d07027140448);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Rsa: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Dsa: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ecdh256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ecdh384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ecdh521: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ecdsa256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ecdsa384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ecdsa521: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyAlgorithmNamesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyAlgorithmNamesStatics2 {
    type Vtable = IKeyAlgorithmNamesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99b5686_e1fd_4a4a_893d_a26f33dd8bb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyAlgorithmNamesStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Ecdsa: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Ecdh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyAttestationHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyAttestationHelperStatics {
    type Vtable = IKeyAttestationHelperStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1648e246_f644_4326_88be_3af102d30e0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyAttestationHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub DecryptTpmAttestationCredentialAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecryptTpmAttestationCredentialAsync: usize,
    pub GetTpmAttestationCredentialId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyAttestationHelperStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyAttestationHelperStatics2 {
    type Vtable = IKeyAttestationHelperStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c590b2c_a6c6_4a5e_9e64_e85d5279df97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyAttestationHelperStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub DecryptTpmAttestationCredentialWithContainerNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, containername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecryptTpmAttestationCredentialWithContainerNameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyStorageProviderNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyStorageProviderNamesStatics {
    type Vtable = IKeyStorageProviderNamesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf186ae0_5529_4602_bd94_0aab91957b5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyStorageProviderNamesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SoftwareKeyStorageProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SmartcardKeyStorageProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PlatformKeyStorageProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyStorageProviderNamesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyStorageProviderNamesStatics2 {
    type Vtable = IKeyStorageProviderNamesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x262d743d_9c2e_41cc_8812_c4d971dd7c60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyStorageProviderNamesStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PassportKeyStorageProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPfxImportParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPfxImportParameters {
    type Vtable = IPfxImportParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x680d3511_9a08_47c8_864a_2edd4d8eb46c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPfxImportParameters_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Exportable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ExportOption) -> ::windows::core::HRESULT,
    pub SetExportable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ExportOption) -> ::windows::core::HRESULT,
    pub KeyProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyProtectionLevel) -> ::windows::core::HRESULT,
    pub SetKeyProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyProtectionLevel) -> ::windows::core::HRESULT,
    pub InstallOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InstallOptions) -> ::windows::core::HRESULT,
    pub SetInstallOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InstallOptions) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetKeyStorageProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContainerNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContainerNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardCertificateStoreNamesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardCertificateStoreNamesStatics {
    type Vtable = IStandardCertificateStoreNamesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c154adb_a496_41f8_8fe5_9e96f36efbf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardCertificateStoreNamesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Personal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TrustedRootCertificationAuthorities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IntermediateCertificationAuthorities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISubjectAlternativeNameInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISubjectAlternativeNameInfo {
    type Vtable = ISubjectAlternativeNameInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x582859f1_569d_4c20_be7b_4e1c9a0bc52b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISubjectAlternativeNameInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EmailName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EmailName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub IPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    IPAddress: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Url: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Url: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DistinguishedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DistinguishedName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrincipalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrincipalName: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISubjectAlternativeNameInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISubjectAlternativeNameInfo2 {
    type Vtable = ISubjectAlternativeNameInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x437a78c6_1c51_41ea_b34a_3d654398a370);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISubjectAlternativeNameInfo2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EmailNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EmailNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub IPAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    IPAddresses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Urls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Urls: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DistinguishedNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DistinguishedNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrincipalNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrincipalNames: usize,
    pub Extension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserCertificateEnrollmentManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserCertificateEnrollmentManager {
    type Vtable = IUserCertificateEnrollmentManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96313718_22e1_4819_b20b_ab46a6eca06e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserCertificateEnrollmentManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateRequestAsync: usize,
    #[cfg(feature = "Foundation")]
    pub InstallCertificateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoption: InstallOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstallCertificateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataToKspAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keystorageprovider: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataToKspAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserCertificateEnrollmentManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserCertificateEnrollmentManager2 {
    type Vtable = IUserCertificateEnrollmentManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dad9cb1_65de_492a_b86d_fc5c482c3747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserCertificateEnrollmentManager2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ImportPfxDataToKspWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pfximportparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportPfxDataToKspWithParametersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserCertificateStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserCertificateStore {
    type Vtable = IUserCertificateStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9fb1d83_789f_4b4e_9180_045a757aac6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserCertificateStore_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAddAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAddAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsync: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InstallOptions(pub u32);
impl InstallOptions {
    pub const None: Self = Self(0u32);
    pub const DeleteExpired: Self = Self(1u32);
}
impl ::core::marker::Copy for InstallOptions {}
impl ::core::clone::Clone for InstallOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InstallOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InstallOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for InstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InstallOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InstallOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InstallOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InstallOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InstallOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InstallOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InstallOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.InstallOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
pub struct KeyAlgorithmNames;
impl KeyAlgorithmNames {
    pub fn Rsa() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Rsa)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Dsa() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Dsa)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdh256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ecdh256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdh384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ecdh384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdh521() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ecdh521)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdsa256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ecdsa256)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdsa384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ecdsa384)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdsa521() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ecdsa521)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdsa() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ecdsa)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdh() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Ecdh)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyAlgorithmNamesStatics<R, F: FnOnce(&IKeyAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyAlgorithmNames, IKeyAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKeyAlgorithmNamesStatics2<R, F: FnOnce(&IKeyAlgorithmNamesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyAlgorithmNames, IKeyAlgorithmNamesStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KeyAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.KeyAlgorithmNames";
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
pub struct KeyAttestationHelper;
impl KeyAttestationHelper {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DecryptTpmAttestationCredentialAsync(credential: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IKeyAttestationHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DecryptTpmAttestationCredentialAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(credential), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    pub fn GetTpmAttestationCredentialId(credential: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAttestationHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetTpmAttestationCredentialId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(credential), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DecryptTpmAttestationCredentialWithContainerNameAsync(credential: &::windows::core::HSTRING, containername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IKeyAttestationHelperStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DecryptTpmAttestationCredentialWithContainerNameAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(credential), ::core::mem::transmute_copy(containername), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyAttestationHelperStatics<R, F: FnOnce(&IKeyAttestationHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyAttestationHelper, IKeyAttestationHelperStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKeyAttestationHelperStatics2<R, F: FnOnce(&IKeyAttestationHelperStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyAttestationHelper, IKeyAttestationHelperStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KeyAttestationHelper {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.KeyAttestationHelper";
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyProtectionLevel(pub i32);
impl KeyProtectionLevel {
    pub const NoConsent: Self = Self(0i32);
    pub const ConsentOnly: Self = Self(1i32);
    pub const ConsentWithPassword: Self = Self(2i32);
    pub const ConsentWithFingerprint: Self = Self(3i32);
}
impl ::core::marker::Copy for KeyProtectionLevel {}
impl ::core::clone::Clone for KeyProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyProtectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KeyProtectionLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyProtectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyProtectionLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyProtectionLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.KeyProtectionLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeySize(pub i32);
impl KeySize {
    pub const Invalid: Self = Self(0i32);
    pub const Rsa2048: Self = Self(2048i32);
    pub const Rsa4096: Self = Self(4096i32);
}
impl ::core::marker::Copy for KeySize {}
impl ::core::clone::Clone for KeySize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeySize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KeySize {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeySize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeySize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeySize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.KeySize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
pub struct KeyStorageProviderNames;
impl KeyStorageProviderNames {
    pub fn SoftwareKeyStorageProvider() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SoftwareKeyStorageProvider)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SmartcardKeyStorageProvider() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SmartcardKeyStorageProvider)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PlatformKeyStorageProvider() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PlatformKeyStorageProvider)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PassportKeyStorageProvider() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PassportKeyStorageProvider)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyStorageProviderNamesStatics<R, F: FnOnce(&IKeyStorageProviderNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyStorageProviderNames, IKeyStorageProviderNamesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKeyStorageProviderNamesStatics2<R, F: FnOnce(&IKeyStorageProviderNamesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyStorageProviderNames, IKeyStorageProviderNamesStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KeyStorageProviderNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.KeyStorageProviderNames";
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct PfxImportParameters(::windows::core::IUnknown);
impl PfxImportParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PfxImportParameters, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Exportable(&self) -> ::windows::core::Result<ExportOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Exportable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ExportOption>(result__)
        }
    }
    pub fn SetExportable(&self, value: ExportOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExportable)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyProtectionLevel(&self) -> ::windows::core::Result<KeyProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyProtectionLevel)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyProtectionLevel>(result__)
        }
    }
    pub fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallOptions(&self) -> ::windows::core::Result<InstallOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InstallOptions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InstallOptions>(result__)
        }
    }
    pub fn SetInstallOptions(&self, value: InstallOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInstallOptions)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FriendlyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFriendlyName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).KeyStorageProviderName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetKeyStorageProviderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeyStorageProviderName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContainerNamePrefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContainerNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContainerNamePrefix)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ReaderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReaderName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetReaderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReaderName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for PfxImportParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PfxImportParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PfxImportParameters {}
impl ::core::fmt::Debug for PfxImportParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PfxImportParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PfxImportParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.PfxImportParameters;{680d3511-9a08-47c8-864a-2edd4d8eb46c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PfxImportParameters {
    type Vtable = IPfxImportParameters_Vtbl;
    const IID: ::windows::core::GUID = <IPfxImportParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PfxImportParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.PfxImportParameters";
}
impl ::core::convert::From<PfxImportParameters> for ::windows::core::IUnknown {
    fn from(value: PfxImportParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PfxImportParameters> for ::windows::core::IUnknown {
    fn from(value: &PfxImportParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PfxImportParameters> for &::windows::core::IUnknown {
    fn from(value: &PfxImportParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PfxImportParameters> for ::windows::core::IInspectable {
    fn from(value: PfxImportParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PfxImportParameters> for ::windows::core::IInspectable {
    fn from(value: &PfxImportParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PfxImportParameters> for &::windows::core::IInspectable {
    fn from(value: &PfxImportParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PfxImportParameters {}
unsafe impl ::core::marker::Sync for PfxImportParameters {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SignatureValidationResult(pub i32);
impl SignatureValidationResult {
    pub const Success: Self = Self(0i32);
    pub const InvalidParameter: Self = Self(1i32);
    pub const BadMessage: Self = Self(2i32);
    pub const InvalidSignature: Self = Self(3i32);
    pub const OtherErrors: Self = Self(4i32);
}
impl ::core::marker::Copy for SignatureValidationResult {}
impl ::core::clone::Clone for SignatureValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SignatureValidationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SignatureValidationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for SignatureValidationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignatureValidationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SignatureValidationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.SignatureValidationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
pub struct StandardCertificateStoreNames;
impl StandardCertificateStoreNames {
    pub fn Personal() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardCertificateStoreNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Personal)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TrustedRootCertificationAuthorities() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardCertificateStoreNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrustedRootCertificationAuthorities)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IntermediateCertificationAuthorities() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardCertificateStoreNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IntermediateCertificationAuthorities)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStandardCertificateStoreNamesStatics<R, F: FnOnce(&IStandardCertificateStoreNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StandardCertificateStoreNames, IStandardCertificateStoreNamesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StandardCertificateStoreNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.StandardCertificateStoreNames";
}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct SubjectAlternativeNameInfo(::windows::core::IUnknown);
impl SubjectAlternativeNameInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SubjectAlternativeNameInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EmailName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EmailName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IPAddress(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IPAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Url(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Url)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DnsName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DistinguishedName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DistinguishedName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrincipalName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrincipalName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EmailNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EmailNames)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IPAddresses(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IPAddresses)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Urls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Urls)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DnsNames)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DistinguishedNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DistinguishedNames)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrincipalNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrincipalNames)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Extension(&self) -> ::windows::core::Result<CertificateExtension> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Extension)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CertificateExtension>(result__)
        }
    }
}
impl ::core::clone::Clone for SubjectAlternativeNameInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SubjectAlternativeNameInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SubjectAlternativeNameInfo {}
impl ::core::fmt::Debug for SubjectAlternativeNameInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SubjectAlternativeNameInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SubjectAlternativeNameInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo;{582859f1-569d-4c20-be7b-4e1c9a0bc52b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SubjectAlternativeNameInfo {
    type Vtable = ISubjectAlternativeNameInfo_Vtbl;
    const IID: ::windows::core::GUID = <ISubjectAlternativeNameInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SubjectAlternativeNameInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo";
}
impl ::core::convert::From<SubjectAlternativeNameInfo> for ::windows::core::IUnknown {
    fn from(value: SubjectAlternativeNameInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SubjectAlternativeNameInfo> for ::windows::core::IUnknown {
    fn from(value: &SubjectAlternativeNameInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SubjectAlternativeNameInfo> for &::windows::core::IUnknown {
    fn from(value: &SubjectAlternativeNameInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<SubjectAlternativeNameInfo> for ::windows::core::IInspectable {
    fn from(value: SubjectAlternativeNameInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SubjectAlternativeNameInfo> for ::windows::core::IInspectable {
    fn from(value: &SubjectAlternativeNameInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&SubjectAlternativeNameInfo> for &::windows::core::IInspectable {
    fn from(value: &SubjectAlternativeNameInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for SubjectAlternativeNameInfo {}
unsafe impl ::core::marker::Sync for SubjectAlternativeNameInfo {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct UserCertificateEnrollmentManager(::windows::core::IUnknown);
impl UserCertificateEnrollmentManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateRequestAsync<'a, P0>(&self, request: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, CertificateRequestProperties>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateRequestAsync)(::windows::core::Interface::as_raw(this), request.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstallCertificateAsync(&self, certificate: &::windows::core::HSTRING, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InstallCertificateAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(certificate), installoption, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportPfxDataAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(pfxdata), ::core::mem::transmute_copy(password), exportable, keyprotectionlevel, installoption, ::core::mem::transmute_copy(friendlyname), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataToKspAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING, keystorageprovider: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportPfxDataToKspAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(pfxdata), ::core::mem::transmute_copy(password), exportable, keyprotectionlevel, installoption, ::core::mem::transmute_copy(friendlyname), ::core::mem::transmute_copy(keystorageprovider), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataToKspWithParametersAsync<'a, P0>(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, pfximportparameters: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PfxImportParameters>>,
    {
        let this = &::windows::core::Interface::cast::<IUserCertificateEnrollmentManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportPfxDataToKspWithParametersAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(pfxdata), ::core::mem::transmute_copy(password), pfximportparameters.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for UserCertificateEnrollmentManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserCertificateEnrollmentManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserCertificateEnrollmentManager {}
impl ::core::fmt::Debug for UserCertificateEnrollmentManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserCertificateEnrollmentManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserCertificateEnrollmentManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager;{96313718-22e1-4819-b20b-ab46a6eca06e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserCertificateEnrollmentManager {
    type Vtable = IUserCertificateEnrollmentManager_Vtbl;
    const IID: ::windows::core::GUID = <IUserCertificateEnrollmentManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserCertificateEnrollmentManager {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager";
}
impl ::core::convert::From<UserCertificateEnrollmentManager> for ::windows::core::IUnknown {
    fn from(value: UserCertificateEnrollmentManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserCertificateEnrollmentManager> for ::windows::core::IUnknown {
    fn from(value: &UserCertificateEnrollmentManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UserCertificateEnrollmentManager> for &::windows::core::IUnknown {
    fn from(value: &UserCertificateEnrollmentManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<UserCertificateEnrollmentManager> for ::windows::core::IInspectable {
    fn from(value: UserCertificateEnrollmentManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserCertificateEnrollmentManager> for ::windows::core::IInspectable {
    fn from(value: &UserCertificateEnrollmentManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UserCertificateEnrollmentManager> for &::windows::core::IInspectable {
    fn from(value: &UserCertificateEnrollmentManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for UserCertificateEnrollmentManager {}
unsafe impl ::core::marker::Sync for UserCertificateEnrollmentManager {}
#[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
#[repr(transparent)]
pub struct UserCertificateStore(::windows::core::IUnknown);
impl UserCertificateStore {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAddAsync<'a, P0>(&self, certificate: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Certificate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestAddAsync)(::windows::core::Interface::as_raw(this), certificate.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsync<'a, P0>(&self, certificate: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Certificate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestDeleteAsync)(::windows::core::Interface::as_raw(this), certificate.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for UserCertificateStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserCertificateStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserCertificateStore {}
impl ::core::fmt::Debug for UserCertificateStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserCertificateStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserCertificateStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.UserCertificateStore;{c9fb1d83-789f-4b4e-9180-045a757aac6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserCertificateStore {
    type Vtable = IUserCertificateStore_Vtbl;
    const IID: ::windows::core::GUID = <IUserCertificateStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserCertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.UserCertificateStore";
}
impl ::core::convert::From<UserCertificateStore> for ::windows::core::IUnknown {
    fn from(value: UserCertificateStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserCertificateStore> for ::windows::core::IUnknown {
    fn from(value: &UserCertificateStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UserCertificateStore> for &::windows::core::IUnknown {
    fn from(value: &UserCertificateStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<UserCertificateStore> for ::windows::core::IInspectable {
    fn from(value: UserCertificateStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserCertificateStore> for ::windows::core::IInspectable {
    fn from(value: &UserCertificateStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&UserCertificateStore> for &::windows::core::IInspectable {
    fn from(value: &UserCertificateStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for UserCertificateStore {}
unsafe impl ::core::marker::Sync for UserCertificateStore {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
