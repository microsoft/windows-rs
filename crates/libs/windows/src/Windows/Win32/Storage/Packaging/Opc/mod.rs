#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcCertificateEnumerator(::windows::core::IUnknown);
impl IOpcCertificateEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MoveNext)(::windows::core::Interface::as_raw(self), hasnext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MovePrevious)(::windows::core::Interface::as_raw(self), hasprevious).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn GetCurrent(&self, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrent)(::windows::core::Interface::as_raw(self), certificate).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcCertificateEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcCertificateEnumerator>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcCertificateEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcCertificateEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcCertificateEnumerator {}
impl ::core::fmt::Debug for IOpcCertificateEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcCertificateEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcCertificateEnumerator {
    type Vtable = IOpcCertificateEnumerator_Vtbl;
}
impl ::core::clone::Clone for IOpcCertificateEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcCertificateEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85131937_8f24_421f_b439_59ab24d140b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcCertificateEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *const *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    GetCurrent: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcCertificateSet(::windows::core::IUnknown);
impl IOpcCertificateSet {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Add(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), certificate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Remove(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), certificate).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcCertificateEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcCertificateEnumerator>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcCertificateSet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcCertificateSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcCertificateSet {}
impl ::core::fmt::Debug for IOpcCertificateSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcCertificateSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcCertificateSet {
    type Vtable = IOpcCertificateSet_Vtbl;
}
impl ::core::clone::Clone for IOpcCertificateSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcCertificateSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56ea4325_8e2d_4167_b1a4_e486d24c8fa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcCertificateSet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Remove: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcDigitalSignature(::windows::core::IUnknown);
impl IOpcDigitalSignature {
    pub unsafe fn GetNamespaces(&self, prefixes: *mut *mut ::windows::core::PWSTR, namespaces: *mut *mut ::windows::core::PWSTR, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNamespaces)(::windows::core::Interface::as_raw(self), prefixes, namespaces, count).ok()
    }
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSignatureId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).GetSignaturePartName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSignatureMethod)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCanonicalizationMethod(&self, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCanonicalizationMethod)(::windows::core::Interface::as_raw(self), canonicalizationmethod).ok()
    }
    pub unsafe fn GetSignatureValue(&self, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSignatureValue)(::windows::core::Interface::as_raw(self), signaturevalue, count).ok()
    }
    pub unsafe fn GetSignaturePartReferenceEnumerator(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignaturePartReferenceEnumerator>();
        (::windows::core::Interface::vtable(self).GetSignaturePartReferenceEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureRelationshipReferenceEnumerator(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureRelationshipReferenceEnumerator>();
        (::windows::core::Interface::vtable(self).GetSignatureRelationshipReferenceEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningTime(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSigningTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTimeFormat(&self, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTimeFormat)(::windows::core::Interface::as_raw(self), timeformat).ok()
    }
    pub unsafe fn GetPackageObjectReference(&self) -> ::windows::core::Result<IOpcSignatureReference> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureReference>();
        (::windows::core::Interface::vtable(self).GetPackageObjectReference)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCertificateEnumerator(&self) -> ::windows::core::Result<IOpcCertificateEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcCertificateEnumerator>();
        (::windows::core::Interface::vtable(self).GetCertificateEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureReferenceEnumerator>();
        (::windows::core::Interface::vtable(self).GetCustomReferenceEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCustomObjectEnumerator(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureCustomObjectEnumerator>();
        (::windows::core::Interface::vtable(self).GetCustomObjectEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSignatureXml)(::windows::core::Interface::as_raw(self), signaturexml, count).ok()
    }
}
::windows::imp::interface_hierarchy!(IOpcDigitalSignature, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcDigitalSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcDigitalSignature {}
impl ::core::fmt::Debug for IOpcDigitalSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcDigitalSignature").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcDigitalSignature {
    type Vtable = IOpcDigitalSignature_Vtbl;
}
impl ::core::clone::Clone for IOpcDigitalSignature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcDigitalSignature {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52ab21dd_1cd0_4949_bc80_0c1232d00cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignature_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefixes: *mut *mut ::windows::core::PWSTR, namespaces: *mut *mut ::windows::core::PWSTR, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignaturePartName: usize,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetCanonicalizationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canonicalizationmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT,
    pub GetSignatureValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturevalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetSignaturePartReferenceEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSignatureRelationshipReferenceEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSigningTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signingtime: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    pub GetPackageObjectReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCertificateEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCustomReferenceEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCustomObjectEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcDigitalSignatureEnumerator(::windows::core::IUnknown);
impl IOpcDigitalSignatureEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MoveNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MovePrevious)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcDigitalSignature> {
        let mut result__ = ::windows::core::zeroed::<IOpcDigitalSignature>();
        (::windows::core::Interface::vtable(self).GetCurrent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcDigitalSignatureEnumerator>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcDigitalSignatureEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcDigitalSignatureEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcDigitalSignatureEnumerator {}
impl ::core::fmt::Debug for IOpcDigitalSignatureEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcDigitalSignatureEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcDigitalSignatureEnumerator {
    type Vtable = IOpcDigitalSignatureEnumerator_Vtbl;
}
impl ::core::clone::Clone for IOpcDigitalSignatureEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcDigitalSignatureEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x967b6882_0ba3_4358_b9e7_b64c75063c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignatureEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcDigitalSignatureManager(::windows::core::IUnknown);
impl IOpcDigitalSignatureManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignatureOriginPartName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).GetSignatureOriginPartName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignatureOriginPartName<P0>(&self, signatureoriginpartname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        (::windows::core::Interface::vtable(self).SetSignatureOriginPartName)(::windows::core::Interface::as_raw(self), signatureoriginpartname.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatureEnumerator(&self) -> ::windows::core::Result<IOpcDigitalSignatureEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcDigitalSignatureEnumerator>();
        (::windows::core::Interface::vtable(self).GetSignatureEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveSignature<P0>(&self, signaturepartname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        (::windows::core::Interface::vtable(self).RemoveSignature)(::windows::core::Interface::as_raw(self), signaturepartname.into_param().abi()).ok()
    }
    pub unsafe fn CreateSigningOptions(&self) -> ::windows::core::Result<IOpcSigningOptions> {
        let mut result__ = ::windows::core::zeroed::<IOpcSigningOptions>();
        (::windows::core::Interface::vtable(self).CreateSigningOptions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Validate<P0>(&self, signature: P0, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcDigitalSignature>,
    {
        (::windows::core::Interface::vtable(self).Validate)(::windows::core::Interface::as_raw(self), signature.into_param().abi(), certificate, validationresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Sign<P0>(&self, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: P0) -> ::windows::core::Result<IOpcDigitalSignature>
    where
        P0: ::windows::core::IntoParam<IOpcSigningOptions>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcDigitalSignature>();
        (::windows::core::Interface::vtable(self).Sign)(::windows::core::Interface::as_raw(self), certificate, signingoptions.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReplaceSignatureXml<P0>(&self, signaturepartname: P0, newsignaturexml: *const u8, count: u32) -> ::windows::core::Result<IOpcDigitalSignature>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcDigitalSignature>();
        (::windows::core::Interface::vtable(self).ReplaceSignatureXml)(::windows::core::Interface::as_raw(self), signaturepartname.into_param().abi(), newsignaturexml, count, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcDigitalSignatureManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcDigitalSignatureManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcDigitalSignatureManager {}
impl ::core::fmt::Debug for IOpcDigitalSignatureManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcDigitalSignatureManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcDigitalSignatureManager {
    type Vtable = IOpcDigitalSignatureManager_Vtbl;
}
impl ::core::clone::Clone for IOpcDigitalSignatureManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcDigitalSignatureManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5e62a0b_696d_462f_94df_72e33cef2659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcDigitalSignatureManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignatureOriginPartName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignatureOriginPartName: usize,
    pub GetSignatureEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveSignature: usize,
    pub CreateSigningOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, validationresult: *mut OPC_SIGNATURE_VALIDATION_RESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Validate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Sign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: *const super::super::super::Security::Cryptography::CERT_CONTEXT, signingoptions: *mut ::core::ffi::c_void, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Sign: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReplaceSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void, newsignaturexml: *const u8, count: u32, digitalsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReplaceSignatureXml: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcFactory(::windows::core::IUnknown);
impl IOpcFactory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageRootUri(&self) -> ::windows::core::Result<IOpcUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcUri>();
        (::windows::core::Interface::vtable(self).CreatePackageRootUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePartUri<P0>(&self, pwzuri: P0) -> ::windows::core::Result<IOpcPartUri>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).CreatePartUri)(::windows::core::Interface::as_raw(self), pwzuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStreamOnFile<P0>(&self, filename: P0, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32) -> ::windows::core::Result<super::super::super::System::Com::IStream>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).CreateStreamOnFile)(::windows::core::Interface::as_raw(self), filename.into_param().abi(), iomode, securityattributes, dwflagsandattributes, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePackage(&self) -> ::windows::core::Result<IOpcPackage> {
        let mut result__ = ::windows::core::zeroed::<IOpcPackage>();
        (::windows::core::Interface::vtable(self).CreatePackage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReadPackageFromStream<P0>(&self, stream: P0, flags: OPC_READ_FLAGS) -> ::windows::core::Result<IOpcPackage>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcPackage>();
        (::windows::core::Interface::vtable(self).ReadPackageFromStream)(::windows::core::Interface::as_raw(self), stream.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WritePackageToStream<P0, P1>(&self, package: P0, flags: OPC_WRITE_FLAGS, stream: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcPackage>,
        P1: ::windows::core::IntoParam<super::super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).WritePackageToStream)(::windows::core::Interface::as_raw(self), package.into_param().abi(), flags, stream.into_param().abi()).ok()
    }
    pub unsafe fn CreateDigitalSignatureManager<P0>(&self, package: P0) -> ::windows::core::Result<IOpcDigitalSignatureManager>
    where
        P0: ::windows::core::IntoParam<IOpcPackage>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcDigitalSignatureManager>();
        (::windows::core::Interface::vtable(self).CreateDigitalSignatureManager)(::windows::core::Interface::as_raw(self), package.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcFactory {}
impl ::core::fmt::Debug for IOpcFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcFactory {
    type Vtable = IOpcFactory_Vtbl;
}
impl ::core::clone::Clone for IOpcFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d0b4446_cd73_4ab3_94f4_8ccdf6116154);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageRootUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rooturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageRootUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzuri: ::windows::core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePartUri: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com"))]
    pub CreateStreamOnFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, iomode: OPC_STREAM_IO_MODE, securityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: u32, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com")))]
    CreateStreamOnFile: usize,
    pub CreatePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReadPackageFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, flags: OPC_READ_FLAGS, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReadPackageFromStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WritePackageToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, flags: OPC_WRITE_FLAGS, stream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WritePackageToStream: usize,
    pub CreateDigitalSignatureManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, signaturemanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcPackage(::windows::core::IUnknown);
impl IOpcPackage {
    pub unsafe fn GetPartSet(&self) -> ::windows::core::Result<IOpcPartSet> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartSet>();
        (::windows::core::Interface::vtable(self).GetPartSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelationshipSet(&self) -> ::windows::core::Result<IOpcRelationshipSet> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipSet>();
        (::windows::core::Interface::vtable(self).GetRelationshipSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcPackage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcPackage {}
impl ::core::fmt::Debug for IOpcPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcPackage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcPackage {
    type Vtable = IOpcPackage_Vtbl;
}
impl ::core::clone::Clone for IOpcPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcPackage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPackage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPartSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRelationshipSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcPart(::windows::core::IUnknown);
impl IOpcPart {
    pub unsafe fn GetRelationshipSet(&self) -> ::windows::core::Result<IOpcRelationshipSet> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipSet>();
        (::windows::core::Interface::vtable(self).GetRelationshipSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetContentStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).GetContentStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContentType(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetContentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCompressionOptions(&self) -> ::windows::core::Result<OPC_COMPRESSION_OPTIONS> {
        let mut result__ = ::windows::core::zeroed::<OPC_COMPRESSION_OPTIONS>();
        (::windows::core::Interface::vtable(self).GetCompressionOptions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcPart, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcPart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcPart {}
impl ::core::fmt::Debug for IOpcPart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcPart").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcPart {
    type Vtable = IOpcPart_Vtbl;
}
impl ::core::clone::Clone for IOpcPart {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcPart {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPart_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRelationshipSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetContentStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetContentStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetName: usize,
    pub GetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetCompressionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compressionoptions: *mut OPC_COMPRESSION_OPTIONS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcPartEnumerator(::windows::core::IUnknown);
impl IOpcPartEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MoveNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MovePrevious)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcPart> {
        let mut result__ = ::windows::core::zeroed::<IOpcPart>();
        (::windows::core::Interface::vtable(self).GetCurrent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcPartEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartEnumerator>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcPartEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcPartEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcPartEnumerator {}
impl ::core::fmt::Debug for IOpcPartEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcPartEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcPartEnumerator {
    type Vtable = IOpcPartEnumerator_Vtbl;
}
impl ::core::clone::Clone for IOpcPartEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcPartEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcPartSet(::windows::core::IUnknown);
impl IOpcPartSet {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPart<P0>(&self, name: P0) -> ::windows::core::Result<IOpcPart>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcPart>();
        (::windows::core::Interface::vtable(self).GetPart)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePart<P0, P1>(&self, name: P0, contenttype: P1, compressionoptions: OPC_COMPRESSION_OPTIONS) -> ::windows::core::Result<IOpcPart>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcPart>();
        (::windows::core::Interface::vtable(self).CreatePart)(::windows::core::Interface::as_raw(self), name.into_param().abi(), contenttype.into_param().abi(), compressionoptions, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeletePart<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        (::windows::core::Interface::vtable(self).DeletePart)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn PartExists<P0>(&self, name: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).PartExists)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcPartEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartEnumerator>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcPartSet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcPartSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcPartSet {}
impl ::core::fmt::Debug for IOpcPartSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcPartSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcPartSet {
    type Vtable = IOpcPartSet_Vtbl;
}
impl ::core::clone::Clone for IOpcPartSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcPartSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartSet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPart: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, contenttype: ::windows::core::PCWSTR, compressionoptions: OPC_COMPRESSION_OPTIONS, part: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePart: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeletePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeletePart: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PartExists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, partexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PartExists: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IOpcPartUri(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IOpcPartUri {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut ::windows::core::BSTR, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPropertyBSTR)(::windows::core::Interface::as_raw(self), uriprop, ::core::mem::transmute(pbstrproperty), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyLength(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPropertyLength)(::windows::core::Interface::as_raw(self), uriprop, pcchproperty, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPropertyDWORD)(::windows::core::Interface::as_raw(self), uriprop, pdwproperty, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HasProperty(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.HasProperty)(::windows::core::Interface::as_raw(self), uriprop, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetAbsoluteUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAuthority(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetAuthority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDisplayUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDomain(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDomain)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtension(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetExtension)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFragment(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetFragment)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHost(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetHost)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPassword(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetPassword)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetPathAndQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetQuery(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRawUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetRawUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSchemeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSchemeName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUserInfo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetUserInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetUserName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHostType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetHostType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetPort)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetScheme(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetScheme)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetZone(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetZone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetProperties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsEqual<P0>(&self, puri: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.IsEqual)(::windows::core::Interface::as_raw(self), puri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsPartUri(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).base__.GetRelationshipsPartUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelativeUri<P0>(&self, targetparturi: P0) -> ::windows::core::Result<super::super::super::System::Com::IUri>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IUri>();
        (::windows::core::Interface::vtable(self).base__.GetRelativeUri)(::windows::core::Interface::as_raw(self), targetparturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CombinePartUri<P0>(&self, relativeuri: P0) -> ::windows::core::Result<IOpcPartUri>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).base__.CombinePartUri)(::windows::core::Interface::as_raw(self), relativeuri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ComparePartUri<P0>(&self, parturi: P0) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).ComparePartUri)(::windows::core::Interface::as_raw(self), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> ::windows::core::Result<IOpcUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcUri>();
        (::windows::core::Interface::vtable(self).GetSourceUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRelationshipsPartUri(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsRelationshipsPartUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IOpcPartUri, ::windows::core::IUnknown, super::super::super::System::Com::IUri, IOpcUri);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IOpcPartUri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IOpcPartUri {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IOpcPartUri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcPartUri").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IOpcPartUri {
    type Vtable = IOpcPartUri_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IOpcPartUri {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IOpcPartUri {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d3babe7_88b2_46ba_85cb_4203cb016c87);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IOpcPartUri_Vtbl {
    pub base__: IOpcUri_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ComparePartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, comparisonresult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ComparePartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRelationshipsPartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isrelationshipuri: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRelationshipsPartUri: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcRelationship(::windows::core::IUnknown);
impl IOpcRelationship {
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelationshipType(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetRelationshipType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> ::windows::core::Result<IOpcUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcUri>();
        (::windows::core::Interface::vtable(self).GetSourceUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTargetUri(&self) -> ::windows::core::Result<super::super::super::System::Com::IUri> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IUri>();
        (::windows::core::Interface::vtable(self).GetTargetUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTargetMode(&self) -> ::windows::core::Result<OPC_URI_TARGET_MODE> {
        let mut result__ = ::windows::core::zeroed::<OPC_URI_TARGET_MODE>();
        (::windows::core::Interface::vtable(self).GetTargetMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcRelationship, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcRelationship {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcRelationship {}
impl ::core::fmt::Debug for IOpcRelationship {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcRelationship").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationship {
    type Vtable = IOpcRelationship_Vtbl;
}
impl ::core::clone::Clone for IOpcRelationship {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcRelationship {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationship_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetRelationshipType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshiptype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTargetUri: usize,
    pub GetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: *mut OPC_URI_TARGET_MODE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcRelationshipEnumerator(::windows::core::IUnknown);
impl IOpcRelationshipEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MoveNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MovePrevious)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcRelationship> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationship>();
        (::windows::core::Interface::vtable(self).GetCurrent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcRelationshipEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipEnumerator>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcRelationshipEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcRelationshipEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcRelationshipEnumerator {}
impl ::core::fmt::Debug for IOpcRelationshipEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcRelationshipEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipEnumerator {
    type Vtable = IOpcRelationshipEnumerator_Vtbl;
}
impl ::core::clone::Clone for IOpcRelationshipEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcRelationshipEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationship: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcRelationshipSelector(::windows::core::IUnknown);
impl IOpcRelationshipSelector {
    pub unsafe fn GetSelectorType(&self) -> ::windows::core::Result<OPC_RELATIONSHIP_SELECTOR> {
        let mut result__ = ::windows::core::zeroed::<OPC_RELATIONSHIP_SELECTOR>();
        (::windows::core::Interface::vtable(self).GetSelectorType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSelectionCriterion(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSelectionCriterion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcRelationshipSelector, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcRelationshipSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcRelationshipSelector {}
impl ::core::fmt::Debug for IOpcRelationshipSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcRelationshipSelector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipSelector {
    type Vtable = IOpcRelationshipSelector_Vtbl;
}
impl ::core::clone::Clone for IOpcRelationshipSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcRelationshipSelector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8f26c7f_b28f_4899_84c8_5d5639ede75f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelector_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSelectorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selector: *mut OPC_RELATIONSHIP_SELECTOR) -> ::windows::core::HRESULT,
    pub GetSelectionCriterion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectioncriterion: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcRelationshipSelectorEnumerator(::windows::core::IUnknown);
impl IOpcRelationshipSelectorEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MoveNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MovePrevious)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcRelationshipSelector> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipSelector>();
        (::windows::core::Interface::vtable(self).GetCurrent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipSelectorEnumerator>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcRelationshipSelectorEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcRelationshipSelectorEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcRelationshipSelectorEnumerator {}
impl ::core::fmt::Debug for IOpcRelationshipSelectorEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcRelationshipSelectorEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipSelectorEnumerator {
    type Vtable = IOpcRelationshipSelectorEnumerator_Vtbl;
}
impl ::core::clone::Clone for IOpcRelationshipSelectorEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcRelationshipSelectorEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e50a181_a91b_48ac_88d2_bca3d8f8c0b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcRelationshipSelectorSet(::windows::core::IUnknown);
impl IOpcRelationshipSelectorSet {
    pub unsafe fn Create<P0>(&self, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: P0) -> ::windows::core::Result<IOpcRelationshipSelector>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipSelector>();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), selector, selectioncriterion.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, relationshipselector: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcRelationshipSelector>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), relationshipselector.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipSelectorEnumerator>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcRelationshipSelectorSet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcRelationshipSelectorSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcRelationshipSelectorSet {}
impl ::core::fmt::Debug for IOpcRelationshipSelectorSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcRelationshipSelectorSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipSelectorSet {
    type Vtable = IOpcRelationshipSelectorSet_Vtbl;
}
impl ::core::clone::Clone for IOpcRelationshipSelectorSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcRelationshipSelectorSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e34c269_a4d3_47c0_b5c4_87ff2b3b6136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSelectorSet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selector: OPC_RELATIONSHIP_SELECTOR, selectioncriterion: ::windows::core::PCWSTR, relationshipselector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipselector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipselectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcRelationshipSet(::windows::core::IUnknown);
impl IOpcRelationshipSet {
    pub unsafe fn GetRelationship<P0>(&self, relationshipidentifier: P0) -> ::windows::core::Result<IOpcRelationship>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationship>();
        (::windows::core::Interface::vtable(self).GetRelationship)(::windows::core::Interface::as_raw(self), relationshipidentifier.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRelationship<P0, P1, P2>(&self, relationshipidentifier: P0, relationshiptype: P1, targeturi: P2, targetmode: OPC_URI_TARGET_MODE) -> ::windows::core::Result<IOpcRelationship>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationship>();
        (::windows::core::Interface::vtable(self).CreateRelationship)(::windows::core::Interface::as_raw(self), relationshipidentifier.into_param().abi(), relationshiptype.into_param().abi(), targeturi.into_param().abi(), targetmode, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteRelationship<P0>(&self, relationshipidentifier: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteRelationship)(::windows::core::Interface::as_raw(self), relationshipidentifier.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelationshipExists<P0>(&self, relationshipidentifier: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).RelationshipExists)(::windows::core::Interface::as_raw(self), relationshipidentifier.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcRelationshipEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipEnumerator>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnumeratorForType<P0>(&self, relationshiptype: P0) -> ::windows::core::Result<IOpcRelationshipEnumerator>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipEnumerator>();
        (::windows::core::Interface::vtable(self).GetEnumeratorForType)(::windows::core::Interface::as_raw(self), relationshiptype.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsContentStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).GetRelationshipsContentStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcRelationshipSet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcRelationshipSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcRelationshipSet {}
impl ::core::fmt::Debug for IOpcRelationshipSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcRelationshipSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcRelationshipSet {
    type Vtable = IOpcRelationshipSet_Vtbl;
}
impl ::core::clone::Clone for IOpcRelationshipSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcRelationshipSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42195949_3b79_4fc8_89c6_fc7fb979ee74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcRelationshipSet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows::core::PCWSTR, relationship: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows::core::PCWSTR, relationshiptype: ::windows::core::PCWSTR, targeturi: *mut ::core::ffi::c_void, targetmode: OPC_URI_TARGET_MODE, relationship: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRelationship: usize,
    pub DeleteRelationship: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RelationshipExists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipidentifier: ::windows::core::PCWSTR, relationshipexists: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RelationshipExists: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEnumeratorForType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshiptype: ::windows::core::PCWSTR, relationshipenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelationshipsContentStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contents: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelationshipsContentStream: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignatureCustomObject(::windows::core::IUnknown);
impl IOpcSignatureCustomObject {
    pub unsafe fn GetXml(&self, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetXml)(::windows::core::Interface::as_raw(self), xmlmarkup, count).ok()
    }
}
::windows::imp::interface_hierarchy!(IOpcSignatureCustomObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignatureCustomObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignatureCustomObject {}
impl ::core::fmt::Debug for IOpcSignatureCustomObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignatureCustomObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureCustomObject {
    type Vtable = IOpcSignatureCustomObject_Vtbl;
}
impl ::core::clone::Clone for IOpcSignatureCustomObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignatureCustomObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d77a19e_62c1_44e7_becd_45da5ae51a56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmlmarkup: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignatureCustomObjectEnumerator(::windows::core::IUnknown);
impl IOpcSignatureCustomObjectEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MoveNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MovePrevious)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignatureCustomObject> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureCustomObject>();
        (::windows::core::Interface::vtable(self).GetCurrent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureCustomObjectEnumerator>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignatureCustomObjectEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignatureCustomObjectEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignatureCustomObjectEnumerator {}
impl ::core::fmt::Debug for IOpcSignatureCustomObjectEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignatureCustomObjectEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureCustomObjectEnumerator {
    type Vtable = IOpcSignatureCustomObjectEnumerator_Vtbl;
}
impl ::core::clone::Clone for IOpcSignatureCustomObjectEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignatureCustomObjectEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ee4fe1d_e1b0_4683_8079_7ea0fcf80b4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignatureCustomObjectSet(::windows::core::IUnknown);
impl IOpcSignatureCustomObjectSet {
    pub unsafe fn Create(&self, xmlmarkup: &[u8]) -> ::windows::core::Result<IOpcSignatureCustomObject> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureCustomObject>();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xmlmarkup.as_ptr()), xmlmarkup.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, customobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcSignatureCustomObject>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), customobject.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureCustomObjectEnumerator>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignatureCustomObjectSet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignatureCustomObjectSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignatureCustomObjectSet {}
impl ::core::fmt::Debug for IOpcSignatureCustomObjectSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignatureCustomObjectSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureCustomObjectSet {
    type Vtable = IOpcSignatureCustomObjectSet_Vtbl;
}
impl ::core::clone::Clone for IOpcSignatureCustomObjectSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignatureCustomObjectSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f792ac5_7947_4e11_bc3d_2659ff046ae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureCustomObjectSet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xmlmarkup: *const u8, count: u32, customobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignaturePartReference(::windows::core::IUnknown);
impl IOpcSignaturePartReference {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).GetPartName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContentType(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetContentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDigestMethod)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDigestValue)(::windows::core::Interface::as_raw(self), digestvalue, count).ok()
    }
    pub unsafe fn GetTransformMethod(&self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = ::windows::core::zeroed::<OPC_CANONICALIZATION_METHOD>();
        (::windows::core::Interface::vtable(self).GetTransformMethod)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignaturePartReference, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignaturePartReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignaturePartReference {}
impl ::core::fmt::Debug for IOpcSignaturePartReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignaturePartReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignaturePartReference {
    type Vtable = IOpcSignaturePartReference_Vtbl;
}
impl ::core::clone::Clone for IOpcSignaturePartReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignaturePartReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe24231ca_59f4_484e_b64b_36eeda36072c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReference_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPartName: usize,
    pub GetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignaturePartReferenceEnumerator(::windows::core::IUnknown);
impl IOpcSignaturePartReferenceEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MoveNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MovePrevious)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignaturePartReference> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignaturePartReference>();
        (::windows::core::Interface::vtable(self).GetCurrent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignaturePartReferenceEnumerator>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignaturePartReferenceEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignaturePartReferenceEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignaturePartReferenceEnumerator {}
impl ::core::fmt::Debug for IOpcSignaturePartReferenceEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignaturePartReferenceEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignaturePartReferenceEnumerator {
    type Vtable = IOpcSignaturePartReferenceEnumerator_Vtbl;
}
impl ::core::clone::Clone for IOpcSignaturePartReferenceEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignaturePartReferenceEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80eb1561_8c77_49cf_8266_459b356ee99a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignaturePartReferenceSet(::windows::core::IUnknown);
impl IOpcSignaturePartReferenceSet {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1>(&self, parturi: P0, digestmethod: P1, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignaturePartReference>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcSignaturePartReference>();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), parturi.into_param().abi(), digestmethod.into_param().abi(), transformmethod, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, partreference: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcSignaturePartReference>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), partreference.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignaturePartReferenceEnumerator>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignaturePartReferenceSet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignaturePartReferenceSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignaturePartReferenceSet {}
impl ::core::fmt::Debug for IOpcSignaturePartReferenceSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignaturePartReferenceSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignaturePartReferenceSet {
    type Vtable = IOpcSignaturePartReferenceSet_Vtbl;
}
impl ::core::clone::Clone for IOpcSignaturePartReferenceSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignaturePartReferenceSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c9fe28c_ecd9_4b22_9d36_7fdde670fec0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignaturePartReferenceSet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, digestmethod: ::windows::core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, partreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignatureReference(::windows::core::IUnknown);
impl IOpcSignatureReference {
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUri(&self) -> ::windows::core::Result<super::super::super::System::Com::IUri> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IUri>();
        (::windows::core::Interface::vtable(self).GetUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformMethod(&self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = ::windows::core::zeroed::<OPC_CANONICALIZATION_METHOD>();
        (::windows::core::Interface::vtable(self).GetTransformMethod)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDigestMethod)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDigestValue)(::windows::core::Interface::as_raw(self), digestvalue, count).ok()
    }
}
::windows::imp::interface_hierarchy!(IOpcSignatureReference, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignatureReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignatureReference {}
impl ::core::fmt::Debug for IOpcSignatureReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignatureReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureReference {
    type Vtable = IOpcSignatureReference_Vtbl;
}
impl ::core::clone::Clone for IOpcSignatureReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignatureReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b47005e_3011_4edc_be6f_0f65e5ab0342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReference_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referenceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referenceuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUri: usize,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignatureReferenceEnumerator(::windows::core::IUnknown);
impl IOpcSignatureReferenceEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MoveNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MovePrevious)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignatureReference> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureReference>();
        (::windows::core::Interface::vtable(self).GetCurrent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureReferenceEnumerator>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignatureReferenceEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignatureReferenceEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignatureReferenceEnumerator {}
impl ::core::fmt::Debug for IOpcSignatureReferenceEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignatureReferenceEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureReferenceEnumerator {
    type Vtable = IOpcSignatureReferenceEnumerator_Vtbl;
}
impl ::core::clone::Clone for IOpcSignatureReferenceEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignatureReferenceEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfa59a45_28b1_4868_969e_fa8097fdc12a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignatureReferenceSet(::windows::core::IUnknown);
impl IOpcSignatureReferenceSet {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1, P2, P3>(&self, referenceuri: P0, referenceid: P1, r#type: P2, digestmethod: P3, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignatureReference>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IUri>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureReference>();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), referenceuri.into_param().abi(), referenceid.into_param().abi(), r#type.into_param().abi(), digestmethod.into_param().abi(), transformmethod, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, reference: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcSignatureReference>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), reference.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignatureReferenceEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureReferenceEnumerator>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignatureReferenceSet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignatureReferenceSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignatureReferenceSet {}
impl ::core::fmt::Debug for IOpcSignatureReferenceSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignatureReferenceSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureReferenceSet {
    type Vtable = IOpcSignatureReferenceSet_Vtbl;
}
impl ::core::clone::Clone for IOpcSignatureReferenceSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignatureReferenceSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b02d31_ab12_42dd_9e2f_2b16761c3c1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureReferenceSet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referenceuri: *mut ::core::ffi::c_void, referenceid: ::windows::core::PCWSTR, r#type: ::windows::core::PCWSTR, digestmethod: ::windows::core::PCWSTR, transformmethod: OPC_CANONICALIZATION_METHOD, reference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignatureRelationshipReference(::windows::core::IUnknown);
impl IOpcSignatureRelationshipReference {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceUri(&self) -> ::windows::core::Result<IOpcUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcUri>();
        (::windows::core::Interface::vtable(self).GetSourceUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDigestMethod)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDigestValue(&self, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDigestValue)(::windows::core::Interface::as_raw(self), digestvalue, count).ok()
    }
    pub unsafe fn GetTransformMethod(&self) -> ::windows::core::Result<OPC_CANONICALIZATION_METHOD> {
        let mut result__ = ::windows::core::zeroed::<OPC_CANONICALIZATION_METHOD>();
        (::windows::core::Interface::vtable(self).GetTransformMethod)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelationshipSigningOption(&self) -> ::windows::core::Result<OPC_RELATIONSHIPS_SIGNING_OPTION> {
        let mut result__ = ::windows::core::zeroed::<OPC_RELATIONSHIPS_SIGNING_OPTION>();
        (::windows::core::Interface::vtable(self).GetRelationshipSigningOption)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelationshipSelectorEnumerator(&self) -> ::windows::core::Result<IOpcRelationshipSelectorEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipSelectorEnumerator>();
        (::windows::core::Interface::vtable(self).GetRelationshipSelectorEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignatureRelationshipReference, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignatureRelationshipReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignatureRelationshipReference {}
impl ::core::fmt::Debug for IOpcSignatureRelationshipReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignatureRelationshipReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureRelationshipReference {
    type Vtable = IOpcSignatureRelationshipReference_Vtbl;
}
impl ::core::clone::Clone for IOpcSignatureRelationshipReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignatureRelationshipReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57babac6_9d4a_4e50_8b86_e5d4051eae7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReference_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceUri: usize,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDigestValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetTransformMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmethod: *mut OPC_CANONICALIZATION_METHOD) -> ::windows::core::HRESULT,
    pub GetRelationshipSigningOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipsigningoption: *mut OPC_RELATIONSHIPS_SIGNING_OPTION) -> ::windows::core::HRESULT,
    pub GetRelationshipSelectorEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectorenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignatureRelationshipReferenceEnumerator(::windows::core::IUnknown);
impl IOpcSignatureRelationshipReferenceEnumerator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MoveNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MovePrevious(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MovePrevious)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReference> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureRelationshipReference>();
        (::windows::core::Interface::vtable(self).GetCurrent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureRelationshipReferenceEnumerator>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignatureRelationshipReferenceEnumerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignatureRelationshipReferenceEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignatureRelationshipReferenceEnumerator {}
impl ::core::fmt::Debug for IOpcSignatureRelationshipReferenceEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignatureRelationshipReferenceEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureRelationshipReferenceEnumerator {
    type Vtable = IOpcSignatureRelationshipReferenceEnumerator_Vtbl;
}
impl ::core::clone::Clone for IOpcSignatureRelationshipReferenceEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignatureRelationshipReferenceEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x773ba3e4_f021_48e4_aa04_9816db5d3495);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReferenceEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasprevious: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MovePrevious: usize,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSignatureRelationshipReferenceSet(::windows::core::IUnknown);
impl IOpcSignatureRelationshipReferenceSet {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1, P2>(&self, sourceuri: P0, digestmethod: P1, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: P2, transformmethod: OPC_CANONICALIZATION_METHOD) -> ::windows::core::Result<IOpcSignatureRelationshipReference>
    where
        P0: ::windows::core::IntoParam<IOpcUri>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IOpcRelationshipSelectorSet>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureRelationshipReference>();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), sourceuri.into_param().abi(), digestmethod.into_param().abi(), relationshipsigningoption, selectorset.into_param().abi(), transformmethod, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRelationshipSelectorSet(&self) -> ::windows::core::Result<IOpcRelationshipSelectorSet> {
        let mut result__ = ::windows::core::zeroed::<IOpcRelationshipSelectorSet>();
        (::windows::core::Interface::vtable(self).CreateRelationshipSelectorSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, relationshipreference: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcSignatureRelationshipReference>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), relationshipreference.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceEnumerator> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureRelationshipReferenceEnumerator>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IOpcSignatureRelationshipReferenceSet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSignatureRelationshipReferenceSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSignatureRelationshipReferenceSet {}
impl ::core::fmt::Debug for IOpcSignatureRelationshipReferenceSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSignatureRelationshipReferenceSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSignatureRelationshipReferenceSet {
    type Vtable = IOpcSignatureRelationshipReferenceSet_Vtbl;
}
impl ::core::clone::Clone for IOpcSignatureRelationshipReferenceSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSignatureRelationshipReferenceSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f863ca5_3631_404c_828d_807e0715069b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSignatureRelationshipReferenceSet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceuri: *mut ::core::ffi::c_void, digestmethod: ::windows::core::PCWSTR, relationshipsigningoption: OPC_RELATIONSHIPS_SIGNING_OPTION, selectorset: *mut ::core::ffi::c_void, transformmethod: OPC_CANONICALIZATION_METHOD, relationshipreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub CreateRelationshipSelectorSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectorset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
pub struct IOpcSigningOptions(::windows::core::IUnknown);
impl IOpcSigningOptions {
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSignatureId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSignatureId<P0>(&self, signatureid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetSignatureId)(::windows::core::Interface::as_raw(self), signatureid.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSignatureMethod)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSignatureMethod<P0>(&self, signaturemethod: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetSignatureMethod)(::windows::core::Interface::as_raw(self), signaturemethod.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDefaultDigestMethod)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultDigestMethod<P0>(&self, digestmethod: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDefaultDigestMethod)(::windows::core::Interface::as_raw(self), digestmethod.into_param().abi()).ok()
    }
    pub unsafe fn GetCertificateEmbeddingOption(&self) -> ::windows::core::Result<OPC_CERTIFICATE_EMBEDDING_OPTION> {
        let mut result__ = ::windows::core::zeroed::<OPC_CERTIFICATE_EMBEDDING_OPTION>();
        (::windows::core::Interface::vtable(self).GetCertificateEmbeddingOption)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCertificateEmbeddingOption(&self, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCertificateEmbeddingOption)(::windows::core::Interface::as_raw(self), embeddingoption).ok()
    }
    pub unsafe fn GetTimeFormat(&self) -> ::windows::core::Result<OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = ::windows::core::zeroed::<OPC_SIGNATURE_TIME_FORMAT>();
        (::windows::core::Interface::vtable(self).GetTimeFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTimeFormat(&self, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimeFormat)(::windows::core::Interface::as_raw(self), timeformat).ok()
    }
    pub unsafe fn GetSignaturePartReferenceSet(&self) -> ::windows::core::Result<IOpcSignaturePartReferenceSet> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignaturePartReferenceSet>();
        (::windows::core::Interface::vtable(self).GetSignaturePartReferenceSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureRelationshipReferenceSet(&self) -> ::windows::core::Result<IOpcSignatureRelationshipReferenceSet> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureRelationshipReferenceSet>();
        (::windows::core::Interface::vtable(self).GetSignatureRelationshipReferenceSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCustomObjectSet(&self) -> ::windows::core::Result<IOpcSignatureCustomObjectSet> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureCustomObjectSet>();
        (::windows::core::Interface::vtable(self).GetCustomObjectSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCustomReferenceSet(&self) -> ::windows::core::Result<IOpcSignatureReferenceSet> {
        let mut result__ = ::windows::core::zeroed::<IOpcSignatureReferenceSet>();
        (::windows::core::Interface::vtable(self).GetCustomReferenceSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCertificateSet(&self) -> ::windows::core::Result<IOpcCertificateSet> {
        let mut result__ = ::windows::core::zeroed::<IOpcCertificateSet>();
        (::windows::core::Interface::vtable(self).GetCertificateSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).GetSignaturePartName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignaturePartName<P0>(&self, signaturepartname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        (::windows::core::Interface::vtable(self).SetSignaturePartName)(::windows::core::Interface::as_raw(self), signaturepartname.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IOpcSigningOptions, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IOpcSigningOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpcSigningOptions {}
impl ::core::fmt::Debug for IOpcSigningOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcSigningOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOpcSigningOptions {
    type Vtable = IOpcSigningOptions_Vtbl;
}
impl ::core::clone::Clone for IOpcSigningOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOpcSigningOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d2d6a5_7aeb_46c0_b241_43ab0e9b407e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpcSigningOptions_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDefaultDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDefaultDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetCertificateEmbeddingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, embeddingoption: *mut OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT,
    pub SetCertificateEmbeddingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, embeddingoption: OPC_CERTIFICATE_EMBEDDING_OPTION) -> ::windows::core::HRESULT,
    pub GetTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    pub SetTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    pub GetSignaturePartReferenceSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSignatureRelationshipReferenceSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCustomObjectSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCustomReferenceSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCertificateSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignaturePartName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignaturePartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IOpcUri(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IOpcUri {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pbstrproperty: *mut ::windows::core::BSTR, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyBSTR)(::windows::core::Interface::as_raw(self), uriprop, ::core::mem::transmute(pbstrproperty), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyLength(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyLength)(::windows::core::Interface::as_raw(self), uriprop, pcchproperty, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyDWORD)(::windows::core::Interface::as_raw(self), uriprop, pdwproperty, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn HasProperty(&self, uriprop: super::super::super::System::Com::Uri_PROPERTY) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.HasProperty)(::windows::core::Interface::as_raw(self), uriprop, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetAbsoluteUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAuthority(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetAuthority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDomain(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDomain)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtension(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetExtension)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFragment(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetFragment)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHost(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetHost)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPassword(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetPassword)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetPathAndQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetQuery(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRawUri(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetRawUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSchemeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetSchemeName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUserInfo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetUserInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetUserName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHostType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetHostType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetPort)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetScheme(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetScheme)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetZone(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetZone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetProperties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsEqual<P0>(&self, puri: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsEqual)(::windows::core::Interface::as_raw(self), puri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelationshipsPartUri(&self) -> ::windows::core::Result<IOpcPartUri> {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).GetRelationshipsPartUri)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRelativeUri<P0>(&self, targetparturi: P0) -> ::windows::core::Result<super::super::super::System::Com::IUri>
    where
        P0: ::windows::core::IntoParam<IOpcPartUri>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::System::Com::IUri>();
        (::windows::core::Interface::vtable(self).GetRelativeUri)(::windows::core::Interface::as_raw(self), targetparturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CombinePartUri<P0>(&self, relativeuri: P0) -> ::windows::core::Result<IOpcPartUri>
    where
        P0: ::windows::core::IntoParam<super::super::super::System::Com::IUri>,
    {
        let mut result__ = ::windows::core::zeroed::<IOpcPartUri>();
        (::windows::core::Interface::vtable(self).CombinePartUri)(::windows::core::Interface::as_raw(self), relativeuri.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IOpcUri, ::windows::core::IUnknown, super::super::super::System::Com::IUri);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IOpcUri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IOpcUri {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IOpcUri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpcUri").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IOpcUri {
    type Vtable = IOpcUri_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IOpcUri {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IOpcUri {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc9c1b9b_d62c_49eb_aef0_3b4e0b28ebed);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IOpcUri_Vtbl {
    pub base__: super::super::super::System::Com::IUri_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelationshipsPartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relationshipparturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelationshipsPartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRelativeUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetparturi: *mut ::core::ffi::c_void, relativeuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRelativeUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CombinePartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeuri: *mut ::core::ffi::c_void, combineduri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CombinePartUri: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_CONFLICTING_SETTINGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175212i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_COULD_NOT_RECOVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175154i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DEFAULT_DIGEST_METHOD_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175161i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DIGEST_VALUE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175206i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DUPLICATE_PACKAGE_OBJECT_REFERENCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175187i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DUPLICATE_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175205i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_DUPLICATE_SIGNATURE_PROPERTY_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175192i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_EXTERNAL_SIGNATURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175202i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_EXTERNAL_SIGNATURE_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175185i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_CANONICALIZATION_METHOD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175198i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_CERTIFICATE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175203i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_OPC_SIGNATURE_TIME_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175196i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_RELATIONSHIPS_SIGNING_OPTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175197i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_RELATIONSHIP_TRANSFORM_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175199i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_SIGNATURE_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175189i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_SIGNATURE_ORIGIN_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175204i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_INVALID_SIGNATURE_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175190i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_CANONICALIZATION_TRANSFORM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175182i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_CERTIFICATE_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175146i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_PACKAGE_OBJECT_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175186i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_ALGORITHM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175188i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_ORIGIN_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175201i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175200i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTIES_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175194i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTY_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175193i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MISSING_SIGNATURE_TIME_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175191i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_MULTIPLE_RELATIONSHIP_TRANSFORMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175183i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_PACKAGE_REFERENCE_URI_RESERVED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175195i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_REFERENCE_MISSING_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175184i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175207i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_METHOD_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175162i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_ORIGIN_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175148i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_PROPERTY_MISSING_TARGET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175163i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_SIGNATURE_REFERENCE_MISSING_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175165i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DS_UNSIGNED_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175147i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_DEFAULT_EXTENSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175217i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_OVERRIDE_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175219i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175221i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_PIECE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175211i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_DUPLICATE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175213i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ENUM_CANNOT_MOVE_NEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175151i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ENUM_CANNOT_MOVE_PREVIOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175150i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ENUM_COLLECTION_CHANGED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175152i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ENUM_INVALID_POSITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175149i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175164i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_CONTENT_TYPE_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175226i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_DEFAULT_EXTENSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175218i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_OVERRIDE_PART_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175220i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_PIECE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175210i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELATIONSHIP_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175216i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELATIONSHIP_TARGET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175214i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELATIONSHIP_TARGET_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175155i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELATIONSHIP_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175215i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_RELS_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175222i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_INVALID_XML_ENCODING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175166i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ATTRIBUTES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175157i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ELEMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175156i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INCONSISTENT_PROCESS_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175158i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_ATTRIBUTES_ON_IGNORABLE_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175168i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_ENUM_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175172i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_PREFIX_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175177i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_QNAME_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175176i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_INVALID_XMLNS_ATTRIBUTE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175167i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_MISSING_CHOICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175173i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_MISSING_REQUIRES_ATTR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175179i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_MULTIPLE_FALLBACK_ELEMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175159i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_NESTED_ALTERNATE_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175175i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNEXPECTED_ATTR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175178i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNEXPECTED_CHOICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175174i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNEXPECTED_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175181i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNEXPECTED_REQUIRES_ATTR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175180i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNKNOWN_NAMESPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175170i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MC_UNKNOWN_PREFIX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175169i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MISSING_CONTENT_TYPES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175225i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_MISSING_PIECE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175209i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NONCONFORMING_CONTENT_TYPES_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175224i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NONCONFORMING_RELS_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175223i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NONCONFORMING_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175231i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NO_SUCH_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175208i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NO_SUCH_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175160i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_NO_SUCH_SETTINGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175145i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_PART_CANNOT_BE_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175228i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_RELATIONSHIP_URI_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175229i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_RELATIVE_URI_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175230i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_UNEXPECTED_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175227i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_UNSUPPORTED_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142175153i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_CENTRAL_DIRECTORY_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171127i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_COMMENT_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171124i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_COMPRESSION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171133i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_CORRUPTED_ARCHIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171134i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_DECOMPRESSION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171132i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_DUPLICATE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171125i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_EXTRA_FIELDS_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171123i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_FILE_HEADER_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171122i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_INCONSISTENT_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171130i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_INCONSISTENT_FILEITEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171131i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_INCORRECT_DATA_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171135i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_MISSING_DATA_DESCRIPTOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171129i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_MISSING_END_OF_CENTRAL_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171121i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_NAME_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171126i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_REQUIRES_64_BIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171120i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_E_ZIP_UNSUPPORTEDARCHIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142171128i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OpcFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b2d6ba0_9f3e_4f27_920b_313cc426a39e);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_CANONICALIZATION_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CANONICALIZATION_NONE: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CANONICALIZATION_C14N: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CANONICALIZATION_C14N_WITH_COMMENTS: OPC_CANONICALIZATION_METHOD = OPC_CANONICALIZATION_METHOD(2i32);
impl ::core::marker::Copy for OPC_CANONICALIZATION_METHOD {}
impl ::core::clone::Clone for OPC_CANONICALIZATION_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_CANONICALIZATION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_CANONICALIZATION_METHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_CANONICALIZATION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_CANONICALIZATION_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_CERTIFICATE_EMBEDDING_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CERTIFICATE_IN_CERTIFICATE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CERTIFICATE_IN_SIGNATURE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CERTIFICATE_NOT_EMBEDDED: OPC_CERTIFICATE_EMBEDDING_OPTION = OPC_CERTIFICATE_EMBEDDING_OPTION(2i32);
impl ::core::marker::Copy for OPC_CERTIFICATE_EMBEDDING_OPTION {}
impl ::core::clone::Clone for OPC_CERTIFICATE_EMBEDDING_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_CERTIFICATE_EMBEDDING_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_CERTIFICATE_EMBEDDING_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_CERTIFICATE_EMBEDDING_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_CERTIFICATE_EMBEDDING_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_COMPRESSION_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_NONE: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(-1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_NORMAL: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_MAXIMUM: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_FAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_COMPRESSION_SUPERFAST: OPC_COMPRESSION_OPTIONS = OPC_COMPRESSION_OPTIONS(3i32);
impl ::core::marker::Copy for OPC_COMPRESSION_OPTIONS {}
impl ::core::clone::Clone for OPC_COMPRESSION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_COMPRESSION_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_COMPRESSION_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_COMPRESSION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_COMPRESSION_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_READ_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_READ_DEFAULT: OPC_READ_FLAGS = OPC_READ_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_VALIDATE_ON_LOAD: OPC_READ_FLAGS = OPC_READ_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_CACHE_ON_ACCESS: OPC_READ_FLAGS = OPC_READ_FLAGS(2i32);
impl ::core::marker::Copy for OPC_READ_FLAGS {}
impl ::core::clone::Clone for OPC_READ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_READ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_READ_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_READ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_READ_FLAGS").field(&self.0).finish()
    }
}
impl OPC_READ_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for OPC_READ_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPC_READ_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPC_READ_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPC_READ_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPC_READ_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_RELATIONSHIPS_SIGNING_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_RELATIONSHIP_SIGN_USING_SELECTORS: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_RELATIONSHIP_SIGN_PART: OPC_RELATIONSHIPS_SIGNING_OPTION = OPC_RELATIONSHIPS_SIGNING_OPTION(1i32);
impl ::core::marker::Copy for OPC_RELATIONSHIPS_SIGNING_OPTION {}
impl ::core::clone::Clone for OPC_RELATIONSHIPS_SIGNING_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_RELATIONSHIPS_SIGNING_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_RELATIONSHIPS_SIGNING_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_RELATIONSHIPS_SIGNING_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_RELATIONSHIPS_SIGNING_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_RELATIONSHIP_SELECTOR(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_RELATIONSHIP_SELECT_BY_ID: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_RELATIONSHIP_SELECT_BY_TYPE: OPC_RELATIONSHIP_SELECTOR = OPC_RELATIONSHIP_SELECTOR(1i32);
impl ::core::marker::Copy for OPC_RELATIONSHIP_SELECTOR {}
impl ::core::clone::Clone for OPC_RELATIONSHIP_SELECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_RELATIONSHIP_SELECTOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_RELATIONSHIP_SELECTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_RELATIONSHIP_SELECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_RELATIONSHIP_SELECTOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_SIGNATURE_TIME_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_SECONDS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_MINUTES: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_DAYS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(3i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_MONTHS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(4i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_TIME_FORMAT_YEARS: OPC_SIGNATURE_TIME_FORMAT = OPC_SIGNATURE_TIME_FORMAT(5i32);
impl ::core::marker::Copy for OPC_SIGNATURE_TIME_FORMAT {}
impl ::core::clone::Clone for OPC_SIGNATURE_TIME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_SIGNATURE_TIME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_SIGNATURE_TIME_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_SIGNATURE_TIME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_SIGNATURE_TIME_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_SIGNATURE_VALIDATION_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_VALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_SIGNATURE_INVALID: OPC_SIGNATURE_VALIDATION_RESULT = OPC_SIGNATURE_VALIDATION_RESULT(-1i32);
impl ::core::marker::Copy for OPC_SIGNATURE_VALIDATION_RESULT {}
impl ::core::clone::Clone for OPC_SIGNATURE_VALIDATION_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_SIGNATURE_VALIDATION_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_SIGNATURE_VALIDATION_RESULT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_SIGNATURE_VALIDATION_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_SIGNATURE_VALIDATION_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_STREAM_IO_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_STREAM_IO_READ: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_STREAM_IO_WRITE: OPC_STREAM_IO_MODE = OPC_STREAM_IO_MODE(2i32);
impl ::core::marker::Copy for OPC_STREAM_IO_MODE {}
impl ::core::clone::Clone for OPC_STREAM_IO_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_STREAM_IO_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_STREAM_IO_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_STREAM_IO_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_STREAM_IO_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_URI_TARGET_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_URI_TARGET_MODE_INTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_URI_TARGET_MODE_EXTERNAL: OPC_URI_TARGET_MODE = OPC_URI_TARGET_MODE(1i32);
impl ::core::marker::Copy for OPC_URI_TARGET_MODE {}
impl ::core::clone::Clone for OPC_URI_TARGET_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_URI_TARGET_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_URI_TARGET_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_URI_TARGET_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_URI_TARGET_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPC_WRITE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_WRITE_DEFAULT: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
pub const OPC_WRITE_FORCE_ZIP32: OPC_WRITE_FLAGS = OPC_WRITE_FLAGS(1i32);
impl ::core::marker::Copy for OPC_WRITE_FLAGS {}
impl ::core::clone::Clone for OPC_WRITE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPC_WRITE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPC_WRITE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPC_WRITE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPC_WRITE_FLAGS").field(&self.0).finish()
    }
}
impl OPC_WRITE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for OPC_WRITE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPC_WRITE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPC_WRITE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPC_WRITE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPC_WRITE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
