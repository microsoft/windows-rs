#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Certificate(pub ::windows::core::IInspectable);
impl Certificate {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn BuildChainAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Certificate>>>(&self, certificates: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), certificates.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<CertificateChain>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn BuildChainWithParametersAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Certificate>>, Param1: ::windows::core::IntoParam<'a, ChainBuildingParameters>>(&self, certificates: Param0, parameters: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), certificates.into_param().abi(), parameters.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<CertificateChain>>(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn GetHashValue(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn GetHashValueWithAlgorithm<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, hashalgorithmname: Param0) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), hashalgorithmname.into_param().abi(), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetCertificateBlob(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Issuer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn HasPrivateKey(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsStronglyProtected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ValidFrom(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ValidTo(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn SetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsSecurityDeviceBound(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn KeyUsages(&self) -> ::windows::core::Result<CertificateKeyUsages> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CertificateKeyUsages>(result__)
        }
    }
    pub fn KeyAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SignatureAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SignatureHashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SubjectAlternativeName(&self) -> ::windows::core::Result<SubjectAlternativeNameInfo> {
        let this = &::windows::core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SubjectAlternativeNameInfo>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(certblob: Param0) -> ::windows::core::Result<Certificate> {
        Self::ICertificateFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), certblob.into_param().abi(), &mut result__).from_abi::<Certificate>(result__)
        })
    }
    pub fn IsPerUser(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn StoreName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ICertificateFactory<R, F: FnOnce(&ICertificateFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Certificate, ICertificateFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Certificate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.Certificate;{333f740c-04d8-43b3-b278-8c5fcc9be5a0})");
}
unsafe impl ::windows::core::Interface for Certificate {
    type Vtable = ICertificate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x333f740c_04d8_43b3_b278_8c5fcc9be5a0);
}
impl ::windows::core::RuntimeName for Certificate {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.Certificate";
}
impl ::core::convert::From<Certificate> for ::windows::core::IUnknown {
    fn from(value: Certificate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Certificate> for ::windows::core::IUnknown {
    fn from(value: &Certificate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Certificate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Certificate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Certificate> for ::windows::core::IInspectable {
    fn from(value: Certificate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Certificate> for ::windows::core::IInspectable {
    fn from(value: &Certificate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Certificate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Certificate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Certificate {}
unsafe impl ::core::marker::Sync for Certificate {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CertificateChain(pub ::windows::core::IInspectable);
impl CertificateChain {
    pub fn Validate(&self) -> ::windows::core::Result<ChainValidationResult> {
        let this = self;
        unsafe {
            let mut result__: ChainValidationResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChainValidationResult>(result__)
        }
    }
    pub fn ValidateWithParameters<'a, Param0: ::windows::core::IntoParam<'a, ChainValidationParameters>>(&self, parameter: Param0) -> ::windows::core::Result<ChainValidationResult> {
        let this = self;
        unsafe {
            let mut result__: ChainValidationResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), parameter.into_param().abi(), &mut result__).from_abi::<ChainValidationResult>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCertificates(&self, includeroot: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), includeroot, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Certificate>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateChain {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateChain;{20bf5385-3691-4501-a62c-fd97278b31ee})");
}
unsafe impl ::windows::core::Interface for CertificateChain {
    type Vtable = ICertificateChain_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20bf5385_3691_4501_a62c_fd97278b31ee);
}
impl ::windows::core::RuntimeName for CertificateChain {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateChain";
}
impl ::core::convert::From<CertificateChain> for ::windows::core::IUnknown {
    fn from(value: CertificateChain) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CertificateChain> for ::windows::core::IUnknown {
    fn from(value: &CertificateChain) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CertificateChain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CertificateChain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CertificateChain> for ::windows::core::IInspectable {
    fn from(value: CertificateChain) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CertificateChain> for ::windows::core::IInspectable {
    fn from(value: &CertificateChain) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CertificateChain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CertificateChain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CertificateChain {}
unsafe impl ::core::marker::Sync for CertificateChain {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CertificateChainPolicy(pub i32);
impl CertificateChainPolicy {
    pub const Base: CertificateChainPolicy = CertificateChainPolicy(0i32);
    pub const Ssl: CertificateChainPolicy = CertificateChainPolicy(1i32);
    pub const NTAuthentication: CertificateChainPolicy = CertificateChainPolicy(2i32);
    pub const MicrosoftRoot: CertificateChainPolicy = CertificateChainPolicy(3i32);
}
impl ::core::convert::From<i32> for CertificateChainPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CertificateChainPolicy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CertificateChainPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.CertificateChainPolicy;i4)");
}
impl ::windows::core::DefaultType for CertificateChainPolicy {
    type DefaultType = Self;
}
pub struct CertificateEnrollmentManager {}
impl CertificateEnrollmentManager {
    #[cfg(feature = "Foundation")]
    pub fn CreateRequestAsync<'a, Param0: ::windows::core::IntoParam<'a, CertificateRequestProperties>>(request: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::ICertificateEnrollmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn InstallCertificateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(certificate: Param0, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), certificate.into_param().abi(), installoption, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(pfxdata: Param0, password: Param1, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: Param5) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pfxdata.into_param().abi(), password.into_param().abi(), exportable, keyprotectionlevel, installoption, friendlyname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn UserCertificateEnrollmentManager() -> ::windows::core::Result<UserCertificateEnrollmentManager> {
        Self::ICertificateEnrollmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserCertificateEnrollmentManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataToKspAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(pfxdata: Param0, password: Param1, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: Param5, keystorageprovider: Param6) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pfxdata.into_param().abi(), password.into_param().abi(), exportable, keyprotectionlevel, installoption, friendlyname.into_param().abi(), keystorageprovider.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataToKspWithParametersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, PfxImportParameters>>(pfxdata: Param0, password: Param1, pfximportparameters: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pfxdata.into_param().abi(), password.into_param().abi(), pfximportparameters.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn ICertificateEnrollmentManagerStatics<R, F: FnOnce(&ICertificateEnrollmentManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CertificateEnrollmentManager, ICertificateEnrollmentManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICertificateEnrollmentManagerStatics2<R, F: FnOnce(&ICertificateEnrollmentManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CertificateEnrollmentManager, ICertificateEnrollmentManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICertificateEnrollmentManagerStatics3<R, F: FnOnce(&ICertificateEnrollmentManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CertificateEnrollmentManager, ICertificateEnrollmentManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CertificateEnrollmentManager {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CertificateExtension(pub ::windows::core::IInspectable);
impl CertificateExtension {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CertificateExtension, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ObjectId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetObjectId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsCritical(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCritical(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EncodeValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn SetValue(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateExtension {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateExtension;{84cf0656-a9e6-454d-8e45-2ea7c4bcd53b})");
}
unsafe impl ::windows::core::Interface for CertificateExtension {
    type Vtable = ICertificateExtension_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84cf0656_a9e6_454d_8e45_2ea7c4bcd53b);
}
impl ::windows::core::RuntimeName for CertificateExtension {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateExtension";
}
impl ::core::convert::From<CertificateExtension> for ::windows::core::IUnknown {
    fn from(value: CertificateExtension) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CertificateExtension> for ::windows::core::IUnknown {
    fn from(value: &CertificateExtension) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CertificateExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CertificateExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CertificateExtension> for ::windows::core::IInspectable {
    fn from(value: CertificateExtension) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CertificateExtension> for ::windows::core::IInspectable {
    fn from(value: &CertificateExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CertificateExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CertificateExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CertificateExtension {}
unsafe impl ::core::marker::Sync for CertificateExtension {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CertificateKeyUsages(pub ::windows::core::IInspectable);
impl CertificateKeyUsages {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CertificateKeyUsages, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EncipherOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetEncipherOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CrlSign(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCrlSign(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn KeyCertificateSign(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetKeyCertificateSign(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn KeyAgreement(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetKeyAgreement(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DataEncipherment(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDataEncipherment(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn KeyEncipherment(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetKeyEncipherment(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NonRepudiation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetNonRepudiation(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DigitalSignature(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDigitalSignature(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateKeyUsages {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateKeyUsages;{6ac6206f-e1cf-486a-b485-a69c83e46fd1})");
}
unsafe impl ::windows::core::Interface for CertificateKeyUsages {
    type Vtable = ICertificateKeyUsages_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac6206f_e1cf_486a_b485_a69c83e46fd1);
}
impl ::windows::core::RuntimeName for CertificateKeyUsages {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateKeyUsages";
}
impl ::core::convert::From<CertificateKeyUsages> for ::windows::core::IUnknown {
    fn from(value: CertificateKeyUsages) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CertificateKeyUsages> for ::windows::core::IUnknown {
    fn from(value: &CertificateKeyUsages) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CertificateKeyUsages {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CertificateKeyUsages {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CertificateKeyUsages> for ::windows::core::IInspectable {
    fn from(value: CertificateKeyUsages) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CertificateKeyUsages> for ::windows::core::IInspectable {
    fn from(value: &CertificateKeyUsages) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CertificateKeyUsages {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CertificateKeyUsages {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CertificateKeyUsages {}
unsafe impl ::core::marker::Sync for CertificateKeyUsages {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CertificateQuery(pub ::windows::core::IInspectable);
impl CertificateQuery {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CertificateQuery, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn IssuerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetIssuerName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Thumbprint(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn SetThumbprint(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn HardwareOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHardwareOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IncludeDuplicates(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeDuplicates(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IncludeExpiredCertificates(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeExpiredCertificates(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn StoreName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetStoreName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateQuery;{5b082a31-a728-4916-b5ee-ffcb8acf2417})");
}
unsafe impl ::windows::core::Interface for CertificateQuery {
    type Vtable = ICertificateQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b082a31_a728_4916_b5ee_ffcb8acf2417);
}
impl ::windows::core::RuntimeName for CertificateQuery {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateQuery";
}
impl ::core::convert::From<CertificateQuery> for ::windows::core::IUnknown {
    fn from(value: CertificateQuery) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CertificateQuery> for ::windows::core::IUnknown {
    fn from(value: &CertificateQuery) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CertificateQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CertificateQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CertificateQuery> for ::windows::core::IInspectable {
    fn from(value: CertificateQuery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CertificateQuery> for ::windows::core::IInspectable {
    fn from(value: &CertificateQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CertificateQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CertificateQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CertificateQuery {}
unsafe impl ::core::marker::Sync for CertificateQuery {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CertificateRequestProperties(pub ::windows::core::IInspectable);
impl CertificateRequestProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CertificateRequestProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn KeyAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetKeyAlgorithmName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn KeySize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetKeySize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn HashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetHashAlgorithmName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Exportable(&self) -> ::windows::core::Result<ExportOption> {
        let this = self;
        unsafe {
            let mut result__: ExportOption = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExportOption>(result__)
        }
    }
    pub fn SetExportable(&self, value: ExportOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn KeyUsages(&self) -> ::windows::core::Result<EnrollKeyUsages> {
        let this = self;
        unsafe {
            let mut result__: EnrollKeyUsages = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnrollKeyUsages>(result__)
        }
    }
    pub fn SetKeyUsages(&self, value: EnrollKeyUsages) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn KeyProtectionLevel(&self) -> ::windows::core::Result<KeyProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: KeyProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyProtectionLevel>(result__)
        }
    }
    pub fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetKeyStorageProviderName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SmartcardReaderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSmartcardReaderName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SigningCertificate(&self) -> ::windows::core::Result<Certificate> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Certificate>(result__)
        }
    }
    pub fn SetSigningCertificate<'a, Param0: ::windows::core::IntoParam<'a, Certificate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AttestationCredentialCertificate(&self) -> ::windows::core::Result<Certificate> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Certificate>(result__)
        }
    }
    pub fn SetAttestationCredentialCertificate<'a, Param0: ::windows::core::IntoParam<'a, Certificate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CurveName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCurveName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CurveParameters(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn SetCurveParameters(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn ContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContainerNamePrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ContainerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContainerName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn UseExistingKey(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetUseExistingKey(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SuppressedDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn SubjectAlternativeName(&self) -> ::windows::core::Result<SubjectAlternativeNameInfo> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SubjectAlternativeNameInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<CertificateExtension>> {
        let this = &::windows::core::Interface::cast::<ICertificateRequestProperties4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<CertificateExtension>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateRequestProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateRequestProperties;{487e84f6-94e2-4dce-8833-1a700a37a29a})");
}
unsafe impl ::windows::core::Interface for CertificateRequestProperties {
    type Vtable = ICertificateRequestProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x487e84f6_94e2_4dce_8833_1a700a37a29a);
}
impl ::windows::core::RuntimeName for CertificateRequestProperties {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateRequestProperties";
}
impl ::core::convert::From<CertificateRequestProperties> for ::windows::core::IUnknown {
    fn from(value: CertificateRequestProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CertificateRequestProperties> for ::windows::core::IUnknown {
    fn from(value: &CertificateRequestProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CertificateRequestProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CertificateRequestProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CertificateRequestProperties> for ::windows::core::IInspectable {
    fn from(value: CertificateRequestProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CertificateRequestProperties> for ::windows::core::IInspectable {
    fn from(value: &CertificateRequestProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CertificateRequestProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CertificateRequestProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CertificateRequestProperties {}
unsafe impl ::core::marker::Sync for CertificateRequestProperties {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CertificateStore(pub ::windows::core::IInspectable);
impl CertificateStore {
    pub fn Add<'a, Param0: ::windows::core::IntoParam<'a, Certificate>>(&self, certificate: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), certificate.into_param().abi()).ok() }
    }
    pub fn Delete<'a, Param0: ::windows::core::IntoParam<'a, Certificate>>(&self, certificate: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), certificate.into_param().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICertificateStore2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CertificateStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CertificateStore;{b0bff720-344e-4331-af14-a7f7a7ebc93a})");
}
unsafe impl ::windows::core::Interface for CertificateStore {
    type Vtable = ICertificateStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0bff720_344e_4331_af14_a7f7a7ebc93a);
}
impl ::windows::core::RuntimeName for CertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateStore";
}
impl ::core::convert::From<CertificateStore> for ::windows::core::IUnknown {
    fn from(value: CertificateStore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CertificateStore> for ::windows::core::IUnknown {
    fn from(value: &CertificateStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CertificateStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CertificateStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CertificateStore> for ::windows::core::IInspectable {
    fn from(value: CertificateStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CertificateStore> for ::windows::core::IInspectable {
    fn from(value: &CertificateStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CertificateStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CertificateStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CertificateStore {}
unsafe impl ::core::marker::Sync for CertificateStore {}
pub struct CertificateStores {}
impl CertificateStores {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllWithQueryAsync<'a, Param0: ::windows::core::IntoParam<'a, CertificateQuery>>(query: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), query.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>(result__)
        })
    }
    pub fn TrustedRootCertificationAuthorities() -> ::windows::core::Result<CertificateStore> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CertificateStore>(result__)
        })
    }
    pub fn IntermediateCertificationAuthorities() -> ::windows::core::Result<CertificateStore> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CertificateStore>(result__)
        })
    }
    pub fn GetStoreByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(storename: Param0) -> ::windows::core::Result<CertificateStore> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), storename.into_param().abi(), &mut result__).from_abi::<CertificateStore>(result__)
        })
    }
    pub fn GetUserStoreByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(storename: Param0) -> ::windows::core::Result<UserCertificateStore> {
        Self::ICertificateStoresStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), storename.into_param().abi(), &mut result__).from_abi::<UserCertificateStore>(result__)
        })
    }
    pub fn ICertificateStoresStatics<R, F: FnOnce(&ICertificateStoresStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CertificateStores, ICertificateStoresStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICertificateStoresStatics2<R, F: FnOnce(&ICertificateStoresStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CertificateStores, ICertificateStoresStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CertificateStores {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateStores";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ChainBuildingParameters(pub ::windows::core::IInspectable);
impl ChainBuildingParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChainBuildingParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ValidationTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetValidationTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RevocationCheckEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetRevocationCheckEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NetworkRetrievalEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetNetworkRetrievalEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AuthorityInformationAccessEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuthorityInformationAccessEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CurrentTimeValidationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCurrentTimeValidationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExclusiveTrustRoots(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<Certificate>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ChainBuildingParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.ChainBuildingParameters;{422ba922-7c8d-47b7-b59b-b12703733ac3})");
}
unsafe impl ::windows::core::Interface for ChainBuildingParameters {
    type Vtable = IChainBuildingParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x422ba922_7c8d_47b7_b59b_b12703733ac3);
}
impl ::windows::core::RuntimeName for ChainBuildingParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ChainBuildingParameters";
}
impl ::core::convert::From<ChainBuildingParameters> for ::windows::core::IUnknown {
    fn from(value: ChainBuildingParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ChainBuildingParameters> for ::windows::core::IUnknown {
    fn from(value: &ChainBuildingParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChainBuildingParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ChainBuildingParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ChainBuildingParameters> for ::windows::core::IInspectable {
    fn from(value: ChainBuildingParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ChainBuildingParameters> for ::windows::core::IInspectable {
    fn from(value: &ChainBuildingParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChainBuildingParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ChainBuildingParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ChainBuildingParameters {}
unsafe impl ::core::marker::Sync for ChainBuildingParameters {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ChainValidationParameters(pub ::windows::core::IInspectable);
impl ChainValidationParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChainValidationParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CertificateChainPolicy(&self) -> ::windows::core::Result<CertificateChainPolicy> {
        let this = self;
        unsafe {
            let mut result__: CertificateChainPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CertificateChainPolicy>(result__)
        }
    }
    pub fn SetCertificateChainPolicy(&self, value: CertificateChainPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Networking")]
    pub fn ServerDnsName(&self) -> ::windows::core::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::HostName>(result__)
        }
    }
    #[cfg(feature = "Networking")]
    pub fn SetServerDnsName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Networking::HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ChainValidationParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.ChainValidationParameters;{c4743b4a-7eb0-4b56-a040-b9c8e655ddf3})");
}
unsafe impl ::windows::core::Interface for ChainValidationParameters {
    type Vtable = IChainValidationParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4743b4a_7eb0_4b56_a040_b9c8e655ddf3);
}
impl ::windows::core::RuntimeName for ChainValidationParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ChainValidationParameters";
}
impl ::core::convert::From<ChainValidationParameters> for ::windows::core::IUnknown {
    fn from(value: ChainValidationParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ChainValidationParameters> for ::windows::core::IUnknown {
    fn from(value: &ChainValidationParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChainValidationParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ChainValidationParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ChainValidationParameters> for ::windows::core::IInspectable {
    fn from(value: ChainValidationParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ChainValidationParameters> for ::windows::core::IInspectable {
    fn from(value: &ChainValidationParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChainValidationParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ChainValidationParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ChainValidationParameters {}
unsafe impl ::core::marker::Sync for ChainValidationParameters {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChainValidationResult(pub i32);
impl ChainValidationResult {
    pub const Success: ChainValidationResult = ChainValidationResult(0i32);
    pub const Untrusted: ChainValidationResult = ChainValidationResult(1i32);
    pub const Revoked: ChainValidationResult = ChainValidationResult(2i32);
    pub const Expired: ChainValidationResult = ChainValidationResult(3i32);
    pub const IncompleteChain: ChainValidationResult = ChainValidationResult(4i32);
    pub const InvalidSignature: ChainValidationResult = ChainValidationResult(5i32);
    pub const WrongUsage: ChainValidationResult = ChainValidationResult(6i32);
    pub const InvalidName: ChainValidationResult = ChainValidationResult(7i32);
    pub const InvalidCertificateAuthorityPolicy: ChainValidationResult = ChainValidationResult(8i32);
    pub const BasicConstraintsError: ChainValidationResult = ChainValidationResult(9i32);
    pub const UnknownCriticalExtension: ChainValidationResult = ChainValidationResult(10i32);
    pub const RevocationInformationMissing: ChainValidationResult = ChainValidationResult(11i32);
    pub const RevocationFailure: ChainValidationResult = ChainValidationResult(12i32);
    pub const OtherErrors: ChainValidationResult = ChainValidationResult(13i32);
}
impl ::core::convert::From<i32> for ChainValidationResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ChainValidationResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ChainValidationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.ChainValidationResult;i4)");
}
impl ::windows::core::DefaultType for ChainValidationResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CmsAttachedSignature(pub ::windows::core::IInspectable);
impl CmsAttachedSignature {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Certificate>>(result__)
        }
    }
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Signers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>(result__)
        }
    }
    pub fn VerifySignature(&self) -> ::windows::core::Result<SignatureValidationResult> {
        let this = self;
        unsafe {
            let mut result__: SignatureValidationResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SignatureValidationResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCmsAttachedSignature<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(inputblob: Param0) -> ::windows::core::Result<CmsAttachedSignature> {
        Self::ICmsAttachedSignatureFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inputblob.into_param().abi(), &mut result__).from_abi::<CmsAttachedSignature>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GenerateSignatureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Certificate>>>(data: Param0, signers: Param1, certificates: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        Self::ICmsAttachedSignatureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi(), signers.into_param().abi(), certificates.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    pub fn ICmsAttachedSignatureFactory<R, F: FnOnce(&ICmsAttachedSignatureFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CmsAttachedSignature, ICmsAttachedSignatureFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICmsAttachedSignatureStatics<R, F: FnOnce(&ICmsAttachedSignatureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CmsAttachedSignature, ICmsAttachedSignatureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CmsAttachedSignature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CmsAttachedSignature;{61899d9d-3757-4ecb-bddc-0ca357d7a936})");
}
unsafe impl ::windows::core::Interface for CmsAttachedSignature {
    type Vtable = ICmsAttachedSignature_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61899d9d_3757_4ecb_bddc_0ca357d7a936);
}
impl ::windows::core::RuntimeName for CmsAttachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsAttachedSignature";
}
impl ::core::convert::From<CmsAttachedSignature> for ::windows::core::IUnknown {
    fn from(value: CmsAttachedSignature) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CmsAttachedSignature> for ::windows::core::IUnknown {
    fn from(value: &CmsAttachedSignature) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CmsAttachedSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CmsAttachedSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CmsAttachedSignature> for ::windows::core::IInspectable {
    fn from(value: CmsAttachedSignature) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CmsAttachedSignature> for ::windows::core::IInspectable {
    fn from(value: &CmsAttachedSignature) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CmsAttachedSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CmsAttachedSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CmsAttachedSignature {}
unsafe impl ::core::marker::Sync for CmsAttachedSignature {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CmsDetachedSignature(pub ::windows::core::IInspectable);
impl CmsDetachedSignature {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Certificate>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Signers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn VerifySignatureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, data: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SignatureValidationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SignatureValidationResult>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCmsDetachedSignature<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(inputblob: Param0) -> ::windows::core::Result<CmsDetachedSignature> {
        Self::ICmsDetachedSignatureFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inputblob.into_param().abi(), &mut result__).from_abi::<CmsDetachedSignature>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GenerateSignatureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<Certificate>>>(data: Param0, signers: Param1, certificates: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        Self::ICmsDetachedSignatureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi(), signers.into_param().abi(), certificates.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    pub fn ICmsDetachedSignatureFactory<R, F: FnOnce(&ICmsDetachedSignatureFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CmsDetachedSignature, ICmsDetachedSignatureFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICmsDetachedSignatureStatics<R, F: FnOnce(&ICmsDetachedSignatureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CmsDetachedSignature, ICmsDetachedSignatureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CmsDetachedSignature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CmsDetachedSignature;{0f1ef154-f65e-4536-8339-5944081db2ca})");
}
unsafe impl ::windows::core::Interface for CmsDetachedSignature {
    type Vtable = ICmsDetachedSignature_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f1ef154_f65e_4536_8339_5944081db2ca);
}
impl ::windows::core::RuntimeName for CmsDetachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsDetachedSignature";
}
impl ::core::convert::From<CmsDetachedSignature> for ::windows::core::IUnknown {
    fn from(value: CmsDetachedSignature) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CmsDetachedSignature> for ::windows::core::IUnknown {
    fn from(value: &CmsDetachedSignature) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CmsDetachedSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CmsDetachedSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CmsDetachedSignature> for ::windows::core::IInspectable {
    fn from(value: CmsDetachedSignature) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CmsDetachedSignature> for ::windows::core::IInspectable {
    fn from(value: &CmsDetachedSignature) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CmsDetachedSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CmsDetachedSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CmsDetachedSignature {}
unsafe impl ::core::marker::Sync for CmsDetachedSignature {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CmsSignerInfo(pub ::windows::core::IInspectable);
impl CmsSignerInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CmsSignerInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Certificate(&self) -> ::windows::core::Result<Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Certificate>(result__)
        }
    }
    pub fn SetCertificate<'a, Param0: ::windows::core::IntoParam<'a, Certificate>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn HashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetHashAlgorithmName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TimestampInfo(&self) -> ::windows::core::Result<CmsTimestampInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CmsTimestampInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CmsSignerInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CmsSignerInfo;{50d020db-1d2f-4c1a-b5c5-d0188ff91f47})");
}
unsafe impl ::windows::core::Interface for CmsSignerInfo {
    type Vtable = ICmsSignerInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d020db_1d2f_4c1a_b5c5_d0188ff91f47);
}
impl ::windows::core::RuntimeName for CmsSignerInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsSignerInfo";
}
impl ::core::convert::From<CmsSignerInfo> for ::windows::core::IUnknown {
    fn from(value: CmsSignerInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CmsSignerInfo> for ::windows::core::IUnknown {
    fn from(value: &CmsSignerInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CmsSignerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CmsSignerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CmsSignerInfo> for ::windows::core::IInspectable {
    fn from(value: CmsSignerInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CmsSignerInfo> for ::windows::core::IInspectable {
    fn from(value: &CmsSignerInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CmsSignerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CmsSignerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CmsSignerInfo {}
unsafe impl ::core::marker::Sync for CmsSignerInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CmsTimestampInfo(pub ::windows::core::IInspectable);
impl CmsTimestampInfo {
    pub fn SigningCertificate(&self) -> ::windows::core::Result<Certificate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Certificate>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<Certificate>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CmsTimestampInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.CmsTimestampInfo;{2f5f00f2-2c18-4f88-8435-c534086076f5})");
}
unsafe impl ::windows::core::Interface for CmsTimestampInfo {
    type Vtable = ICmsTimestampInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f5f00f2_2c18_4f88_8435_c534086076f5);
}
impl ::windows::core::RuntimeName for CmsTimestampInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsTimestampInfo";
}
impl ::core::convert::From<CmsTimestampInfo> for ::windows::core::IUnknown {
    fn from(value: CmsTimestampInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CmsTimestampInfo> for ::windows::core::IUnknown {
    fn from(value: &CmsTimestampInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CmsTimestampInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CmsTimestampInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CmsTimestampInfo> for ::windows::core::IInspectable {
    fn from(value: CmsTimestampInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CmsTimestampInfo> for ::windows::core::IInspectable {
    fn from(value: &CmsTimestampInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CmsTimestampInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CmsTimestampInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CmsTimestampInfo {}
unsafe impl ::core::marker::Sync for CmsTimestampInfo {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EnrollKeyUsages(pub u32);
impl EnrollKeyUsages {
    pub const None: EnrollKeyUsages = EnrollKeyUsages(0u32);
    pub const Decryption: EnrollKeyUsages = EnrollKeyUsages(1u32);
    pub const Signing: EnrollKeyUsages = EnrollKeyUsages(2u32);
    pub const KeyAgreement: EnrollKeyUsages = EnrollKeyUsages(4u32);
    pub const All: EnrollKeyUsages = EnrollKeyUsages(16777215u32);
}
impl ::core::convert::From<u32> for EnrollKeyUsages {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EnrollKeyUsages {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EnrollKeyUsages {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.EnrollKeyUsages;u4)");
}
impl ::windows::core::DefaultType for EnrollKeyUsages {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for EnrollKeyUsages {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for EnrollKeyUsages {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for EnrollKeyUsages {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for EnrollKeyUsages {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for EnrollKeyUsages {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExportOption(pub i32);
impl ExportOption {
    pub const NotExportable: ExportOption = ExportOption(0i32);
    pub const Exportable: ExportOption = ExportOption(1i32);
}
impl ::core::convert::From<i32> for ExportOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExportOption {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExportOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.ExportOption;i4)");
}
impl ::windows::core::DefaultType for ExportOption {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificate(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificate {
    type Vtable = ICertificate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x333f740c_04d8_43b3_b278_8c5fcc9be5a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificates: ::windows::core::RawPtr, parameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hashalgorithmname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificate2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificate2 {
    type Vtable = ICertificate2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17b8374c_8a25_4d96_a492_8fc29ac4fda6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificate2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificate3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificate3 {
    type Vtable = ICertificate3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe51a966_ae5f_4652_ace7_c6d7e7724cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificate3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateChain(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateChain {
    type Vtable = ICertificateChain_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20bf5385_3691_4501_a62c_fd97278b31ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateChain_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ChainValidationResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parameter: ::windows::core::RawPtr, result__: *mut ChainValidationResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, includeroot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateEnrollmentManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateEnrollmentManagerStatics {
    type Vtable = ICertificateEnrollmentManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8846ef3f_a986_48fb_9fd7_9aec06935bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoption: InstallOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateEnrollmentManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateEnrollmentManagerStatics2 {
    type Vtable = ICertificateEnrollmentManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc5b1c33_6429_4014_999c_5d9735802d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keystorageprovider: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateEnrollmentManagerStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateEnrollmentManagerStatics3 {
    type Vtable = ICertificateEnrollmentManagerStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdec82be_617c_425a_b72d_398b26ac7264);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pfximportparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateExtension(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateExtension {
    type Vtable = ICertificateExtension_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84cf0656_a9e6_454d_8e45_2ea7c4bcd53b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateExtension_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateFactory {
    type Vtable = ICertificateFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17b4221c_4baf_44a2_9608_04fb62b16942);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateKeyUsages(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateKeyUsages {
    type Vtable = ICertificateKeyUsages_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac6206f_e1cf_486a_b485_a69c83e46fd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateKeyUsages_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateQuery(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateQuery {
    type Vtable = ICertificateQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b082a31_a728_4916_b5ee_ffcb8acf2417);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateQuery2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateQuery2 {
    type Vtable = ICertificateQuery2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x935a0af7_0bd9_4f75_b8c2_e27a7f74eecd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateQuery2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateRequestProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateRequestProperties {
    type Vtable = ICertificateRequestProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x487e84f6_94e2_4dce_8833_1a700a37a29a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateRequestProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ExportOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ExportOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EnrollKeyUsages) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: EnrollKeyUsages) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut KeyProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: KeyProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateRequestProperties2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateRequestProperties2 {
    type Vtable = ICertificateRequestProperties2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da0c954_d73f_4ff3_a0a6_0677c0ada05b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateRequestProperties2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateRequestProperties3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateRequestProperties3 {
    type Vtable = ICertificateRequestProperties3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe687f616_734d_46b1_9d4c_6edfdbfc845b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateRequestProperties3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateRequestProperties4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateRequestProperties4 {
    type Vtable = ICertificateRequestProperties4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e429ad2_1c61_4fea_b8fe_135fb19cdce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateRequestProperties4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateStore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateStore {
    type Vtable = ICertificateStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0bff720_344e_4331_af14_a7f7a7ebc93a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateStore2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateStore2 {
    type Vtable = ICertificateStore2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7e68e4a_417d_4d1a_babd_15687e549974);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateStore2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateStoresStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateStoresStatics {
    type Vtable = ICertificateStoresStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbecc739_c6fe_4de7_99cf_74c3e596e032);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateStoresStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, query: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICertificateStoresStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICertificateStoresStatics2 {
    type Vtable = ICertificateStoresStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa900b79_a0d4_4b8c_bc55_c0a37eb141ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateStoresStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IChainBuildingParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IChainBuildingParameters {
    type Vtable = IChainBuildingParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x422ba922_7c8d_47b7_b59b_b12703733ac3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChainBuildingParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IChainValidationParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IChainValidationParameters {
    type Vtable = IChainValidationParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4743b4a_7eb0_4b56_a040_b9c8e655ddf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChainValidationParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CertificateChainPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CertificateChainPolicy) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICmsAttachedSignature(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICmsAttachedSignature {
    type Vtable = ICmsAttachedSignature_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61899d9d_3757_4ecb_bddc_0ca357d7a936);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsAttachedSignature_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SignatureValidationResult) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICmsAttachedSignatureFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICmsAttachedSignatureFactory {
    type Vtable = ICmsAttachedSignatureFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0c8fc15_f757_4c64_a362_52cc1c77cffb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsAttachedSignatureFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICmsAttachedSignatureStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICmsAttachedSignatureStatics {
    type Vtable = ICmsAttachedSignatureStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87989c8e_b0ad_498d_a7f5_78b59bce4b36);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsAttachedSignatureStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::windows::core::RawPtr, signers: ::windows::core::RawPtr, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICmsDetachedSignature(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICmsDetachedSignature {
    type Vtable = ICmsDetachedSignature_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f1ef154_f65e_4536_8339_5944081db2ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsDetachedSignature_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICmsDetachedSignatureFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICmsDetachedSignatureFactory {
    type Vtable = ICmsDetachedSignatureFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4ab3503_ae7f_4387_ad19_00f150e48ebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsDetachedSignatureFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputblob: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICmsDetachedSignatureStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICmsDetachedSignatureStatics {
    type Vtable = ICmsDetachedSignatureStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d114cfd_bf9b_4682_9be6_91f57c053808);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsDetachedSignatureStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::windows::core::RawPtr, signers: ::windows::core::RawPtr, certificates: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICmsSignerInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICmsSignerInfo {
    type Vtable = ICmsSignerInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d020db_1d2f_4c1a_b5c5_d0188ff91f47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsSignerInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICmsTimestampInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICmsTimestampInfo {
    type Vtable = ICmsTimestampInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f5f00f2_2c18_4f88_8435_c534086076f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICmsTimestampInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyAlgorithmNamesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyAlgorithmNamesStatics {
    type Vtable = IKeyAlgorithmNamesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x479065d7_7ac7_4581_8c3b_d07027140448);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyAlgorithmNamesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyAlgorithmNamesStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyAlgorithmNamesStatics2 {
    type Vtable = IKeyAlgorithmNamesStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99b5686_e1fd_4a4a_893d_a26f33dd8bb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyAlgorithmNamesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyAttestationHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyAttestationHelperStatics {
    type Vtable = IKeyAttestationHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1648e246_f644_4326_88be_3af102d30e0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyAttestationHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyAttestationHelperStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyAttestationHelperStatics2 {
    type Vtable = IKeyAttestationHelperStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c590b2c_a6c6_4a5e_9e64_e85d5279df97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyAttestationHelperStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credential: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, containername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyStorageProviderNamesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyStorageProviderNamesStatics {
    type Vtable = IKeyStorageProviderNamesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf186ae0_5529_4602_bd94_0aab91957b5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyStorageProviderNamesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyStorageProviderNamesStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyStorageProviderNamesStatics2 {
    type Vtable = IKeyStorageProviderNamesStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x262d743d_9c2e_41cc_8812_c4d971dd7c60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyStorageProviderNamesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPfxImportParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPfxImportParameters {
    type Vtable = IPfxImportParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x680d3511_9a08_47c8_864a_2edd4d8eb46c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPfxImportParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ExportOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ExportOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut KeyProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: KeyProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InstallOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: InstallOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardCertificateStoreNamesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardCertificateStoreNamesStatics {
    type Vtable = IStandardCertificateStoreNamesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c154adb_a496_41f8_8fe5_9e96f36efbf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardCertificateStoreNamesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISubjectAlternativeNameInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISubjectAlternativeNameInfo {
    type Vtable = ISubjectAlternativeNameInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x582859f1_569d_4c20_be7b_4e1c9a0bc52b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISubjectAlternativeNameInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISubjectAlternativeNameInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISubjectAlternativeNameInfo2 {
    type Vtable = ISubjectAlternativeNameInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x437a78c6_1c51_41ea_b34a_3d654398a370);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISubjectAlternativeNameInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserCertificateEnrollmentManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserCertificateEnrollmentManager {
    type Vtable = IUserCertificateEnrollmentManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96313718_22e1_4819_b20b_ab46a6eca06e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserCertificateEnrollmentManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoption: InstallOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keystorageprovider: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserCertificateEnrollmentManager2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserCertificateEnrollmentManager2 {
    type Vtable = IUserCertificateEnrollmentManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dad9cb1_65de_492a_b86d_fc5c482c3747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserCertificateEnrollmentManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfxdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pfximportparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserCertificateStore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserCertificateStore {
    type Vtable = IUserCertificateStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9fb1d83_789f_4b4e_9180_045a757aac6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserCertificateStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InstallOptions(pub u32);
impl InstallOptions {
    pub const None: InstallOptions = InstallOptions(0u32);
    pub const DeleteExpired: InstallOptions = InstallOptions(1u32);
}
impl ::core::convert::From<u32> for InstallOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InstallOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InstallOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.InstallOptions;u4)");
}
impl ::windows::core::DefaultType for InstallOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for InstallOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for InstallOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for InstallOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for InstallOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for InstallOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct KeyAlgorithmNames {}
impl KeyAlgorithmNames {
    pub fn Rsa() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Dsa() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdh256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdh384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdh521() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdsa256() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdsa384() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdsa521() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdsa() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Ecdh() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IKeyAlgorithmNamesStatics<R, F: FnOnce(&IKeyAlgorithmNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyAlgorithmNames, IKeyAlgorithmNamesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKeyAlgorithmNamesStatics2<R, F: FnOnce(&IKeyAlgorithmNamesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyAlgorithmNames, IKeyAlgorithmNamesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KeyAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.KeyAlgorithmNames";
}
pub struct KeyAttestationHelper {}
impl KeyAttestationHelper {
    #[cfg(feature = "Foundation")]
    pub fn DecryptTpmAttestationCredentialAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(credential: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IKeyAttestationHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), credential.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    pub fn GetTpmAttestationCredentialId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(credential: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyAttestationHelperStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), credential.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn DecryptTpmAttestationCredentialWithContainerNameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(credential: Param0, containername: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IKeyAttestationHelperStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), credential.into_param().abi(), containername.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    pub fn IKeyAttestationHelperStatics<R, F: FnOnce(&IKeyAttestationHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyAttestationHelper, IKeyAttestationHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKeyAttestationHelperStatics2<R, F: FnOnce(&IKeyAttestationHelperStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyAttestationHelper, IKeyAttestationHelperStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KeyAttestationHelper {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.KeyAttestationHelper";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KeyProtectionLevel(pub i32);
impl KeyProtectionLevel {
    pub const NoConsent: KeyProtectionLevel = KeyProtectionLevel(0i32);
    pub const ConsentOnly: KeyProtectionLevel = KeyProtectionLevel(1i32);
    pub const ConsentWithPassword: KeyProtectionLevel = KeyProtectionLevel(2i32);
    pub const ConsentWithFingerprint: KeyProtectionLevel = KeyProtectionLevel(3i32);
}
impl ::core::convert::From<i32> for KeyProtectionLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KeyProtectionLevel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KeyProtectionLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.KeyProtectionLevel;i4)");
}
impl ::windows::core::DefaultType for KeyProtectionLevel {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KeySize(pub i32);
impl KeySize {
    pub const Invalid: KeySize = KeySize(0i32);
    pub const Rsa2048: KeySize = KeySize(2048i32);
    pub const Rsa4096: KeySize = KeySize(4096i32);
}
impl ::core::convert::From<i32> for KeySize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KeySize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KeySize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.KeySize;i4)");
}
impl ::windows::core::DefaultType for KeySize {
    type DefaultType = Self;
}
pub struct KeyStorageProviderNames {}
impl KeyStorageProviderNames {
    pub fn SoftwareKeyStorageProvider() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SmartcardKeyStorageProvider() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PlatformKeyStorageProvider() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PassportKeyStorageProvider() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IKeyStorageProviderNamesStatics<R, F: FnOnce(&IKeyStorageProviderNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyStorageProviderNames, IKeyStorageProviderNamesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKeyStorageProviderNamesStatics2<R, F: FnOnce(&IKeyStorageProviderNamesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyStorageProviderNames, IKeyStorageProviderNamesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KeyStorageProviderNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.KeyStorageProviderNames";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PfxImportParameters(pub ::windows::core::IInspectable);
impl PfxImportParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PfxImportParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Exportable(&self) -> ::windows::core::Result<ExportOption> {
        let this = self;
        unsafe {
            let mut result__: ExportOption = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExportOption>(result__)
        }
    }
    pub fn SetExportable(&self, value: ExportOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn KeyProtectionLevel(&self) -> ::windows::core::Result<KeyProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: KeyProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyProtectionLevel>(result__)
        }
    }
    pub fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InstallOptions(&self) -> ::windows::core::Result<InstallOptions> {
        let this = self;
        unsafe {
            let mut result__: InstallOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InstallOptions>(result__)
        }
    }
    pub fn SetInstallOptions(&self, value: InstallOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetKeyStorageProviderName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContainerNamePrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ReaderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetReaderName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PfxImportParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.PfxImportParameters;{680d3511-9a08-47c8-864a-2edd4d8eb46c})");
}
unsafe impl ::windows::core::Interface for PfxImportParameters {
    type Vtable = IPfxImportParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x680d3511_9a08_47c8_864a_2edd4d8eb46c);
}
impl ::windows::core::RuntimeName for PfxImportParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.PfxImportParameters";
}
impl ::core::convert::From<PfxImportParameters> for ::windows::core::IUnknown {
    fn from(value: PfxImportParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PfxImportParameters> for ::windows::core::IUnknown {
    fn from(value: &PfxImportParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PfxImportParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PfxImportParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PfxImportParameters> for ::windows::core::IInspectable {
    fn from(value: PfxImportParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PfxImportParameters> for ::windows::core::IInspectable {
    fn from(value: &PfxImportParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PfxImportParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PfxImportParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PfxImportParameters {}
unsafe impl ::core::marker::Sync for PfxImportParameters {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SignatureValidationResult(pub i32);
impl SignatureValidationResult {
    pub const Success: SignatureValidationResult = SignatureValidationResult(0i32);
    pub const InvalidParameter: SignatureValidationResult = SignatureValidationResult(1i32);
    pub const BadMessage: SignatureValidationResult = SignatureValidationResult(2i32);
    pub const InvalidSignature: SignatureValidationResult = SignatureValidationResult(3i32);
    pub const OtherErrors: SignatureValidationResult = SignatureValidationResult(4i32);
}
impl ::core::convert::From<i32> for SignatureValidationResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SignatureValidationResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SignatureValidationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.SignatureValidationResult;i4)");
}
impl ::windows::core::DefaultType for SignatureValidationResult {
    type DefaultType = Self;
}
pub struct StandardCertificateStoreNames {}
impl StandardCertificateStoreNames {
    pub fn Personal() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardCertificateStoreNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TrustedRootCertificationAuthorities() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardCertificateStoreNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IntermediateCertificationAuthorities() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardCertificateStoreNamesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IStandardCertificateStoreNamesStatics<R, F: FnOnce(&IStandardCertificateStoreNamesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StandardCertificateStoreNames, IStandardCertificateStoreNamesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StandardCertificateStoreNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.StandardCertificateStoreNames";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SubjectAlternativeNameInfo(pub ::windows::core::IInspectable);
impl SubjectAlternativeNameInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SubjectAlternativeNameInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EmailName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IPAddress(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Url(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DistinguishedName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrincipalName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EmailNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IPAddresses(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Urls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DistinguishedNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrincipalNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Extension(&self) -> ::windows::core::Result<CertificateExtension> {
        let this = &::windows::core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CertificateExtension>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SubjectAlternativeNameInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo;{582859f1-569d-4c20-be7b-4e1c9a0bc52b})");
}
unsafe impl ::windows::core::Interface for SubjectAlternativeNameInfo {
    type Vtable = ISubjectAlternativeNameInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x582859f1_569d_4c20_be7b_4e1c9a0bc52b);
}
impl ::windows::core::RuntimeName for SubjectAlternativeNameInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo";
}
impl ::core::convert::From<SubjectAlternativeNameInfo> for ::windows::core::IUnknown {
    fn from(value: SubjectAlternativeNameInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SubjectAlternativeNameInfo> for ::windows::core::IUnknown {
    fn from(value: &SubjectAlternativeNameInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SubjectAlternativeNameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SubjectAlternativeNameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SubjectAlternativeNameInfo> for ::windows::core::IInspectable {
    fn from(value: SubjectAlternativeNameInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SubjectAlternativeNameInfo> for ::windows::core::IInspectable {
    fn from(value: &SubjectAlternativeNameInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SubjectAlternativeNameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SubjectAlternativeNameInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SubjectAlternativeNameInfo {}
unsafe impl ::core::marker::Sync for SubjectAlternativeNameInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserCertificateEnrollmentManager(pub ::windows::core::IInspectable);
impl UserCertificateEnrollmentManager {
    #[cfg(feature = "Foundation")]
    pub fn CreateRequestAsync<'a, Param0: ::windows::core::IntoParam<'a, CertificateRequestProperties>>(&self, request: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn InstallCertificateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, certificate: Param0, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), certificate.into_param().abi(), installoption, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, pfxdata: Param0, password: Param1, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: Param5) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pfxdata.into_param().abi(), password.into_param().abi(), exportable, keyprotectionlevel, installoption, friendlyname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataToKspAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, pfxdata: Param0, password: Param1, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: Param5, keystorageprovider: Param6) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), pfxdata.into_param().abi(), password.into_param().abi(), exportable, keyprotectionlevel, installoption, friendlyname.into_param().abi(), keystorageprovider.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ImportPfxDataToKspWithParametersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, PfxImportParameters>>(&self, pfxdata: Param0, password: Param1, pfximportparameters: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IUserCertificateEnrollmentManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pfxdata.into_param().abi(), password.into_param().abi(), pfximportparameters.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserCertificateEnrollmentManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager;{96313718-22e1-4819-b20b-ab46a6eca06e})");
}
unsafe impl ::windows::core::Interface for UserCertificateEnrollmentManager {
    type Vtable = IUserCertificateEnrollmentManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96313718_22e1_4819_b20b_ab46a6eca06e);
}
impl ::windows::core::RuntimeName for UserCertificateEnrollmentManager {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager";
}
impl ::core::convert::From<UserCertificateEnrollmentManager> for ::windows::core::IUnknown {
    fn from(value: UserCertificateEnrollmentManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserCertificateEnrollmentManager> for ::windows::core::IUnknown {
    fn from(value: &UserCertificateEnrollmentManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserCertificateEnrollmentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserCertificateEnrollmentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserCertificateEnrollmentManager> for ::windows::core::IInspectable {
    fn from(value: UserCertificateEnrollmentManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserCertificateEnrollmentManager> for ::windows::core::IInspectable {
    fn from(value: &UserCertificateEnrollmentManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserCertificateEnrollmentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserCertificateEnrollmentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserCertificateEnrollmentManager {}
unsafe impl ::core::marker::Sync for UserCertificateEnrollmentManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserCertificateStore(pub ::windows::core::IInspectable);
impl UserCertificateStore {
    #[cfg(feature = "Foundation")]
    pub fn RequestAddAsync<'a, Param0: ::windows::core::IntoParam<'a, Certificate>>(&self, certificate: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), certificate.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsync<'a, Param0: ::windows::core::IntoParam<'a, Certificate>>(&self, certificate: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), certificate.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserCertificateStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Certificates.UserCertificateStore;{c9fb1d83-789f-4b4e-9180-045a757aac6d})");
}
unsafe impl ::windows::core::Interface for UserCertificateStore {
    type Vtable = IUserCertificateStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9fb1d83_789f_4b4e_9180_045a757aac6d);
}
impl ::windows::core::RuntimeName for UserCertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.UserCertificateStore";
}
impl ::core::convert::From<UserCertificateStore> for ::windows::core::IUnknown {
    fn from(value: UserCertificateStore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserCertificateStore> for ::windows::core::IUnknown {
    fn from(value: &UserCertificateStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserCertificateStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserCertificateStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserCertificateStore> for ::windows::core::IInspectable {
    fn from(value: UserCertificateStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserCertificateStore> for ::windows::core::IInspectable {
    fn from(value: &UserCertificateStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserCertificateStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserCertificateStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserCertificateStore {}
unsafe impl ::core::marker::Sync for UserCertificateStore {}
