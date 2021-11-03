#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcCertificateEnumerator(::windows::runtime::IUnknown);
impl IOpcCertificateEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(hasnext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MovePrevious(&self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(hasprevious)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    pub unsafe fn GetCurrent(&self, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(certificate)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IOpcCertificateEnumerator> {
        let mut result__: <IOpcCertificateEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcCertificateEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcCertificateEnumerator {
    type Vtable = IOpcCertificateEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2232621367, 36644, 16927, [180, 57, 89, 171, 36, 209, 64, 184]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcCertificateEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcCertificateSet(::windows::runtime::IUnknown);
impl IOpcCertificateSet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    pub unsafe fn Add(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(certificate)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    pub unsafe fn Remove(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(certificate)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::runtime::Result<IOpcCertificateEnumerator> {
        let mut result__: <IOpcCertificateEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcCertificateEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcCertificateSet {
    type Vtable = IOpcCertificateSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1458193189, 36397, 16743, [177, 164, 228, 134, 210, 76, 143, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcCertificateSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, certificateenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcDigitalSignature(::windows::runtime::IUnknown);
impl IOpcDigitalSignature {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetNamespaces(&self, prefixes: *mut *mut super::super::super::Foundation::PWSTR, namespaces: *mut *mut super::super::super::Foundation::PWSTR, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(prefixes), ::std::mem::transmute(namespaces), ::std::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetSignatureId(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCanonicalizationMethod(&self, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(canonicalizationmethod)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignatureValue(&self, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(signaturevalue), ::std::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignaturePartReferenceEnumerator(&self) -> ::windows::runtime::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__: <IOpcSignaturePartReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReferenceEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignatureRelationshipReferenceEnumerator(&self) -> ::windows::runtime::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__: <IOpcSignatureRelationshipReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReferenceEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetSigningTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetTimeFormat(&self, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(timeformat)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPackageObjectReference(&self) -> ::windows::runtime::Result<IOpcSignatureReference> {
        let mut result__: <IOpcSignatureReference as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCertificateEnumerator(&self) -> ::windows::runtime::Result<IOpcCertificateEnumerator> {
        let mut result__: <IOpcCertificateEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcCertificateEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> ::windows::runtime::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__: <IOpcSignatureReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReferenceEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCustomObjectEnumerator(&self) -> ::windows::runtime::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__: <IOpcSignatureCustomObjectEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObjectEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(signaturexml), ::std::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOpcDigitalSignature {
    type Vtable = IOpcDigitalSignature_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1386947037, 7376, 18761, [188, 128, 12, 18, 50, 208, 12, 180]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignature_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prefixes: *mut *mut super::super::super::Foundation::PWSTR, namespaces: *mut *mut super::super::super::Foundation::PWSTR, count: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturepartname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partreferenceenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipreferenceenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signingtime: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageobjectreference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, certificateenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customreferenceenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customobjectenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcDigitalSignatureEnumerator(::windows::runtime::IUnknown);
impl IOpcDigitalSignatureEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MovePrevious(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IOpcDigitalSignature> {
        let mut result__: <IOpcDigitalSignature as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcDigitalSignature>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__: <IOpcDigitalSignatureEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcDigitalSignatureEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcDigitalSignatureEnumerator {
    type Vtable = IOpcDigitalSignatureEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2524670082, 2979, 17240, [185, 231, 182, 76, 117, 6, 60, 94]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignatureEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digitalsignature: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcDigitalSignatureManager(::windows::runtime::IUnknown);
impl IOpcDigitalSignatureManager {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignatureOriginPartName(&self) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetSignatureOriginPartName<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, signatureoriginpartname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), signatureoriginpartname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignatureEnumerator(&self) -> ::windows::runtime::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__: <IOpcDigitalSignatureEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcDigitalSignatureEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn RemoveSignature<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, signaturepartname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), signaturepartname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateSigningOptions(&self) -> ::windows::runtime::Result<IOpcSigningOptions> {
        let mut result__: <IOpcSigningOptions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSigningOptions>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    pub unsafe fn Validate<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcDigitalSignature>>(&self, signature: Param0, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), signature.into_param().abi(), ::std::mem::transmute(certificate), ::std::mem::transmute(validationresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    pub unsafe fn Sign<'a, Param1: ::windows::runtime::IntoParam<'a, IOpcSigningOptions>>(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: Param1) -> ::windows::runtime::Result<IOpcDigitalSignature> {
        let mut result__: <IOpcDigitalSignature as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(certificate), signingoptions.into_param().abi(), &mut result__).from_abi::<IOpcDigitalSignature>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn ReplaceSignatureXml<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, signaturepartname: Param0, newsignaturexml: *const u8, count: u32) -> ::windows::runtime::Result<IOpcDigitalSignature> {
        let mut result__: <IOpcDigitalSignature as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), signaturepartname.into_param().abi(), ::std::mem::transmute(newsignaturexml), ::std::mem::transmute(count), &mut result__).from_abi::<IOpcDigitalSignature>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcDigitalSignatureManager {
    type Vtable = IOpcDigitalSignatureManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3588631051, 26989, 17967, [148, 223, 114, 227, 60, 239, 38, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignatureManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureoriginpartname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureoriginpartname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturepartname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signingoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signature: ::windows::runtime::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: ::windows::runtime::RawPtr, digitalsignature: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturepartname: ::windows::runtime::RawPtr, newsignaturexml: *const u8, count: u32, digitalsignature: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcFactory(::windows::runtime::IUnknown);
impl IOpcFactory {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePackageRootUri(&self) -> ::windows::runtime::Result<IOpcUri> {
        let mut result__: <IOpcUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn CreatePartUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzuri: Param0) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pwzuri.into_param().abi(), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Com`*"]
    pub unsafe fn CreateStreamOnFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), filename.into_param().abi(), ::std::mem::transmute(iomode), ::std::mem::transmute(securityattributes), ::std::mem::transmute(dwflagsandattributes), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreatePackage(&self) -> ::windows::runtime::Result<IOpcPackage> {
        let mut result__: <IOpcPackage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPackage>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn ReadPackageFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, stream: Param0, flags: OPC_READ_FLAGS) -> ::windows::runtime::Result<IOpcPackage> {
        let mut result__: <IOpcPackage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), stream.into_param().abi(), ::std::mem::transmute(flags), &mut result__).from_abi::<IOpcPackage>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn WritePackageToStream<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPackage>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, package: Param0, flags: OPC_WRITE_FLAGS, stream: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), package.into_param().abi(), ::std::mem::transmute(flags), stream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateDigitalSignatureManager<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPackage>>(&self, package: Param0) -> ::windows::runtime::Result<IOpcDigitalSignatureManager> {
        let mut result__: <IOpcDigitalSignatureManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), package.into_param().abi(), &mut result__).from_abi::<IOpcDigitalSignatureManager>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcFactory {
    type Vtable = IOpcFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1829454918, 52595, 19123, [148, 244, 140, 205, 246, 17, 97, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rooturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwzuri: super::super::super::Foundation::PWSTR, parturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, flags: OPC_READ_FLAGS, package: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: ::windows::runtime::RawPtr, flags: OPC_WRITE_FLAGS, stream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, package: ::windows::runtime::RawPtr, signaturemanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcPackage(::windows::runtime::IUnknown);
impl IOpcPackage {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartSet(&self) -> ::windows::runtime::Result<IOpcPartSet> {
        let mut result__: <IOpcPartSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartSet>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetRelationshipSet(&self) -> ::windows::runtime::Result<IOpcRelationshipSet> {
        let mut result__: <IOpcRelationshipSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSet>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcPackage {
    type Vtable = IOpcPackage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1108957513, 15225, 20424, [137, 198, 252, 127, 185, 121, 238, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPackage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcPart(::windows::runtime::IUnknown);
impl IOpcPart {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetRelationshipSet(&self) -> ::windows::runtime::Result<IOpcRelationshipSet> {
        let mut result__: <IOpcRelationshipSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSet>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetContentStream(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetContentType(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCompressionOptions(&self) -> ::windows::runtime::Result<OPC_COMPRESSION_OPTIONS> {
        let mut result__: <OPC_COMPRESSION_OPTIONS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OPC_COMPRESSION_OPTIONS>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcPart {
    type Vtable = IOpcPart_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1108957513, 15225, 20424, [137, 198, 252, 127, 185, 121, 238, 113]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPart_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcPartEnumerator(::windows::runtime::IUnknown);
impl IOpcPartEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MovePrevious(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IOpcPart> {
        let mut result__: <IOpcPart as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPart>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IOpcPartEnumerator> {
        let mut result__: <IOpcPartEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcPartEnumerator {
    type Vtable = IOpcPartEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1108957513, 15225, 20424, [137, 198, 252, 127, 185, 121, 238, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, part: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcPartSet(::windows::runtime::IUnknown);
impl IOpcPartSet {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPart<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, name: Param0) -> ::windows::runtime::Result<IOpcPart> {
        let mut result__: <IOpcPart as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IOpcPart>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn CreatePart<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0, contenttype: Param1, compressionoptions: OPC_COMPRESSION_OPTIONS) -> ::windows::runtime::Result<IOpcPart> {
        let mut result__: <IOpcPart as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), name.into_param().abi(), contenttype.into_param().abi(), ::std::mem::transmute(compressionoptions), &mut result__).from_abi::<IOpcPart>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn DeletePart<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn PartExists<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::runtime::Result<IOpcPartEnumerator> {
        let mut result__: <IOpcPartEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcPartSet {
    type Vtable = IOpcPartSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1108957513, 15225, 20424, [137, 198, 252, 127, 185, 121, 238, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::windows::runtime::RawPtr, part: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::windows::runtime::RawPtr, contenttype: super::super::super::Foundation::PWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::windows::runtime::RawPtr, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcPartUri(::windows::runtime::IUnknown);
impl IOpcPartUri {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut super::super::super::Foundation::BSTR, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), ::std::mem::transmute(pbstrproperty), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetPropertyLength(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), ::std::mem::transmute(pcchproperty), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), ::std::mem::transmute(pdwproperty), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn HasProperty(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetAuthority(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetDomain(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetExtension(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetFragment(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetHost(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetPassword(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetQuery(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetRawUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemeName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetUserInfo(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetUserName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetHostType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPort(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetScheme(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetZone(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, puri: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), puri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetRelationshipsPartUri(&self) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetRelativeUri<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, targetparturi: Param0) -> ::windows::runtime::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), targetparturi.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CombinePartUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, relativeuri: Param0) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), relativeuri.into_param().abi(), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn ComparePartUri<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, parturi: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), parturi.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSourceUri(&self) -> ::windows::runtime::Result<IOpcUri> {
        let mut result__: <IOpcUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn IsRelationshipsPartUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcPartUri {
    type Vtable = IOpcPartUri_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2101062631, 34994, 18106, [133, 203, 66, 3, 203, 1, 108, 135]);
}
impl ::std::convert::From<IOpcPartUri> for IOpcUri {
    fn from(value: IOpcPartUri) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOpcPartUri> for IOpcUri {
    fn from(value: &IOpcPartUri) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOpcUri> for IOpcPartUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOpcUri> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IOpcUri>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IOpcUri> for &IOpcPartUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, IOpcUri> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IOpcUri>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IOpcPartUri> for super::super::super::System::Com::IUri {
    fn from(value: IOpcPartUri) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IOpcPartUri> for super::super::super::System::Com::IUri {
    fn from(value: &IOpcPartUri) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri> for IOpcPartUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IUri> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Com::IUri>::into(self))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri> for &IOpcPartUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IUri> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Com::IUri>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartUri_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pfhasproperty: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrabsoluteuri: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrauthority: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdisplaystring: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdomain: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextension: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfragment: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrhost: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpassword: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpathandquery: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrquery: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrrawuri: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrschemename: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruserinfo: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrusername: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwhosttype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwport: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwscheme: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwzone: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puri: ::windows::runtime::RawPtr, pfequal: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipparturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetparturi: ::windows::runtime::RawPtr, relativeuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeuri: ::windows::runtime::RawPtr, combineduri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, comparisonresult: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcRelationship(::windows::runtime::IUnknown);
impl IOpcRelationship {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetId(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetRelationshipType(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSourceUri(&self) -> ::windows::runtime::Result<IOpcUri> {
        let mut result__: <IOpcUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetTargetUri(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetTargetMode(&self) -> ::windows::runtime::Result<OPC_URI_TARGET_MODE> {
        let mut result__: <OPC_URI_TARGET_MODE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OPC_URI_TARGET_MODE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcRelationship {
    type Vtable = IOpcRelationship_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1108957513, 15225, 20424, [137, 198, 252, 127, 185, 121, 238, 114]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationship_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipidentifier: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshiptype: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targeturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcRelationshipEnumerator(::windows::runtime::IUnknown);
impl IOpcRelationshipEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MovePrevious(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IOpcRelationship> {
        let mut result__: <IOpcRelationship as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationship>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IOpcRelationshipEnumerator> {
        let mut result__: <IOpcRelationshipEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcRelationshipEnumerator {
    type Vtable = IOpcRelationshipEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1108957513, 15225, 20424, [137, 198, 252, 127, 185, 121, 238, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationship: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcRelationshipSelector(::windows::runtime::IUnknown);
impl IOpcRelationshipSelector {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSelectorType(&self) -> ::windows::runtime::Result<OPC_RELATIONSHIP_SELECTOR> {
        let mut result__: <OPC_RELATIONSHIP_SELECTOR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OPC_RELATIONSHIP_SELECTOR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetSelectionCriterion(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcRelationshipSelector {
    type Vtable = IOpcRelationshipSelector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4176637055, 45711, 18585, [132, 200, 93, 86, 57, 237, 231, 95]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selectioncriterion: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcRelationshipSelectorEnumerator(::windows::runtime::IUnknown);
impl IOpcRelationshipSelectorEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MovePrevious(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IOpcRelationshipSelector> {
        let mut result__: <IOpcRelationshipSelector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelector>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__: <IOpcRelationshipSelectorEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelectorEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcRelationshipSelectorEnumerator {
    type Vtable = IOpcRelationshipSelectorEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1582342529, 43291, 18604, [136, 210, 188, 163, 216, 248, 192, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipselector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcRelationshipSelectorSet(::windows::runtime::IUnknown);
impl IOpcRelationshipSelectorSet {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn Create<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: Param1) -> ::windows::runtime::Result<IOpcRelationshipSelector> {
        let mut result__: <IOpcRelationshipSelector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(selector), selectioncriterion.into_param().abi(), &mut result__).from_abi::<IOpcRelationshipSelector>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcRelationshipSelector>>(&self, relationshipselector: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), relationshipselector.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::runtime::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__: <IOpcRelationshipSelectorEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelectorEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcRelationshipSelectorSet {
    type Vtable = IOpcRelationshipSelectorSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1848951401, 42195, 18368, [181, 196, 135, 255, 43, 59, 97, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: super::super::super::Foundation::PWSTR, relationshipselector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipselector: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipselectorenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcRelationshipSet(::windows::runtime::IUnknown);
impl IOpcRelationshipSet {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetRelationship<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, relationshipidentifier: Param0) -> ::windows::runtime::Result<IOpcRelationship> {
        let mut result__: <IOpcRelationship as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), relationshipidentifier.into_param().abi(), &mut result__).from_abi::<IOpcRelationship>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateRelationship<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, relationshipidentifier: Param0, relationshiptype: Param1, targeturi: Param2, targetmode: OPC_URI_TARGET_MODE) -> ::windows::runtime::Result<IOpcRelationship> {
        let mut result__: <IOpcRelationship as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), relationshipidentifier.into_param().abi(), relationshiptype.into_param().abi(), targeturi.into_param().abi(), ::std::mem::transmute(targetmode), &mut result__).from_abi::<IOpcRelationship>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn DeleteRelationship<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, relationshipidentifier: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), relationshipidentifier.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn RelationshipExists<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, relationshipidentifier: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), relationshipidentifier.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::runtime::Result<IOpcRelationshipEnumerator> {
        let mut result__: <IOpcRelationshipEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetEnumeratorForType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, relationshiptype: Param0) -> ::windows::runtime::Result<IOpcRelationshipEnumerator> {
        let mut result__: <IOpcRelationshipEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), relationshiptype.into_param().abi(), &mut result__).from_abi::<IOpcRelationshipEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetRelationshipsContentStream(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcRelationshipSet {
    type Vtable = IOpcRelationshipSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1108957513, 15225, 20424, [137, 198, 252, 127, 185, 121, 238, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipidentifier: super::super::super::Foundation::PWSTR, relationship: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshiptype: super::super::super::Foundation::PWSTR, targeturi: ::windows::runtime::RawPtr, targetmode: OPC_URI_TARGET_MODE, relationship: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipidentifier: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshiptype: super::super::super::Foundation::PWSTR, relationshipenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignatureCustomObject(::windows::runtime::IUnknown);
impl IOpcSignatureCustomObject {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetXml(&self, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(xmlmarkup), ::std::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignatureCustomObject {
    type Vtable = IOpcSignatureCustomObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1568121246, 25281, 17639, [190, 205, 69, 218, 90, 229, 26, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignatureCustomObjectEnumerator(::windows::runtime::IUnknown);
impl IOpcSignatureCustomObjectEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MovePrevious(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IOpcSignatureCustomObject> {
        let mut result__: <IOpcSignatureCustomObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObject>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__: <IOpcSignatureCustomObjectEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObjectEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignatureCustomObjectEnumerator {
    type Vtable = IOpcSignatureCustomObjectEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1592065565, 57776, 18051, [128, 121, 126, 160, 252, 248, 11, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignatureCustomObjectSet(::windows::runtime::IUnknown);
impl IOpcSignatureCustomObjectSet {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Create(&self, xmlmarkup: *const u8, count: u32) -> ::windows::runtime::Result<IOpcSignatureCustomObject> {
        let mut result__: <IOpcSignatureCustomObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(xmlmarkup), ::std::mem::transmute(count), &mut result__).from_abi::<IOpcSignatureCustomObject>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcSignatureCustomObject>>(&self, customobject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), customobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::runtime::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__: <IOpcSignatureCustomObjectEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObjectEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignatureCustomObjectSet {
    type Vtable = IOpcSignatureCustomObjectSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2407082693, 31047, 19985, [188, 61, 38, 89, 255, 4, 106, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xmlmarkup: *const u8, count: u32, customobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customobjectenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignaturePartReference(::windows::runtime::IUnknown);
impl IOpcSignaturePartReference {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPartName(&self) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetContentType(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetDigestMethod(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(digestvalue), ::std::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetTransformMethod(&self) -> ::windows::runtime::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__: <OPC_CANONICALIZATION_METHOD as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OPC_CANONICALIZATION_METHOD>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignaturePartReference {
    type Vtable = IOpcSignaturePartReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3795988938, 23028, 18510, [182, 75, 54, 238, 218, 54, 7, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignaturePartReferenceEnumerator(::windows::runtime::IUnknown);
impl IOpcSignaturePartReferenceEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MovePrevious(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IOpcSignaturePartReference> {
        let mut result__: <IOpcSignaturePartReference as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__: <IOpcSignaturePartReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignaturePartReferenceEnumerator {
    type Vtable = IOpcSignaturePartReferenceEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2162890081, 35959, 18895, [130, 102, 69, 155, 53, 110, 233, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partreference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignaturePartReferenceSet(::windows::runtime::IUnknown);
impl IOpcSignaturePartReferenceSet {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, parturi: Param0, digestmethod: Param1, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::runtime::Result<IOpcSignaturePartReference> {
        let mut result__: <IOpcSignaturePartReference as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), parturi.into_param().abi(), digestmethod.into_param().abi(), ::std::mem::transmute(transformmethod), &mut result__).from_abi::<IOpcSignaturePartReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcSignaturePartReference>>(&self, partreference: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), partreference.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::runtime::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__: <IOpcSignaturePartReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignaturePartReferenceSet {
    type Vtable = IOpcSignaturePartReferenceSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1822417548, 60633, 19234, [157, 54, 127, 221, 230, 112, 254, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parturi: ::windows::runtime::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partreference: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partreferenceenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignatureReference(::windows::runtime::IUnknown);
impl IOpcSignatureReference {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetId(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetUri(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetTransformMethod(&self) -> ::windows::runtime::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__: <OPC_CANONICALIZATION_METHOD as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OPC_CANONICALIZATION_METHOD>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetDigestMethod(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(digestvalue), ::std::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignatureReference {
    type Vtable = IOpcSignatureReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(457637982, 12305, 20188, [190, 111, 15, 101, 229, 171, 3, 66]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, referenceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, referenceuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignatureReferenceEnumerator(::windows::runtime::IUnknown);
impl IOpcSignatureReferenceEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MovePrevious(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IOpcSignatureReference> {
        let mut result__: <IOpcSignatureReference as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__: <IOpcSignatureReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignatureReferenceEnumerator {
    type Vtable = IOpcSignatureReferenceEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3483736645, 10417, 18536, [150, 158, 250, 128, 151, 253, 193, 42]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignatureReferenceSet(::windows::runtime::IUnknown);
impl IOpcSignatureReferenceSet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
        &self,
        referenceuri: Param0,
        referenceid: Param1,
        r#type: Param2,
        digestmethod: Param3,
        transformmethod: OPC_CANONICALIZATION_METHOD,
    ) -> ::windows::runtime::Result<IOpcSignatureReference> {
        let mut result__: <IOpcSignatureReference as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), referenceuri.into_param().abi(), referenceid.into_param().abi(), r#type.into_param().abi(), digestmethod.into_param().abi(), ::std::mem::transmute(transformmethod), &mut result__).from_abi::<IOpcSignatureReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcSignatureReference>>(&self, reference: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), reference.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::runtime::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__: <IOpcSignatureReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignatureReferenceSet {
    type Vtable = IOpcSignatureReferenceSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4088409393, 43794, 17117, [158, 47, 43, 22, 118, 28, 60, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, referenceuri: ::windows::runtime::RawPtr, referenceid: super::super::super::Foundation::PWSTR, r#type: super::super::super::Foundation::PWSTR, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reference: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, referenceenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignatureRelationshipReference(::windows::runtime::IUnknown);
impl IOpcSignatureRelationshipReference {
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSourceUri(&self) -> ::windows::runtime::Result<IOpcUri> {
        let mut result__: <IOpcUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetDigestMethod(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(digestvalue), ::std::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetTransformMethod(&self) -> ::windows::runtime::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__: <OPC_CANONICALIZATION_METHOD as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OPC_CANONICALIZATION_METHOD>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetRelationshipSigningOption(&self) -> ::windows::runtime::Result<OPC_RELATIONSHIPS_SIGNING_OPTION> {
        let mut result__: <OPC_RELATIONSHIPS_SIGNING_OPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OPC_RELATIONSHIPS_SIGNING_OPTION>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetRelationshipSelectorEnumerator(&self) -> ::windows::runtime::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__: <IOpcRelationshipSelectorEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelectorEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignatureRelationshipReference {
    type Vtable = IOpcSignatureRelationshipReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1471855302, 40266, 20048, [139, 134, 229, 212, 5, 30, 174, 124]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selectorenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignatureRelationshipReferenceEnumerator(::windows::runtime::IUnknown);
impl IOpcSignatureRelationshipReferenceEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn MovePrevious(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IOpcSignatureRelationshipReference> {
        let mut result__: <IOpcSignatureRelationshipReference as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__: <IOpcSignatureRelationshipReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignatureRelationshipReferenceEnumerator {
    type Vtable = IOpcSignatureRelationshipReferenceEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2000397284, 61473, 18660, [170, 4, 152, 22, 219, 93, 52, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReferenceEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipreference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSignatureRelationshipReferenceSet(::windows::runtime::IUnknown);
impl IOpcSignatureRelationshipReferenceSet {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcUri>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, IOpcRelationshipSelectorSet>>(&self, sourceuri: Param0, digestmethod: Param1, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: Param3, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::runtime::Result<IOpcSignatureRelationshipReference> {
        let mut result__: <IOpcSignatureRelationshipReference as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), sourceuri.into_param().abi(), digestmethod.into_param().abi(), ::std::mem::transmute(relationshipsigningoption), selectorset.into_param().abi(), ::std::mem::transmute(transformmethod), &mut result__).from_abi::<IOpcSignatureRelationshipReference>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn CreateRelationshipSelectorSet(&self) -> ::windows::runtime::Result<IOpcRelationshipSelectorSet> {
        let mut result__: <IOpcRelationshipSelectorSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelectorSet>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcSignatureRelationshipReference>>(&self, relationshipreference: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), relationshipreference.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetEnumerator(&self) -> ::windows::runtime::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__: <IOpcSignatureRelationshipReferenceEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSignatureRelationshipReferenceSet {
    type Vtable = IOpcSignatureRelationshipReferenceSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2676374693, 13873, 16460, [130, 141, 128, 126, 7, 21, 6, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReferenceSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceuri: ::windows::runtime::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: ::windows::runtime::RawPtr, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selectorset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipreference: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipreferenceenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcSigningOptions(::windows::runtime::IUnknown);
impl IOpcSigningOptions {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetSignatureId(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn SetSignatureId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, signatureid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), signatureid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn SetSignatureMethod<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, signaturemethod: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), signaturemethod.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetDefaultDigestMethod(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn SetDefaultDigestMethod<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, digestmethod: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), digestmethod.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCertificateEmbeddingOption(&self) -> ::windows::runtime::Result<OPC_CERTIFICATE_EMBEDDING_OPTION> {
        let mut result__: <OPC_CERTIFICATE_EMBEDDING_OPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OPC_CERTIFICATE_EMBEDDING_OPTION>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetCertificateEmbeddingOption(&self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(embeddingoption)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetTimeFormat(&self) -> ::windows::runtime::Result<OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__: <OPC_SIGNATURE_TIME_FORMAT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<OPC_SIGNATURE_TIME_FORMAT>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetTimeFormat(&self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(timeformat)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignaturePartReferenceSet(&self) -> ::windows::runtime::Result<IOpcSignaturePartReferenceSet> {
        let mut result__: <IOpcSignaturePartReferenceSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReferenceSet>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignatureRelationshipReferenceSet(&self) -> ::windows::runtime::Result<IOpcSignatureRelationshipReferenceSet> {
        let mut result__: <IOpcSignatureRelationshipReferenceSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReferenceSet>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCustomObjectSet(&self) -> ::windows::runtime::Result<IOpcSignatureCustomObjectSet> {
        let mut result__: <IOpcSignatureCustomObjectSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObjectSet>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCustomReferenceSet(&self) -> ::windows::runtime::Result<IOpcSignatureReferenceSet> {
        let mut result__: <IOpcSignatureReferenceSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReferenceSet>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetCertificateSet(&self) -> ::windows::runtime::Result<IOpcCertificateSet> {
        let mut result__: <IOpcCertificateSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcCertificateSet>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn SetSignaturePartName<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, signaturepartname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), signaturepartname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOpcSigningOptions {
    type Vtable = IOpcSigningOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1355994789, 31467, 18112, [178, 65, 67, 171, 14, 155, 64, 126]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSigningOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signatureid: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturemethod: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, digestmethod: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, partreferenceset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipreferenceset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customobjectset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customreferenceset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, certificateset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturepartname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signaturepartname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOpcUri(::windows::runtime::IUnknown);
impl IOpcUri {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut super::super::super::Foundation::BSTR, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), ::std::mem::transmute(pbstrproperty), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetPropertyLength(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), ::std::mem::transmute(pcchproperty), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), ::std::mem::transmute(pdwproperty), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn HasProperty(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetAuthority(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetDomain(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetExtension(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetFragment(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetHost(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetPassword(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetQuery(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetRawUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemeName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetUserInfo(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`*"]
    pub unsafe fn GetUserName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetHostType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetPort(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetScheme(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetZone(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, puri: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), puri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
    pub unsafe fn GetRelationshipsPartUri(&self) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn GetRelativeUri<'a, Param0: ::windows::runtime::IntoParam<'a, IOpcPartUri>>(&self, targetparturi: Param0) -> ::windows::runtime::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), targetparturi.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Opc`, `Win32_System_Com`*"]
    pub unsafe fn CombinePartUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, relativeuri: Param0) -> ::windows::runtime::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), relativeuri.into_param().abi(), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOpcUri {
    type Vtable = IOpcUri_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3164347291, 54828, 18923, [174, 240, 59, 78, 11, 40, 235, 237]);
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IOpcUri> for super::super::super::System::Com::IUri {
    fn from(value: IOpcUri) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IOpcUri> for super::super::super::System::Com::IUri {
    fn from(value: &IOpcUri) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri> for IOpcUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IUri> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Com::IUri>::into(self))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IUri> for &IOpcUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::System::Com::IUri> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::System::Com::IUri>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcUri_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pfhasproperty: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrabsoluteuri: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrauthority: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdisplaystring: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdomain: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextension: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfragment: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrhost: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpassword: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpathandquery: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrquery: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrrawuri: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrschemename: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruserinfo: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrusername: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwhosttype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwport: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwscheme: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwzone: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puri: ::windows::runtime::RawPtr, pfequal: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relationshipparturi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetparturi: ::windows::runtime::RawPtr, relativeuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeuri: ::windows::runtime::RawPtr, combineduri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_CANONICALIZATION_METHOD(pub i32);
pub const OPC_CANONICALIZATION_NONE: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(0i32);
pub const OPC_CANONICALIZATION_C14N: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(1i32);
pub const OPC_CANONICALIZATION_C14N_WITH_COMMENTS: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(2i32);
impl ::std::convert::From<i32> for OPC_CANONICALIZATION_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_CANONICALIZATION_METHOD {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_CERTIFICATE_EMBEDDING_OPTION(pub i32);
pub const OPC_CERTIFICATE_IN_CERTIFICATE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(0i32);
pub const OPC_CERTIFICATE_IN_SIGNATURE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(1i32);
pub const OPC_CERTIFICATE_NOT_EMBEDDED: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(2i32);
impl ::std::convert::From<i32> for OPC_CERTIFICATE_EMBEDDING_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_CERTIFICATE_EMBEDDING_OPTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_COMPRESSION_OPTIONS(pub i32);
pub const OPC_COMPRESSION_NONE: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(-1i32);
pub const OPC_COMPRESSION_NORMAL: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(0i32);
pub const OPC_COMPRESSION_MAXIMUM: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(1i32);
pub const OPC_COMPRESSION_FAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(2i32);
pub const OPC_COMPRESSION_SUPERFAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(3i32);
impl ::std::convert::From<i32> for OPC_COMPRESSION_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_COMPRESSION_OPTIONS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_CONFLICTING_SETTINGS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175212i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_COULD_NOT_RECOVER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175154i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_DEFAULT_DIGEST_METHOD_NOT_SET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175161i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_DIGEST_VALUE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175206i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_DUPLICATE_PACKAGE_OBJECT_REFERENCES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175187i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_DUPLICATE_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175205i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_DUPLICATE_SIGNATURE_PROPERTY_ELEMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175192i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_EXTERNAL_SIGNATURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175202i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_EXTERNAL_SIGNATURE_REFERENCE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175185i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_INVALID_CANONICALIZATION_METHOD: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175198i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_INVALID_CERTIFICATE_RELATIONSHIP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175203i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_INVALID_OPC_SIGNATURE_TIME_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175196i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_INVALID_RELATIONSHIPS_SIGNING_OPTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175197i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_INVALID_RELATIONSHIP_TRANSFORM_XML: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175199i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_INVALID_SIGNATURE_COUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175189i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_INVALID_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175204i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_INVALID_SIGNATURE_XML: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175190i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MISSING_CANONICALIZATION_TRANSFORM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175182i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MISSING_CERTIFICATE_PART: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175146i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MISSING_PACKAGE_OBJECT_REFERENCE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175186i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_ALGORITHM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175188i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_ORIGIN_PART: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175201i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_PART: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175200i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTIES_ELEMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175194i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTY_ELEMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175193i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_TIME_PROPERTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175191i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_MULTIPLE_RELATIONSHIP_TRANSFORMS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175183i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_PACKAGE_REFERENCE_URI_RESERVED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175195i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_REFERENCE_MISSING_CONTENT_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175184i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_SIGNATURE_CORRUPT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175207i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_SIGNATURE_METHOD_NOT_SET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175162i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_SIGNATURE_ORIGIN_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175148i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_SIGNATURE_PROPERTY_MISSING_TARGET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175163i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_SIGNATURE_REFERENCE_MISSING_URI: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175165i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DS_UNSIGNED_PACKAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175147i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DUPLICATE_DEFAULT_EXTENSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175217i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DUPLICATE_OVERRIDE_PART: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175219i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DUPLICATE_PART: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175221i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DUPLICATE_PIECE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175211i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_DUPLICATE_RELATIONSHIP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175213i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ENUM_CANNOT_MOVE_NEXT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175151i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ENUM_CANNOT_MOVE_PREVIOUS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175150i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ENUM_COLLECTION_CHANGED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175152i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ENUM_INVALID_POSITION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175149i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_CONTENT_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175164i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_CONTENT_TYPE_XML: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175226i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_DEFAULT_EXTENSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175218i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_OVERRIDE_PART_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175220i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_PIECE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175210i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_RELATIONSHIP_ID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175216i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_RELATIONSHIP_TARGET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175214i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_RELATIONSHIP_TARGET_MODE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175155i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_RELATIONSHIP_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175215i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_RELS_XML: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175222i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_INVALID_XML_ENCODING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175166i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ATTRIBUTES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175157i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ELEMENTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175156i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_INCONSISTENT_PROCESS_CONTENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175158i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_INVALID_ATTRIBUTES_ON_IGNORABLE_ELEMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175168i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_INVALID_ENUM_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175172i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_INVALID_PREFIX_LIST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175177i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_INVALID_QNAME_LIST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175176i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_INVALID_XMLNS_ATTRIBUTE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175167i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_MISSING_CHOICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175173i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_MISSING_REQUIRES_ATTR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175179i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_MULTIPLE_FALLBACK_ELEMENTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175159i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_NESTED_ALTERNATE_CONTENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175175i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_UNEXPECTED_ATTR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175178i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_UNEXPECTED_CHOICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175174i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_UNEXPECTED_ELEMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175181i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_UNEXPECTED_REQUIRES_ATTR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175180i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_UNKNOWN_NAMESPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175170i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MC_UNKNOWN_PREFIX: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175169i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MISSING_CONTENT_TYPES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175225i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_MISSING_PIECE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175209i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_NONCONFORMING_CONTENT_TYPES_XML: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175224i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_NONCONFORMING_RELS_XML: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175223i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_NONCONFORMING_URI: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175231i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_NO_SUCH_PART: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175208i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_NO_SUCH_RELATIONSHIP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175160i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_NO_SUCH_SETTINGS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175145i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_PART_CANNOT_BE_DIRECTORY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175228i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_RELATIONSHIP_URI_REQUIRED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175229i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_RELATIVE_URI_REQUIRED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175230i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_UNEXPECTED_CONTENT_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175227i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_UNSUPPORTED_PACKAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142175153i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_CENTRAL_DIRECTORY_TOO_LARGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171127i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_COMMENT_TOO_LARGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171124i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_COMPRESSION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171133i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_CORRUPTED_ARCHIVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171134i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_DECOMPRESSION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171132i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_DUPLICATE_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171125i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_EXTRA_FIELDS_TOO_LARGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171123i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_FILE_HEADER_TOO_LARGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171122i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_INCONSISTENT_DIRECTORY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171130i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_INCONSISTENT_FILEITEM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171131i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_INCORRECT_DATA_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171135i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_MISSING_DATA_DESCRIPTOR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171129i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_MISSING_END_OF_CENTRAL_DIRECTORY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171121i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_NAME_TOO_LARGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171126i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_REQUIRES_64_BIT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171120i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
pub const OPC_E_ZIP_UNSUPPORTEDARCHIVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2142171128i32 as _);
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_READ_FLAGS(pub u32);
pub const OPC_READ_DEFAULT: OPC_READ_FLAGS = OPC_READ_FLAGS(0u32);
pub const OPC_VALIDATE_ON_LOAD: OPC_READ_FLAGS = OPC_READ_FLAGS(1u32);
pub const OPC_CACHE_ON_ACCESS: OPC_READ_FLAGS = OPC_READ_FLAGS(2u32);
impl ::std::convert::From<u32> for OPC_READ_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_READ_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for OPC_READ_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for OPC_READ_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for OPC_READ_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for OPC_READ_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for OPC_READ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_RELATIONSHIPS_SIGNING_OPTION(pub i32);
pub const OPC_RELATIONSHIP_SIGN_USING_SELECTORS: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(0i32);
pub const OPC_RELATIONSHIP_SIGN_PART: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(1i32);
impl ::std::convert::From<i32> for OPC_RELATIONSHIPS_SIGNING_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_RELATIONSHIPS_SIGNING_OPTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_RELATIONSHIP_SELECTOR(pub i32);
pub const OPC_RELATIONSHIP_SELECT_BY_ID: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(0i32);
pub const OPC_RELATIONSHIP_SELECT_BY_TYPE: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(1i32);
impl ::std::convert::From<i32> for OPC_RELATIONSHIP_SELECTOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_RELATIONSHIP_SELECTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_SIGNATURE_TIME_FORMAT(pub i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(0i32);
pub const OPC_SIGNATURE_TIME_FORMAT_SECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(1i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MINUTES: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(2i32);
pub const OPC_SIGNATURE_TIME_FORMAT_DAYS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(3i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MONTHS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(4i32);
pub const OPC_SIGNATURE_TIME_FORMAT_YEARS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(5i32);
impl ::std::convert::From<i32> for OPC_SIGNATURE_TIME_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_SIGNATURE_TIME_FORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_SIGNATURE_VALIDATION_RESULT(pub i32);
pub const OPC_SIGNATURE_VALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(0i32);
pub const OPC_SIGNATURE_INVALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(-1i32);
impl ::std::convert::From<i32> for OPC_SIGNATURE_VALIDATION_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_SIGNATURE_VALIDATION_RESULT {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_STREAM_IO_MODE(pub i32);
pub const OPC_STREAM_IO_READ: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(1i32);
pub const OPC_STREAM_IO_WRITE: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(2i32);
impl ::std::convert::From<i32> for OPC_STREAM_IO_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_STREAM_IO_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_URI_TARGET_MODE(pub i32);
pub const OPC_URI_TARGET_MODE_INTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(0i32);
pub const OPC_URI_TARGET_MODE_EXTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(1i32);
impl ::std::convert::From<i32> for OPC_URI_TARGET_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_URI_TARGET_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Opc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_WRITE_FLAGS(pub u32);
pub const OPC_WRITE_DEFAULT: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(0u32);
pub const OPC_WRITE_FORCE_ZIP32: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(1u32);
impl ::std::convert::From<u32> for OPC_WRITE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPC_WRITE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for OPC_WRITE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for OPC_WRITE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for OPC_WRITE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for OPC_WRITE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for OPC_WRITE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const OpcFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1798138784, 40766, 20263, [146, 11, 49, 60, 196, 38, 163, 158]);
