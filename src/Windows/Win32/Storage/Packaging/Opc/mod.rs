#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcCertificateEnumerator(pub ::windows::core::IUnknown);
impl IOpcCertificateEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hasnext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hasprevious)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn GetCurrent(&self, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(certificate)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcCertificateEnumerator> {
        let mut result__: <IOpcCertificateEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcCertificateEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcCertificateEnumerator {
    type Vtable = IOpcCertificateEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85131937_8f24_421f_b439_59ab24d140b8);
}
impl ::core::convert::From<IOpcCertificateEnumerator> for ::windows::core::IUnknown {
    fn from(value: IOpcCertificateEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcCertificateEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IOpcCertificateEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcCertificateEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcCertificateEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcCertificateEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcCertificateSet(pub ::windows::core::IUnknown);
impl IOpcCertificateSet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Add(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(certificate)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Remove(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(certificate)).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcCertificateEnumerator> {
        let mut result__: <IOpcCertificateEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcCertificateEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcCertificateSet {
    type Vtable = IOpcCertificateSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56ea4325_8e2d_4167_b1a4_e486d24c8fa7);
}
impl ::core::convert::From<IOpcCertificateSet> for ::windows::core::IUnknown {
    fn from(value: IOpcCertificateSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcCertificateSet> for ::windows::core::IUnknown {
    fn from(value: &IOpcCertificateSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcCertificateSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcCertificateSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcCertificateSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcDigitalSignature(pub ::windows::core::IUnknown);
impl IOpcDigitalSignature {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamespaces(&self, prefixes: *mut *mut super::super::super::Foundation::PWSTR, namespaces: *mut *mut super::super::super::Foundation::PWSTR, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(prefixes), ::core::mem::transmute(namespaces), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetCanonicalizationMethod(&self, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(canonicalizationmethod)).ok()
    }
    pub unsafe fn GetSignatureValue(&self, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturevalue), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetSignaturePartReferenceEnumerator(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__: <IOpcSignaturePartReferenceEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReferenceEnumerator>(result__)
    }
    pub unsafe fn GetSignatureRelationshipReferenceEnumerator(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__: <IOpcSignatureRelationshipReferenceEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReferenceEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSigningTime(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetTimeFormat(&self, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeformat)).ok()
    }
    pub unsafe fn GetPackageObjectReference(&self) -> ::windows::core::Result<IOpcSignatureReference> {
        let mut result__: <IOpcSignatureReference as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReference>(result__)
    }
    pub unsafe fn GetCertificateEnumerator(&self) -> ::windows::core::Result<IOpcCertificateEnumerator> {
        let mut result__: <IOpcCertificateEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcCertificateEnumerator>(result__)
    }
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__: <IOpcSignatureReferenceEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReferenceEnumerator>(result__)
    }
    pub unsafe fn GetCustomObjectEnumerator(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__: <IOpcSignatureCustomObjectEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObjectEnumerator>(result__)
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(signaturexml), ::core::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::core::Interface for IOpcDigitalSignature {
    type Vtable = IOpcDigitalSignature_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52ab21dd_1cd0_4949_bc80_0c1232d00cb4);
}
impl ::core::convert::From<IOpcDigitalSignature> for ::windows::core::IUnknown {
    fn from(value: IOpcDigitalSignature) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcDigitalSignature> for ::windows::core::IUnknown {
    fn from(value: &IOpcDigitalSignature) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcDigitalSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcDigitalSignature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignature_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prefixes: *mut *mut super::super::super::Foundation::PWSTR, namespaces: *mut *mut super::super::super::Foundation::PWSTR, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, partreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signingtime: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageobjectreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificateenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcDigitalSignatureEnumerator(pub ::windows::core::IUnknown);
impl IOpcDigitalSignatureEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcDigitalSignature> {
        let mut result__: <IOpcDigitalSignature as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcDigitalSignature>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__: <IOpcDigitalSignatureEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcDigitalSignatureEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcDigitalSignatureEnumerator {
    type Vtable = IOpcDigitalSignatureEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x967b6882_0ba3_4358_b9e7_b64c75063c5e);
}
impl ::core::convert::From<IOpcDigitalSignatureEnumerator> for ::windows::core::IUnknown {
    fn from(value: IOpcDigitalSignatureEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcDigitalSignatureEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IOpcDigitalSignatureEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcDigitalSignatureEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcDigitalSignatureEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignatureEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcDigitalSignatureManager(pub ::windows::core::IUnknown);
impl IOpcDigitalSignatureManager {
    pub unsafe fn GetSignatureOriginPartName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    pub unsafe fn SetSignatureOriginPartName<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, signatureoriginpartname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), signatureoriginpartname.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatureEnumerator(&self) -> ::windows::core::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__: <IOpcDigitalSignatureEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcDigitalSignatureEnumerator>(result__)
    }
    pub unsafe fn RemoveSignature<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, signaturepartname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), signaturepartname.into_param().abi()).ok()
    }
    pub unsafe fn CreateSigningOptions(&self) -> ::windows::core::Result<IOpcSigningOptions> {
        let mut result__: <IOpcSigningOptions as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSigningOptions>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Validate<'a, Param0: ::windows::core::IntoParam<'a, IOpcDigitalSignature>>(&self, signature: Param0, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), signature.into_param().abi(), ::core::mem::transmute(certificate), ::core::mem::transmute(validationresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Sign<'a, Param1: ::windows::core::IntoParam<'a, IOpcSigningOptions>>(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: Param1) -> ::windows::core::Result<IOpcDigitalSignature> {
        let mut result__: <IOpcDigitalSignature as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(certificate), signingoptions.into_param().abi(), &mut result__).from_abi::<IOpcDigitalSignature>(result__)
    }
    pub unsafe fn ReplaceSignatureXml<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, signaturepartname: Param0, newsignaturexml: *const u8, count: u32) -> ::windows::core::Result<IOpcDigitalSignature> {
        let mut result__: <IOpcDigitalSignature as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), signaturepartname.into_param().abi(), ::core::mem::transmute(newsignaturexml), ::core::mem::transmute(count), &mut result__).from_abi::<IOpcDigitalSignature>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcDigitalSignatureManager {
    type Vtable = IOpcDigitalSignatureManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5e62a0b_696d_462f_94df_72e33cef2659);
}
impl ::core::convert::From<IOpcDigitalSignatureManager> for ::windows::core::IUnknown {
    fn from(value: IOpcDigitalSignatureManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcDigitalSignatureManager> for ::windows::core::IUnknown {
    fn from(value: &IOpcDigitalSignatureManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcDigitalSignatureManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcDigitalSignatureManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignatureManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signatureoriginpartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signatureoriginpartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signatureenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signature: ::windows::core::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: ::windows::core::RawPtr, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturepartname: ::windows::core::RawPtr, newsignaturexml: *const u8, count: u32, digitalsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcFactory(pub ::windows::core::IUnknown);
impl IOpcFactory {
    pub unsafe fn CreatePackageRootUri(&self) -> ::windows::core::Result<IOpcUri> {
        let mut result__: <IOpcUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePartUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzuri: Param0) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwzuri.into_param().abi(), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStreamOnFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(iomode), ::core::mem::transmute(securityattributes), ::core::mem::transmute(dwflagsandattributes), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn CreatePackage(&self) -> ::windows::core::Result<IOpcPackage> {
        let mut result__: <IOpcPackage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPackage>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReadPackageFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, stream: Param0, flags: OPC_READ_FLAGS) -> ::windows::core::Result<IOpcPackage> {
        let mut result__: <IOpcPackage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), stream.into_param().abi(), ::core::mem::transmute(flags), &mut result__).from_abi::<IOpcPackage>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WritePackageToStream<'a, Param0: ::windows::core::IntoParam<'a, IOpcPackage>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, package: Param0, flags: OPC_WRITE_FLAGS, stream: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), package.into_param().abi(), ::core::mem::transmute(flags), stream.into_param().abi()).ok()
    }
    pub unsafe fn CreateDigitalSignatureManager<'a, Param0: ::windows::core::IntoParam<'a, IOpcPackage>>(&self, package: Param0) -> ::windows::core::Result<IOpcDigitalSignatureManager> {
        let mut result__: <IOpcDigitalSignatureManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), package.into_param().abi(), &mut result__).from_abi::<IOpcDigitalSignatureManager>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcFactory {
    type Vtable = IOpcFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d0b4446_cd73_4ab3_94f4_8ccdf6116154);
}
impl ::core::convert::From<IOpcFactory> for ::windows::core::IUnknown {
    fn from(value: IOpcFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcFactory> for ::windows::core::IUnknown {
    fn from(value: &IOpcFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rooturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzuri: super::super::super::Foundation::PWSTR, parturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, flags: OPC_READ_FLAGS, package: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, package: ::windows::core::RawPtr, flags: OPC_WRITE_FLAGS, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, package: ::windows::core::RawPtr, signaturemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcPackage(pub ::windows::core::IUnknown);
impl IOpcPackage {
    pub unsafe fn GetPartSet(&self) -> ::windows::core::Result<IOpcPartSet> {
        let mut result__: <IOpcPartSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartSet>(result__)
    }
    pub unsafe fn GetRelationshipSet(&self) -> ::windows::core::Result<IOpcRelationshipSet> {
        let mut result__: <IOpcRelationshipSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSet>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcPackage {
    type Vtable = IOpcPackage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee70);
}
impl ::core::convert::From<IOpcPackage> for ::windows::core::IUnknown {
    fn from(value: IOpcPackage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcPackage> for ::windows::core::IUnknown {
    fn from(value: &IOpcPackage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPackage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, partset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcPart(pub ::windows::core::IUnknown);
impl IOpcPart {
    pub unsafe fn GetRelationshipSet(&self) -> ::windows::core::Result<IOpcRelationshipSet> {
        let mut result__: <IOpcRelationshipSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSet>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetContentStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentType(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetCompressionOptions(&self) -> ::windows::core::Result<OPC_COMPRESSION_OPTIONS> {
        let mut result__: <OPC_COMPRESSION_OPTIONS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<OPC_COMPRESSION_OPTIONS>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcPart {
    type Vtable = IOpcPart_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee71);
}
impl ::core::convert::From<IOpcPart> for ::windows::core::IUnknown {
    fn from(value: IOpcPart) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcPart> for ::windows::core::IUnknown {
    fn from(value: &IOpcPart) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPart_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcPartEnumerator(pub ::windows::core::IUnknown);
impl IOpcPartEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcPart> {
        let mut result__: <IOpcPart as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPart>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcPartEnumerator> {
        let mut result__: <IOpcPartEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcPartEnumerator {
    type Vtable = IOpcPartEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee75);
}
impl ::core::convert::From<IOpcPartEnumerator> for ::windows::core::IUnknown {
    fn from(value: IOpcPartEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcPartEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IOpcPartEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcPartEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcPartEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcPartSet(pub ::windows::core::IUnknown);
impl IOpcPartSet {
    pub unsafe fn GetPart<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, name: Param0) -> ::windows::core::Result<IOpcPart> {
        let mut result__: <IOpcPart as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IOpcPart>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePart<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0, contenttype: Param1, compressionoptions: OPC_COMPRESSION_OPTIONS) -> ::windows::core::Result<IOpcPart> {
        let mut result__: <IOpcPart as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), contenttype.into_param().abi(), ::core::mem::transmute(compressionoptions), &mut result__).from_abi::<IOpcPart>(result__)
    }
    pub unsafe fn DeletePart<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PartExists<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, name: Param0) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcPartEnumerator> {
        let mut result__: <IOpcPartEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcPartSet {
    type Vtable = IOpcPartSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee73);
}
impl ::core::convert::From<IOpcPartSet> for ::windows::core::IUnknown {
    fn from(value: IOpcPartSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcPartSet> for ::windows::core::IUnknown {
    fn from(value: &IOpcPartSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcPartSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcPartSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::windows::core::RawPtr, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::windows::core::RawPtr, contenttype: super::super::super::Foundation::PWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::windows::core::RawPtr, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, partenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcPartUri(pub ::windows::core::IUnknown);
impl IOpcPartUri {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut super::super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pbstrproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyLength(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pcchproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pdwproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HasProperty(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthority(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDomain(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtension(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFragment(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHost(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPassword(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuery(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRawUri(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemeName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserInfo(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetHostType(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetScheme(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetZone(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, puri: Param0) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), puri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetRelationshipsPartUri(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelativeUri<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, targetparturi: Param0) -> ::windows::core::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), targetparturi.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CombinePartUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, relativeuri: Param0) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), relativeuri.into_param().abi(), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    pub unsafe fn ComparePartUri<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, parturi: Param0) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), parturi.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn GetSourceUri(&self) -> ::windows::core::Result<IOpcUri> {
        let mut result__: <IOpcUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRelationshipsPartUri(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcPartUri {
    type Vtable = IOpcPartUri_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d3babe7_88b2_46ba_85cb_4203cb016c87);
}
impl ::core::convert::From<IOpcPartUri> for ::windows::core::IUnknown {
    fn from(value: IOpcPartUri) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcPartUri> for ::windows::core::IUnknown {
    fn from(value: &IOpcPartUri) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcPartUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcPartUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IOpcPartUri> for IOpcUri {
    fn from(value: IOpcPartUri) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IOpcPartUri> for IOpcUri {
    fn from(value: &IOpcPartUri) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOpcUri> for IOpcPartUri {
    fn into_param(self) -> ::windows::core::Param<'a, IOpcUri> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IOpcUri> for &IOpcPartUri {
    fn into_param(self) -> ::windows::core::Param<'a, IOpcUri> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IOpcPartUri> for super::super::super::System::Com::IUri {
    fn from(value: IOpcPartUri) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IOpcPartUri> for super::super::super::System::Com::IUri {
    fn from(value: &IOpcPartUri) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri> for IOpcPartUri {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IUri> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri> for &IOpcPartUri {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IUri> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartUri_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pfhasproperty: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrabsoluteuri: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrauthority: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdisplaystring: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdomain: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrextension: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfragment: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrhost: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpassword: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathandquery: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrquery: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrawuri: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrschemename: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruserinfo: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwhosttype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwport: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwscheme: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, pfequal: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetparturi: ::windows::core::RawPtr, relativeuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeuri: ::windows::core::RawPtr, combineduri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, comparisonresult: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcRelationship(pub ::windows::core::IUnknown);
impl IOpcRelationship {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetId(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRelationshipType(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetSourceUri(&self) -> ::windows::core::Result<IOpcUri> {
        let mut result__: <IOpcUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTargetUri(&self) -> ::windows::core::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    pub unsafe fn GetTargetMode(&self) -> ::windows::core::Result<OPC_URI_TARGET_MODE> {
        let mut result__: <OPC_URI_TARGET_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<OPC_URI_TARGET_MODE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationship {
    type Vtable = IOpcRelationship_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee72);
}
impl ::core::convert::From<IOpcRelationship> for ::windows::core::IUnknown {
    fn from(value: IOpcRelationship) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcRelationship> for ::windows::core::IUnknown {
    fn from(value: &IOpcRelationship) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcRelationship {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcRelationship {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationship_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipidentifier: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshiptype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targeturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcRelationshipEnumerator(pub ::windows::core::IUnknown);
impl IOpcRelationshipEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcRelationship> {
        let mut result__: <IOpcRelationship as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationship>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcRelationshipEnumerator> {
        let mut result__: <IOpcRelationshipEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipEnumerator {
    type Vtable = IOpcRelationshipEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee76);
}
impl ::core::convert::From<IOpcRelationshipEnumerator> for ::windows::core::IUnknown {
    fn from(value: IOpcRelationshipEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcRelationshipEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IOpcRelationshipEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcRelationshipEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcRelationshipEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcRelationshipSelector(pub ::windows::core::IUnknown);
impl IOpcRelationshipSelector {
    pub unsafe fn GetSelectorType(&self) -> ::windows::core::Result<OPC_RELATIONSHIP_SELECTOR> {
        let mut result__: <OPC_RELATIONSHIP_SELECTOR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<OPC_RELATIONSHIP_SELECTOR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelectionCriterion(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipSelector {
    type Vtable = IOpcRelationshipSelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8f26c7f_b28f_4899_84c8_5d5639ede75f);
}
impl ::core::convert::From<IOpcRelationshipSelector> for ::windows::core::IUnknown {
    fn from(value: IOpcRelationshipSelector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcRelationshipSelector> for ::windows::core::IUnknown {
    fn from(value: &IOpcRelationshipSelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcRelationshipSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcRelationshipSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selectioncriterion: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcRelationshipSelectorEnumerator(pub ::windows::core::IUnknown);
impl IOpcRelationshipSelectorEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcRelationshipSelector> {
        let mut result__: <IOpcRelationshipSelector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelector>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__: <IOpcRelationshipSelectorEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelectorEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipSelectorEnumerator {
    type Vtable = IOpcRelationshipSelectorEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e50a181_a91b_48ac_88d2_bca3d8f8c0b1);
}
impl ::core::convert::From<IOpcRelationshipSelectorEnumerator> for ::windows::core::IUnknown {
    fn from(value: IOpcRelationshipSelectorEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcRelationshipSelectorEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IOpcRelationshipSelectorEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcRelationshipSelectorEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcRelationshipSelectorEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipselector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcRelationshipSelectorSet(pub ::windows::core::IUnknown);
impl IOpcRelationshipSelectorSet {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: Param1) -> ::windows::core::Result<IOpcRelationshipSelector> {
        let mut result__: <IOpcRelationshipSelector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(selector), selectioncriterion.into_param().abi(), &mut result__).from_abi::<IOpcRelationshipSelector>(result__)
    }
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, IOpcRelationshipSelector>>(&self, relationshipselector: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), relationshipselector.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__: <IOpcRelationshipSelectorEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelectorEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipSelectorSet {
    type Vtable = IOpcRelationshipSelectorSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e34c269_a4d3_47c0_b5c4_87ff2b3b6136);
}
impl ::core::convert::From<IOpcRelationshipSelectorSet> for ::windows::core::IUnknown {
    fn from(value: IOpcRelationshipSelectorSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcRelationshipSelectorSet> for ::windows::core::IUnknown {
    fn from(value: &IOpcRelationshipSelectorSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcRelationshipSelectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcRelationshipSelectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: super::super::super::Foundation::PWSTR, relationshipselector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipselector: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipselectorenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcRelationshipSet(pub ::windows::core::IUnknown);
impl IOpcRelationshipSet {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, relationshipidentifier: Param0) -> ::windows::core::Result<IOpcRelationship> {
        let mut result__: <IOpcRelationship as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), relationshipidentifier.into_param().abi(), &mut result__).from_abi::<IOpcRelationship>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, relationshipidentifier: Param0, relationshiptype: Param1, targeturi: Param2, targetmode: OPC_URI_TARGET_MODE) -> ::windows::core::Result<IOpcRelationship> {
        let mut result__: <IOpcRelationship as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), relationshipidentifier.into_param().abi(), relationshiptype.into_param().abi(), targeturi.into_param().abi(), ::core::mem::transmute(targetmode), &mut result__).from_abi::<IOpcRelationship>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteRelationship<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, relationshipidentifier: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), relationshipidentifier.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelationshipExists<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, relationshipidentifier: Param0) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), relationshipidentifier.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcRelationshipEnumerator> {
        let mut result__: <IOpcRelationshipEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnumeratorForType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, relationshiptype: Param0) -> ::windows::core::Result<IOpcRelationshipEnumerator> {
        let mut result__: <IOpcRelationshipEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), relationshiptype.into_param().abi(), &mut result__).from_abi::<IOpcRelationshipEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsContentStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipSet {
    type Vtable = IOpcRelationshipSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee74);
}
impl ::core::convert::From<IOpcRelationshipSet> for ::windows::core::IUnknown {
    fn from(value: IOpcRelationshipSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcRelationshipSet> for ::windows::core::IUnknown {
    fn from(value: &IOpcRelationshipSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcRelationshipSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcRelationshipSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipidentifier: super::super::super::Foundation::PWSTR, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshiptype: super::super::super::Foundation::PWSTR, targeturi: ::windows::core::RawPtr, targetmode: OPC_URI_TARGET_MODE, relationship: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipidentifier: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipidentifier: super::super::super::Foundation::PWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshiptype: super::super::super::Foundation::PWSTR, relationshipenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignatureCustomObject(pub ::windows::core::IUnknown);
impl IOpcSignatureCustomObject {
    pub unsafe fn GetXml(&self, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(xmlmarkup), ::core::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureCustomObject {
    type Vtable = IOpcSignatureCustomObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d77a19e_62c1_44e7_becd_45da5ae51a56);
}
impl ::core::convert::From<IOpcSignatureCustomObject> for ::windows::core::IUnknown {
    fn from(value: IOpcSignatureCustomObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignatureCustomObject> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignatureCustomObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignatureCustomObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignatureCustomObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignatureCustomObjectEnumerator(pub ::windows::core::IUnknown);
impl IOpcSignatureCustomObjectEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignatureCustomObject> {
        let mut result__: <IOpcSignatureCustomObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObject>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__: <IOpcSignatureCustomObjectEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObjectEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureCustomObjectEnumerator {
    type Vtable = IOpcSignatureCustomObjectEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ee4fe1d_e1b0_4683_8079_7ea0fcf80b4c);
}
impl ::core::convert::From<IOpcSignatureCustomObjectEnumerator> for ::windows::core::IUnknown {
    fn from(value: IOpcSignatureCustomObjectEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignatureCustomObjectEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignatureCustomObjectEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignatureCustomObjectEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignatureCustomObjectEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignatureCustomObjectSet(pub ::windows::core::IUnknown);
impl IOpcSignatureCustomObjectSet {
    pub unsafe fn Create(&self, xmlmarkup: *const u8, count: u32) -> ::windows::core::Result<IOpcSignatureCustomObject> {
        let mut result__: <IOpcSignatureCustomObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(xmlmarkup), ::core::mem::transmute(count), &mut result__).from_abi::<IOpcSignatureCustomObject>(result__)
    }
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, IOpcSignatureCustomObject>>(&self, customobject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), customobject.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__: <IOpcSignatureCustomObjectEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObjectEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureCustomObjectSet {
    type Vtable = IOpcSignatureCustomObjectSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f792ac5_7947_4e11_bc3d_2659ff046ae1);
}
impl ::core::convert::From<IOpcSignatureCustomObjectSet> for ::windows::core::IUnknown {
    fn from(value: IOpcSignatureCustomObjectSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignatureCustomObjectSet> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignatureCustomObjectSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignatureCustomObjectSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignatureCustomObjectSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xmlmarkup: *const u8, count: u32, customobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customobjectenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignaturePartReference(pub ::windows::core::IUnknown);
impl IOpcSignaturePartReference {
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentType(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDigestMethod(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(digestvalue), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetTransformMethod(&self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__: <OPC_CANONICALIZATION_METHOD as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<OPC_CANONICALIZATION_METHOD>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignaturePartReference {
    type Vtable = IOpcSignaturePartReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe24231ca_59f4_484e_b64b_36eeda36072c);
}
impl ::core::convert::From<IOpcSignaturePartReference> for ::windows::core::IUnknown {
    fn from(value: IOpcSignaturePartReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignaturePartReference> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignaturePartReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignaturePartReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignaturePartReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, partname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignaturePartReferenceEnumerator(pub ::windows::core::IUnknown);
impl IOpcSignaturePartReferenceEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignaturePartReference> {
        let mut result__: <IOpcSignaturePartReference as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReference>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__: <IOpcSignaturePartReferenceEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignaturePartReferenceEnumerator {
    type Vtable = IOpcSignaturePartReferenceEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80eb1561_8c77_49cf_8266_459b356ee99a);
}
impl ::core::convert::From<IOpcSignaturePartReferenceEnumerator> for ::windows::core::IUnknown {
    fn from(value: IOpcSignaturePartReferenceEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignaturePartReferenceEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignaturePartReferenceEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignaturePartReferenceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignaturePartReferenceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, partreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignaturePartReferenceSet(pub ::windows::core::IUnknown);
impl IOpcSignaturePartReferenceSet {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, parturi: Param0, digestmethod: Param1, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignaturePartReference> {
        let mut result__: <IOpcSignaturePartReference as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), parturi.into_param().abi(), digestmethod.into_param().abi(), ::core::mem::transmute(transformmethod), &mut result__).from_abi::<IOpcSignaturePartReference>(result__)
    }
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, IOpcSignaturePartReference>>(&self, partreference: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), partreference.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__: <IOpcSignaturePartReferenceEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignaturePartReferenceSet {
    type Vtable = IOpcSignaturePartReferenceSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c9fe28c_ecd9_4b22_9d36_7fdde670fec0);
}
impl ::core::convert::From<IOpcSignaturePartReferenceSet> for ::windows::core::IUnknown {
    fn from(value: IOpcSignaturePartReferenceSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignaturePartReferenceSet> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignaturePartReferenceSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignaturePartReferenceSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignaturePartReferenceSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parturi: ::windows::core::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, partreference: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, partreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignatureReference(pub ::windows::core::IUnknown);
impl IOpcSignatureReference {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetId(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetTransformMethod(&self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__: <OPC_CANONICALIZATION_METHOD as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<OPC_CANONICALIZATION_METHOD>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDigestMethod(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(digestvalue), ::core::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureReference {
    type Vtable = IOpcSignatureReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b47005e_3011_4edc_be6f_0f65e5ab0342);
}
impl ::core::convert::From<IOpcSignatureReference> for ::windows::core::IUnknown {
    fn from(value: IOpcSignatureReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignatureReference> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignatureReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignatureReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignatureReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, referenceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, referenceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignatureReferenceEnumerator(pub ::windows::core::IUnknown);
impl IOpcSignatureReferenceEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignatureReference> {
        let mut result__: <IOpcSignatureReference as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReference>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__: <IOpcSignatureReferenceEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureReferenceEnumerator {
    type Vtable = IOpcSignatureReferenceEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfa59a45_28b1_4868_969e_fa8097fdc12a);
}
impl ::core::convert::From<IOpcSignatureReferenceEnumerator> for ::windows::core::IUnknown {
    fn from(value: IOpcSignatureReferenceEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignatureReferenceEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignatureReferenceEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignatureReferenceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignatureReferenceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignatureReferenceSet(pub ::windows::core::IUnknown);
impl IOpcSignatureReferenceSet {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
        &self,
        referenceuri: Param0,
        referenceid: Param1,
        r#type: Param2,
        digestmethod: Param3,
        transformmethod: OPC_CANONICALIZATION_METHOD,
    ) -> ::windows::core::Result<IOpcSignatureReference> {
        let mut result__: <IOpcSignatureReference as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), referenceuri.into_param().abi(), referenceid.into_param().abi(), r#type.into_param().abi(), digestmethod.into_param().abi(), ::core::mem::transmute(transformmethod), &mut result__).from_abi::<IOpcSignatureReference>(result__)
    }
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, IOpcSignatureReference>>(&self, reference: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), reference.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__: <IOpcSignatureReferenceEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureReferenceSet {
    type Vtable = IOpcSignatureReferenceSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b02d31_ab12_42dd_9e2f_2b16761c3c1e);
}
impl ::core::convert::From<IOpcSignatureReferenceSet> for ::windows::core::IUnknown {
    fn from(value: IOpcSignatureReferenceSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignatureReferenceSet> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignatureReferenceSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignatureReferenceSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignatureReferenceSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, referenceuri: ::windows::core::RawPtr, referenceid: super::super::super::Foundation::PWSTR, r#type: super::super::super::Foundation::PWSTR, digestmethod: super::super::super::Foundation::PWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reference: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, referenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignatureRelationshipReference(pub ::windows::core::IUnknown);
impl IOpcSignatureRelationshipReference {
    pub unsafe fn GetSourceUri(&self) -> ::windows::core::Result<IOpcUri> {
        let mut result__: <IOpcUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcUri>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDigestMethod(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(digestvalue), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn GetTransformMethod(&self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__: <OPC_CANONICALIZATION_METHOD as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<OPC_CANONICALIZATION_METHOD>(result__)
    }
    pub unsafe fn GetRelationshipSigningOption(&self) -> ::windows::core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION> {
        let mut result__: <OPC_RELATIONSHIPS_SIGNING_OPTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<OPC_RELATIONSHIPS_SIGNING_OPTION>(result__)
    }
    pub unsafe fn GetRelationshipSelectorEnumerator(&self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__: <IOpcRelationshipSelectorEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelectorEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureRelationshipReference {
    type Vtable = IOpcSignatureRelationshipReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57babac6_9d4a_4e50_8b86_e5d4051eae7c);
}
impl ::core::convert::From<IOpcSignatureRelationshipReference> for ::windows::core::IUnknown {
    fn from(value: IOpcSignatureRelationshipReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignatureRelationshipReference> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignatureRelationshipReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignatureRelationshipReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignatureRelationshipReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourceuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selectorenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignatureRelationshipReferenceEnumerator(pub ::windows::core::IUnknown);
impl IOpcSignatureRelationshipReferenceEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReference> {
        let mut result__: <IOpcSignatureRelationshipReference as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReference>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__: <IOpcSignatureRelationshipReferenceEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureRelationshipReferenceEnumerator {
    type Vtable = IOpcSignatureRelationshipReferenceEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x773ba3e4_f021_48e4_aa04_9816db5d3495);
}
impl ::core::convert::From<IOpcSignatureRelationshipReferenceEnumerator> for ::windows::core::IUnknown {
    fn from(value: IOpcSignatureRelationshipReferenceEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignatureRelationshipReferenceEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignatureRelationshipReferenceEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignatureRelationshipReferenceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignatureRelationshipReferenceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReferenceEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSignatureRelationshipReferenceSet(pub ::windows::core::IUnknown);
impl IOpcSignatureRelationshipReferenceSet {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, IOpcUri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IOpcRelationshipSelectorSet>>(&self, sourceuri: Param0, digestmethod: Param1, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: Param3, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignatureRelationshipReference> {
        let mut result__: <IOpcSignatureRelationshipReference as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), sourceuri.into_param().abi(), digestmethod.into_param().abi(), ::core::mem::transmute(relationshipsigningoption), selectorset.into_param().abi(), ::core::mem::transmute(transformmethod), &mut result__).from_abi::<IOpcSignatureRelationshipReference>(result__)
    }
    pub unsafe fn CreateRelationshipSelectorSet(&self) -> ::windows::core::Result<IOpcRelationshipSelectorSet> {
        let mut result__: <IOpcRelationshipSelectorSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcRelationshipSelectorSet>(result__)
    }
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, IOpcSignatureRelationshipReference>>(&self, relationshipreference: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), relationshipreference.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__: <IOpcSignatureRelationshipReferenceEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReferenceEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureRelationshipReferenceSet {
    type Vtable = IOpcSignatureRelationshipReferenceSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f863ca5_3631_404c_828d_807e0715069b);
}
impl ::core::convert::From<IOpcSignatureRelationshipReferenceSet> for ::windows::core::IUnknown {
    fn from(value: IOpcSignatureRelationshipReferenceSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSignatureRelationshipReferenceSet> for ::windows::core::IUnknown {
    fn from(value: &IOpcSignatureRelationshipReferenceSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSignatureRelationshipReferenceSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSignatureRelationshipReferenceSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReferenceSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourceuri: ::windows::core::RawPtr, digestmethod: super::super::super::Foundation::PWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: ::windows::core::RawPtr, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selectorset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipreference: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipreferenceenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcSigningOptions(pub ::windows::core::IUnknown);
impl IOpcSigningOptions {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSignatureId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, signatureid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), signatureid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSignatureMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, signaturemethod: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), signaturemethod.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultDigestMethod(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefaultDigestMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, digestmethod: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), digestmethod.into_param().abi()).ok()
    }
    pub unsafe fn GetCertificateEmbeddingOption(&self) -> ::windows::core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION> {
        let mut result__: <OPC_CERTIFICATE_EMBEDDING_OPTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<OPC_CERTIFICATE_EMBEDDING_OPTION>(result__)
    }
    pub unsafe fn SetCertificateEmbeddingOption(&self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(embeddingoption)).ok()
    }
    pub unsafe fn GetTimeFormat(&self) -> ::windows::core::Result<OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__: <OPC_SIGNATURE_TIME_FORMAT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<OPC_SIGNATURE_TIME_FORMAT>(result__)
    }
    pub unsafe fn SetTimeFormat(&self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeformat)).ok()
    }
    pub unsafe fn GetSignaturePartReferenceSet(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceSet> {
        let mut result__: <IOpcSignaturePartReferenceSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignaturePartReferenceSet>(result__)
    }
    pub unsafe fn GetSignatureRelationshipReferenceSet(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceSet> {
        let mut result__: <IOpcSignatureRelationshipReferenceSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureRelationshipReferenceSet>(result__)
    }
    pub unsafe fn GetCustomObjectSet(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectSet> {
        let mut result__: <IOpcSignatureCustomObjectSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureCustomObjectSet>(result__)
    }
    pub unsafe fn GetCustomReferenceSet(&self) -> ::windows::core::Result<IOpcSignatureReferenceSet> {
        let mut result__: <IOpcSignatureReferenceSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcSignatureReferenceSet>(result__)
    }
    pub unsafe fn GetCertificateSet(&self) -> ::windows::core::Result<IOpcCertificateSet> {
        let mut result__: <IOpcCertificateSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcCertificateSet>(result__)
    }
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    pub unsafe fn SetSignaturePartName<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, signaturepartname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), signaturepartname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IOpcSigningOptions {
    type Vtable = IOpcSigningOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d2d6a5_7aeb_46c0_b241_43ab0e9b407e);
}
impl ::core::convert::From<IOpcSigningOptions> for ::windows::core::IUnknown {
    fn from(value: IOpcSigningOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcSigningOptions> for ::windows::core::IUnknown {
    fn from(value: &IOpcSigningOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcSigningOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcSigningOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSigningOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signatureid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signatureid: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturemethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturemethod: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digestmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, digestmethod: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, partreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customreferenceset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certificateset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturepartname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signaturepartname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOpcUri(pub ::windows::core::IUnknown);
impl IOpcUri {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut super::super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pbstrproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyLength(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pcchproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pdwproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HasProperty(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthority(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDomain(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtension(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFragment(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHost(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPassword(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuery(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRawUri(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemeName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserInfo(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetHostType(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetScheme(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetZone(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, puri: Param0) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), puri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetRelationshipsPartUri(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelativeUri<'a, Param0: ::windows::core::IntoParam<'a, IOpcPartUri>>(&self, targetparturi: Param0) -> ::windows::core::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), targetparturi.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CombinePartUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri>>(&self, relativeuri: Param0) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__: <IOpcPartUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), relativeuri.into_param().abi(), &mut result__).from_abi::<IOpcPartUri>(result__)
    }
}
unsafe impl ::windows::core::Interface for IOpcUri {
    type Vtable = IOpcUri_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc9c1b9b_d62c_49eb_aef0_3b4e0b28ebed);
}
impl ::core::convert::From<IOpcUri> for ::windows::core::IUnknown {
    fn from(value: IOpcUri) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOpcUri> for ::windows::core::IUnknown {
    fn from(value: &IOpcUri) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOpcUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOpcUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IOpcUri> for super::super::super::System::Com::IUri {
    fn from(value: IOpcUri) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IOpcUri> for super::super::super::System::Com::IUri {
    fn from(value: &IOpcUri) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri> for IOpcUri {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IUri> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IUri> for &IOpcUri {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IUri> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcUri_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: super::super::super::System::Com::Uri_PROPERTY, pfhasproperty: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrabsoluteuri: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrauthority: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdisplaystring: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdomain: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrextension: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfragment: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrhost: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpassword: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathandquery: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrquery: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrawuri: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrschemename: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruserinfo: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwhosttype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwport: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwscheme: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, pfequal: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relationshipparturi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetparturi: ::windows::core::RawPtr, relativeuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeuri: ::windows::core::RawPtr, combineduri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_CANONICALIZATION_METHOD(pub i32);
pub const OPC_CANONICALIZATION_NONE: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(0i32);
pub const OPC_CANONICALIZATION_C14N: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(1i32);
pub const OPC_CANONICALIZATION_C14N_WITH_COMMENTS: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(2i32);
impl ::core::convert::From<i32> for OPC_CANONICALIZATION_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_CANONICALIZATION_METHOD {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_CERTIFICATE_EMBEDDING_OPTION(pub i32);
pub const OPC_CERTIFICATE_IN_CERTIFICATE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(0i32);
pub const OPC_CERTIFICATE_IN_SIGNATURE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(1i32);
pub const OPC_CERTIFICATE_NOT_EMBEDDED: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(2i32);
impl ::core::convert::From<i32> for OPC_CERTIFICATE_EMBEDDING_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_CERTIFICATE_EMBEDDING_OPTION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_COMPRESSION_OPTIONS(pub i32);
pub const OPC_COMPRESSION_NONE: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(-1i32);
pub const OPC_COMPRESSION_NORMAL: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(0i32);
pub const OPC_COMPRESSION_MAXIMUM: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(1i32);
pub const OPC_COMPRESSION_FAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(2i32);
pub const OPC_COMPRESSION_SUPERFAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(3i32);
impl ::core::convert::From<i32> for OPC_COMPRESSION_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_COMPRESSION_OPTIONS {
    type Abi = Self;
}
pub const OPC_E_CONFLICTING_SETTINGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175212i32 as _);
pub const OPC_E_COULD_NOT_RECOVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175154i32 as _);
pub const OPC_E_DS_DEFAULT_DIGEST_METHOD_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175161i32 as _);
pub const OPC_E_DS_DIGEST_VALUE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175206i32 as _);
pub const OPC_E_DS_DUPLICATE_PACKAGE_OBJECT_REFERENCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175187i32 as _);
pub const OPC_E_DS_DUPLICATE_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175205i32 as _);
pub const OPC_E_DS_DUPLICATE_SIGNATURE_PROPERTY_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175192i32 as _);
pub const OPC_E_DS_EXTERNAL_SIGNATURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175202i32 as _);
pub const OPC_E_DS_EXTERNAL_SIGNATURE_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175185i32 as _);
pub const OPC_E_DS_INVALID_CANONICALIZATION_METHOD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175198i32 as _);
pub const OPC_E_DS_INVALID_CERTIFICATE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175203i32 as _);
pub const OPC_E_DS_INVALID_OPC_SIGNATURE_TIME_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175196i32 as _);
pub const OPC_E_DS_INVALID_RELATIONSHIPS_SIGNING_OPTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175197i32 as _);
pub const OPC_E_DS_INVALID_RELATIONSHIP_TRANSFORM_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175199i32 as _);
pub const OPC_E_DS_INVALID_SIGNATURE_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175189i32 as _);
pub const OPC_E_DS_INVALID_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175204i32 as _);
pub const OPC_E_DS_INVALID_SIGNATURE_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175190i32 as _);
pub const OPC_E_DS_MISSING_CANONICALIZATION_TRANSFORM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175182i32 as _);
pub const OPC_E_DS_MISSING_CERTIFICATE_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175146i32 as _);
pub const OPC_E_DS_MISSING_PACKAGE_OBJECT_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175186i32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_ALGORITHM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175188i32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_ORIGIN_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175201i32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175200i32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTIES_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175194i32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTY_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175193i32 as _);
pub const OPC_E_DS_MISSING_SIGNATURE_TIME_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175191i32 as _);
pub const OPC_E_DS_MULTIPLE_RELATIONSHIP_TRANSFORMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175183i32 as _);
pub const OPC_E_DS_PACKAGE_REFERENCE_URI_RESERVED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175195i32 as _);
pub const OPC_E_DS_REFERENCE_MISSING_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175184i32 as _);
pub const OPC_E_DS_SIGNATURE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175207i32 as _);
pub const OPC_E_DS_SIGNATURE_METHOD_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175162i32 as _);
pub const OPC_E_DS_SIGNATURE_ORIGIN_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175148i32 as _);
pub const OPC_E_DS_SIGNATURE_PROPERTY_MISSING_TARGET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175163i32 as _);
pub const OPC_E_DS_SIGNATURE_REFERENCE_MISSING_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175165i32 as _);
pub const OPC_E_DS_UNSIGNED_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175147i32 as _);
pub const OPC_E_DUPLICATE_DEFAULT_EXTENSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175217i32 as _);
pub const OPC_E_DUPLICATE_OVERRIDE_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175219i32 as _);
pub const OPC_E_DUPLICATE_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175221i32 as _);
pub const OPC_E_DUPLICATE_PIECE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175211i32 as _);
pub const OPC_E_DUPLICATE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175213i32 as _);
pub const OPC_E_ENUM_CANNOT_MOVE_NEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175151i32 as _);
pub const OPC_E_ENUM_CANNOT_MOVE_PREVIOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175150i32 as _);
pub const OPC_E_ENUM_COLLECTION_CHANGED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175152i32 as _);
pub const OPC_E_ENUM_INVALID_POSITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175149i32 as _);
pub const OPC_E_INVALID_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175164i32 as _);
pub const OPC_E_INVALID_CONTENT_TYPE_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175226i32 as _);
pub const OPC_E_INVALID_DEFAULT_EXTENSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175218i32 as _);
pub const OPC_E_INVALID_OVERRIDE_PART_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175220i32 as _);
pub const OPC_E_INVALID_PIECE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175210i32 as _);
pub const OPC_E_INVALID_RELATIONSHIP_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175216i32 as _);
pub const OPC_E_INVALID_RELATIONSHIP_TARGET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175214i32 as _);
pub const OPC_E_INVALID_RELATIONSHIP_TARGET_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175155i32 as _);
pub const OPC_E_INVALID_RELATIONSHIP_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175215i32 as _);
pub const OPC_E_INVALID_RELS_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175222i32 as _);
pub const OPC_E_INVALID_XML_ENCODING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175166i32 as _);
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ATTRIBUTES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175157i32 as _);
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ELEMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175156i32 as _);
pub const OPC_E_MC_INCONSISTENT_PROCESS_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175158i32 as _);
pub const OPC_E_MC_INVALID_ATTRIBUTES_ON_IGNORABLE_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175168i32 as _);
pub const OPC_E_MC_INVALID_ENUM_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175172i32 as _);
pub const OPC_E_MC_INVALID_PREFIX_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175177i32 as _);
pub const OPC_E_MC_INVALID_QNAME_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175176i32 as _);
pub const OPC_E_MC_INVALID_XMLNS_ATTRIBUTE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175167i32 as _);
pub const OPC_E_MC_MISSING_CHOICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175173i32 as _);
pub const OPC_E_MC_MISSING_REQUIRES_ATTR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175179i32 as _);
pub const OPC_E_MC_MULTIPLE_FALLBACK_ELEMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175159i32 as _);
pub const OPC_E_MC_NESTED_ALTERNATE_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175175i32 as _);
pub const OPC_E_MC_UNEXPECTED_ATTR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175178i32 as _);
pub const OPC_E_MC_UNEXPECTED_CHOICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175174i32 as _);
pub const OPC_E_MC_UNEXPECTED_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175181i32 as _);
pub const OPC_E_MC_UNEXPECTED_REQUIRES_ATTR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175180i32 as _);
pub const OPC_E_MC_UNKNOWN_NAMESPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175170i32 as _);
pub const OPC_E_MC_UNKNOWN_PREFIX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175169i32 as _);
pub const OPC_E_MISSING_CONTENT_TYPES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175225i32 as _);
pub const OPC_E_MISSING_PIECE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175209i32 as _);
pub const OPC_E_NONCONFORMING_CONTENT_TYPES_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175224i32 as _);
pub const OPC_E_NONCONFORMING_RELS_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175223i32 as _);
pub const OPC_E_NONCONFORMING_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175231i32 as _);
pub const OPC_E_NO_SUCH_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175208i32 as _);
pub const OPC_E_NO_SUCH_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175160i32 as _);
pub const OPC_E_NO_SUCH_SETTINGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175145i32 as _);
pub const OPC_E_PART_CANNOT_BE_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175228i32 as _);
pub const OPC_E_RELATIONSHIP_URI_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175229i32 as _);
pub const OPC_E_RELATIVE_URI_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175230i32 as _);
pub const OPC_E_UNEXPECTED_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175227i32 as _);
pub const OPC_E_UNSUPPORTED_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175153i32 as _);
pub const OPC_E_ZIP_CENTRAL_DIRECTORY_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171127i32 as _);
pub const OPC_E_ZIP_COMMENT_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171124i32 as _);
pub const OPC_E_ZIP_COMPRESSION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171133i32 as _);
pub const OPC_E_ZIP_CORRUPTED_ARCHIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171134i32 as _);
pub const OPC_E_ZIP_DECOMPRESSION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171132i32 as _);
pub const OPC_E_ZIP_DUPLICATE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171125i32 as _);
pub const OPC_E_ZIP_EXTRA_FIELDS_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171123i32 as _);
pub const OPC_E_ZIP_FILE_HEADER_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171122i32 as _);
pub const OPC_E_ZIP_INCONSISTENT_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171130i32 as _);
pub const OPC_E_ZIP_INCONSISTENT_FILEITEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171131i32 as _);
pub const OPC_E_ZIP_INCORRECT_DATA_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171135i32 as _);
pub const OPC_E_ZIP_MISSING_DATA_DESCRIPTOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171129i32 as _);
pub const OPC_E_ZIP_MISSING_END_OF_CENTRAL_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171121i32 as _);
pub const OPC_E_ZIP_NAME_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171126i32 as _);
pub const OPC_E_ZIP_REQUIRES_64_BIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171120i32 as _);
pub const OPC_E_ZIP_UNSUPPORTEDARCHIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171128i32 as _);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_READ_FLAGS(pub u32);
pub const OPC_READ_DEFAULT: OPC_READ_FLAGS = OPC_READ_FLAGS(0u32);
pub const OPC_VALIDATE_ON_LOAD: OPC_READ_FLAGS = OPC_READ_FLAGS(1u32);
pub const OPC_CACHE_ON_ACCESS: OPC_READ_FLAGS = OPC_READ_FLAGS(2u32);
impl ::core::convert::From<u32> for OPC_READ_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_READ_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for OPC_READ_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for OPC_READ_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for OPC_READ_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for OPC_READ_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for OPC_READ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_RELATIONSHIPS_SIGNING_OPTION(pub i32);
pub const OPC_RELATIONSHIP_SIGN_USING_SELECTORS: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(0i32);
pub const OPC_RELATIONSHIP_SIGN_PART: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(1i32);
impl ::core::convert::From<i32> for OPC_RELATIONSHIPS_SIGNING_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_RELATIONSHIPS_SIGNING_OPTION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_RELATIONSHIP_SELECTOR(pub i32);
pub const OPC_RELATIONSHIP_SELECT_BY_ID: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(0i32);
pub const OPC_RELATIONSHIP_SELECT_BY_TYPE: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(1i32);
impl ::core::convert::From<i32> for OPC_RELATIONSHIP_SELECTOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_RELATIONSHIP_SELECTOR {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_SIGNATURE_TIME_FORMAT(pub i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(0i32);
pub const OPC_SIGNATURE_TIME_FORMAT_SECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(1i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MINUTES: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(2i32);
pub const OPC_SIGNATURE_TIME_FORMAT_DAYS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(3i32);
pub const OPC_SIGNATURE_TIME_FORMAT_MONTHS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(4i32);
pub const OPC_SIGNATURE_TIME_FORMAT_YEARS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(5i32);
impl ::core::convert::From<i32> for OPC_SIGNATURE_TIME_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_SIGNATURE_TIME_FORMAT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_SIGNATURE_VALIDATION_RESULT(pub i32);
pub const OPC_SIGNATURE_VALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(0i32);
pub const OPC_SIGNATURE_INVALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(-1i32);
impl ::core::convert::From<i32> for OPC_SIGNATURE_VALIDATION_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_SIGNATURE_VALIDATION_RESULT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_STREAM_IO_MODE(pub i32);
pub const OPC_STREAM_IO_READ: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(1i32);
pub const OPC_STREAM_IO_WRITE: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(2i32);
impl ::core::convert::From<i32> for OPC_STREAM_IO_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_STREAM_IO_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_URI_TARGET_MODE(pub i32);
pub const OPC_URI_TARGET_MODE_INTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(0i32);
pub const OPC_URI_TARGET_MODE_EXTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(1i32);
impl ::core::convert::From<i32> for OPC_URI_TARGET_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_URI_TARGET_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPC_WRITE_FLAGS(pub u32);
pub const OPC_WRITE_DEFAULT: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(0u32);
pub const OPC_WRITE_FORCE_ZIP32: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(1u32);
impl ::core::convert::From<u32> for OPC_WRITE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OPC_WRITE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for OPC_WRITE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for OPC_WRITE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for OPC_WRITE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for OPC_WRITE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for OPC_WRITE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const OpcFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b2d6ba0_9f3e_4f27_920b_313cc426a39e);
